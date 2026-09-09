# Experimental flowing story: author, inspect, revise

This is a disposable **checkout example**, not HUD's production API or an approved
component language. Use it to try a lead with genuine visual emphasis, connected
ordered guidance, and quieter supporting facts. Choose existing panels instead
when a comparison table, gauges, or simultaneous independent regions matter.
No higher-resolution images are provided by this probe.

Start with audience and intent, as in [SKILL.md](SKILL.md). Then write **one JSON
file**, not TOML, shell readers, and multiple payloads. Here is the complete shape:

```json
{
  "title": "An audience-facing title",
  "provenance": "Fictional supplied facts / not live",
  "lead": {
    "label": "What matters now",
    "headline": "The one thing to notice first",
    "lines": ["Essential context that belongs with the headline."]
  },
  "route": {
    "label": "What to do next",
    "steps": ["First instruction.", "Then this instruction."]
  },
  "support": {
    "label": "What to know",
    "lines": ["Evidence, qualifications, and what is unknown."]
  }
}
```

All fields are required. Collections contain 1–12 nonempty strings, each at most
500 characters. Labels/headline are also nonempty, at most 500 characters. Title
and provenance fit 60 terminal cells each. Whole document: at most 64 KiB.
Unknown fields, terminal control codes, embedded newlines/tabs, and bidi controls
are rejected. Use separate entries for separate lines. JSON escaping rules apply.

- **Lead** gets the full width and a contrasting headline. Choose its emphasis
  deliberately; it is not limited to a departure time or a recommendation.
- **Route** numbers and connects steps automatically. Do not embed numbers or
  simulate a current step; this static example has no live progress knowledge.
- **Support** follows a divider. Its ordinary text remains readable; uncertainty
  must not be hidden just to make a polished overview.
- Ordering is fixed: lead → route → support. No coordinates, card sizing, nested
  components, Markdown/ANSI interpretation, shell execution, or automatic refresh.
  This may be too constrained for other stories: record that rather than inventing
  unsupported fields or manufacturing instructions to fill the required route.

## Run from the checkout root

Resolve `../..` from this file's directory. These are **Cargo example arguments**,
not flags supported by the installed `hud` binary. No installation is needed.
For a new story, use your own file in an authorized temporary directory:

```sh
cargo run --quiet --example expressive_story -- /absolute/path/story.json --check
cargo run --quiet --example expressive_story -- /absolute/path/story.json --preview 93 35
cargo run --quiet --example expressive_story -- /absolute/path/story.json --preview 80 24
# Preview the end if the footer reports MORE below (offset clamps to the end).
cargo run --quiet --example expressive_story -- /absolute/path/story.json --preview 80 24 65535
# Neutral presentation reference: identical source, no emphasis or connectors.
cargo run --quiet --example expressive_story -- /absolute/path/story.json --plain --preview 80 24
# Live terminal, only in a pane you are authorized to use:
cargo run --quiet --example expressive_story -- /absolute/path/story.json
```

The preview uses the same renderer as this example's live view, via TestBackend.
It accepts your document, not just bundled fixtures. It is rendered text cells,
not a screenshot, proof of perceived colors, or proof of human understanding.
Live: j/k or arrows scroll, PgUp/PgDn page, Home/End jump, q/Esc/Ctrl-C quit.
The footer says MORE below or END. Minimum live size is 64x12. Shorter/narrower
viewports show a resize message. Header and provenance stay visible while scrolling.

Inspect both requested sizes. Record intent, validation/repair attempts, commands,
actual wrapping/overflow, and a revision you made after inspection. Keep original
versions when changing purpose. Relaunch after edits; the live view reads once.
Do not claim a fresh-agent trial or product acceptance merely from a valid file.

The example reads the named file as display data. It does not execute story text,
follow embedded file paths, run actions, call a shell, fetch network data, or
publish to someone else's screen. Cargo is a trusted local development invocation,
not a sandbox. Preserve that distinction from command-backed HUD configs.

Example: [coach-story.json](../../examples/coach-story.json).
Scope and evidence: [expressive-story-probe.md](../../docs/expressive-story-probe.md).
