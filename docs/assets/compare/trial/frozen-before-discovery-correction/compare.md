---
name: hud-compare
description: Choose the bounded Compare idiom when people need to judge alternatives against the same named questions.
---

# Compare alternatives in the terminal

Compare is an **experimental data-only reference**, not an installed HUD API.
Start with the communicative relationship, not with a table shape.

## Meaning first

Use Compare when a person needs to judge a small set of **alternatives against the
same named criteria**. HUD groups each criterion with every alternative's supplied
answer so correspondence is easy to inspect. That alignment means “these answer
the same question”; it does not mean equal quality, interchangeability,
commensurability, ranking, or recommendation.

Write this sentence before composing:

> For **[audience]**, help them judge **[alternatives]** for **[decision]** using
> **[common questions]**, while preserving **[important unknowns/provenance]**.

Use plain prose when there is no useful common question, when the qualifications
would become harder to read when aligned, when the real message is one conclusion
and its reasons, or when a short explanation is clearer.

## Compare, Sequence, or prose?

- **Compare:** independent alternatives answer the same questions. Example: three
  printer quotes compared by quoted amount, fulfillment terms, and recycled-paper
  availability. An unknown delivery charge stays beside Cedar's amount.
- **Sequence:** supplied procedure or chronology where order itself matters. Do
  not turn alternatives sorted by price into a numbered journey.
- **Prose:** unrelated facts, a recommendation with reasoning, conflicting
  measurement bases, or any case where alignment would require assumptions.

Counterexamples:

- “A, then B, then C” is wrong for three suppliers; ascending price is not a
  procedure and does not establish preference.
- A column called “total cost” is wrong when only a quote is supplied and a
  delivery charge is unknown.
- Blank, zero, false, and “N/A” are not substitutes for unknown or unavailable.
- Do not add a winner, score, sort, percentage, or best styling unless that
  meaning was supplied and the bounded contract explicitly supports it.
- If alternatives use different currencies, quantities, time bases, bundles, or
  source accounts and no truthful common basis is supplied, choose prose.

## Semantic inputs

The example document supplies an audience-facing `title`, `provenance`, `scope`,
and `intent`; named `criteria` with a `name` and `question`; and named
`alternatives` in supplied order. Every alternative answers every criterion once.
Each answer has a `criterion`, a `state` (`known`, `unknown`, or `unavailable`),
and an optional `value` and `qualification`. Known answers require a value;
unknown and unavailable answers have no value and remain visibly distinct.
Qualifications stay with the smallest answer they alter.

The parser rejects unknown fields, malformed correspondence, unsupported states,
control/bidi characters, oversized content, and values attached to unknown or
unavailable states. It does not judge truth or decide whether Compare was the
right idiom. Input is inert display data: no commands, shell interpolation,
network, includes, actions, Markdown, sorting, formulas, or layout controls.

## What HUD presents

HUD keeps title, provenance, and scope in a persistent header with one outer top
breathing row. It groups a criterion heading and question with all alternatives'
answers, uses a stable alternative label gutter, wraps answers under their value
column, and keeps qualifications adjacent. Unknown and unavailable are explicit.
The renderer adds a small “same questions · no ranking” landmark as a safeguard;
it does not add scores or a recommendation. At narrow sizes, criterion grouping
and alternative ownership remain visible instead of silently becoming a sequence
or clipping caveats. Overflow is marked and can be inspected with scrolling.

## Example and invocation

The supplier example retains the corrected prose baseline's exact facts: fictional
quotes not live or independently verified; 40/55/70 USD for one batch of 100 on
6 May 2026; pickup Friday/Thursday or delivery Thursday; recycled-paper
unavailable/available/available; Cedar's delivery charge not supplied and final
delivered total unknown; no selected supplier or winner.

A materially different access-format example exercises non-price criteria and
explicit unknown/unavailable answers. Both are fictional and offline.

From the repository root:

```sh
cargo run --quiet --example compare -- examples/compare-suppliers.json --check
cargo run --quiet --example compare -- examples/compare-suppliers.json --preview 72 35
cargo run --quiet --example compare -- examples/compare-suppliers.json --preview 80 24
cargo run --quiet --example compare -- examples/compare-suppliers.json --preview 80 24 65535
cargo run --quiet --example compare -- examples/compare-access.json --preview 72 35
```

These are Cargo example arguments, not installed `hud` flags. Inspect the opening,
criterion groups, unknowns, qualifications, and an end view. Preview uses an
in-memory terminal backend and cannot prove color perception, live key behavior,
human comprehension, or product acceptance. Do not claim Compare was selected
independently from these coached examples; a later coordinator-owned fresh-author
trial must evaluate selection among Compare, Sequence, and prose.
