# Figma Agent for Linux

[![CI](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml)

A lightweight local service that makes your locally installed fonts available on [figma.com](https://www.figma.com/).

## Features

- **System font integration** — Automatically discovers your installed system fonts.
- **Custom font directories** — Add your own font directories alongside system fonts.
- **Variable fonts** — Full support for variable fonts, including named instances.
- **Font preview** — Preview fonts directly in the Figma font picker.
- **Automatic rescanning** — Detects newly installed or updated fonts without restarting the service.

## Prerequisites

> [!IMPORTANT]
> For this service to work, you **must** override your browser's user agent string to a Windows one. Without this, Figma will not attempt to connect to the local font helper.
>
> See [this thread](https://forum.figma.com/report-a-problem-6/requests-to-font-helper-on-linux-stopped-working-16569) for details and instructions.

## Installation

Run the following command to download the latest release and set it up automatically:

```sh
bash -c "$(curl -fsSL https://raw.githubusercontent.com/neetly/figma-agent-linux/main/files/install.sh)"
```

> [!TIP]
> You can run the same command again at any time to update to the latest version.

### Package Managers

| Platform   | Package                                                                                                                                                       |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Arch Linux | [figma-agent-linux](https://aur.archlinux.org/packages/figma-agent-linux) / [figma-agent-linux-bin](https://aur.archlinux.org/packages/figma-agent-linux-bin) |
| Nix        | [figma-agent](https://search.nixos.org/packages?show=figma-agent) (community-maintained)                                                                      |

### Uninstallation

<details>
<summary>Click to expand</summary>

```sh
systemctl --user disable --now figma-agent.{service,socket}
rm -rf ~/.local/share/figma-agent ~/.local/share/systemd/user/figma-agent.{service,socket}
systemctl --user daemon-reload
```

</details>

## Configuration

The configuration file is located at `~/.config/figma-agent/config.json`. All fields are optional — the service works out of the box without any configuration.

| Key                   | Default             | Description                                                      |
| --------------------- | ------------------- | ---------------------------------------------------------------- |
| `bind`                | `"127.0.0.1:44950"` | Address and port to listen on.                                   |
| `use_system_fonts`    | `true`              | Include system fonts.                                            |
| `font_directories`    | `[]`                | Additional directories to scan for fonts. Supports `~` for home. |
| `enable_font_rescan`  | `true`              | Automatically pick up newly installed or updated fonts.          |
| `enable_font_preview` | `true`              | Enable font previews in the Figma font picker.                   |

Example:

```jsonc
// ~/.config/figma-agent/config.json
{
  "font_directories": ["~/Fonts"],
}
```

> [!TIP]
> If you have a large number of fonts installed and notice slowness when switching fonts in Figma, try setting `enable_font_rescan` to `false`. The service will then only scan fonts once at startup; restart it manually after installing new fonts.

> [!WARNING]
> Font preview is currently experimental and may cause unexpected issues. If you experience problems, set `enable_font_preview` to `false`.

> [!NOTE]
> You must restart the service for configuration changes to take effect:
>
> ```sh
> systemctl --user restart figma-agent.service
> ```

## Troubleshooting

Check whether the service is running:

```sh
systemctl --user status figma-agent.{service,socket}
```

View logs:

```sh
journalctl --user --unit figma-agent.service --follow
```

### Chromium-Based Browsers (Chrome, Brave, Edge, etc.)

Chromium-based browsers are rolling out [Local Network Access](https://developer.chrome.com/blog/local-network-access) restrictions that require websites to request permission before connecting to local network devices or apps on your device (localhost). Since Figma Agent listens on localhost, you need to grant [figma.com](https://www.figma.com/) the **Apps on device** permission when prompted.

If you previously dismissed the prompt, you can manage the permission via **Site Settings** for [figma.com](https://www.figma.com/).

> [!NOTE]
> Brave has shipped its own localhost permission independently and may label it differently. See [the Brave documentation](https://brave.com/privacy-updates/27-localhost-permission/) for details.

### Firefox

Firefox is progressively rolling out [local network access permissions](https://support.mozilla.org/en-US/kb/control-personal-device-local-network-permissions-firefox) that require websites to request permission before connecting to apps on your device (localhost) or local network devices. When [figma.com](https://www.figma.com/) tries to connect to Figma Agent, Firefox will prompt you for the **Device apps and services** permission. You need to allow it for font loading to work.

If you dismissed the prompt, you can change the permission in **Settings → Privacy & Security → Permissions → Device apps and services**.

> [!NOTE]
> This feature is currently being rolled out progressively. In standard Firefox releases, only users with Enhanced Tracking Protection set to **Strict** are included. It is available by default in Beta and Nightly builds.

### Ad Blockers

Some ad blockers and privacy extensions block websites from connecting to localhost, which prevents Figma from communicating with the local font service. If your fonts are not showing up despite the service running, your ad blocker is a likely cause.

- **uBlock Origin** — By default, the filter list [uBlock filters – Privacy](https://github.com/uBlockOrigin/uAssets/blob/master/filters/privacy.txt) blocks requests to `localhost` and `127.0.0.1`. To fix this, add the following to **My filters**:

  ```
  @@||127.0.0.1^$domain=figma.com
  @@||localhost^$domain=figma.com
  ```

- **AdGuard** — AdGuard may similarly block localhost requests. Add [figma.com](https://www.figma.com/) to your allowlist, or add equivalent exception rules in your user rules.

If you use a different ad blocker or privacy extension, check whether it has rules that block connections to `localhost` or `127.0.0.1` and add an exception for [figma.com](https://www.figma.com/).

## Credits

This project is inspired by [Figma Linux Font Helper](https://github.com/Figma-Linux/figma-linux-font-helper).
