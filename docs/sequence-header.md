# Sequence: quieter opening hierarchy

**Final PO verdict after top padding: “That looks great!”** The demonstrated
presentation is the accepted Sequence visual reference. Preserve this treatment;
no further visual iteration, broader palette, production API or publication
approval is implied. The implementation/trial limitations below remain honest.

**Subsequent PO acceptance:** Travis found the opening “much easier to read” and
said it was looking like what he had hoped for. He requested one blank row before
the first line, likening the grouping to web padding/margins. Add exactly one outer
top-padding row; preserve all other spacing, content and hierarchy. The header now
occupies six rows (including that padding) instead of five, so the body viewport
has one fewer row. This is spacing, not a request to draw enclosing boxes.
The comparison/results below describe the preceding, unpadded checkpoint.

## Feedback and contract before code

Travis accepted the improved section boundaries (“They are much better”) but said
“it is still busy at the top,” supplying
`/home/tnez/Pictures/screenshot-2026-09-09_12-19-32.png`. The coordinator inspected
that screenshot. It highlights four adjacent title/metadata lines plus the purpose
paragraph; the renderer's cyan relationship explanation competes with the title.
This is limited acceptance of section grouping, not the whole presentation/API.

Keep that successful section rhythm, numbering and connectors. Hold both story
JSON files byte-for-byte constant. Change only renderer-owned presentation:

- Persistent five-row header: title, blank row, provenance and scope together,
  final blank row. No dimming, shortening or dropping authored qualifications.
- Replace the timeline's bare rule with a meaningful section landmark:
  `──── Procedure · not execution status` or
  `──── Chronology · not causation`. These fixed strings fit in one row even at
  the minimum item text width; they replace an existing row, not add one.
- The purpose paragraph and all other author text stay unchanged. No extra
  scrolling rows or new schema/CLI fields. Title/source/scope remain persistent.
- Explicitly relax the earlier presentation choice: the renderer-owned type and
  interpretation caveat now scroll with the sequence landmark rather than remain
  in the header. Discovery/offscreen content is not the same as immediate
  visibility. Inspect both cases mid-scroll for unsupported execution/causal
  implications; retain the original safe paragraph/heading positioning rules.

Dottie `9d91281b-c531-4ab8-a758-f51048951547` supports testing that bounded relaxation,
not treating every renderer caveat as a permanently sticky PO requirement. She
warned about mid-scroll interpretation and preserving the one-row heading
invariant. She did not see the screenshot or inspect implementation/tests. This
is advice, not approval; no remote project/memory writes requested or reported.

## Verification plan

Preserve the preceding section-rhythm source/entrypoint and baseline executable,
then compare identical JSON at72x35,80x24 and matched live widths. Assert unchanged
body height/word content, intact one-row type/caveat headings at64columns, persistent
source/scope at every offset, no stranded headings, and existing page/negation
regressions. Add direct header-row/landmark tests, run `./scripts/check`, and record
actual mid-scroll evidence and the visibility tradeoff, not a blanket fidelity claim.

Before evidence: `docs/assets/sequence/header/before/`; after evidence is separate.
This is a renderer-only follow-up, not a fresh author trial or palette expansion.
No production runtime, dependency, fixture, installation, publication or remote
knowledge changes. The Product Owner judges whether the opening is calmer.

## Result and review

The proposed five-row header and labeled sequence landmarks are implemented.
The fixed landmark strings fit the minimum54-cell item text column in one row.
Body heights remain unchanged: intake29/27 rows and observatory24/21 rows at
72x35/80x24 respectively. Tests assert those totals and exact title/source/scope
placement. Both authored JSON hashes remain unchanged.

Dottie reviewed the actual opening and mid-scroll cells in
`6a5ea0f7-918e-4fab-ac5d-c4c5ca2b8673` and found no newly affirmative execution or
causal claim in those supplied fragments. She considered it a reasonable candidate,
not a universal semantic guarantee. In the chronology's64x12 offset8 view, the
anti-causal caveat is offscreen: report attribution and estimated-time wording
remain visible, but “unverified” does not substitute for “not causation.” The
procedure retains persistent “not executed.” This is an explicit visibility
tradeoff, not a claim that all qualifications stay visible.

[Design advice](assets/sequence/header/dottie-design.json) and
[evidence review](assets/sequence/header/dottie-review.json) are preserved. She did
not inspect the PO screenshot/full source or execute tests. The source delta was
described; rendered cells were pasted. No independent code-execution review claimed.

Before/after [cells](assets/sequence/header/) include opening, offset8 and End at
72x35,80x24 and64x12 for both stories. The final live pair shows the same intake at
**88x35**: [before](assets/sequence/header/before/live-intake.txt),
[after](assets/sequence/header/after/live-intake.txt). Both reach END at line1/26;
headless captures at the same size match. No extra scrolling cost was introduced
by this pass. Screenshot acceptability remains for the PO to judge.

Existing owned panes were quit and verified at shell before reuse. `w7:pC` now
runs `/tmp/hud-sequence-before-header`; `w7:pD` runs the current example, both on
`examples/sequence-intake.json`. The baseline PTY is temporarily88columns to match
the candidate, restoring90columns on normal exit (its physical width at setup).
A later resize invalidates that comparison/restoration assumption; rediscover
actual dimensions before cleanup. Caller focus/cwd and unrelated `w7:p2` remain
untouched. No new pane or author worker was created. The comparison is left open.

`./scripts/check` passes **74 tests**, formatting and Clippy; one new regression
covers header hierarchy, unchanged body totals and one-row landmarks. Existing
page, orphan-heading and negation coverage stays green.
[Gate log](assets/sequence/header/check.log). No dependency/audit/CI claim.

```sh
sha256sum -c docs/assets/sequence/header/before/archive.sha256
sha256sum -c docs/assets/sequence/header/fixtures.sha256
# Accepted, unpadded checkpoint is now archived:
sha256sum -c docs/assets/sequence/header/after/accepted-archive.sha256
sha256sum -c docs/assets/sequence/header/padding/checkpoint.sha256
```

Source/entrypoint manifests from previous checkpoints remain historical. The
preceding section-rhythm implementation is archived in this pass's `before/`
directory; original fresh-author evidence is still unchanged.

## Accepted finishing adjustment: outer top padding

Added exactly one blank row before the title, as requested. All author text,
landmarks and internal gaps are unchanged; the body viewport is one row shorter.
The existing opening regression now asserts the blank first row and unchanged
body totals. Scroll/orphan checks cover the reduced minimum body height too.
`./scripts/check` remains **74 passing tests** plus formatting/Clippy.
[Padding gate](assets/sequence/header/padding/check.log).

The existing owned before/after panes now compare the accepted unpadded header
with the padded version at matched **71x35**. [Before](assets/sequence/header/padding/live-before.txt),
[after](assets/sequence/header/padding/live-after.txt). Baseline executable is
`/tmp/hud-sequence-before-top-padding`; candidate is the current example. Caller
focus/cwd preserved; no new panes or external consultation were needed for this
explicit finishing request. The baseline shell now restores72columns on exit,
its physical width at this setup; rediscover dimensions after any later resize.
