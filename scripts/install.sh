#!/bin/bash
set -eo pipefail

XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"

echo ":: Figma Agent for Linux"
echo ":: ====================="

if systemctl --user is-active figma-agent.service > /dev/null; then
  echo ":: Stopping figma-agent.service"
  systemctl --user stop figma-agent.service
fi

echo ":: Downloading figma-agent"
mkdir -p "$XDG_DATA_HOME/figma-agent"
curl --location https://github.com/neetly/figma-agent-linux/releases/latest/download/figma-agent-x86_64-unknown-linux-gnu \
  --output "$XDG_DATA_HOME/figma-agent/figma-agent" --fail
chmod +x "$XDG_DATA_HOME/figma-agent/figma-agent"

echo ":: Writing to figma-agent.service"
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

echo ":: Enabling figma-agent.service"
systemctl --user daemon-reload
systemctl --user enable --now figma-agent.service

echo ":: Done"
