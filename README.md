# Figma Agent for Linux

[![CI](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml)

(a.k.a. Font Helper)

## Installation

> [!IMPORTANT]
> To make this service work, you need to override the browser's user agent to a Windows one. See [this thread](https://forum.figma.com/report-a-problem-6/requests-to-font-helper-on-linux-stopped-working-16569) for more information.

```sh
bash -c "$(curl -fsSL https://raw.githubusercontent.com/neetly/figma-agent-linux/main/files/install.sh)"
```

### Package Managers

| Package Manager | Package                                                                                                                                                       |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Arch Linux      | [figma-agent-linux](https://aur.archlinux.org/packages/figma-agent-linux) / [figma-agent-linux-bin](https://aur.archlinux.org/packages/figma-agent-linux-bin) |
| Nix             | [figma-agent](https://search.nixos.org/packages?show=figma-agent)                                                                                             |

### Uninstallation

<details>

```sh
systemctl --user disable --now figma-agent.service figma-agent.socket
rm -rf ~/.local/share/figma-agent ~/.local/share/systemd/user/figma-agent.{service,socket}
```

</details>

## Configuration

```jsonc
// ~/.config/figma-agent/config.json
{
  // Default: "localhost:44950"
  "bind": "localhost:44950",
  // Default: true
  "use_system_fonts": true,
  // Default: []
  "font_directories": ["~/Fonts"],
}
```

## Caveats

### Ad Blockers

Ad blockers may prevent websites from connecting to localhost for privacy concerns. Please disable the relevant rules or create an exception rule for [figma.com](https://www.figma.com/).

### Brave Browser

In Brave browser, websites require special permissions to access localhost. Please follow the instructions in [the documentation](https://brave.com/privacy-updates/27-localhost-permission/) to grant this permission to [figma.com](https://www.figma.com/).

## Credits

This project is inspired by [Figma Linux Font Helper](https://github.com/Figma-Linux/figma-linux-font-helper).
