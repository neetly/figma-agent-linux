use std::{fs, iter, path::Path};

use harfrust::{ShaperData, ShaperInstance, UnicodeBuffer};
use skrifa::{
    FontRef, GlyphId, MetadataProvider,
    instance::Size,
    outline::{DrawError, DrawSettings, OutlinePen},
};
use svg::{
    Document,
    node::element::{
        self,
        path::{Command, Position},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Failed to read font file")]
    Read(#[from] std::io::Error),
    #[error("Failed to parse font file")]
    Parse(#[from] read_fonts::ReadError),
    #[error("Failed to draw glyph")]
    Draw(#[from] DrawError),
}

#[derive(Debug, Clone)]
pub struct RenderOptions<'a> {
    pub font: (&'a Path, usize),
    pub size: f32,
    pub named_instance_index: Option<usize>,
}

// TODO: The official implementation seems to use some other methods to calculate
// the height of the generated image and the position of the text. Our logic based
// on baseline and ascent/descent may cause some fonts to not be vertically centered
// properly in the font picker. Needs further investigation.
pub fn render_text(
    text: impl AsRef<str>,
    RenderOptions {
        font: (font_path, font_index),
        size,
        named_instance_index,
    }: RenderOptions,
) -> Result<Option<String>, RenderError> {
    let data = fs::read(font_path)?;
    let font = FontRef::from_index(&data, font_index as u32)?;

    let size = Size::new(size);
    let location = named_instance_index
        .and_then(|index| font.named_instances().get(index))
        .map(|named_instance| named_instance.location())
        .unwrap_or_default();

    let metrics = font.metrics(size, &location);
    let charmap = font.charmap();
    let outlines = font.outline_glyphs();

    let text = text.as_ref();
    if text.chars().any(|char| charmap.map(char).is_none()) {
        return Ok(None);
    }

    let shaper_data = ShaperData::new(&font);
    let shaper_instance = named_instance_index
        .map(|index| ShaperInstance::from_named_instance(&font, index))
        .unwrap_or_default();
    let shaper = shaper_data
        .shaper(&font)
        .point_size(size.ppem())
        .instance(Some(&shaper_instance))
        .build();

    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.guess_segment_properties();

    let glyph_buffer = shaper.shape(buffer, &[]);

    let mut text_path = TextPath::new();

    let scale = size.linear_scale(metrics.units_per_em);
    let scale_unit = |unit: i32| unit as f32 * scale;

    let (mut cursor_x, mut cursor_y) = (0.0, metrics.ascent);

    for (info, position) in iter::zip(glyph_buffer.glyph_infos(), glyph_buffer.glyph_positions()) {
        let glyph = outlines
            .get(GlyphId::new(info.glyph_id))
            .ok_or_else(|| DrawError::GlyphNotFound(GlyphId::new(info.glyph_id)))?;

        text_path.origin_x = cursor_x + scale_unit(position.x_offset);
        text_path.origin_y = cursor_y + scale_unit(position.y_offset);

        glyph.draw(DrawSettings::unhinted(size, &location), &mut text_path)?;

        cursor_x += scale_unit(position.x_advance);
        cursor_y += scale_unit(position.y_advance);
    }

    let width = cursor_x;
    let height = metrics.ascent - metrics.descent;

    let document = Document::new()
        .set("width", width)
        .set("height", height)
        .set("viewBox", (0.0, 0.0, width, height))
        .add(element::Path::new().set("d", text_path.data));

    Ok(Some(document.to_string()))
}

#[derive(Debug, Clone, Default)]
pub struct TextPath {
    origin_x: f32,
    origin_y: f32,
    data: element::path::Data,
}

impl TextPath {
    pub fn new() -> Self {
        TextPath::default()
    }
}

// Because the Y-axis in text rendering is opposite to SVG, we need to invert the Y values.
impl OutlinePen for TextPath {
    fn move_to(&mut self, x: f32, y: f32) {
        self.data.append(Command::Move(
            Position::Absolute,
            (self.origin_x + x, self.origin_y - y).into(),
        ));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.data.append(Command::Line(
            Position::Absolute,
            (self.origin_x + x, self.origin_y - y).into(),
        ));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.data.append(Command::QuadraticCurve(
            Position::Absolute,
            (
                self.origin_x + x1,
                self.origin_y - y1,
                self.origin_x + x,
                self.origin_y - y,
            )
                .into(),
        ));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.data.append(Command::CubicCurve(
            Position::Absolute,
            (
                self.origin_x + x1,
                self.origin_y - y1,
                self.origin_x + x2,
                self.origin_y - y2,
                self.origin_x + x,
                self.origin_y - y,
            )
                .into(),
        ));
    }

    fn close(&mut self) {
        self.data.append(Command::Close);
    }
}
