# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-05-27

The first usable release of `hud`, a programmable terminal cockpit for getting
oriented, finding the next thing that needs attention, and jumping into action
from the keyboard.

### Added

- Static TOML config with panel definitions, actions, and timeout overrides.
- Command-backed panels with plain text, structured table (`table-json`), and
  metrics gauge (`metrics-json`) output protocols.
- Manual refresh for the focused panel (`r`) and all panels (`R`).
- Vim-like keyboard navigation (`h`/`j`/`k`/`l`, arrow keys, `Tab`/`Shift-Tab`)
  through the dashboard card grid.
- Drill-in from the dashboard into panel detail views (`Enter`).
- Row selection (`j`/`k`) in detail views for `table-json` panels.
- Configured row drill-in commands with `{{Column Name}}` template substitution
  from the selected row.
- Fire-and-forget focused-panel actions, shown in the footer and triggered by
  configured single-key shortcuts.
- Help/actions overlay (`?`) showing available keybindings and panel actions.
- Bounded command execution with a 120-second default timeout and per-panel
  `timeout_secs` overrides.
- Visible error states for command failures, timeouts, and malformed structured
  output — broken panels cannot crash the app.
- `--demo` flag for a zero-config tour of the cockpit loop.
- `--check-config` flag to validate a config file without opening the TUI.
- `--config` flag to load a specific TOML file.
- tmux popup integration via `display-popup -E`.
- Example configs: `dogfood.toml` and `kitchen-sink.toml`.
- Source-install path: `cargo install --git git@github.com:tnezdev/hud.git`.
- CI pipeline: `cargo fmt`, `cargo clippy`, and `cargo test` on every PR and
  push to `main`.

### Known Limitations

- Manual refresh only — no background polling, intervals, or live tails.
- Local command model — all panel sources and actions run shell commands on
  the same machine. No remote data sources, network adapters, or plugin host.
- No persistent state — no dismissals, action history, caching, or session
  recall between runs.
- No plugin host — adding panel types or output protocols requires source
  changes, not a plugin API.
- Structured protocols are intentionally small — `table-json` and
  `metrics-json` cover the initial use cases; additional semantic components
  are reserved for later releases.
- Single dashboard — no multi-page or tabbed layouts.

[0.1.0]: https://github.com/tnezdev/hud/releases/tag/v0.1.0
