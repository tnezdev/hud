# hud Engineering Spec

## Product Constraint

`hud` is a single-user local cockpit. Simplicity and legibility win over general plugin-host architecture.

The core should make local scripts and commands feel coherent, spacious, and action-oriented. It should not begin life as a full extension platform with complex process supervision, version negotiation, or remote lifecycle management.

## Fat-Marker Architecture

```text
static config
  -> app state builder
  -> panel runner boundary
  -> panel output parser
  -> dashboard state
  -> ratatui renderer
  -> input/action dispatcher
  -> command launcher boundary
```

## Core Invariants

1. A broken panel cannot crash the app.
2. Command execution is an edge effect, not mixed into rendering or state decisions.
3. Rendering consumes typed dashboard state, not raw command output.
4. Tests never depend on real time, real sleeps, real terminals, or real shell commands.
5. Plain text output is valid v1 panel content.
6. Structured output is parsed once at the boundary, not defensively interpreted everywhere.
7. The v1 architecture optimizes for one local user editing config and scripts directly.

## Initial Module Boundaries

```text
src/main.rs          // binary wiring only
src/app.rs           // event loop coordination
src/config.rs        // static config parse + validation
src/panel.rs         // panel model, panel states, output contract
src/command.rs       // command execution trait + real adapter
src/action.rs        // action model + dispatch decisions
src/ui.rs            // ratatui rendering from typed state
```

These boundaries are provisional. They should appear only when code needs them, not as empty architecture scaffolding.

The current implementation has crossed the first useful threshold for these modules: config parsing, command execution, dashboard state, app coordination, action resolution, and ratatui rendering now exist because V1 needs them.

## Testing Strategy

- Unit tests cover config validation, output parsing, focus movement, and action resolution.
- Command execution is tested through an injectable runner, not real shell commands.
- Contract tests appear when there is more than one command-runner implementation.
- Rendering tests wait until the UI structure stabilizes enough to make snapshots useful.
- One smoke spec should eventually run a tiny fixture config with a fake command runner.

## Initial Biases

- Prefer TOML for static config unless it proves awkward.
- Start with plain text panel output, but define the structured-output boundary early.
- Use `ratatui` and `crossterm` for the TUI once the core state/config/panel contract is testable.
- Start with manual refresh only.
- Avoid an async runtime until interval refresh, command execution, or input handling proves it is needed.
- Treat long-running plugins, generated UI, approvals, and agent supervision as later design directions.

## Config Shape

V1 uses static TOML discovered at `$XDG_CONFIG_HOME/.hud/config.toml`, falling back to `$HOME/.config/.hud/config.toml` when `XDG_CONFIG_HOME` is unset.

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

Config parsing happens once at the boundary into typed Rust values. Readers consume typed config, not TOML values.

## Command Boundary

Panel refreshes run through an injectable command runner. The real runner is a shell-command adapter at the app edge. Unit tests use fakes rather than real shell commands.

Command results distinguish:

- stdout
- stderr
- exit status
- timeout
- launch failure

The V1 default command timeout is 120 seconds. Panels can override it with `timeout_secs`.

Action commands use the same injectable boundary but are fire-and-forget: `hud` verifies that the action launches, then returns to the dashboard without waiting for completion.

Configured row-detail commands also use the panel refresh boundary. Entering a configured row detail view runs a bounded command and stores the result in typed app state; rendering never launches commands.

## Output Protocol

Plain text stdout is valid V1 panel content and remains the default.

Panels can opt into the first structured protocol with:

```toml
[[panels]]
id = "repos"
title = "Repos"
command = "./scripts/repos-json"
output = "table-json"
```

`table-json` expects stdout to be a single JSON document:

```json
{
  "type": "table",
  "columns": ["Repo", "State"],
  "rows": [
    ["hud", "active"],
    ["relay", "watch"]
  ]
}
```

The parser boundary is `panel`: command stdout is converted once into typed panel content. Rendering consumes typed text or typed table data and does not inspect raw JSON. Malformed structured stdout renders as a panel error instead of falling back to plain text.

The first table protocol intentionally only supports string cells. Richer cell types are later work.

Panels can also opt into the first aggregate widget protocol with `output = "metrics-json"`:

```json
{
  "type": "metrics",
  "metrics": [
    { "label": "Budget", "value": 72, "max": 100 },
    { "label": "Quota", "value": 43, "max": 100 }
  ]
}
```

`metrics-json` is parsed into typed metrics at the panel boundary and rendered as local line gauges. Values must be finite numbers between `0` and `max`, and `max` must be greater than zero. Chart families beyond line gauges, richer metric units, trends, and categorical bar charts are later slices.

## View Stack And Row Drill-In

The first explicit navigation stack is:

```text
Dashboard
  -> Panel detail
    -> Row detail
```

`q`, `x`, or `Esc` pops one view. `Enter` on a dashboard card pushes panel detail. `Enter` on a selected table row can push row detail when the panel has configured row drill-in.

The first row drill-in config shape is intentionally small:

```toml
[[panels]]
id = "issues"
title = "Issues"
command = "./scripts/issues-json"
output = "table-json"

[panels.row_detail]
title = "Issue detail"
command = "gh issue view {{Issue}}"
```

Row detail commands use `{{Column Name}}` placeholders. Placeholder values come from the selected typed table row, matched against table column names. Unknown placeholders fail closed and render a row-detail error. This is string substitution for local commands, not a general template language.

Row detail output is plain text in this slice. Structured nested detail views, loading spinners beyond the existing panel states, and deeper configured stacks are later work.

## Refresh Model

V1 starts with manual refresh only.

- Manual refresh reruns either the focused panel or all panels, depending on the keybinding.
- There is no background polling in the first implementation slice.
- Manually triggered panel commands run outside the terminal input loop so a slow panel does not block navigation or quitting.
- Long-running panel processes are out of scope for v1; each refresh is a bounded command invocation.
- Per-panel interval refresh remains a likely later extension. When added, time must enter through an injectable clock or tick source so tests can advance time deterministically.

## Open Decisions

1. When should per-panel interval refresh be introduced?
2. What is the minimum semantic component set after plain text?
3. Should structured panel output be newline-delimited JSON, a single JSON document per refresh, or both behind explicit protocol markers?
