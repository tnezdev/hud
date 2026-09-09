# Compare reference contract — bounded experiment

Status: engineering hypothesis for the approved semantic-language direction,
not a production API or Product Owner acceptance. Travis explicitly approved
Compare as the one next reference after accepting Sequence. This brief was
written before implementation.

## Consultation and decision boundary

Focused read-only Dottie consultation `938f3d12-574f-44c5-b8bf-3215d22e45bb`
recommended that Compare promise correspondence, not equivalence: alternatives
answer the same named questions, without asserting interchangeability,
commensurability, or preference. She recommended audience/decision intent,
explicit scope and provenance, named alternatives, shared named criteria, and an
explicit response for every alternative/criterion pair. She said missing values
alone need not decline; a meaningful common question with an explicit gap is
useful. She advised declining when alignment requires unsupported normalization,
assumptions, or obscures consequential qualifications. Her answer is design
advice, not approval or implementation authority; she did not inspect this
checkout. It referred to historical September 9, 2026 evidence and confirmed
that the newer supplied report of Travis's Compare approval supersedes the old
lack of next-feature commitment. No remote writes were requested or reported.

The product checkpoint is:

> Does Compare make alternatives easier for a person to judge than prose without
> losing important facts or forcing an inappropriate visual form?

Travis remains Product Owner. This experiment does not claim that Compare is
accepted, that a production authoring API exists, or that a separate resolver or
palette is authorized.

## Meaning, why, and when

Compare presents **named alternatives answering the same explicitly named
questions**. The visual relationship is correspondence by criterion: the person
can inspect what each alternative says about the same consideration. It does not
mean that the alternatives are equivalent, interchangeable, equally measured,
or ranked.

Choose it when a person must judge a small set of alternatives side by side and
prose would make correspondence expensive to find. State the decision or
question, the audience, the scope, and the provenance. Preserve the supplied
alternative order; it is not an automatic sort or recommendation. Use ordinary
prose when a short explanation is clearer, when correspondence offers no real
advantage, or when a recommendation and its reasons are the actual message.

Do not use Compare for:

- a procedure, chronology, dependency, or parallel work (use Sequence only for
  its bounded supplied procedure/chronology meaning, otherwise use prose);
- alternatives that answer different questions or cannot share a defensible
  criterion without normalization;
- scores, totals, ranks, winners, “best” styling, or recommendation machinery
  not supplied by the agent;
- values whose currencies, quantities, time bases, bundles, or source accounts
  conflict unless the agent supplies a truthful common basis;
- a collection that merely happens to have several records.

A missing answer is not a zero, false value, or automatic reason to decline. Show
an explicit unknown or unavailable state when the criterion is meaningful. Decline
if the gap or its qualification cannot stay intelligible near the claim at the
available width. If prose communicates the caveat more faithfully than aligned
responses, choose prose.

## Semantic inputs: agent responsibility

The bounded example-only document contains:

- `title`: audience-facing subject;
- `provenance`: supplied/fictional/live status and verification limits;
- `scope`: the population, date, quantity, currency or other boundaries;
- `intent`: what the audience needs to judge;
- `criteria`: 2–8 objects, each with a unique named `name` and a concise
  `question` describing the same question for every alternative;
- `alternatives`: 2–6 objects in supplied order, each with a unique `name` and
  exactly one `answer` for each criterion;
- each answer's `criterion`, explicit `state` (`known`, `unknown`, or
  `unavailable`), optional `value`, and optional `qualification`.

A `known` answer has a nonblank value. `unknown` and `unavailable` have no value;
they are rendered as distinct visible states. A qualification is required for an
unknown/unavailable answer when the agent has a reason the audience needs to
interpret that state, and is shown with the smallest answer it alters. Global
provenance and scope are not a distant notes graveyard: they stay visible while
scrolling. The parser validates shape and bounded vocabulary, not factual truth,
comparability, or whether the agent chose Compare well.

The first implementation bounds the UTF-8 input to 64 KiB; all display strings to
500 Unicode scalar values; title, provenance and scope to 60 terminal cells;
2–6 alternatives; 2–8 criteria; and exactly 2–8 answers per alternative. It rejects unknown fields at every object level,
missing/duplicate criteria or answers, nonblank violations, terminal or bidi
controls, unsupported states, and values on unknown/unavailable answers. It does
not accept styling, coordinates, sorting, formulas, totals, commands, includes,
links, interpolation, Markdown interpretation, or natural-language resolution.
These are experiment limits, not negotiated production requirements.

## HUD presentation obligations

HUD owns the display consequences of the semantic composition:

- Keep the title, provenance, and scope in a calm persistent header with one
  outer top breathing row, following the accepted Sequence hierarchy without
  copying Sequence's numbered spine.
- Keep intent near the comparison landmark, then group each common criterion with
  its question and all alternatives' answers. Use a stable alternative label
  gutter and a value column; wrapped values remain in that value column and keep
  their alternative identity.
- Make `unknown` and `unavailable` visibly different from known values and from
  one another. Never render them as blank, zero, false, or a favorable/unfavorable
  marker. Keep answer qualifications adjacent to the answer they change.
- Do not add rank numbers, winner emphasis, score bars, totals, sorting, or
  recommendation language. The renderer may label the relation as comparison
  and no ranking, because that is a presentation safeguard, not supplied fact.
- Use modest section rhythm: related answer lines stay compact; criteria get
  clear landmarks and spacing. Do not turn each answer into a card or use a
  generic table API as the semantic contract.
- At narrow widths, retain criterion grouping and alternative ownership. Wrap
  under the value column rather than silently rotating the layout into a
  chronology, clipping qualifications, or pretending a side-by-side matrix is
  still legible. Below the supported minimum, show a resize message.
- Make overflow and continuation explicit. Every body line must be reachable
  through headless preview offsets and live j/k, PageUp/PageDown, Home/End. The
  preview is rendered with the same function as live mode. Persistent context
  remains visible while scrolling, at the cost of body height.

The presentation deliberately chooses criterion-first rows instead of
alternative-first columns. This costs repeated alternative labels and some
vertical space, but protects correspondence and qualifications at 72×35 and
80×24. It is a bounded terminal treatment, not a general layout framework.

## Anchor: supplier alternatives

`examples/sequence-suppliers.txt` is preserved byte-for-byte as the corrected
fictional prose baseline. The Compare fixture uses its exact supplied facts:
three independent quotes in USD for 100 handouts and one batch on 6 May 2026;
Alder 40 USD, pickup Friday, recycled paper unavailable; Birch 55 USD, pickup
Thursday, recycled paper available; Cedar 70 USD, delivery Thursday, recycled
paper available, with Cedar's delivery charge not supplied and final delivered
total unknown. It retains that no supplier is selected, the list is in ascending
quoted price rather than preference, no quality ratings/opening hours/budget
ceiling are supplied, and the material does not establish a winner or buying
from all three. Provenance says fictional and not live or independently verified.

The Compare fixture must not infer a calendar date from Thursday/Friday, free
pickup, final totals for pickup suppliers, quality, a budget decision, or a
winner. Cedar's missing delivery charge and final delivered total qualify its
amount where shown, not only in a trailing note.

A second materially different fixture, if retained, should challenge the same
contract with a non-price decision and explicit unknowns; it must not introduce a
new visual idiom or imply that the supplier facts generalize.

## Example invocation and evidence plan

This is an isolated data-only Cargo example, not installed HUD and not the
production command-backed runtime:

```sh
cargo run --quiet --example compare -- examples/compare-suppliers.json --check
cargo run --quiet --example compare -- examples/compare-suppliers.json --preview 72 35
cargo run --quiet --example compare -- examples/compare-suppliers.json --preview 80 24
cargo run --quiet --example compare -- examples/compare-suppliers.json --preview 80 24 65535
cargo run --quiet --example compare -- examples/compare-access.json --preview 72 35
```

The exact accepted syntax remains local experiment syntax. Display input is
inert: no shell, network, actions, async/background refresh, persistent state,
commands, or executable configuration. File reading is the explicit edge and
is byte-bounded, not a device timeout. No new dependency is needed.

Record actual rendered cells, dimensions, scroll/end observations, validation
failures and repairs under `docs/assets/compare/`. These cells cannot establish
human comprehension, color perception, live-key behavior, product acceptance,
or broad reliability. Do not run a fresh-author trial or claim independent
selection/fidelity/product acceptance in this implementation task; those belong
to the coordinator's later separate evaluation.

## Open questions and limits

Travis has not yet accepted the exact common-question boundary, whether differing
measurement bases can ever be admitted, or the acceptable scrolling cost. This
implementation therefore declines mixed bases rather than normalizing them. It
also does not decide recommendations, numeric comparison, confidence, or a
production schema. One supplier fixture and one non-price fixture can challenge
this bounded contract but cannot prove that Compare is broadly useful or easier
to judge than prose.
