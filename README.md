# Figma Agent for Linux

[![CI](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml)

This service allows you to use your locally installed fonts on [figma.com](https://www.figma.com/).

## Features

- Variable fonts support
- Preview fonts in the font picker

## Installation

> [!IMPORTANT]
> To make this service work, you need to override the browser's user agent to a Windows one. See [this thread](https://forum.figma.com/report-a-problem-6/requests-to-font-helper-on-linux-stopped-working-16569) for more information.

```sh
bash -c "$(curl -fsSL https://raw.githubusercontent.com/neetly/figma-agent-linux/main/files/install.sh)"
```

> [!TIP]
> You can run the command again to update this service to the latest version.

### Package Managers

| Package Manager | Package                                                                                                                                                       |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Arch Linux      | [figma-agent-linux](https://aur.archlinux.org/packages/figma-agent-linux) / [figma-agent-linux-bin](https://aur.archlinux.org/packages/figma-agent-linux-bin) |
| Nix             | [figma-agent](https://search.nixos.org/packages?show=figma-agent)                                                                                             |

### Uninstallation

<details>

```sh
systemctl --user disable --now figma-agent.{service,socket}
rm -rf ~/.local/share/figma-agent ~/.local/share/systemd/user/figma-agent.{service,socket}
```

</details>

## Configuration

```jsonc
// ~/.config/figma-agent/config.json
{
  // Default: "127.0.0.1:44950"
  "bind": "127.0.0.1:44950",
  // Default: true
  "use_system_fonts": true,
  // Default: []
  "font_directories": ["~/Fonts"],
  // Default: true
  "enable_font_rescan": true,
  // Default: true
  "enable_font_preview": true,
}
```

> [!NOTE]
> You need to restart this service to apply the configuration changes.
>
> ```sh
> systemctl --user restart figma-agent.service
> ```

## Caveats

### Ad Blockers

Ad blockers may prevent websites from connecting to localhost for privacy concerns. Please disable the relevant rules or create an exception rule for [figma.com](https://www.figma.com/).

### Brave Browser

In Brave browser, websites require special permissions to access localhost. Please follow the instructions in [the documentation](https://brave.com/privacy-updates/27-localhost-permission/) to grant the permission to [figma.com](https://www.figma.com/).

## Credits

This project is inspired by [Figma Linux Font Helper](https://github.com/Figma-Linux/figma-linux-font-helper).
