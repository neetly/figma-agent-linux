#!/bin/bash
set -euo pipefail

XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"

echo ":: Figma Agent for Linux"
echo

echo ":: Downloading figma-agent..."
TMPFILE="$(mktemp)"
curl --fail --location "https://github.com/neetly/figma-agent-linux/releases/latest/download/figma-agent-$(uname -m)-unknown-linux-gnu" \
  --output "$TMPFILE"
chmod +x "$TMPFILE"
mkdir -p "$XDG_DATA_HOME/figma-agent"
mv "$TMPFILE" "$XDG_DATA_HOME/figma-agent/figma-agent"
echo

echo ":: Creating figma-agent.service and figma-agent.socket..."
mkdir -p "$XDG_DATA_HOME/systemd/user"
cat > "$XDG_DATA_HOME/systemd/user/figma-agent.service" << EOF
[Unit]
Description=Figma Agent for Linux Service
Requires=figma-agent.socket

[Service]
Type=exec
ExecStart="$XDG_DATA_HOME/figma-agent/figma-agent"

[Install]
WantedBy=default.target
EOF
cat > "$XDG_DATA_HOME/systemd/user/figma-agent.socket" << EOF
[Unit]
Description=Figma Agent for Linux Socket

[Socket]
ListenStream=127.0.0.1:44950

[Install]
WantedBy=sockets.target
EOF
echo

echo ":: Enabling figma-agent.socket..."
systemctl --user daemon-reload
systemctl --user enable --now figma-agent.socket
echo

if systemctl --user is-active figma-agent.service > /dev/null; then
  echo ":: Restarting figma-agent.service..."
  systemctl --user restart figma-agent.service
  echo
fi

echo ":: Done"
