# Next session: Sequence reference contract

**Historical entry handoff:** the subsequent session implemented the bounded
reference and ran its trial. Read [Sequence trial and corrections](sequence-trial.md)
and [the reference brief](sequence-reference.md) for the newer state. The original
handoff below is preserved, not the current implementation queue.

## Start here

The Product Owner approved a pivot to **the semantic language contract before
palette expansion**, then requested documentation cleanup, full context capture,
a fresh feature branch, removal of our demo splits, and a fresh session before
starting the next slice. This is the handoff, not a request to continue the prior
calmer-layout experiment.

Read completely before non-trivial work:

1. `AGENTS.md`, `README.md`, `docs/engineering-spec.md`.
2. `docs/semantic-visual-language.md` — current direction, exact product feedback,
   agreed responsibility split and next slice. This outranks older experiment plans.
3. `skills/hud/SKILL.md`, `skills/hud/expressive-probe.md` — existing authoring paths,
   **not** the new Sequence contract.
4. `docs/expressive-story-trial.md` and its actual linked JSON/cells as needed.
5. `examples/expressive_story.rs` before borrowing or changing the prototype.

## Repository checkpoint and working branch

- Checkout: `/home/tnez/Work/tnezdev/hud`.
- `31ab9cf` preserves the entire expressive-story prototype, tests, and evidence.
- The documentation-only cleanup commit containing this handoff records the pivot.
  No renderer, schema, dependency, or fixture changes belong to that cleanup.
- Prepared next branch: `feat/sequence-reference`, from the cleanup checkpoint on
  `feat/expressive-story-probe`, **not** from main. It intentionally retains the
  working prototype as evidence and possible reusable code. Verify the live branch
  and clean status instead of assuming the handoff is eternally current.
- `main` remains at `e26e273`; no pushes/PRs/merges were performed in this session.
- Preserve `feat/interval-refresh-document-panels` at `78f110b`: separate unpushed
  WIP, not part of Sequence work. Do not fold it in or restart it.
- `feat/agent-canvas-skill` contains original prerebase commits from already-merged
  PR #41. It is not pending new work. GitHub is the sole public home; no HUD
  Forgejo repository exists or is wanted.

## What Travis actually decided

HUD may grow beyond dashboards (and may eventually be renamed). Its purpose is to
establish a semantic visual language an agent can speak. The agent chooses
communicative meaning and idioms; HUD resolves those into dependable presentation.

Working boundary:

> Agent: facts + intent → semantic composition.
> HUD: semantic composition + display constraints → visual presentation.

Do not insert an AI/natural-language resolver inside HUD by assumption. The
working hypothesis is a discoverable structured semantic vocabulary that lets the
agent choose how to communicate without drawing mechanics. A catalogue describing
geometry and invocation alone would put the wrong work back on the author.
Discovery should explain **meaning, why, when, counterexamples, semantic inputs,
presentation responsibilities, and examples of different semantics**.

Established web/information-design principles should inform the terminal language:
hierarchy, proximity, whitespace, reading rhythm, continuation grouping, disclosure,
and visible relationships. This is not web UI imitation in cells. The successful
journey timeline carries meaning through numbering, connection, and placement.
Pixels/higher-resolution graphics remain open, not rejected or scheduled.

Travis said the demo's semantic intent/action orientation clearly directs his
attention, but wrapped busy prose makes it hard to retain focus. He then explicitly
praised the Journey timeline as immediately scannable 1-2-3 notes. **Keep its
numbers, connectors, and rhythm.** Our initial connector-removal proposal is
superseded. No `--calm` implementation or hanging-indent patch was made.

Your responsibility: solve routine engineering with other agents and Dottie; bring
Travis meaningful demonstrations/tradeoffs, not schema choices, test debugging,
reviewer coordination, or code-management questionnaires.

## Next move, not yet implemented

Write the small Sequence reference brief with Dottie's input **before code**:

- What ordered relationship Sequence expresses and what it helps a person perceive.
- Why/when to choose it and when not to (unrelated items or non-ordered alternatives
  are candidate counterexamples, not a finalized test set).
- Semantic inputs and presentation obligations. The prototype's mandatory
  lead/route/support fields are not an approved contract. Concrete syntax is open.
- Multiple semantic examples, not just another journey with renamed locations.

Then freeze the trial entrypoint and give a fresh author **two suitable stories
and one deliberately unsuitable case**, without coaching which should use Sequence.
Selecting a non-Sequence response is valid; do not expand the palette for the
negative case. Avoid leaking the rubric or example answer into the trial prompt.
Demonstrate appropriate selection, composition without drawing/layout coaching,
and legibility at constrained sizes. Use 72x35 (the actual prior paired panes) and
80x24 as relevant checks, not guaranteed universal requirements. Story selection,
implementation details, and exact tests remain for agents to resolve.

Keep facts and temporal qualifiers intact. Document validation/repair attempts,
inspection, interventions, and limitations. If comparing visual treatments, hold
the wording and purpose constant. Different departure/arrival stories demonstrate
intent adaptation, not an isolated visual comparison.

Product checkpoint:

> Does teaching this idiom help an agent communicate well, rather than merely
> invoke a component correctly?

Stop at one complete reference idiom and an honest demonstration. No catalogue,
framework, automatic publishing, production schema commitment, or broad rewrite.

## What the current code actually does

- Production Rust HUD: static TOML + trusted shell commands + typed text/table/
  metrics content, card/detail rendering, manual refresh. Production `src/` was
  unchanged by the expressive-story experiment.
- `cargo run --example expressive_story -- FILE.json`: separate example-only,
  data-only JSON parser and read-only flowing TUI. `--check`,
  `--preview WIDTH HEIGHT [OFFSET]`, and optional preceding `--plain` are **example
  arguments**, not installed `hud` flags. Shares one live/headless draw function.
- Current schema requires title, provenance, lead, route, support. Unknown fields
  and controls fail closed, input is byte-limited, and there is no shell/network/
  action path. File reads are not bounded in time for arbitrary devices/FIFOs.
- Existing timeline + lead + notes is one successful guided-story template, not
  demonstrated broad expressiveness. Plain mode removes connector rows as well as
  styling: compare decorated versus compact flow, not color-only treatments.
- Ratatui wrapped-line measurement is enabled by a dev-dependency feature. There
  were no dependency version/lockfile changes. Preserve explicit effect boundaries.
- Current example supports j/k/arrows, PgUp/PgDn, Home/End, q/Esc/Ctrl-C; no refresh.
  Minimum view 64x12, resize message below that. Terminal wrapping remains imperfect.

## Evidence and review limits

Early history: fictional film-night organizer → guests was demonstrated and merged
by Travis in PR #41. A later fresh author used the frozen main skill for route
comparison → coach journey, which Travis praised live. The previous handoff is
preserved locally as `/tmp/hud-next-session-pre-language-31ab9cf.md`; original
route evidence remains in `/tmp/hud-route-evidence-tdk7LQ/` if still present.

This session's artifacts are durable under `docs/assets/expressive-story/`:

- Original journey text copied verbatim to `examples/coach-baseline/`, portable
  `examples/coach-baseline.toml`, and identical-wording `examples/coach-story.json`.
- Cards, decorated flow, compact/plain flow at 93x35 and 80x24. Candidate exposes
  more lead/route at80x24; notes still scroll. Existing cards have detail access.
- Fresh pi author `hud-story-author` used the frozen example entrypoint and supplied
  facts, created one JSON per purpose, inspected both sizes, and refined wrapping.
  It received the semantic correction to emphasize arrival reassurance rather than
  departure. No renderer/repo edits or private layout coaching occurred.
- Final first/second uncoached artifacts are frozen in `author-round-one/` and
  `author-round-two/`. Intermediate drafts/full session history are not archived;
  no controlled speed, overall effort, or general reliability claim is justified.
- Dottie caught loss of the explicit one-day qualifier in round two. The coordinator
  restored it in `examples/coach-arrival.json`, leaving raw trial evidence unchanged.
  `examples/coach-departure.json` is the unchanged final round-one composition.
  The correction is automated-test verified, not separately independently approved.
- Live candidate scrolling was observed at144x22 using j/k to END and back;
  q returned to shell. Herdr rejected Home/End key names; those keys are unit-tested
  but were not verified live. Final paired departure/arrival views were72x35.
- Cells and style assertions are not pixel screenshots or reader comprehension.
  The explicit Product Owner feedback above adds limited human evaluation, not
  blanket acceptance of every treatment or contract.

## Dottie collaboration and stale advice

Load `/home/tnez/.agents/skills/dottie/SKILL.md`. Use the structured `dottie ask`
bridge from a verified Herdr working pane; never inject into her personal composer.
Read-only consultation does not authorize memory/project writes or implementation.
Dottie sessions do not automatically inherit this local discussion. **Provide the
new approved pivot and timeline clarification in the next focused consultation.**
She has not reviewed the new Sequence contract because none exists yet.

Relevant IDs:

- `b1e9b488-7c48-4d61-9b0d-248fb3b4d241`: two-axis expressiveness/authoring scope;
  recommended separating evidence and testing flow against cards.
- `a39c2fe1-d96b-4cac-8a86-90bffe44f940`: pasted example non-test source, JSON, and
  80x24 artifacts review. Qualified experiment acceptance; noted one-day omission,
  narrow scope, connector cost, and hanging-indent issue. She did not run the TUI,
  check local hashes/tests, or inspect93x35 in that review.
- `9ec4e381-5583-4e0e-b44a-0a313703d3f4`: completed consultation on the preliminary
  calmer-layout proposal, **before** Travis praised connectors and approved the
  semantic-language pivot. Retrieved during cleanup; no pending job. Its advice
  about preserving wording and observing scroll costs remains useful; its connector
  removal/standalone calmer-pass framing is superseded. Do not resume that plan.
- Earlier main trial: `f69d6c80-47ec-4deb-bade-763aa3e687f9` (actual artifacts review),
  `781a03bd-a099-4c81-825e-49ad6c4d2ee0` (rubric).
- Earlier film-night: `d0c3300c-d320-4052-8f6d-0adb87b62b57` (scope),
  `f1224ac9-2d67-45ef-bfb3-bca503b0694c` and
  `84a46375-3454-472b-bf69-d269a2c7212c` (PR review/closure).

Remote prior project notes may still center weekly orientation or Forgejo; current
explicit PO direction and local docs outrank that history. No remote memory updates
were made or authorized. The Dottie bridge limits questions to24,000 UTF-8 bytes;
send focused context/artifacts and label omissions rather than a whole transcript.

## Herdr cleanup and next-session boundaries

This session created demo panes `w7:p6` and `w7:pA`, verified their foreground
processes, quit the example normally, verified shell foreground, and closed them.
The temporary author pane `w7:p7` and sizing panes `w7:p8`/`w7:p9` had already been
closed. Current tab `w7:t1` was verified with only caller `w7:p1`, focused, no splits.
Existing shell `w7:p2` in `w7:t2` was not created for this trial and was left untouched.

Treat these as historical IDs. Inspect live topology before future demos; don't
reuse occupied panes or close unrelated work. Load Herdr's current skill/CLI when
controlling panes. Preserve caller focus/cwd. No global HUD or skill installation
occurred. Starting a fresh agent conversation is the next user/session action;
this handoff does not itself reset or replace the current pi process.

## Gates and final guardrails

- `./scripts/check` is required even for this docs-only cleanup. Prototype baseline
  is60 passing tests; final cleanup check result and exact head are recorded in
  `/tmp/hud-next-session.md` after commit/branch preparation.
- `./scripts/audit` was attempted earlier: cargo-audit is not installed. No local
  advisory pass or current CI result claimed; do not install tools without authority.
- Preserve original experiments and WIP. No merge, release, publication, install,
  access change, or remote memory write is implied by approval to explore Sequence.
- Begin by grounding the Sequence brief with Dottie, not by adding widgets or
  resurrecting the proposed calmer-layout branch.
