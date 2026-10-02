# Changelog

This changelog starts with 0.4.4 and covers changes since 0.4.3.

<a name="0.4.4"></a>

## 0.4.4

### Fixed

- Follow file and directory symlinks when scanning fonts, including links to
  fonts outside configured directories.
- Avoid repeatedly scanning the same directory through overlapping roots or
  symlinks.
- Ignore files without readable fonts while keeping readable fonts from
  partially unreadable collections.
- Detect replaced or modified fonts even when their modification time has not
  increased, and remove stale cached entries when a font can no longer be loaded.
- Discover configured font directories created after the service starts during
  automatic rescans.
- Correct vertical glyph offsets and advances in font previews.

### Changed

- Update Rust and dependencies, including the font parsing and shaping libraries.
- Clarify installation, configuration, and troubleshooting documentation,
  including browser local network permissions and ad blockers.
- Add contribution and AI assistance guidelines.

[Full comparison](https://github.com/neetly/figma-agent-linux/compare/0.4.3...0.4.4)
