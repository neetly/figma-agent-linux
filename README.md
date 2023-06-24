# Figma Agent for Linux

[![CI](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml)

(a.k.a. Font Helper)

[![Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/tokenize)

## Installation

```sh
bash -c "$(curl -fsSL https://raw.githubusercontent.com/neetly/figma-agent-linux/main/scripts/install.sh)"
```

### Arch Linux

```sh
paru -S --needed figma-agent-linux
systemctl --user enable --now figma-agent.socket
```

<details>
<summary><h3>Uninstallation</h3></summary>

```sh
systemctl --user disable --now figma-agent.{service,socket}
rm -rf ~/.local/share/figma-agent
rm -rf ~/.local/share/systemd/user/figma-agent.{service,socket}
```

</details>

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
