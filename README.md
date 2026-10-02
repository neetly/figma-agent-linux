# Figma Agent for Linux

[![CI](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml)

A lightweight local service that makes your locally installed fonts available on [figma.com](https://www.figma.com/) in your Linux browser.

Install the service, allow Figma to connect to apps on your device, and use your local fonts in the Figma font picker.

## Features

- **System font integration** — Automatically discovers your installed system fonts.
- **Custom font directories** — Add your own font directories alongside system fonts.
- **Variable fonts** — Full support for variable fonts, including named instances.
- **Font preview** — Preview fonts directly in the Figma font picker.
- **Automatic rescanning** — Detects newly installed or updated fonts without restarting the service.

## Installation

The automatic installer requires `bash`, `curl`, and a running systemd user session. Run the following command as your regular user to download the latest release and enable the service:

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

Follow your package's instructions to enable and start the service, then connect Figma as described below.

### Connect Figma to your local fonts

Figma works on Linux without changing your browser's user agent. It needs permission to connect to apps on your device to access Figma Agent. Figma provides its own permission guidance, but it may not prompt you automatically on first use.

After installing the service:

1. Open a design file on [figma.com](https://www.figma.com/).
2. Click the **Figma logo (menu) → Preferences → Permissions and helpers...**.
3. In the **Permissions and helpers** dialog, find the **Apps on device** section.
4. If permission has not been granted, you may see **Show me how to connect**. Click it to reveal the **Connect** button and instructions, then click **Connect** and allow access when your browser asks.
5. Check that the section shows **Connected**. This confirms that permission is granted and Figma is connected to the service.

Your installed fonts should now be available in the Figma font picker. If you cannot connect or your fonts are missing, see [Troubleshooting](#troubleshooting).

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

| Key                   | Default             | Description                                                                |
| --------------------- | ------------------- | -------------------------------------------------------------------------- |
| `bind`                | `"127.0.0.1:44950"` | Address and port to listen on. Has no effect when using socket activation. |
| `use_system_fonts`    | `true`              | Include fonts discovered via Fontconfig.                                   |
| `font_directories`    | `[]`                | Additional directories to scan for fonts. Supports `~` for home.           |
| `enable_font_rescan`  | `true`              | Automatically pick up newly installed or updated fonts.                    |
| `enable_font_preview` | `true`              | Enable font previews in the Figma font picker.                             |

Font scanning supports file and directory symlinks, including links to fonts outside
the configured directories. Files without readable fonts are ignored; collections
with some unreadable fonts retain their readable fonts.

To add a custom font directory, create `~/.config/figma-agent/config.json` with the following content, replacing `~/Fonts` with your font directory. Create the parent directory if it does not exist.

```jsonc
// ~/.config/figma-agent/config.json
{
  "font_directories": ["~/Fonts"],
}
```

> [!NOTE]
> You must restart the service for configuration changes to take effect:
>
> ```sh
> systemctl --user restart figma-agent.service
> ```

> [!TIP]
> If you have a large number of fonts installed and notice slowness when switching fonts in Figma, try setting `enable_font_rescan` to `false`. The service will then only scan fonts once at startup; restart it manually after installing new fonts.

> [!WARNING]
> Font preview is currently experimental and may cause unexpected issues. If you experience problems, set `enable_font_preview` to `false`.

## Troubleshooting

### Figma does not connect

First, check **Apps on device** in Figma's [Permissions and helpers dialog](#connect-figma-to-your-local-fonts). If access was previously denied, open your browser's site permissions for [figma.com](https://www.figma.com/), allow access to apps on your device, and try connecting again. The browser's permission label may vary.

Check the service and socket status:

```sh
systemctl --user status figma-agent.{service,socket}
```

With the automatic installer, the socket starts at login and launches the service when Figma connects. An inactive service before the first connection is normal if the socket is active. If the socket is inactive, start it:

```sh
systemctl --user start figma-agent.socket
```

Restart the service:

```sh
systemctl --user restart figma-agent.service
```

View logs:

```sh
journalctl --user --unit figma-agent.service --follow
```

Some ad blockers and privacy extensions block connections to `localhost` or `127.0.0.1`. If Figma still cannot connect while the socket is active, check your extension's rules and add an exception for [figma.com](https://www.figma.com/) if needed.

### Figma connects, but a font is missing

Check that the font is installed on your system or stored in a directory listed in `font_directories`. See [Configuration](#configuration) to add a custom directory.

If you disabled `enable_font_rescan`, restart the service after installing or updating fonts. Configuration changes also require a service restart.

## Credits

This project is inspired by [Figma Linux Font Helper](https://github.com/Figma-Linux/figma-linux-font-helper).
