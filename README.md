# Figma Agent for Linux

[![CI](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/neetly/figma-agent-linux/actions/workflows/ci.yml)

Use the fonts installed on your Linux computer in [Figma](https://www.figma.com/).
Figma Agent runs in the background and makes them available in your browser's
Figma font picker. It supports system fonts, custom font folders, and variable
fonts, including named instances. New and updated fonts are picked up automatically.

## Install

The installer needs `bash`, `curl`, and a running systemd user session. Run it as
your regular user:

```sh
bash -c "$(curl -fsSL https://raw.githubusercontent.com/neetly/figma-agent-linux/main/files/install.sh)"
```

This downloads the latest release and sets it up to start when Figma connects.
Run the same command again to update.

If you prefer a package manager, these packages are available:

| Platform   | Package                                                                                                                                                                                   |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Arch Linux | [figma-agent-linux](https://aur.archlinux.org/packages/figma-agent-linux)<sup>AUR</sup> / [figma-agent-linux-bin](https://aur.archlinux.org/packages/figma-agent-linux-bin)<sup>AUR</sup> |
| Nix        | [figma-agent](https://search.nixos.org/packages?show=figma-agent) (community-maintained)                                                                                                  |

Follow your package's instructions to start the service, then connect it to Figma
using the steps below.

## Connect Figma to your local fonts

Once Figma Agent is running, give Figma permission to connect to it. You do not
need to change your browser's user agent.

1. Open a design file on [figma.com](https://www.figma.com/).
2. Click the **Figma logo** in the top-left corner, then choose
   **Preferences → Permissions and helpers…**.
3. Under **Apps on device**, click **Show me how to connect** if the status is
   **Not connected**.
4. Click **Connect** in the instructions that appear. The message mentions
   Figma's font installer or desktop app; with Figma Agent running, you can go
   straight to **Connect**.
5. If your browser asks to **Access other apps and services on this device**,
   click **Allow**. The wording may differ between browsers.
6. Check that **Apps on device** now shows **Connected**, then close the dialog
   and try your local fonts in the font picker.

**Clipboard access** and **Microphone access** are separate permissions. They can
stay **Blocked** when using local fonts.

If the connection fails or a font is missing, see [Troubleshooting](#troubleshooting).

## Configuration

No configuration is needed to use your system fonts. To change the defaults,
create `~/.config/figma-agent/config.json` (and its parent directory if needed).
All settings are optional.

For example, to include fonts from `~/Fonts` alongside your system fonts:

```json
{
  "font_directories": ["~/Fonts"]
}
```

Restart the service after changing the configuration:

```sh
systemctl --user restart figma-agent.service
```

| Setting               | Default             | What it does                                                                                          |
| --------------------- | ------------------- | ----------------------------------------------------------------------------------------------------- |
| `font_directories`    | `[]`                | Additional font folders to scan. Use `~` for your home directory.                                     |
| `use_system_fonts`    | `true`              | Include system fonts found through Fontconfig.                                                        |
| `enable_font_rescan`  | `true`              | Pick up newly installed or updated fonts automatically.                                               |
| `enable_font_preview` | `true`              | Show font previews in Figma's font picker. This feature is experimental.                              |
| `bind`                | `"127.0.0.1:44950"` | Address and port to listen on. Ignored when using socket activation, as the automatic installer does. |

Font folders can contain symlinks to files or other folders, including fonts
outside the configured folders. Unreadable fonts are skipped; readable fonts in
the same collection are still available.

## Troubleshooting

### Figma does not connect

Open [Permissions and helpers](#connect-figma-to-your-local-fonts) and check
**Apps on device**. If you previously blocked access, open your browser's site
permissions for figma.com, allow access to apps on your device, and try
**Connect** again. The browser's permission label may vary.

If permission is allowed, check whether Figma Agent is running:

```sh
systemctl --user status figma-agent.{service,socket}
```

With the automatic installer, the socket starts at login and launches the service
when Figma connects. An inactive service is normal before the first connection,
as long as the socket is active. If the socket is inactive, start it:

```sh
systemctl --user start figma-agent.socket
```

If it still won't connect, check whether an ad blocker or privacy extension is
blocking connections to `localhost` or `127.0.0.1`. Add an exception for figma.com
if needed.

You can also restart the service and check its logs for errors:

```sh
systemctl --user restart figma-agent.service
journalctl --user --unit figma-agent.service --follow
```

### A font is missing

Check that the font is installed on your system or is in a folder listed in
`font_directories`. See [Configuration](#configuration) to add a folder.

If you turned off `enable_font_rescan`, restart the service after installing or
updating fonts. You also need to restart after changing the configuration:

```sh
systemctl --user restart figma-agent.service
```

### Switching fonts is slow

If you have a large font collection, try setting `enable_font_rescan` to `false`
in your configuration and restarting the service. Fonts will then be scanned
only at startup, so you'll need to restart the service whenever you install or
update fonts.

### Font previews cause problems

Font previews are experimental. Set `enable_font_preview` to `false` in your
configuration and restart the service to turn them off.

## Uninstall

If you used the automatic installer, stop Figma Agent and remove its installed
files:

```sh
systemctl --user disable --now figma-agent.{service,socket}
rm -rf "${XDG_DATA_HOME:-$HOME/.local/share}/figma-agent" \
  "${XDG_DATA_HOME:-$HOME/.local/share}/systemd/user/figma-agent.service" \
  "${XDG_DATA_HOME:-$HOME/.local/share}/systemd/user/figma-agent.socket"
systemctl --user daemon-reload
```

This leaves your configuration file in place. If you installed through a package
manager, use that package manager to remove it instead.

## Credits

Inspired by [Figma Linux Font Helper](https://github.com/Figma-Linux/figma-linux-font-helper).
