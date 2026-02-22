use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use interp::{InterpMode, interp};
use skrifa::{MetadataProvider, string::StringId};

#[derive(Debug, thiserror::Error)]
pub enum FontError {
    #[error("Failed to read font file")]
    Read(#[from] std::io::Error),
    #[error("Failed to parse font file")]
    Parse(Vec<(usize, read_fonts::ReadError)>, Option<FontFile>),
}

#[derive(Debug, Clone)]
pub struct FontFile {
    pub path: PathBuf,
    pub fonts: Vec<Font>,
    pub modified_at: Option<SystemTime>,
}

impl FontFile {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, FontError> {
        let path = path.as_ref();

        let data = fs::read(path)?;
        let metadata = fs::metadata(path)?;

        let mut errors = Vec::new();
        let fonts = skrifa::FontRef::fonts(&data)
            .enumerate()
            .filter_map(|(index, font)| match font {
                Ok(font) => Some(Font::from_skrifa(&font, index)),
                Err(error) => {
                    errors.push((index, error));
                    None
                }
            })
            .collect();

        let font_file = FontFile {
            path: path.into(),
            fonts,
            modified_at: metadata.modified().ok(),
        };

        if errors.is_empty() {
            Ok(font_file)
        } else {
            Err(FontError::Parse(errors, Some(font_file)))
        }
    }
}

#[derive(Debug, Clone)]
pub struct FontQuery<'a> {
    pub family_name: Option<&'a str>,
    pub subfamily_name: Option<&'a str>,
    pub postscript_name: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct FontQueryResult<'a> {
    pub font: &'a Font,
    pub named_instance: Option<&'a NamedInstance>,
}

impl<'a> FontFile {
    pub fn query(&'a self, query: FontQuery<'_>) -> Option<FontQueryResult<'a>> {
        fn matches(value: &Option<impl AsRef<str>>, query: &Option<impl AsRef<str>>) -> bool {
            match (value, query) {
                (Some(value), Some(query)) => value.as_ref() == query.as_ref(),
                (None, Some(_)) => false,
                (_, None) => true,
            }
        }

        self.fonts.iter().find_map(|font| {
            if matches(&font.family_name, &query.family_name) {
                if matches(&font.subfamily_name, &query.subfamily_name)
                    && matches(&font.postscript_name, &query.postscript_name)
                {
                    Some(FontQueryResult {
                        font,
                        named_instance: None,
                    })
                } else {
                    font.named_instances.iter().find_map(|named_instance| {
                        if matches(&named_instance.subfamily_name, &query.subfamily_name)
                            && matches(&named_instance.postscript_name, &query.postscript_name)
                        {
                            Some(FontQueryResult {
                                font,
                                named_instance: Some(named_instance),
                            })
                        } else {
                            None
                        }
                    })
                }
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Font {
    pub index: usize,
    pub family_name: Option<String>,
    pub subfamily_name: Option<String>,
    pub postscript_name: Option<String>,
    pub weight: f32,
    pub width: f32,
    pub is_italic: bool,
    pub is_oblique: bool,
    pub axes: Vec<Axis>,
    pub named_instances: Vec<NamedInstance>,
}

impl Font {
    pub fn from_skrifa(font: &skrifa::FontRef, index: usize) -> Self {
        let attributes = font.attributes();

        Font {
            index,
            family_name: font
                .string(StringId::TYPOGRAPHIC_FAMILY_NAME)
                .or_else(|| font.string(StringId::FAMILY_NAME)),
            subfamily_name: font
                .string(StringId::TYPOGRAPHIC_SUBFAMILY_NAME)
                .or_else(|| font.string(StringId::SUBFAMILY_NAME)),
            postscript_name: font.string(StringId::POSTSCRIPT_NAME),
            weight: attributes.weight.value(),
            width: attributes.stretch.percentage(),
            is_italic: matches!(attributes.style, skrifa::attribute::Style::Italic),
            is_oblique: matches!(attributes.style, skrifa::attribute::Style::Oblique(_)),
            axes: font
                .axes()
                .iter()
                .enumerate()
                .map(|(index, axis)| Axis::from_skrifa(font, &axis, index))
                .collect(),
            named_instances: font
                .named_instances()
                .iter()
                .enumerate()
                .map(|(index, named_instance)| {
                    NamedInstance::from_skrifa(font, &named_instance, index)
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Axis {
    pub index: usize,
    pub tag: String,
    pub name: Option<String>,
    pub min_value: f32,
    pub max_value: f32,
    pub default_value: f32,
    pub is_hidden: bool,
}

impl Axis {
    pub fn from_skrifa(font: &skrifa::FontRef, axis: &skrifa::Axis, index: usize) -> Self {
        Axis {
            index,
            tag: axis.tag().to_string(),
            name: font.string(axis.name_id()),
            min_value: axis.min_value(),
            max_value: axis.max_value(),
            default_value: axis.default_value(),
            is_hidden: axis.is_hidden(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NamedInstance {
    pub index: usize,
    pub subfamily_name: Option<String>,
    pub postscript_name: Option<String>,
    pub coordinates: Vec<f32>,
}

impl NamedInstance {
    pub fn from_skrifa(
        font: &skrifa::FontRef,
        named_instance: &skrifa::NamedInstance,
        index: usize,
    ) -> Self {
        NamedInstance {
            index,
            subfamily_name: font.string(named_instance.subfamily_name_id()),
            postscript_name: named_instance
                .postscript_name_id()
                .and_then(|id| font.string(id))
                .or_else(|| {
                    // https://adobe-type-tools.github.io/font-tech-notes/pdfs/5902.AdobePSNameGeneration.pdf
                    font.string(StringId::VARIATIONS_POSTSCRIPT_NAME_PREFIX)
                        .or_else(|| {
                            font.string(StringId::TYPOGRAPHIC_FAMILY_NAME)
                                .map(|family_name| family_name.postscript())
                        })
                        .zip(font.string(named_instance.subfamily_name_id()))
                        .map(|(postscript_family_prefix, subfamily)| {
                            format!("{}-{}", postscript_family_prefix, subfamily.postscript())
                        })
                }),
            coordinates: named_instance.user_coords().collect(),
        }
    }
}

pub trait StringExt {
    fn postscript(&self) -> String;
}

impl StringExt for String {
    fn postscript(&self) -> String {
        self.chars()
            .filter(|char| char.is_ascii_alphanumeric())
            .collect()
    }
}

pub trait SkrifaFontRefExt {
    fn string(&self, id: StringId) -> Option<String>;
}

impl SkrifaFontRefExt for skrifa::FontRef<'_> {
    fn string(&self, id: StringId) -> Option<String> {
        self.localized_strings(id)
            .english_or_first()
            .map(|localized_string| localized_string.to_string())
    }
}

/// Convert weight axis (wght) to OS/2 usWeightClass.
///
/// https://learn.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass
pub fn to_us_weight_class(weight: f32) -> u16 {
    weight.round() as u16
}

/// Convert width axis (wdth) to OS/2 usWidthClass.
///
/// https://learn.microsoft.com/en-us/typography/opentype/spec/os2#uswidthclass
pub fn to_us_width_class(width: f32) -> u16 {
    static WIDTH_VALUES: [f32; 9] = [50.0, 62.5, 75.0, 87.5, 100.0, 112.5, 125.0, 150.0, 200.0];
    static US_WIDTH_CLASS_VALUES: [f32; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

    interp(
        &WIDTH_VALUES,
        &US_WIDTH_CLASS_VALUES,
        width,
        &InterpMode::FirstLast,
    )
    .round() as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_us_width_class() {
        assert_eq!(to_us_width_class(50.0), 1);
        assert_eq!(to_us_width_class(62.5), 2);
        assert_eq!(to_us_width_class(75.0), 3);
        assert_eq!(to_us_width_class(87.5), 4);
        assert_eq!(to_us_width_class(100.0), 5);
        assert_eq!(to_us_width_class(112.5), 6);
        assert_eq!(to_us_width_class(125.0), 7);
        assert_eq!(to_us_width_class(150.0), 8);
        assert_eq!(to_us_width_class(200.0), 9);
    }
}
