# hud

Toward a semantic visual language agents can speak to communicate intentionally
with people. Agents choose meaning and visual idioms; HUD makes them legible.

![hud starter dashboard](docs/assets/hud-starter.svg)

**Current runtime:** `hud` turns a small TOML file into a local terminal dashboard.
Each panel runs a shell command, renders the result, and gives you keyboard-first
ways to refresh, inspect, and launch follow-up actions.

**Next direction:** establish the [semantic language contract](docs/semantic-visual-language.md)
before expanding the palette. Start with Sequence as one complete reference idiom,
teaching why/when to use it as well as how. This is a product direction, not a
claim that a new production authoring API already exists. A dashboard is one
possible expression; the current name is provisional, with no rename underway.

## What the Current Runtime Is

- A single-user local TUI for command-backed dashboards.
- A lightweight way to collect `task`, `gh`, `tmux`, scripts, and status checks in one place.
- A static-config tool: edit TOML, run `hud`, iterate.
- Manual refresh first: no background polling or plugin daemons in V1.

## What It Is Not

- Not a plugin host or remote agent runner.
- Not a web dashboard.
- Not a process supervisor for long-running jobs.
- Not a multi-user service.

## Install

Download the latest archive for your platform from GitHub Releases, unpack it, and put `hud` somewhere on your `PATH`.

```sh
tar -xzf hud-aarch64-apple-darwin.tar.gz
sudo install -m 0755 hud-aarch64-apple-darwin/hud /usr/local/bin/hud
hud --version
```

Release artifacts are built for:

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

Install from source when developing or testing unreleased changes:

```sh
cargo install --git git@github.com:tnezdev/hud.git
```

## 60-Second Demo

Run the built-in demo:

```sh
hud --demo
```

Or run the starter config from a checkout:

```sh
cargo run -- --config examples/starter.toml
```

Validate a config without opening the TUI:

```sh
hud --config examples/starter.toml --check-config
```

## Preserved Agent Authoring Experiments

These experiments informed the [current direction](docs/semantic-visual-language.md).
Their patterns and JSON format are useful evidence, not the new language contract.
The new [Sequence reference brief](docs/sequence-reference.md) and
[discovery entrypoint](skills/hud/sequence.md) test one idiom: supplied procedures
and chronologies, including when to decline it. The isolated example keeps the
connected timeline and groups wrapped text within each item:

```sh
cargo run --example sequence -- examples/sequence-observatory.json
cargo run --example sequence -- examples/sequence-intake.json --preview 80 24
```

The [trial and review corrections](docs/sequence-trial.md) separate appropriate
selection from factual fidelity and scrolling failures. The
[section-rhythm follow-up](docs/sequence-rhythm.md) compares identical story wording
with clearer section boundaries; the [opening-hierarchy pass](docs/sequence-header.md)
addresses the busy header without changing author text. This remains experimental,
data-only syntax, not the installed HUD API.

The bounded [Compare reference](docs/compare-reference.md) is the next isolated
semantic experiment: it groups independent alternatives by common named criteria
without inventing ranking, totals, preference, or recommendation. Its supplier
anchor preserves the prose baseline and explicit unknown delivery facts:

```sh
cargo run --quiet --example compare -- examples/compare-suppliers.json --check
cargo run --quiet --example compare -- examples/compare-suppliers.json --preview 80 24
```

This data-only example shares its live/headless renderer and is not the production
CLI or a generic table API. The [Compare authoring entrypoint](skills/hud/compare.md)
teaches when to choose Compare, Sequence, or prose. The
[trial and independent review](docs/compare-trial.md) record appropriate Compare
selection, real fidelity omissions, and post-trial discovery corrections; none
of these alone establish Product Owner acceptance or author reliability. The
[bounded fidelity trial](docs/authoring-fidelity-trial.md) subsequently exercised
the corrected discovery path on five new packets: all final compositions passed
independent factual review after recorded self-repairs, including actual Sequence
invocation. In the subsequent reading-collection demo, the Product Owner found
the grouping easier to follow and its scanning benefit well worth the scrolling.
Preserve that demonstrated treatment; general reliability remains unestablished.

Start with the draft [HUD authoring skill](skills/hud/SKILL.md), then try a
fictional film-night story for two different audiences:

```sh
# Run from the checkout root. Quit one view before launching the other.
cargo run -- --config examples/film-night-organizer.toml
cargo run -- --config examples/film-night-guests.toml
```

The [design brief](docs/agent-canvas.md) separates the confirmed direction from
experimental authoring patterns. See the [trial and rendered evidence](docs/agent-canvas-trial.md)
for what we inspected and changed. This is not yet a free-position canvas or an
automated agent-to-screen publishing API.

A second, **example-only** [expressiveness probe](docs/expressive-story-probe.md)
tries a flowing lead → steps → notes composition and one display-only JSON file:

```sh
cargo run --example expressive_story -- examples/coach-story.json
cargo run --example expressive_story -- examples/coach-story.json --preview 80 24
```

The [experimental authoring entrypoint](skills/hud/expressive-probe.md) documents
validation, plain-reference mode, scrolling, and inspection of your own document.
This disposable syntax is not a production HUD API; the installed binary and
command-backed dashboard are unchanged. Higher-resolution graphics remain open.
The [trial and demo](docs/expressive-story-trial.md) compare the original journey,
flowing treatments, and a fresh author's departure → arrival revision; run
`examples/coach-departure.json` or `examples/coach-arrival.json` with the same example.

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
hud --config ./examples/starter.toml
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

[panels.row_detail]
title = "Task detail"
command = "task {{ID}}"

[[panels.actions]]
key = "t"
label = "Open tasks"
command = "taskwarrior-tui"
```

Config fields:

- `title`: dashboard title.
- `default_timeout_secs`: optional command timeout, default `120`.
- `[[panels]]`: one command-backed panel.
- `panels.id`: unique panel id.
- `panels.title`: panel title.
- `panels.command`: shell command used to refresh the panel.
- `panels.output`: optional output protocol: `text`, `table-json`, or `metrics-json`; default `text`.
- `panels.timeout_secs`: optional panel timeout override.
- `[panels.row_detail]`: optional selected-row drill-in command for `table-json` panels.
- `panels.row_detail.title`: row detail view title.
- `panels.row_detail.command`: shell command with `{{Column Name}}` placeholders from the selected row.
- `[[panels.actions]]`: optional fire-and-forget action for the focused panel.
- `panels.actions.key`: single-character keyboard shortcut.
- `panels.actions.label`: footer label.
- `panels.actions.command`: shell command launched without waiting for completion.

## Output Protocols

Plain text stdout is the default panel content protocol.

Use `output = "table-json"` for typed table rendering:

```json
{
  "type": "table",
  "columns": ["Repo", "State"],
  "rows": [["hud", "active"]]
}
```

Use `output = "metrics-json"` for aggregate line gauges:

```json
{
  "type": "metrics",
  "metrics": [
    { "label": "Budget", "value": 72, "max": 100 }
  ]
}
```

Metrics use Unicode block characters by default. If your terminal font renders those poorly, set `HUD_ASCII_BARS=1` to use ASCII bars.

Malformed structured output, non-zero exits, timeouts, and launch failures render as panel error states instead of crashing the dashboard.

## Row Drill-In And Actions

Press `Enter` on a dashboard card to open the panel detail view. For `table-json` panels, configure `[panels.row_detail]` to run a command for the selected row:

```toml
[[panels]]
id = "services"
title = "Services"
command = "./scripts/services-json"
output = "table-json"

[panels.row_detail]
title = "Service Detail"
command = "./scripts/service-detail '{{Service}}'"
```

Focused-panel actions are fire-and-forget commands. They appear in the footer and in the `?` overlay:

```toml
[[panels.actions]]
key = "e"
label = "edit config"
command = "${EDITOR:-vi} ~/.config/.hud/config.toml"
```

## Keybindings

- `q`, `Esc`, or `Ctrl-C`: quit.
- `h`/`j`/`k`/`l` or arrow keys: move focus through the card grid.
- `Tab` / `Shift-Tab`: cycle focus through panels.
- `Enter`: drill into the focused panel.
- In detail view, `Enter`: open configured row detail for the selected row.
- `?`: toggle help/actions overlay.
- `q`, `x`, or `Esc`: step back from detail views.
- `q`, `x`, `Esc`, or `?`: close help/actions overlay.
- In detail view, `j`/down and `k`/up select output rows; scrolling follows selection.
- `r`: refresh focused panel.
- `R`: refresh all panels.
- Focused-panel action keys are shown in the footer.

## Examples

- `examples/starter.toml`: safe first-run config with no external tool dependencies.
- `examples/kitchen-sink.toml`: static showcase for text, metrics, tables, row drill-in, and actions.
- `examples/dogfood.toml`: local working cockpit for tools like `tmux`, `gh`, and `task`.
- `examples/aesthetic-lab.toml`: dependency-free visual spike that pushes typography, symbols, spacing, and the four-panel mission-control layout.
- `examples/film-night-organizer.toml` and `examples/film-night-guests.toml`: fictional decision brief recomposed as an arrival guide; offline fixture commands, no actions or live data.

Run an example from a checkout:

```sh
cargo run -- --config examples/kitchen-sink.toml
```

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

Run locally:

```sh
cargo run
```

Run the normal quality gate:

```sh
./scripts/check
```

Run the separate dependency advisory gate when release/security posture matters:

```sh
./scripts/audit
```

Release notes and tag steps live in `docs/release.md`.

## License

MIT
