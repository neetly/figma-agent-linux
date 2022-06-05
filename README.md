# Figma Agent for Linux <small>(a.k.a. Font Helper)</small>

![CI](https://github.com/neetly/figma-agent-linux/workflows/CI/badge.svg)

## Features

- Variable fonts support.
- Performant thanks to [Fontconfig][].

## Comparison

|                                    | Figma Agent for Linux | [Figma Linux Font Helper][] |
| ---------------------------------- | --------------------- | --------------------------- |
| [Variable fonts][] support         | ✔️                    | ❌                          |
| [XDG Base Directory][] integration | ✔️                    | ❌ ([#14][])                |
| [Fontconfig][] integration         | ✔️                    | ❌                          |
| Run as a non-root user             | ✔️                    | ❌ ([#14][])                |

[figma linux font helper]: https://github.com/Figma-Linux/figma-linux-font-helper
[variable fonts]: https://www.figma.com/typography/variable-fonts/
[xdg base directory]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
[fontconfig]: https://www.freedesktop.org/wiki/Software/fontconfig/
[#14]: https://github.com/Figma-Linux/figma-linux-font-helper/issues/14
