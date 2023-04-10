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
systemctl --user enable --now figma-agent.service
```

## Features

- Support [Variable Fonts][]
- High Performance (Thanks to [Fontconfig][])

## Comparison

|                            | Figma Agent for Linux | [Figma Linux Font Helper][] |
| -------------------------- | --------------------- | --------------------------- |
| [Variable Fonts][]         | ✔️                    | ❌                          |
| [XDG Base Directory][]     | ✔️                    | ✔️                          |
| [Fontconfig][] Integration | ✔️                    | ❌                          |
| Run as a non-root user     | ✔️                    | ✔️                          |

## Credit

This project was inspired by [Figma Linux Font Helper][].

[variable fonts]: https://www.figma.com/typography/variable-fonts/
[fontconfig]: https://www.freedesktop.org/wiki/Software/fontconfig/
[xdg base directory]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
[figma linux font helper]: https://github.com/Figma-Linux/figma-linux-font-helper
[#14]: https://github.com/Figma-Linux/figma-linux-font-helper/issues/14
