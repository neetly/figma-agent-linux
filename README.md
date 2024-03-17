# Figma Agent for Linux

[![CI](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml)

(a.k.a. Font Helper)

## Installation

> [!IMPORTANT]  
> Due to changes to the Figma website, we must override the browser's user agent
> in order for this service to function. You might use a browser extension to do
> this. Choosing a Windows/macOS user agent is sufficient.
>
> Please vote up this thread on Figma's official forum.  
> https://forum.figma.com/t/requests-to-font-helper-on-linux-stopped-working/61389

```sh
bash -c "$(curl -fsSL https://raw.githubusercontent.com/neetly/figma-agent-linux/main/scripts/install.sh)"
```

### Arch Linux

```sh
paru -S --needed figma-agent-linux-bin # or figma-agent-linux, if you prefer to compile it youself
systemctl --user enable --now figma-agent.socket
```

### Uninstallation

<details>

```sh
systemctl --user disable --now figma-agent.{service,socket}
rm -rf ~/.local/share/figma-agent
rm -rf ~/.local/share/systemd/user/figma-agent.{service,socket}
```

</details>

## Features

- Support [Variable Fonts][]
- High Performance (Thanks to [Fontconfig][])

## Comparisons

|                            | Figma Agent for Linux | [Figma Linux Font Helper][] |
| -------------------------- | --------------------- | --------------------------- |
| [Variable Fonts][]         | ✔️                    | ❌                          |
| Font Preview               | ❌                    | ❌                          |
| [XDG Base Directory][]     | ✔️                    | ✔️                          |
| [Fontconfig][] Integration | ✔️                    | ❌                          |
| Run as a non-root user     | ✔️                    | ✔️                          |

## Alternatives

Another option is to use [Wine][] to run the official `figma_agent.exe` binary.
We use this method for reverse engineering throughout development.

## Troubleshooting

### Ad Blocker

Please check to see if any rules block figma.com from accessing localhost, such
as [Block Outsider Intrusion into LAN].

### Brave Browser

Please grant figma.com permission to access localhost.  
https://brave.com/privacy-updates/27-localhost-permission/

## Credit

This project was inspired by [Figma Linux Font Helper][].

[Variable Fonts]: https://www.figma.com/typography/variable-fonts/
[Fontconfig]: https://www.freedesktop.org/wiki/Software/fontconfig/
[XDG Base Directory]:
  https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
[Figma Linux Font Helper]:
  https://github.com/Figma-Linux/figma-linux-font-helper
[Wine]: https://www.winehq.org/
[Block Outsider Intrusion into LAN]:
  https://github.com/uBlockOrigin/uAssets/blob/master/filters/lan-block.txt
