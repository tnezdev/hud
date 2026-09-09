# Compare implementation evidence

This is implementation evidence, not a fresh-author trial, independent review,
human-comprehension result, or Product Owner acceptance.

## Consultation record

Read-only Dottie consultation `938f3d12-574f-44c5-b8bf-3215d22e45bb` advised
“correspondence, not equivalence”: named alternatives answer the same named
questions without implying interchangeability, commensurability, or preference.
She recommended explicit scope/provenance, one response per alternative/criterion,
and visible distinctions between unknown and unavailable. She advised falling back
to prose when a common criterion requires unsupported normalization or obscures a
qualification. She did not inspect or execute this implementation. No remote write
was requested or made.

## Captures

Generated from the same `examples/compare.rs` renderer used by live mode, with
Ratatui `TestBackend` and no terminal, shell, network, clock, or sleep dependency:

- `suppliers-72x35-offset0.txt` and `suppliers-72x35-end.txt`: the supplier
  composition fits in 28 body lines at this width; the requested end clamps to
  offset 0, so the captures are intentionally identical.
- `suppliers-80x24-offset0.txt`: 29 body lines need continuation. The opening
  reaches through the first criterion's Cedar qualification and starts the next
  criterion; the footer says `MORE below`.
- `suppliers-80x24-end.txt`: End starts at the `Fulfillment terms` criterion,
  not at an answer without its criterion heading, and reaches the recycled-paper
  unknown/unavailable distinctions. Footer says `END`.
- `access-72x35-offset0.txt`: the non-price example fits at this size and shows
  explicit missing accessibility information.
- `access-80x24-offset0.txt` and `access-80x24-end.txt`: the opening costs one
  continuation; the end view retains the Captioning criterion and both distinct
  states. The recorded replay's unspecified access window is explicitly
  `UNKNOWN`, not a known value.
- `suppliers-64x12-offset0.txt` and `suppliers-64x12-end.txt`: the supported
  narrow edge demonstrates greater continuation cost and persistent provenance/
  scope. End starts within a criterion group only with an explicit warning naming
  the criterion (`Recycled paper: not standalone`), rather than silently
  presenting an answer as standalone.
  It is evidence of bounded behavior, not a claim of ideal comprehension.

The captured limits are preview 1–240 columns by 1–100 rows; live rendering asks
for at least 64 columns by 12 rows. Input is at most 65,536 bytes. There are 2–6
alternatives, 2–8 criteria, 500 Unicode-scalar display strings, 60-cell title /
provenance / scope fields, 40-cell criterion names, and 24-cell alternative names.
Unknown fields, unsupported state values, terminal/bidi controls, duplicate or
missing correspondence, and values on unknown/unavailable answers are rejected.

## Fidelity and presentation observations

The supplier baseline is preserved at `examples/sequence-suppliers.txt`; its
SHA-256 is recorded in `sequence-suppliers-baseline.sha256`. The Compare fixture
keeps the three quotes, USD, one batch of 100, 6 May 2026, pickup/delivery terms,
recycled-paper facts, Cedar's unsupplied delivery charge and unknown final
delivered total, no selected supplier/winner, ascending quoted-price ordering, and
fictional/not-live/not-independently-verified provenance. Cedar's qualification is
rendered beside its amount. No score, sort, total, or winner is generated.

The criterion-first treatment repeats alternative labels vertically instead of
using alternative columns. This uses more rows than a wide matrix but keeps
criterion correspondence, ownership, and qualifications legible when the terminal
narrows. At 80×24 the tradeoff is scrolling; at 72×35 the supplier example fits.
No live key behavior, color perception, or reader judgment was tested here.
