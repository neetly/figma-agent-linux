#!/bin/bash
set -eo pipefail

XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"

echo ":: Figma Agent for Linux"
echo ":: ====================="

if systemctl --user is-enabled figma-agent.service > /dev/null; then
  echo ":: Disabling figma-agent.service"
  systemctl --user disable figma-agent.service
fi

echo ":: Downloading figma-agent"
TMPFILE="$(mktemp)"
curl --location "https://github.com/neetly/figma-agent-linux/releases/latest/download/figma-agent-$(uname -m)-unknown-linux-gnu" \
  --output "$TMPFILE" --fail
chmod +x "$TMPFILE"
mkdir -p "$XDG_DATA_HOME/figma-agent"
mv "$TMPFILE" "$XDG_DATA_HOME/figma-agent/figma-agent"

echo ":: Writing to figma-agent.service and figma-agent.socket"
mkdir -p "$XDG_DATA_HOME/systemd/user"
cat > "$XDG_DATA_HOME/systemd/user/figma-agent.service" << EOF
[Unit]
Description=Figma Agent for Linux
Wants=network-online.target
After=network-online.target

[Service]
Type=exec
ExecStart="$XDG_DATA_HOME/figma-agent/figma-agent"

[Install]
WantedBy=default.target
EOF
cat > "$XDG_DATA_HOME/systemd/user/figma-agent.socket" << EOF
[Unit]
Description=Figma Agent for Linux

[Socket]
ListenStream=44950

[Install]
WantedBy=sockets.target
EOF

echo ":: Enabling figma-agent.socket"
systemctl --user daemon-reload
systemctl --user enable --now figma-agent.socket

echo ":: Done"
