# hud

A programmable terminal cockpit for getting oriented, finding the next thing that needs attention, and jumping into action from the keyboard.

## Status

Early V1 implementation. The accepted product PRD lives in `tnezdev/proposals` at `proposals/hud.md`.

V1 proves the cockpit loop:

```text
open -> orient -> move focus -> refresh -> act
```

## Install From Source

Prerequisites:

- Rust toolchain with Cargo.
- A terminal that supports alternate-screen TUIs.
- Optional local tools used by your panel commands, such as `tmux`, `gh`, or `task`.

Install the current GitHub source:

```sh
cargo install --git git@github.com:tnezdev/hud.git
```

Update to the latest source:

```sh
cargo install --git git@github.com:tnezdev/hud.git --force
```

Verify:

```sh
hud --version
```

## Configuration

By default, `hud` reads:

```text
$XDG_CONFIG_HOME/.hud/config.toml
```

If `XDG_CONFIG_HOME` is unset, the fallback is:

```text
$HOME/.config/.hud/config.toml
```

Use a specific file with:

```sh
hud --config ./examples/dogfood.toml
```

Validate config without opening the TUI:

```sh
hud --config ./examples/dogfood.toml --check-config
```

Minimal config:

```toml
title = "Work cockpit"
default_timeout_secs = 120

[[panels]]
id = "tasks"
title = "Tasks"
command = "task mine"
timeout_secs = 10

[[panels.actions]]
key = "t"
label = "Open tasks"
command = "taskwarrior-tui"
```

Config shape:

- `title`: dashboard title.
- `default_timeout_secs`: optional command timeout, default `120`.
- `[[panels]]`: one command-backed panel.
- `panels.id`: unique panel id.
- `panels.title`: panel title.
- `panels.command`: shell command used to refresh the panel.
- `panels.output`: optional output protocol, either `text` or `table-json`; default `text`.
- `panels.timeout_secs`: optional panel timeout override.
- `[[panels.actions]]`: optional fire-and-forget action for the focused panel.
- `panels.actions.key`: single-character keyboard shortcut.
- `panels.actions.label`: footer label.
- `panels.actions.command`: shell command launched without waiting for completion.

Plain text stdout is the default V1 panel content protocol. Panels can opt into typed table rendering with `output = "table-json"` and stdout shaped as:

```json
{
  "type": "table",
  "columns": ["Repo", "State"],
  "rows": [["hud", "active"]]
}
```

Malformed structured output, non-zero exits, timeouts, and launch failures render as panel error states instead of crashing the dashboard.

## Usage

Run with your default config:

```sh
hud
```

Try the built-in demo:

```sh
hud --demo
```

Try repository example configs:

```sh
cargo run -- --config examples/dogfood.toml
cargo run -- --config examples/kitchen-sink.toml
```

Keybindings:

- `q`, `Esc`, or `Ctrl-C`: quit.
- `h`/`j`/`k`/`l` or arrow keys: move focus left/down/up/right through the card grid.
- `Tab` / `Shift-Tab`: cycle focus through panels.
- `Enter`: drill into the focused panel.
- `?`: toggle help/actions overlay.
- `q`, `x`, or `Esc`: return from detail view.
- `q`, `x`, `Esc`, or `?`: close help/actions overlay.
- `q` or `Esc`: quit from the grid.
- In detail view, `j`/down and `k`/up select output rows; scrolling follows selection.
- `r`: refresh focused panel.
- `R`: refresh all panels.
- Focused-panel action keys are shown in the footer.

## tmux Popup

One-off popup:

```sh
tmux display-popup -E -w 90% -h 80% 'hud'
```

With a local config during development:

```sh
tmux display-popup -E -w 90% -h 80% 'cd /path/to/hud && cargo run -- --config examples/dogfood.toml'
```

Example keybinding:

```tmux
bind-key H display-popup -E -w 90% -h 80% 'hud'
```

The dashboard is most comfortable at roughly 100 columns by 30 rows or larger. Quit returns control cleanly to tmux because `hud` restores the terminal alternate screen on exit.

## Development

```sh
cargo run
```

```sh
./scripts/check
```

## Engineering Questions

Before implementation, settle the first architecture slice:

- What is the minimal core loop for config, panel execution, rendering, input, and actions?
- What should the v1 panel output protocol guarantee?
- What test boundaries give fast confidence before every deploy?
- What belongs in core now, and what should stay as shell scripts or examples?
