# Sequence: section rhythm, same-wording comparison

**Subsequent PO feedback:** section boundaries are “much better”; the opening is
still busy. The [header follow-up](sequence-header.md) preserves this body treatment.
This record and its captures describe the preceding section-rhythm checkpoint.

## Product observation and bounded hypothesis

After the corrected Sequence demonstration, Travis said:

> “Almost. What looks off to me is what in Web UX world I think we would call
> vertical rythym. My eyes have trouble navigating because they don't know how to
> break-up the sections. I think that's what it is.”

The observed difficulty is section navigation. Vertical rhythm is his tentative
diagnosis, not approval of a particular spacing prescription. Preserve the earlier
positive timeline feedback: numbers, connectors, quick 1–2–3 scan. Agents own the
presentation work; Travis evaluates the experience as Product Owner.

Dottie `87980e62-906d-406e-aa13-99244dbc4baa` recommends prioritizing larger section
boundaries rather than inserting whitespace between every label and detail. The
current one-row gaps and similar heading weights give unlike relationships similar
signals. That is a design inference, not a verified diagnosis. She did not inspect
the current TUI or verify the reported71tests. Consultation is read-only; no remote
project/memory writes requested or reported.

## Presentation contract — written before code

Hold both demonstration JSON files byte-for-byte constant. No schema, authoring
requirements, parser, commands, production runtime, dependency, or palette changes.
This is not a new fresh-author trial or removal of the connected timeline.

Use a small hierarchy of spacing and landmarks:

- **Within an item:** zero blank rows between wrapped bold label and normal-weight
  elaboration. Same text column; qualifiers remain close to the claim they change.
- **Between items:** retain the single connector-only row and existing numbering.
- **Between sections:** two blank rows instead of one between opening context and
  timeline, and between timeline and supporting notes. A short, open rule marks
  the timeline start. The existing “Background and sources” heading gets the same
  short-rule vocabulary and a distinct section-heading style. No invented story
  headings, boxes, enclosing borders, or separators around every item.
- **Persistent context:** title, provenance, scope, relationship and caveat stay
  together and unchanged. No dimmed-away caveats or extra gap inside this group.
- **Heading ownership:** the rule/section heading belongs to the following first
  paragraph, not a separate navigable orphan. Keep their measurement together.
  If only the heading would fit at the bottom, defer it and reserve that row;
  page movement must not skip the deferred unit. Existing paragraph-top snapping,
  oversized-fragment warnings, and bottom continuation disclosure remain in force.

A combined heading-plus-paragraph can exceed the body even if the text alone fits;
it then uses the existing explicitly marked oversized-paragraph path. This is a
structural grouping rule, not a claim that paragraphs are semantically independent.

The implementation remains a small internal linear render plan. Section treatment
is resolved by HUD, not an author-selectable styling flag or a layout framework.

## Evidence and checks

Preserve before source/entrypoint and actual cells under
`docs/assets/sequence/rhythm/before/`; the previous reviewed checkpoint remains
historical. Baseline executable `/tmp/hud-sequence-before-rhythm` is local only.
Same fixtures and dimensions:72x35 and80x24, opening/end views for both stories.
Capture candidate separately; report extra body lines and scrolling, not merely
that the screen looks cleaner. Keep all facts, qualifiers and source status.

Tests cover unequal section/item spacing, label/detail adjacency, distinct section
style, intact numbers/connectors, no heading stranded at the bottom, all body words
reachable via both page directions, and the existing negation regression. Run
`./scripts/check`. Independent evidence scrutiny is bounded, not another broad
architecture or authoring exercise.

For PO inspection, present the same intake story before/after in the two existing
owned demo panes, preserve caller focus, and verify actual dimensions. The question
is whether sections are easier to locate while the numbered spine remains easy to
follow. More whitespace can cause earlier scrolling; that cost is part of the demo,
not something to compensate for by editing the story.

## Implemented result and independent scrutiny

The candidate implements those three spacing levels. Generated one-row landmarks
share the outer margin and are measured with the following paragraph; every wrapped
label line retains bold weight, detail stays normal and adjacent, and the numbered
spine remains intact. Neither fixture changed (`fixtures.sha256` verifies both).

Dottie reviewed the actual render-plan/scroll source and pasted before/after cells
in `c4987a4f-d182-495b-b2e7-aac835530940`. She found the section transitions more
distinguishable without fragmenting item interiors and found no blocking defect by
inspection. [Design consultation](assets/sequence/rhythm/dottie-design.json),
[evidence review](assets/sequence/rhythm/dottie-review.json). She did not execute
code or inspect the omitted parser, CLI, tests, hashes, or live terminal styling.
Her judgment is not PO acceptance.

She identified a minor footer mismatch: a wholly deferred heading/paragraph could
be described as already “continuing.” The coordinator moved continuation detection
after the deferral decision and added a direct regression, including the following
PageDown reaching the withheld unit. This correction is test-verified, not separately
independently re-reviewed. A pre-existing connector-only End position remains a
small navigation blemish; no broader scrolling redesign was folded into this pass.

The first candidate rule was indented into the item text column; coordinator
inspection moved it to the outer section margin before independent review. The
final fixtures and line counts below were not editorially shortened to fit.

## Same-wording evidence and cost

| Story / dimensions | Before body rows | After body rows | Opening view |
| --- | ---: | ---: | --- |
| Intake /72x35 |26|29|Both fit.|
| Intake /80x24 |24|27|Both need scrolling; candidate cuts a detail at bottom and explicitly marks continuation.|
| Observatory /72x35 |21|24|Both fit.|
| Observatory /80x24 |18|21|Before fits; candidate now needs scrolling.|

Matched headless cells: [before](assets/sequence/rhythm/before/) and
[after](assets/sequence/rhythm/after/), with opening/end views for both stories.
The source header, scope, caveats, intent, labels and details are unchanged; only
HUD's presentation differs. In the80x24 intake opening, the prerequisite remains
in the label but its elaboration continues below. Compact grouping is not a
promise that every qualification is fully visible at every viewport position.

The live comparison initially exposed different actual PTY widths (72 versus71),
despite visually equal Herdr layout rectangles. An expected line-count wait timed
out because of that difference. The left baseline was quit and its owned PTY
width temporarily set to71, with restoration to its saved width after normal exit.
Both app viewports were then verified as **71x35**, and the captures match the
corresponding71x35 headless output. Thus the final comparison does not attribute
an extra wrap caused by unequal widths to the presentation change.

Final live [before](assets/sequence/rhythm/before/live-intake.txt) and
[after](assets/sequence/rhythm/after/live-intake.txt) show the same intake file.
The added spacing means this71-column candidate needs one j movement to show the
last note; [End](assets/sequence/rhythm/after/live-end.txt) was observed and k
returned to the opening view. No pixel screenshot or reader-comprehension claim.

At capture, existing owned `w7:pC` is the baseline, `w7:pD` the candidate, caller
`w7:p1` stays focused with cwd unchanged. No new panes or author agents were created;
unrelated `w7:p2` is untouched. Both demo panes remain open for PO inspection.
Rediscover topology before further control; baseline resizing can invalidate the
matched-width setup. Its shell restores the saved PTY width when the demo exits.

## Checks and boundaries

`./scripts/check`: **73 passing tests**, formatting and Clippy. Two new tests cover
spacing/style/adjacency and non-orphaned landmarks, including the footer correction.
Existing actual PageUp/PageDown word coverage and negation regressions remain green.
[Gate log](assets/sequence/rhythm/check.log). No new dependency/advisory claim.

```sh
sha256sum -c docs/assets/sequence/rhythm/before/archive.sha256
sha256sum -c docs/assets/sequence/rhythm/fixtures.sha256
# The accepted body treatment's source/entrypoint were archived before
# the later header refinement; its old live-path manifest is historical.
sha256sum -c docs/assets/sequence/header/before/archive.sha256
```

This is a controlled presentation follow-up, not another fresh-author trial or
Product Owner acceptance. No schema, production runtime, palette, dependencies,
interval/document WIP, publication, installation, or remote-memory changes.
