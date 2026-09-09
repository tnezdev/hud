# Sequence reference contract — bounded experiment

Status: engineering hypothesis for the approved semantic-language direction,
not a production API or Product Owner acceptance. Written before implementation.
Continuity/design consultation: Dottie `18bb2220-f26b-4261-939b-383c12159b39`.
She recovered older September 9, 2026 advice but could not independently access
our current local product document. New feedback was supplied to her explicitly.
No remote knowledge writes were requested or reported.

## Meaning, why, and when

Sequence presents supplied items whose relative position communicates one stated
ordered relationship. Numbers identify local positions; connectors express
continuity of that relationship, **not** elapsed time, causation, importance,
execution, completion, or completeness.

Choose Sequence when seeing successive positions together helps someone follow a
procedure or understand a chronology. Ask whether swapping items would
misrepresent the stated relationship, not merely change the reading experience.
This test is necessary, not sufficient: ranking also depends on order but is not
Sequence. The first experiment supports only:

- **Procedure:** a supplied order for doing something; not an execution engine.
- **Chronology:** a supplied temporal order of events; not evidence of causation.

Use one relationship throughout. A procedure can mention dates, but mixing past
events, recommended actions, and predicted phases as though they share one order
is misleading. Do not manufacture steps to fill a template.

Decline for alternatives, rankings, independent checks, tied/unknown event order,
branching or parallel dependencies, or a preferred order for explaining topics.
Do not linearize interchangeable actions without evidence. Ordinary prose is a
valid response; another HUD idiom is not required. General explanatory progression
and process-stage variants are deferred, not permanently rejected.

## Semantic inputs: agent responsibility

- Audience-facing title and intent: why this sequence matters to this audience.
- Provenance: supplied, fictional, observed, or otherwise qualified source status.
- Scope: complete supplied procedure, selected events, excerpt, or one specified
  path. Never imply a partial procedure is safe to execute as a complete one.
- One relationship (`procedure` or `chronology`).
- At least two ordered items, each with a meaningful label and optional detail.
  Relative order must be supported by the facts. Labels are semantic claims,
  not typography knobs; preserve necessary conditions in them.
- Optional common notes for background/source detail, not exceptions that reverse
  an earlier claim. Empty filler sections are unnecessary.

Qualifications belong with the smallest claim they change and before reliance:
“After approval, submit” rather than “Submit” with approval buried in notes;
“Estimated 14:00 arrival” rather than an unqualified time plus an end disclaimer.
Sequence-wide temporal/source scope remains visible while scrolling. Detail may
elaborate; it must not be the only location of a condition that reverses the label.
The parser can validate shape, not truth, suitability, or correct qualification.

## Presentation obligations: HUD responsibility

- Preserve the demonstrated numbered connected spine and reading rhythm.
- Separate scanable item labels from elaboration; wrap within the item's text
  column, not back beneath its number. Keep continuation associated with its
  numbered item, including when the top of a scrolled view begins mid-item.
- Labels may wrap. No forced editorial shortening, ellipsizing, collapsing, or
  dropping details to make a screenshot fit. A label that spans screens remains a
  real limit; authors must not move essential conditions into trailing detail.
- Keep title, provenance and scope persistent. After the PO's header-busyness
  feedback, the [opening-hierarchy refinement](sequence-header.md) places the
  relationship/caveat on the sequence landmark; it scrolls with the sequence,
  not the header. This explicitly relaxes the earlier all-four-persistent choice.
  Use ordinary emphasis for labels, never unsupplied current/completed-state
  markers or proportional time spacing. Connectors mean order only.
- Make overflow and position explicit; support access to every body line.
  **Post-trial correction, specified before code:** one legal-position rule for
  keys, preview, End and resize snaps fitted paragraphs to their beginning, never
  leaving an unmarked leading fragment. j/k move between legal positions. Page
  movement progresses without skipping unread text, overlapping where possible.
  End uses the first legal position whose remaining tail fits (blank bottom rows
  are acceptable). A paragraph taller than the body permits interior positions,
  with a reserved top row explicitly marking earlier hidden text; classification
  uses the full body height consistently, and the marked fragment uses one less
  row. End may lie inside such an oversized final paragraph. A footer explicitly
  marks a paragraph cut at the bottom as continuing, rather than merely saying
  more content exists. This protects the top edge, not semantic self-sufficiency
  of every visible paragraph: detail can still depend on preceding context.
  Scrolling costs remain evidence, not concealed by “more below.” At smaller than
  the supported minimum show a resize message, not clipped authoritative guidance.
- After PO feedback about locating sections, the bounded
  [section-rhythm pass](sequence-rhythm.md) gives major transitions more space and
  open-rule landmarks while keeping each label/detail unit compact. Section
  landmarks belong with their following first paragraph, not a stranded heading.
  This presentation refinement changes neither schema nor author wording.
- Display data stays inert. No shell/network/actions, includes or interpolation.

## Different semantic examples

**Procedure:** help a volunteer hand over a loaned audio kit. The supplied procedure
is to count the returned kit, record discrepancies, then sign the receipt only
after the inventory agrees. The final label must retain “only after inventory
agrees.” This order guides actions; it reports no completed work.

**Chronology:** help a reader understand selected observatory log entries: first a
sensor warning, then an inspection, then a later normal reading. Scope says
selected entries on one date, not a complete incident record. The order establishes
neither that inspection fixed the sensor nor that the warning's cause is known.

**Counterexample:** three venues sorted by price are alternatives, not stages of
visiting every venue. Numbers and a sorted source list do not justify Sequence.
Likewise, two simultaneous detections cannot honestly become first/then events.

## Example implementation plan (frozen before author trial)

A separate `examples/sequence.rs` reads one data-only JSON file. Do not change the
old expressive prototype, its fixtures, production `src/`, dependencies, or WIP.
Concrete experimental fields: `title`, `provenance`, `scope`, `intent`,
`relationship`, `items: [{label, detail?}]`, optional `notes: [string]`.
No lead/route/support adapter or generic component tree.

Boundary: deny unknown fields at every object level; 64 KiB maximum UTF-8 input;
2–12 items; 0–12 detail strings per item and common notes; nonblank strings of
at most 500 characters, rejecting terminal and bidi controls. Persistent title,
provenance, and scope are limited to 60 terminal cells each. Relationship is an
enum. This is a bounded authoring experiment, not a negotiated production format.
Use regular local files; a byte limit is not a timeout for devices/FIFOs.

CLI: `cargo run --example sequence -- FILE.json [--check | --preview W H [OFFSET]]`.
Live/headless share rendering. Preview supports 1–240 columns, 1–100 rows; live
minimum 64x12. j/k/arrows, PgUp/PgDn, Home/End and q/Esc/Ctrl-C match the existing
example; no refresh. I/O stays at the edge. Ratatui measures/wraps prose in separate
paragraphs; the renderer supplies a fixed relationship gutter, not a custom word
wrapper. Overflow does not reorder content. No new dependencies.

Tests cover boundary errors and inert strings, qualifier/fact visibility across
scrolling, hanging alignment and mid-item identity, Unicode, small/long views,
scroll clamping and real emphasis. Fixed-wording legacy journey comparison is
separate from fresh-author semantic transfer. Preserve original baseline captures.

## Trial and review plan

Freeze the author entrypoint and implementation with hashes. Give one fresh
author two suitable stories of different relationships and one unsuitable case
with superficially attractive ordering. Do not reveal which should use Sequence,
this evaluator brief, expected output, or layout coaching. The discovery reference
itself must teach selection. Each case needs intent, choice/reason, preserved facts
and qualifiers, and actual validation/inspection/refinement evidence at 72x35 and
80x24 including scrolled content. Choosing prose needs no render.

Archive prompt, initial/final artifacts, available trace, captures, validation
failures, repairs, and coordinator interventions honestly. No forced revision if
inspection suggests none. Hold source wording constant for renderer comparisons.
Obtain independent review of actual evidence and run `./scripts/check`.

The frozen author trial revealed a negation lost at the top scroll boundary:
“no” disappeared before “actual identifier is supplied.” Dottie evidence review
`11eea3bb-9060-4cb2-9bb4-c68aaf9f436d` required a presentation correction as well as
factual demo corrections. Follow-up `04458abd-59a2-4a7f-8838-f1bcaf2f4b51` supported
the bounded snapping/disclosure approach above as advice, not implementation
verification. Preserve the original trial/code/entrypoint; later regressions and
corrected captures are post-trial engineering, not a coached rerun. Test the exact
negation through keys, preview, End and reflow, plus page coverage, effective-height
edges and an oversized final paragraph.

Observe separately: selection quality; semantic fidelity (including invented
necessity, status, certainty, completeness or causality); visual grouping and
scroll cost; authoring effort. One author and two positives do not prove broad
reliability, every possible Sequence meaning, or human comprehension. Present the
experience and meaningful tradeoffs to Travis as Product Owner. Stop before
palette expansion, production promotion, merge, release, installation, publication,
or remote-memory writes.
