# Figma Agent for Linux

[![CI](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml)

(a.k.a. Font Helper)

## Installation

```sh
bash -c "$(curl -fsSL https://raw.githubusercontent.com/neetly/figma-agent-linux/main/scripts/install.sh)"
```

### Arch Linux

```sh
paru -S figma-agent-linux
systemctl --user enable --now figma-agent-linux.service
```

## Features

- Variable fonts support.
- Performant thanks to [Fontconfig][].

## Comparison

|                            | Figma Agent for Linux | [Figma Linux Font Helper][] |
| -------------------------- | --------------------- | --------------------------- |
| [Variable fonts][]         | ✔️                    | ❌                          |
| [XDG Base Directory][]     | ✔️                    | ❌ ([#14][])                |
| [Fontconfig][] integration | ✔️                    | ❌                          |
| Run as a non-root user     | ✔️                    | ❌ ([#14][])                |

[figma linux font helper]: https://github.com/Figma-Linux/figma-linux-font-helper
[variable fonts]: https://www.figma.com/typography/variable-fonts/
[xdg base directory]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
[fontconfig]: https://www.freedesktop.org/wiki/Software/fontconfig/
[#14]: https://github.com/Figma-Linux/figma-linux-font-helper/issues/14
