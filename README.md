# zellij-attention

Know which Zellij tab needs your attention — without checking each one.

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a>
</p>

A standalone Zellij WASM plugin that adds notification icons directly to tab names. Works with both the default Zellij tab bar and [zjstatus](https://github.com/dj95/zjstatus). When an external process (like Claude Code) needs your attention, the tab is renamed with an indicator — e.g., `terminal` becomes `terminal 🔴`. Focusing the pane clears the notification automatically.

https://github.com/user-attachments/assets/646effc0-1c24-413d-bef3-3d85591cd89b

## Features

- **Tab-level notifications** — icons appended to tab names, visible at a glance
- **Auto-clear on focus** — switch to the pane and the notification disappears
- **Three notification states** — 🔴 waiting (needs input), 🟡 working, and 🟢 completed
- **Memory-only state** — lightweight, no disk I/O; stale icons cleaned up automatically on restart
- **Configurable icons** — use any character or emoji as notification indicator
- **Hidden background pane** — suppresses itself even when launched or reloaded manually
- **Standalone plugin** — works independently, no zjstatus or other status bar plugins needed

## Installation

Download the plugin and add it to your Zellij config:

```bash
mkdir -p ~/.config/zellij/plugins
curl -L https://github.com/KiryuuLight/zellij-attention/releases/latest/download/zellij-attention.wasm \
  -o ~/.config/zellij/plugins/zellij-attention.wasm
```

Add to `~/.config/zellij/config.kdl`:

```kdl
load_plugins {
    "file:~/.config/zellij/plugins/zellij-attention.wasm" {
        // All options are optional — defaults shown
        enabled "true"
        waiting_icon "🔴"
        working_icon "🟡"
        completed_icon "🟢"
    }
}
```

The plugin loads in the background with no visible pane — it won't consume any screen space.

## Quick Start

After installing, restart Zellij and test with a pipe command:

```bash
# Send a waiting notification to the current pane
zellij pipe --name "zellij-attention::waiting::$ZELLIJ_PANE_ID"

# Send a working notification
zellij pipe --name "zellij-attention::working::$ZELLIJ_PANE_ID"

# Send a completed notification
zellij pipe --name "zellij-attention::completed::$ZELLIJ_PANE_ID"
```

Switch to the tab — the icon should appear. Focus the pane to clear it.

## Claude Code Integration

```bash
claude plugin marketplace add fresh2dev/zellij-attention
claude plugin install zellij-attention@zellij-attention
```

## GitHub Copilot CLI Integration

Add a user-level hook file under `~/.copilot/hooks/`:

```json
{
  "version": 1,
  "hooks": {
    "userPromptSubmitted": [
      {
        "type": "command",
        "bash": "[ -n \"${ZELLIJ_PANE_ID:-}\" ] && zellij pipe --name \"zellij-attention::working::$ZELLIJ_PANE_ID\" >/dev/null 2>&1 || true",
        "timeoutSec": 2
      }
    ],
    "notification": [
      {
        "type": "command",
        "matcher": "permission_prompt|elicitation_dialog|agent_idle",
        "bash": "[ -n \"${ZELLIJ_PANE_ID:-}\" ] && zellij pipe --name \"zellij-attention::waiting::$ZELLIJ_PANE_ID\" >/dev/null 2>&1 || true",
        "timeoutSec": 2
      },
      {
        "type": "command",
        "matcher": "agent_completed",
        "bash": "[ -n \"${ZELLIJ_PANE_ID:-}\" ] && zellij pipe --name \"zellij-attention::completed::$ZELLIJ_PANE_ID\" >/dev/null 2>&1 || true",
        "timeoutSec": 2
      }
    ],
    "agentStop": [
      {
        "type": "command",
        "bash": "[ -n \"${ZELLIJ_PANE_ID:-}\" ] && zellij pipe --name \"zellij-attention::completed::$ZELLIJ_PANE_ID\" >/dev/null 2>&1 || true",
        "timeoutSec": 2
      }
    ]
  }
}
```

| Hook                  | Notification | Meaning                   |
| --------------------- | ------------ | ------------------------- |
| `userPromptSubmitted` | 🟡 working    | Copilot started a turn    |
| `notification`        | 🔴 waiting   | Copilot needs user input  |
| `notification`        | 🟢 completed | A background agent stopped |
| `agentStop`           | 🟢 completed | Copilot finished the turn |

Copilot CLI loads hook configuration when it starts, so restart existing CLI processes after
changing the hook file.

## Configuration

All configuration is optional — the plugin works out of the box.

| Option           | Default  | Description                     |
| ---------------- | -------- | ------------------------------- |
| `enabled`        | `"true"` | Enable or disable notifications |
| `waiting_icon`   | `"🔴"`   | Icon for waiting state          |
| `working_icon`   | `"🟡"`    | Icon for working state          |
| `completed_icon` | `"🟢"`   | Icon for completed state        |

Icons are appended to the end of tab names (e.g., `terminal 🔴`).

## Pipe Message Format

```
zellij-attention::EVENT_TYPE::PANE_ID
```

- `EVENT_TYPE` — `waiting`, `working`, or `completed` (case-insensitive)
- `PANE_ID` — numeric pane ID from `$ZELLIJ_PANE_ID`

> **Important:** Always use `--name` (broadcast pipe), never `--plugin` (targeted). Targeted pipes create new plugin instances instead of reaching existing ones.

## Shell Functions

For manual testing or integration with other tools:

```bash
notify-waiting() {
    [ -z "$ZELLIJ_PANE_ID" ] && echo "Not in Zellij" && return 1
    zellij pipe --name "zellij-attention::waiting::$ZELLIJ_PANE_ID"
}

notify-working() {
    [ -z "$ZELLIJ_PANE_ID" ] && echo "Not in Zellij" && return 1
    zellij pipe --name "zellij-attention::working::$ZELLIJ_PANE_ID"
}

notify-completed() {
    [ -z "$ZELLIJ_PANE_ID" ] && echo "Not in Zellij" && return 1
    zellij pipe --name "zellij-attention::completed::$ZELLIJ_PANE_ID"
}
```

## Development

```bash
# Build
cargo build --target wasm32-wasip1 --release

# Install
cp target/wasm32-wasip1/release/zellij-attention.wasm ~/.config/zellij/plugins/

# Debug build (enables verbose logging)
cargo build --target wasm32-wasip1
tail -f /tmp/zellij-*/zellij-log-*/zellij.log | grep "zellij-attention"
```

## Troubleshooting

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues and solutions.

## License

MIT
