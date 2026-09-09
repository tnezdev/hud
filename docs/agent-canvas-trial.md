# Film-night authoring trial

## What this trial demonstrates

Two fictional, offline compositions over the existing HUD runtime, developed
with Dottie's product/continuity input and local implementation. Start with the
[authoring skill](../skills/hud/SKILL.md) and [experiment contract](agent-canvas.md).
The names Lead, Compare, Explain, Direct, and Measure remain experimental.

**Organizer intention:** help choose a rain-safe venue by showing the recommendation,
reasons, and alternatives together. Three panels: Lead, Explain, Compare. No gauge:
there is no meaningful bounded quantity needed for this decision.

**Guest intention:** help arriving guests find the hall while retaining reassurance
about the film and time. Two panels: Lead and Direct. The scripted correction is
"The move is decided. This screen is for arriving guests now." We remove the
comparison, change the titles and content, and give the route. This is not a live
conversation listener or automatic publication/update mechanism.

## Actual inspection and refinement

The initial implementation is commit `92ee161e65703df723cf868debd4c2f541b279c4`,
based on published main `3f9a12a9d5be89bb96cffa63d9c106e77d2e999f`. The separate
interval/document WIP is not included. The initial 80x24 render is preserved as
[organizer-first-80x24.txt](assets/film-night/organizer-first-80x24.txt).

The local agent read the production renderer's output cells, not just the input
TOML/JSON. At 80x24, the final "Fictional planning exercise" line in **Why move?**
was clipped. The header already labeled the overall screen fictional, but the
panel-local qualifier was missing. Removing a blank line and shortening the
forecast label made both provenance lines visible. A regression assertion now
checks their presence at 80x24 and 120x40.

The final snapshots use the payloads committed alongside this record. They are
historical inspection evidence, not hand-drawn mockups or pixel screenshots:

| View | 80x24 | 120x40 |
| --- | --- | --- |
| Organizer | [cells](assets/film-night/organizer-80x24.txt) | [cells](assets/film-night/organizer-120x40.txt) |
| Guests | [cells](assets/film-night/guests-80x24.txt) | [cells](assets/film-night/guests-120x40.txt) |
| Organizer comparison detail | [cells](assets/film-night/organizer-detail-80x24.txt) | — |

Observed after refinement:

- The organizer's recommendation and reasons sit above a full-width comparison;
  the two alternatives, rain-cover criterion, and unchanged start time are readable.
- The guest view contains destination, route, and reassurance without an obsolete
  decision table. The layout changes from three panels/two rows to two panels/one row.
- The evidence labels are visible at both recorded sizes. Text cells do not prove
  contrast, font rendering, or that a person noticed or understood those labels.
- At 80x24, the comparison detail's long header breadcrumb is truncated, although
  the table and its panel title remain visible. This is a remaining renderer/content
  fit limitation, not a reason to redesign navigation in this slice.
- At 120x40, short content leaves substantial empty card space. Equal-size panels
  and uniform text treatment do not provide a true lead/secondary visual hierarchy.
  Purpose-first names help the author, but do not yet create semantic layout behavior.

## Reproduce

From the checkout root, with Rust/Cargo available:

```sh
cargo run -- --config examples/film-night-organizer.toml --check-config
cargo run -- --config examples/film-night-guests.toml --check-config

# Actual local TUI; quit one before launching the other.
cargo run -- --config examples/film-night-organizer.toml
cargo run -- --config examples/film-night-guests.toml

# Fixed-fixture cell inspection; these commands do not execute panel commands.
env -u HUD_ASCII_UI -u HUD_ASCII_BARS cargo run --quiet --example story_preview -- organizer 120 40
env -u HUD_ASCII_UI -u HUD_ASCII_BARS cargo run --quiet --example story_preview -- organizer 80 24
env -u HUD_ASCII_UI -u HUD_ASCII_BARS cargo run --quiet --example story_preview -- organizer 80 24 compare
env -u HUD_ASCII_UI -u HUD_ASCII_BARS cargo run --quiet --example story_preview -- guests 120 40
env -u HUD_ASCII_UI -u HUD_ASCII_BARS cargo run --quiet --example story_preview -- guests 80 24
```

The fixture helper compiles the exact TOML/data files into the example, matches
only the five known file-reading commands, feeds their contents through the
production command-result/parser boundary, and calls `ui::draw` with `TestBackend`.
Its optional detail argument enters the real detail state, bypassing keyboard I/O.
It is deliberately not a generic preview subsystem.

## Verification and limits

- `./scripts/check`: format, Clippy, and **52 tests** pass (48 existing + 4 fixture
  tests). Both new TOML files also pass the real CLI's `--check-config`.
- A separate manual 80x24 PTY exercise launched the actual binary for both configs,
  observed file-output markers, sent Tab/Enter into the comparison or route detail,
  observed Esc return to the dashboard, and quit with status 0 and alternate-screen
  restoration. This verifies those I/O paths, not pixel appearance or comprehension.
  Reproduce interactively with two Tabs/Enter for the organizer's comparison or
  one Tab/Enter for the guests' route, then Esc and q.
- The first PTY probe incorrectly expected contiguous full sentences in the raw
  ANSI stream. Ratatui's differential writes skip unchanged cells; the corrected
  probe checks emitted markers. Full layout observations above come from the
  in-memory rendered cells, not reconstructed PTY screenshots.
- `./scripts/audit` was attempted but cargo-audit is not installed locally. No
  dependency changes are introduced. GitHub CI's advisory result is tracked in
  the PR; no local advisory pass is claimed.
- No native-terminal image inspection, fresh-agent independent composition trial,
  live conversational update, remote transport, or Product Owner acceptance has
  been established. Dottie's non-author review and CI disposition belong on the PR
  with the exact reviewed head, not as unqualified permanent approval here.

## Next product-facing question

Does the changed screen communicate clearly to its new audience, and does this
vocabulary invite another story? Bring that demonstration to the Product Owner.
Do not ask them to reconcile test failures, diffs, or reviewer handoff mechanics.
Any follow-up proposal should distinguish a better authoring guide from a new
runtime capability (for example, genuine semantic emphasis or general inspection).
