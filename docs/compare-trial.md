# Compare trial and review record

Status: bounded trial and review completed; Product Owner checkpoint remains
unresolved. This is evidence, not Product Owner acceptance. Travis remains Product
Owner.

## Boundary under test

The approved checkpoint is: **Does Compare make alternatives easier to judge than
prose without losing important facts or forcing an inappropriate visual form?**
This trial tests that question without treating valid syntax, a correct idiom
choice, or a polished capture as success by itself.

Compare is reviewed as a narrow relational idiom: alternatives may be placed
against explicitly shared, decision-relevant criteria. It must not invent a
ranking, winner, recommendation, score, total, causality, certainty, preference,
or equivalence. Unknown, unavailable, and not supplied remain distinct. A
criterion's unit, date/time basis, source scope, and consequential qualifier stay
with the claim they qualify. If no faithful shared question exists, prose is a
valid outcome.

## Dottie evidence

Read-only consultation `8fe922e1-ed4d-42be-ba91-2f21e3555811` (2026-09-09)
identified the sharpest additional test as distinguishing “an unknown answer to
a valid shared question” from “no valid shared question.” It recommended checking
that a neat grid does not manufacture correspondence, and that wrapping/scrolling
does not strand a value away from its alternative, criterion, unit, or condition.
It also suggested testing omission of a consequential exception. Dottie found no
independently verified additional Product Owner decision and no HUD-specific
contract in the searched knowledge base. This advice is evidence, not approval.

## Independent author packet

The frozen author-facing input consists of the finished Compare discovery
entrypoint plus the five fact packets under
`docs/assets/compare/trial/fact-packets/`. `supplier-anchor.txt` is a byte-for-byte
copy of `examples/sequence-suppliers.txt`; that supplied prose is retained as the
baseline, not counted as novel transfer. The packet deliberately does not reveal
expected idiom choices.

The freeze was made at 2026-09-09T19:13:11Z. SHA-256 fingerprints for the
pre-trial entrypoint, implementation, contract brief, and all packets are
recorded in `docs/assets/compare/trial/frozen-inputs.sha256`; an exact
pre-trial source/discovery archive is under
`docs/assets/compare/trial/frozen-before-discovery-correction/`, and an exact
author-input copy is under `/tmp/hud-compare-author-input/` with its own
manifest. After the trial, the discovery-only correction added supported limits,
a complete minimal JSON shape, and the unchanged Sequence authoring link. The
old manifest is checked against the archived frozen copies separately from the
current corrected files; current-file hashes and the recorded limits must be
reported independently.

Cases:

- supplier alternatives: three fictional quotes with USD, one-batch/date scope,
  pickup/delivery and recycled-paper facts, unknown delivery charge/final total,
  and explicit no-winner provenance;
- venue options: two workshop venues with shared date/time, units, an unknown
  access detail, an explicitly unavailable access detail, and other missing terms;
- archive services: two offers where transfer-start and museum-availability dates
  must not be conflated, and one-time setup must not be totaled with recurring
  storage;
- reimbursement note: one claim with no alternatives or supplied procedure, where
  concise prose should remain a valid form.

The fixed Sequence counterexample is the exhibit run sheet: it contains one
supplied operational order, and is included to test uncoached selection among
Sequence, Compare, and prose rather than to require a Compare rendering.

## Author protocol

After the builder's implementation and discovery are frozen and hashed, a fresh
author (not the builder or coordinator) receives only the frozen entrypoint, the
five packets, and operational constraints. The author must choose the form and
compose from facts without selection or layout coaching. The author records draft,
final, validation, and inspection evidence under a separate temporary directory;
repository source and fixtures are read-only.

The author must validate/inspect at **72x35** and **80x24**, including all content.
The record must state actual failures, repairs, revisions, or why none occurred.
Prose, a declined idiom, or valid JSON are all permitted outcomes. Re-running
after coaching is not a second fresh success.

## Review dimensions

Review is independent of author choice:

1. **Selection** — is the selected idiom appropriate to the communicative intent,
   including justified declines?
2. **Fidelity** — are every material fact, unit, date/time, source scope,
   unknown/unavailable state, and consequential qualifier preserved without
   unsupported inference?
3. **Relational integrity** — can a reader identify each value's alternative and
   criterion at both sizes, without a misleading sequence, ranking, or false
   equivalence?
4. **Legibility and effort** — do wrapping, scrolling, hierarchy, and density make
   judgment easier than the prose baseline, and is the cost of inspecting all
   content recorded?
5. **Isolation** — are the example data and renderer changes confined to the
   approved data-only slice, with no production runtime/dependency effects?

Reviewers preserve immutable author drafts, finals, raw previews, and reports.
A material omission or rendering failure is recorded as a trial result; history is
not rewritten into a flawless run. Any correction is a separate reviewed demo,
clearly labeled post-trial.

## Completion evidence

Before handoff, record actual values (not lifecycle assumptions):

- builder report, discovery/source and implementation paths;
- frozen entrypoint, implementation, and packet hashes;
- author raw/final artifacts and actual 72x35/80x24 inspections;
- independent review findings, interventions, and bounded limits;
- `./scripts/check` output and test count;
- protected Sequence fingerprint verification against
  `/tmp/hud-compare-sequence-baseline.json`;
- demo pane/PTY dimensions, capture paths, and any remaining process handles.

This record does not claim Compare acceptance or authorize merge, publication,
release, installation, or a broader palette.

## Builder handoff and bounded repair

The builder (`hud-compare-builder`, `w7:pC`) completed its report after writing the
brief before implementation, then stopped. Its report was retrieved from the
pane and recorded in the repository-local implementation evidence under
`docs/assets/compare/`. It reported `./scripts/check` passing with 80 tests and
172 protected Sequence fingerprints unchanged. Independent coordinator review
found two narrow issues before freeze: the supported-minimum 64x12 end capture's
partial warning did not identify its criterion, and the access fixture encoded a
semantically unspecified replay window as `known`. The builder repaired the
warning to name the criterion, changed the replay window to explicit `unknown`
with qualification, refreshed captures/tests, and reran `./scripts/check` (80
tests, pass). No Sequence file or coordinator-owned trial file was changed by
that repair.

## Fresh-author result (one uncoached run)

The fresh author was `hud-compare-fresh-author` in the reused owned `w7:pC` pane.
It received only the frozen Compare entrypoint and five fact packets, not this
record or evaluator guidance. It chose Compare for `archive-services`,
`supplier-anchor`, and `venue-options`; ordinary prose for
`reimbursement-note`; and described `exhibit-run-sheet` as a Sequence choice.
That last result is a semantic selection observation only: the final artifact is
numbered Markdown prose, not Sequence JSON, and was not invoked or rendered
through HUD's Sequence entrypoint. No successful Sequence invocation is claimed.
This demonstrates an uncoached choice pattern, not broad reliability or Product
Owner acceptance.

The complete raw run, immutable drafts/finals, hashes, commands, and report are
archived at `docs/assets/compare/trial/author-run/`; the original report is also
at `/tmp/hud-compare-fresh-author/report.md`. The author had real shape-check
failures from the 60-cell metadata limit, then independently shortened fields,
repaired a missing JSON comma, and achieved exit-0 checks. It rendered all three
Compare finals at exactly 72x35 and 80x24, plus 80x24 end views. It reported
explicit continuation costs: supplier content fit at 72x35 but scrolled at
80x24; venue and archive content continued at both sizes. The sequence and prose
outputs were fully inspected as text. No author coaching or post-coaching rerun
occurred.

## Independent fidelity review and correction

Coordinator inspection of the final files and captures found a material
qualification omission in `final/archive-services.json`: the packet said the
meaning of “available” beyond the supplied wording was not supplied, but the
final placed the two dates under `Collection available to museum` without that
qualification. The final compressed egress/retention unknowns and retained the
first-six-month storage qualifier, but did not preserve this separate
interpretive limitation. The supplier final also omitted the supplied absence of
quality ratings, opening hours, and a budget ceiling; the venue final compressed
missing cancellation terms, microphone availability, and final taxes into
“other terms unknown.” These are recorded as fidelity gaps, not silently treated
as author success. The visible repetition of `UNKNOWN · not supplied` followed
by `not supplied` is an effort/readability cost, not a semantic failure in this
run; ownership and unknown/unavailable distinctions remained visible.

Read-only Dottie scrutiny `614bfc10-39f6-461f-82d0-b3b098593255` agreed that the
“available” omission is a material, narrowly scoped fidelity failure, while the
repeated unknown wording is redundant cost unless it breaks association or
state. This was based on the supplied artifact summary, not an independent
inspection and is not PO authority.

The original author evidence remains unchanged. A separately labeled reviewer
correction is archived at
`docs/assets/compare/trial/reviewed/archive-services-corrected.json`, with actual
72x35, 80x24, and 80x24-end captures and a passing shape check beside it. The
correction attaches the missing “available” qualification to both date answers
and records the other omitted scope limits in intent. It is a post-trial demo of
qualification handling, not a second fresh success and not a rewrite of history.

## Final coordinator verification

After the trial artifacts and review docs were written, `./scripts/check` passed
with exit 0: 80 tests passed (39 library, 8 CLI integration, 1 examples
integration, 6 Compare, 8 expressive-story, 14 Sequence, and 4 story-preview).
The post-correction captured output is
`docs/assets/compare/trial/check-posttrial-discovery.log` and its exit marker is
`check-posttrial-discovery.exit`; `git diff --check` also passed. The original
frozen manifest is checked against archived copies via
`frozen-before-discovery-correction/frozen-copy-sha256.sha256`, separately from
current files. Current-file hashes are recorded in
`current-after-discovery-correction.sha256`; the discovery correction's
supported limits are recorded in `skills/hud/compare.md` and are not represented
as having been trialed: 65,536 input bytes; 60-cell title/provenance/scope,
40-cell criterion names, 24-cell alternative names; 500-scalar other strings;
2–8 criteria; 2–6 alternatives; exactly 2–8 answers per alternative; 1–240 ×
1–100 headless preview; and 64 × 12 minimum live area.
The protected Sequence manifest was rechecked: all 172 files matched
`/tmp/hud-compare-sequence-baseline.json`.

The actual demo used only owned panes: Compare ran in `w7:pD` and was captured
against the retained supplier prose at the same measured PTY size, 71 columns ×
35 rows, in `docs/assets/compare/trial/demo/`. The Compare process was quit
cleanly; `w7:pD` and reused builder pane `w7:pC` were shell-verified afterward.
No fresh-author or builder process remains. The coordinator pane `w7:pG` remains
alive for handoff; the primary `w7:p1` retained focus throughout. No git,
remote-memory, publication, dependency, installation, or production-runtime
operation was performed.
