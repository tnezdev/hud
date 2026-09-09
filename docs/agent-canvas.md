# Agent canvas: first authoring experiment

## Confirmed direction

HUD gives agents a terminal-native visual vocabulary and canvas for communicating
intentionally with people. An agent should discover useful, semantically named
primitives through `skills/hud`, choose what it wants to say, compose a view, and
inspect what it actually made. Terminal conversation and visual communication
complement each other. A status dashboard or weekly orientation is one possible
story, not the product boundary.

The Product Owner owns direction, principles, priorities, and demonstrated
usefulness. Agents handle routine design, implementation, checks, and review;
bring back outcomes and consequential tradeoffs rather than code-management tasks.
GitHub is the sole public code/issue/PR home. Private integrations remain separate.

## Proposed slice, not a new runtime API

Start with a repository-local draft [authoring skill](../skills/hud/SKILL.md) and
one fictional, offline story: **Save the outdoor film night**. This is a design
probe, not a promise that today's panel API is the final canvas model.

The initial intention is: help an organizer choose the indoor hall rather than
the courtyard, by placing a recommendation alongside its reasons and alternatives.

After rendering and inspection, apply a **scripted audience correction**:
"The move is decided. This screen is for arriving guests now." Recompose as an
arrival guide: destination and route first, same film and start time as reassurance.
Remove the obsolete comparison instead of merely changing values in its rows.
The correction is part of the experiment, not a claim of a live human/agent update.

Dottie proposed the story and purpose-first vocabulary during collaboration;
the local coding agent owns the implementation and evidence. These are design
proposals under the confirmed direction, not additional Product Owner requirements.

## Scope and boundaries

- Experimental patterns: **Lead**, **Compare**, **Explain**, **Direct**, **Measure**.
  They map to existing text, table-json, and integer metrics-json output. They are
  not new TOML fields, commands, or special widgets. Use only patterns the story
  needs; no invented percentages to demonstrate a gauge.
- Composition currently means content, panel order/count, and automatic layout.
  No coordinates, arbitrary drawing, semantic layout API, live config replacement,
  remote publishing protocol, or agent supervision is added.
- Use published-main capabilities only. Interval refresh and document previews
  remain on a separate WIP branch; this experiment does not depend on them.
- Trusted, fixed local file-reading commands supply display-only fictional facts.
  No network, personal information, live clock/weather, actions, or row-command
  interpolation is necessary.
- A small fixture-only Rust example may call the production renderer through
  ratatui's `TestBackend` to make its cells inspectable. It is not a general
  screenshot/export API and must not execute panel commands.
- Do not expand this slice into renderer redesign. Record limitations exposed by
  composition and let the next slice address the most consequential one.

## Observable acceptance

1. **Discover:** the skill explains when to reach for each pattern, maps it to a
   real supported type, and provides runnable examples and honest limits.
   An independent agent reviews it; a fresh-agent authoring trial remains a
   separate claim unless actually performed.
2. **Intend:** record the audience, intended understanding/action, and why spatial
   composition helps before producing each view.
3. **Compose:** both versions validate and run offline using reviewed file readers.
4. **Render and inspect:** exercise `ui::draw` at 120x40 and 80x24; inspect the
   lead, reading order, wrapping/clipping, comparison, and an Enter detail view.
   Preserve rendered-cell evidence and reproducible commands. Buffer checks do
   not establish terminal colors, real keyboard handling, or product usefulness.
5. **Refine:** change the audience and composition, render again, and document an
   actual observation rather than inventing a usability finding.
6. **Review:** Dottie reviews the actual artifacts, with the evidence level named.
   Run `./scripts/check`; report the independent advisory gate separately.

Product acceptance remains open: **Does the changed screen communicate clearly
to its new audience, and does the vocabulary invite another story?** Engineering
checks or an agent review cannot answer that on the Product Owner's behalf.

## Collaboration and delivery

One writer owns this feature branch; Dottie supplies product/continuity scrutiny
and non-author review. A linked GitHub issue/PR holds the public scope, exact
revision, evidence, limitations, and next handoff. No parallel public backlog.
Travis authorized trying this collaboration and publishing some work; this slice
stops at a draft PR, with no merge, release, global installation, or access changes.
