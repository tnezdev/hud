# hud Engineering Spec

## Product Constraint

HUD's current product direction is a single-user, local-first **semantic visual
language for agent communication**. A dashboard is one application, not the product
boundary. Simplicity and legibility win over general plugin-host architecture.

The current runtime is still a command-backed terminal cockpit. Preserve its
explicit effect boundaries; the pivot does not authorize a full extension platform,
complex process supervision, version negotiation, or remote lifecycle management.

## Agent Authoring Direction

The [current product decision](semantic-visual-language.md) takes precedence over
historical experiment assumptions: the agent selects communicative idioms; HUD
resolves their visual presentation. Discovery must teach meaning, why/when,
counterexamples, semantic inputs, and examples—not only invocation syntax.
Establish Sequence as one reference idiom before expanding the palette. Concrete
syntax and presentation changes need a bounded spec before implementation; no
separate AI resolver or new production schema is currently chosen.

The [Sequence reference brief](sequence-reference.md) bounds a new isolated
`examples/sequence.rs`: explicit procedure/chronology semantics, optional item
detail and shared notes, persistent scope, and a numbered relationship gutter.
Ratatui wraps/measures individual paragraphs within their text column. A shared
legal-position rule snaps fitted paragraph tops, marks oversized fragments and
bottom cuts, and keeps item identity while scrolling. The
[section-rhythm refinement](sequence-rhythm.md) uses larger major-section gaps and
open-rule landmarks attached to their first paragraph; item interiors stay compact.
The [opening-hierarchy refinement](sequence-header.md) separates title from its
persistent source/scope pair and moves the relationship/caveat to the sequence
landmark. That cue now scrolls; author text and body height are unchanged.
Following PO acceptance, one outer blank row above the title supplies top padding;
it consumes one viewport row without adding boxes or changing internal spacing.
Parsing and effects remain at the edge; the production runtime, old expressive
example and dependencies are unchanged. The
[author entrypoint](../skills/hud/sequence.md) teaches selection before syntax.
The approved next bounded [Compare reference](compare-reference.md) adds an
isolated data-only example for alternatives answering common named questions; it
preserves unknown/unavailable answers and declines unsupported normalization,
ranking, totals, and recommendations. Its [author entrypoint](../skills/hud/compare.md)
teaches Compare versus Sequence and plain prose. It does not change production
`src/`, the table protocol, or the old experiments. The [Compare trial](compare-trial.md)
separates semantic selection from actual invocation, factual fidelity, and
post-trial teaching corrections. The [Sequence trial](sequence-trial.md)
distinguishes its frozen author run from factual and scroll-boundary corrections.

### Example diagnostic output boundary

The [diagnostic-safety follow-up](example-diagnostic-safety.md) addresses PR #42's
review finding: Serde errors can contain decoded controls before display-field
validation. The three data-only examples encode handled errors once at their
stderr boundary, escaping terminal/bidi controls and line separators while
retaining readable diagnostics. Ordinary live cleanup uses non-printing
`try_restore`; a restoration failure returns failure, preserving any original
operation error before both reach the same encoder. An example-only helper with
an injected writer/results keeps this testable without a terminal. Parsing rules,
valid rendering, production
`src/`, dependencies and all frozen trial evidence remain unchanged. This is
output encoding, not a semantic content resolver or a renderer redesign.

### Preserved experiments, not the language contract

HUD's earlier work explored a terminal-native visual vocabulary and canvas for agents
communicating intentionally with people. The first [authoring experiment](agent-canvas.md)
adds a draft skill and fictional compositions over the existing runtime, not a
new semantic schema, remote publication mechanism, or layout engine. Its Lead,
Compare, Explain, Direct, and Measure patterns are experimental guidance, not
protocol types. The existing simplicity and command-boundary invariants still apply.

The follow-up [expressive story probe](expressive-story-probe.md) is isolated in a
Cargo example: a bounded data-only JSON input, flowing semantic hierarchy, and
shared live/headless rendering. It does not change the production CLI, panel
model, or renderer and does not incorporate the interval/document WIP. Its parser,
rendering, and scrolling are in-memory testable; filesystem and terminal I/O remain
explicit edges. The disposable syntax and ratatui wrapped-line measurement feature
(dev dependency only) are experimental, not chosen production architecture.

## Current Runtime Architecture

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

## Current Priority and Deferred Runtime Questions

The immediate priority is the bounded semantic-reference work approved in [the
product decision](semantic-visual-language.md): Sequence is accepted as the first
reference, and Compare is the one next reference. Compare's local brief bounds
common-question correspondence rather than a generic table API. Do not expand a
component catalogue first.

Older runtime questions remain deferred, not the next work queue:

1. When should per-panel interval refresh be introduced?
2. What additional panel output types are useful?
3. Should structured panel output support newline-delimited JSON as well as a
   single JSON document per refresh?
