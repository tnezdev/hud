# Sequence: choose an ordered relationship, not a numbered list

Experimental checkout reference, not installed HUD's production API.

## Meaning and why

Sequence shows a supplied procedure or chronology as a connected series. Use it
when seeing relative positions helps your audience follow actions or understand
events. HUD supplies numbers, connections, alignment and wrapping; you supply
meaning. Numbers mean local positions, not priority, progress or completeness.

Before composing, state: for whom, and what understanding or next action matters?
Then ask: would swapping items misrepresent the supplied relationship?

## When to use it — and when not to

- `procedure`: a supplied order for doing something. It does not execute actions
  or claim anything is done. Example: count a returned kit, record discrepancies,
  then sign only after inventory agrees.
- `chronology`: a supplied temporal order of events. It does not establish cause,
  duration or predicted progress. Example: selected dated log entries describing
  a warning, later inspection, and later normal reading, with cause still unknown.

Choose one relationship throughout. Dates in procedure detail do not turn actions
into observed events. Never invent events or instructions to fill the shape.

**Do not use Sequence** for ranked alternatives, independent checks, simultaneous
or unknown-order events, parallel/branching dependencies, or merely your preferred
order for explaining topics. A price-sorted list of venues is not a journey through
all venues. A ranking fails the swap test too, so that test alone is not enough.
If Sequence is inappropriate, a plain conversational explanation is a valid result;
you need not build another idiom or use the older HUD experiments.

## Semantic contract

Supply an audience-facing title, intent, source status, explicit scope, relationship
and ordered items. Say whether this is a complete supplied procedure, selected
events, an excerpt, or one scoped path. Never imply omitted required actions are
unnecessary. Fictional, static and one-day-only mean different things.

Each item has a label and optional elaborating detail. Keep a qualification with
the smallest claim it changes: “After approval, submit,” “Estimated 14:00 arrival.”
Do not put a condition that reverses an action only in detail or common notes.
Short labels are useful, but fidelity wins over brevity. Detail is optional only
when unnecessary, not disposable when crowded. Shared notes are for background.

HUD preserves the numbered connected spine, wraps labels/details in their text
column and identifies items even when scrolling starts in their continuation.
Title, provenance, scope and relationship remain visible. Overflow is scrollable
and explicit; no silent shortening or live current/completed status. At less than
64 columns or 12 rows it asks for a resize. Preview cells cannot prove perceived
color, human comprehension, or real terminal-key behavior.

## Complete experimental shape

```json
{
  "title": "Return the loaned kit",
  "provenance": "Fictional supplied procedure / not executed",
  "scope": "Complete supplied return procedure / today only",
  "intent": "Help the volunteer hand back the kit without an ambiguous receipt.",
  "relationship": "procedure",
  "items": [
    {"label": "Count the returned kit", "detail": ["Use the supplied inventory."]},
    {"label": "Record discrepancies"},
    {"label": "Sign only after inventory agrees"}
  ],
  "notes": ["No live inventory verification is provided."]
}
```

All top-level fields except `notes` required. `items`: 2–12. `detail` and `notes`
are optional lists of 0–12 nonblank strings. All strings: at most 500 characters;
`title`, `provenance`, `scope`: also at most 60 terminal cells. Whole input: at most
64 KiB. Unknown fields, controls, tabs/newlines and bidi controls are rejected.
Relationship must be exactly `procedure` or `chronology`. No styling/geometry,
commands, includes, links to fetch, interpolation, or Markdown interpretation.
Labels are not required to have detail; notes need not exist. Don't add filler.

## Author, inspect, refine

Resolve `../..` from this file's directory to the checkout root. Use your own
regular local JSON file in an authorized output directory. No installation needed.
These are Cargo example arguments, not installed `hud` flags:

```sh
cargo run --quiet --example sequence -- /absolute/path/story.json --check
cargo run --quiet --example sequence -- /absolute/path/story.json --preview 72 35
cargo run --quiet --example sequence -- /absolute/path/story.json --preview 80 24
# Offset is a body-line index; large values clamp to the last view.
cargo run --quiet --example sequence -- /absolute/path/story.json --preview 80 24 65535
# Live only in a terminal you are authorized to use:
cargo run --quiet --example sequence -- /absolute/path/story.json
```

Inspect every part, not just top/end if a long middle lies between. The footer
reports position and MORE below / END. Vary offset to inspect intermediate lines.
Live: j/k or arrows, PgUp/PgDn, Home/End; q/Esc/Ctrl-C quits. Edits require relaunch;
no refresh or publishing. Keep initial drafts when refining. Record choice/reason,
intent, exact commands/dimensions, validation failures and repairs, what you actually
saw, and any revision (or why none). A valid file proves shape, not appropriateness
or truth. Report friction rather than shortening away inconvenient qualifications.

Story data is inert: no shell, actions, network, or executable config. Cargo itself
is a trusted development invocation, not a sandbox. File reads are byte-limited,
not time-bounded for devices/FIFOs. This trial teaches one idiom, not a catalogue.
