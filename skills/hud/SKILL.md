---
name: hud
description: Compose an intentional terminal-native visual story with HUD. Use when a comparison, explanation, decision brief, or guide would communicate better alongside conversation than as another long reply.
---

# Paint a story in the terminal

**Current direction:** establish a semantic visual language before expanding the
palette. The agent chooses communicative idioms; HUD handles visual presentation.
Discovery should teach meaning, why/when, counterexamples, and semantic examples,
not only invocation. See the [product decision](../../docs/semantic-visual-language.md).
Start with the experimental [Sequence reference](sequence.md) for supplied
procedures and chronologies, and the bounded [Compare reference](compare.md) for
alternatives answering common named questions. Both teach why/when to decline;
their syntax is experimental, not a chosen production architecture. The recipes
below preserve earlier experiments and remain runnable, but their pattern names
and schemas are not the chosen language architecture.

Start with what you want someone to understand—not with a list of widgets to fill.
HUD can keep a recommendation, its evidence, and its alternatives visible together.
It can also remove the deliberation once your audience needs directions instead.

This is an **experimental authoring skill over today's HUD**, not a new canvas
API. The vocabulary below names communicative purposes; HUD currently implements
those purposes with command-backed panels and automatic layout.

For the newer **example-only** flowing story experiment, see
[expressive-probe.md](expressive-probe.md): a genuinely emphasized lead, connected
steps, supporting notes, and one data-only JSON document with a general preview
within that example. Its disposable syntax is not the production HUD API; the
command-backed patterns below remain available for tables and gauges.

## Before composing

Write one sentence:

> For **[audience]**, make **[understanding or next action]** clear by showing
> **[things that benefit from being seen together]**.

If a short conversational reply would serve better, use that. A canvas is useful
when relationships, simultaneous context, or a view worth returning to matter.

## Choose a purpose

| Pattern | Reach for it when… | Current implementation |
| --- | --- | --- |
| **Lead** | One conclusion or destination should frame everything else. | `output = "text"`; put the essential sentence first. |
| **Compare** | Someone needs to judge alternatives against the same named questions. | Isolated data-only `examples/compare.rs`; not a choice control, scorecard, or production API. See [Compare](compare.md). |
| **Explain** | The conclusion needs reasons, provenance, or uncertainty. | `output = "text"`; Enter opens the same content in a larger, scrollable view. |
| **Direct** | Someone needs a next step or a short route, not more deliberation. | `output = "text"`; ordered instructions, not executable actions. |
| **Measure** | A bounded quantity genuinely helps understanding. | `output = "metrics-json"`; integer `value`/`max`, shown as a percentage gauge. |

These names are authoring patterns, **not valid values of `output`**. Lead does not
receive special typography or guaranteed prominence. Don't add a gauge without a
meaningful denominator; don't retain a comparison after the decision is settled.

### Give the pattern real content

HUD loads a TOML composition. Each panel's trusted command emits display data.
For example, from the root of a HUD checkout:

```toml
title = "Film night / fictional demo"

[[panels]]
id = "lead"
title = "The recommendation"
output = "text"
command = "cat examples/film-night/organizer-lead.txt"
```

The isolated Compare reference is invoked with a data-only JSON file, not the
command-backed table protocol:

```sh
cargo run --quiet --example compare -- examples/compare-suppliers.json --preview 80 24
```

For historical command-backed panels, `output = "table-json"` still emits one
JSON document:

```json
{"type":"table","columns":["Place","Rain cover"],"rows":[["Courtyard","No"],["Hall","Yes"]]}
```

For Measure, set `output = "metrics-json"` and emit:

```json
{"type":"metrics","metrics":[{"label":"Seats reserved","value":24,"max":40}]}
```

Table rows must match the column count. Metrics require nonnegative integers,
`max > 0`, and `value <= max`; labels should explain what is measured. These
examples are fictional, not live measurements. Text is plain text, not Markdown,
ANSI drawing instructions, or commands. Structured output is parsed, not executed.

## Compose deliberately

- Give each panel a job; use audience-facing titles, not internal pattern names.
- Put the essential message at the start of the first panel. Supporting content
  should still make sense when read independently.
- Keep the first view short. Enter expands a focused panel; it does not fetch a
  separate explanation for text panels. Long content may be clipped on the card.
- Layout is automatic. Normally panels fill two columns, with an odd final panel
  spanning the last row. Four panels become three across plus one wide below at
  roomy widths. Panel order is not a coordinate or size API. Avoid adding filler
  solely to trigger a layout.
- Narrow widths change wrapping and available space. Inspect at the intended size,
  not just the largest convenient terminal. Start with 120x40 and 80x24 for this trial.
- Show sources, uncertainty, and whether facts are supplied, fictional, or live.
  A polished presentation must not imply verification that never happened.

## Try the story, then change its purpose

This skill lives at `<hud-checkout>/skills/hud/SKILL.md`. Resolve `../..` from its
folder to the checkout root and run the following commands **there**. If copied
elsewhere, first locate a checkout containing these examples; they aren't bundled
inside the skill. No global skill or HUD installation is required. Cargo/Rust and
a trusted shell environment with `cat` are needed. After building, the fixture
commands themselves require no network; shell startup files may have other effects.

```sh
cargo run -- --config examples/film-night-organizer.toml --check-config
cargo run -- --config examples/film-night-organizer.toml
```

Use Tab to focus a panel, Enter to expand it, j/k to move through detail, Esc to
return, and q on the dashboard to quit. `r` refreshes content, `R` refreshes all.
Config/layout edits require quitting and relaunching; refresh does not reload TOML.

Now apply this scripted correction:

> The move is decided. This screen is for arriving guests now.

Remove the comparison. Lead with the destination, give the route, and reassure
people about what hasn't changed. A runnable second composition is:

```sh
cargo run -- --config examples/film-night-guests.toml
```

Read those two TOML files and their text/JSON payloads as a small working recipe.
For your own composition, use a new config and reviewed data files; don't overwrite
the demo or a user's active config. Use fixed, safely quoted file paths in commands.
HUD doesn't silently publish or replace another person's screen for you.

## Step back and look

Valid TOML or JSON is not evidence that the view communicates. Look at the rendered
result: can you find the lead, follow the intended order, read the comparison, and
reach the details? What should you remove, shorten, reorder, or reframe?

This experiment includes a **fixture-only** rendered-cell preview:

```sh
cargo run --quiet --example story_preview -- organizer 120 40
cargo run --quiet --example story_preview -- organizer 80 24 compare
cargo run --quiet --example story_preview -- guests 80 24
```

It uses the production renderer with fixed file contents and an in-memory backend;
it does not run shell commands or show an actual terminal screenshot. It accepts
the two film-night fixtures and the frozen `journey` baseline, not arbitrary
compositions. Inspect its output
as text when image/terminal observation isn't available, and say so. For other
command-backed compositions, current HUD has no general headless preview API:
use an authorized terminal observation mechanism or report that visual inspection
is blocked. The separate expressive example can preview its own data-only
format; it does not preview arbitrary HUD configs.

Record the intent, fixture/revision, dimensions, what you actually inspected, and
one change made after inspection. Rerender under the same conditions. Never claim
human acceptance, keyboard verification, colors, or live updates from buffer text.

## Trust and current limits

HUD executes configured shell commands. Treat a composition as executable config:
review it before running it, use authorized local paths, and keep story content in
data files. The current runner invokes the selected shell with `-lc`, so its startup
files may run too: fixed file-reading commands are not a sandbox. Never splice
untrusted prose or table cells into shell commands. This
story needs neither configured actions nor row-detail command templates.

Current scope here: text, tables, gauges, automatic layout, manual refresh, and
local invocation. No arbitrary canvas placement, image primitives, remote publishing,
conversation listener, or automatic agent feedback loop is provided. The separate
interval/document experiment is not required or claimed by this skill.

The [trial record](../../docs/agent-canvas-trial.md) shows what we actually inspected
and refined. The [design brief](../../docs/agent-canvas.md) separates confirmed
direction from this vocabulary experiment. The [engineering spec](../../docs/engineering-spec.md)
and [README](../../README.md) describe the actual runtime contracts.
