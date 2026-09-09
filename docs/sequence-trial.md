# Sequence reference: trial, corrections, and demonstration

This is one bounded reference idiom, not a production API or acceptance claim.
**Subsequent PO feedback:** section navigation remained difficult. The
[section-rhythm follow-up](sequence-rhythm.md) preserves this trial and compares
unchanged stories with a newer presentation; captures/counts below are historical.
[Product direction](semantic-visual-language.md) → [reference brief](sequence-reference.md)
→ [current author entrypoint](../skills/hud/sequence.md).

## Product-facing result

A fresh author chose chronology for selected observatory reports, procedure for a
volunteer intake handoff, and ordinary prose for price-ordered supplier alternatives.
The idiom transferred between two kinds of ordered relationship without layout or
selection coaching. It also exposed fidelity and scrolling failures: choosing the
right idiom and validating its syntax were not sufficient.

The corrected example keeps numbers, connectors, and reading rhythm. Wrapped text
stays in its item's column. Fitted paragraphs now start intact when scrolling;
longer-than-screen fragments and bottom cuts are explicitly identified. There is
no guarantee that arbitrary prose fragments, or even full dependent paragraphs,
are semantically self-contained. No natural-language resolver was added.

Tradeoff for the Product Owner: more dependable grouping and persistent source/scope
cost vertical space. The corrected intake fits at 72x35 but requires scrolling at
80x24. Paragraph-boundary movement is coarser than line movement and can leave
blank space at the end. Bottom-cut paragraphs are marked, not deferred until an
entire paragraph fits. Is that reading experience useful enough, or should we spend
more screen space to keep fitted paragraphs whole everywhere?

## Demonstrate

From the checkout root, one command per authorized terminal:

```sh
cargo run --quiet --example sequence -- examples/sequence-observatory.json
cargo run --quiet --example sequence -- examples/sequence-intake.json
# The unsuitable case is intentionally prose, not another component:
# examples/sequence-suppliers.txt

cargo run --quiet --example sequence -- examples/sequence-intake.json --preview 80 24
cargo run --quiet --example sequence -- examples/sequence-intake.json --preview 80 24 65535
```

j/k/arrows move between legal paragraph/fragment positions; PgUp/PgDn page;
Home/End jump; q/Esc/Ctrl-C quit. Relaunch after edits. No refresh, execution of
story data, automatic publication, or global installation.

Corrected captures: [observatory 72x35](assets/sequence/reviewed/observatory-72x35-offset0.txt),
[80x24](assets/sequence/reviewed/observatory-80x24-offset0.txt),
[intake 72x35](assets/sequence/reviewed/intake-72x35-offset0.txt),
[80x24 top](assets/sequence/reviewed/intake-80x24-offset0.txt),
[end](assets/sequence/reviewed/intake-80x24-offset65535.txt).
These are actual rendered cells, not screenshots or human comprehension evidence.

## Contract and freeze

Dottie consultation `18bb2220-f26b-4261-939b-383c12159b39` helped narrow supplied
procedure/chronology versus ranking or an author-chosen explanation order. She
strengthened explicit scope and qualification placement. This was design advice,
not additional PO requirements. She could recover historical consultation evidence
but could not independently retrieve the current local product document; newer PO
feedback was supplied explicitly. [Returned answer](assets/sequence/dottie-brief.json).
No remote-memory/project writes were requested or reported.

The brief and discovery entrypoint preceded implementation. The separate Cargo
example changes neither production `src/` nor the old expressive prototype or its
fixtures. It uses no new dependencies, general component tree, resolver, or palette.

The original trial-time paths/hashes are in
[frozen-entrypoint.sha256](assets/sequence/frozen-entrypoint.sha256). Both matched
before and after the author run. Subsequent review corrections intentionally
changed the live implementation/entrypoint; they are **not** a frozen-trial rerun.
The exact old [entrypoint](assets/sequence/frozen-entrypoint.md) and
[implementation](assets/sequence/frozen-implementation.rs.txt) remain archived;
[frozen-archive.sha256](assets/sequence/frozen-archive.sha256) checks those copies.
Run these from the checkout root (do **not** check the trial-time manifest against
intentionally modified live paths):

```sh
sha256sum -c docs/assets/sequence/frozen-archive.sha256
# The post-correction implementation/entrypoint were later archived before
# the section-rhythm pass. Their old live-path manifest is historical too.
sha256sum -c docs/assets/sequence/rhythm/before/archive.sha256
```

## Fresh author: actual selection, effort, and limits

Fresh pi author `hud-sequence-author`, created pane `w7:pB`, received this
[exact prompt](assets/sequence/author-prompt.md) and the frozen discovery reference.
Its normal project instruction context was present; the prompt prohibited reading
the evaluator brief, existing fixtures, source, or other-agent artifacts. No
selection answers or layout coaching were supplied, and no coordinator follow-up
was needed. No claim of context-free invention or tool-enforced isolation is made.

The packet contained, without identifying expected choices:

- A: selected fictional observatory reports, one date/local times, approximate
  middle timestamp, report attribution, unknown cause and current state.
- B: numbered ascending-price quotes for independent alternatives, including an
  unknown delivered total, explicit USD, batch/date scope and fictional status.
- C: a complete supplied intake procedure, an identifier source, annotation
  cautions and a mandatory recorded-permission prerequisite. This is one gated
  linear path, not a branching procedure.

Preserved [author directory](assets/sequence/author/),
[author report](assets/sequence/author/authoring-record.md), and
[terminal excerpt](assets/sequence/author-terminal-excerpt.txt) retain drafts,
finals, checks, captures, commands, observed reads and the author's reasoning.
The excerpt is not a complete session transcript. The author-created
`case-a-initial-check-error.txt` was reconstructed after the fact and omits “each”
from the actual diagnostic; it is **not original stderr**. The failed check was
also observed in the live tool trace. No exhaustive operation count is claimed.

| Observation | Actual result |
| --- | --- |
| Selection | A chronology, C procedure, B prose; appropriate for all three supplied cases. |
| Validation | Initial A draft exceeded the persistent scope's 60-cell limit. The batch stopped there. The author preserved its draft, shortened scope without losing date/local/incomplete-log qualifiers, then validated A/C successfully. |
| Inspection | A/C at 72x35 and 80x24, offsets 0/12 and 0/8/16/end respectively. Offset requests beyond the document end clamped; several captures are identical, not extra distinct screenfuls. |
| Refinement | Only A's validation repair. No post-render prose revision was considered warranted. B/C draft and final match. |
| Effort | One data file for each Sequence plus a prose file; one observed repair. No matched timing, baseline effort measure, or broad reliability claim. |
| Fidelity | A retained material facts. C omitted the source of the identifier. B lost explicit currency and fictional/not-live/not-independently-verified provenance. All persisted despite correct selection. |
| Presentation | Grouping works at inspected widths, but original End hid a negation and showed a misleading fragment. The author did not identify this error. |

The first local independent reviewer (`hud-sequence-review`) read code/spec/tests
and rerendered the journey comparison; it reported no actionable defects and
passed all 67 then-existing tests. It did not run live TTY interaction or review
the author's trial. [Review excerpt](assets/sequence/reviewer-terminal-excerpt.txt).
Its result did not catch the subsequently demonstrated scroll defect.

## Dottie evidence review and corrections

Dottie `11eea3bb-9060-4cb2-9bb4-c68aaf9f436d` reviewed the actual supplied prompt,
final A/B/C artifacts and narrow captures pasted into the request. She did not
retrieve local files, run code, check hashes, or inspect the omitted source/tests,
author report, drafts and trace. [Answer](assets/sequence/dottie-evidence.json).

Findings and coordinator corrections, separate from the immutable raw trial:

1. **C operative omission:** restore “from the intake sheet” in the first label of
   `examples/sequence-intake.json`. Other source values are unchanged.
2. **B specificity/provenance:** `examples/sequence-suppliers.txt` explicitly says
   fictional, not live or independently verified, and 40/55/70 USD. Fix the minor
   “numbers below” mismatch to “Listed in ascending quoted price.” No winner added.
3. **C material presentation failure:**
   [original End](assets/sequence/author/captures/case-c-80x24-end.txt) began
   `01 │ actual identifier is supplied for this fictional example.` The preceding
   “no” had scrolled away. The gutter retained item ownership, not meaning.

A is copied byte-for-byte to `examples/sequence-observatory.json`. No author was
coached into repairing the trial or asked to pretend a second clean run occurred.

Dottie follow-up `04458abd-59a2-4a7f-8838-f1bcaf2f4b51` supported a bounded correction:
one shared legal-position rule for keys/preview/End/reflow; fitted paragraphs
begin intact; oversized interior fragments reserve a warning row; bottom cuts
explicitly disclose continuation. She stressed that structural paragraphs are not
proof of semantic independence and did not inspect the resulting implementation.
[Design answer](assets/sequence/dottie-scroll-design.json). The brief and current
entrypoint were updated **before** this code change.

Post-trial regressions exercise the exact raw and corrected C fixtures through
all preview positions and legal key paths, reflow, oversized final paragraphs,
effective-height edges, forward page coverage, and fixture correction equality.
The unchanged raw C under the corrected renderer is separately captured at
[offset 5](assets/sequence/reviewed/unchanged-author-intake-80x24-offset5.txt)
and [End](assets/sequence/reviewed/unchanged-author-intake-80x24-offset65535.txt).
The first now retains both the beginning and “no”; End no longer begins mid-detail.
This is a fixed-wording renderer check, unlike comparing different story intents.

## Earlier fixed-wording timeline comparison

[Journey captures](assets/sequence/journey/) compare the preserved old prototype
with the initial Sequence renderer at 72x35/80x24. Route strings and supporting
sentences are identical, checked by a unit test. The full composition is **not**
an identical-wording A/B: Sequence merges introductory lines with punctuation,
promotes supplied one-day scope into persistent context, and changes labels.
Claim improved timeline continuation grouping, not a controlled whole-screen
readability or speed advantage. These initial captures remain unchanged after the
post-trial scroll correction.

## Engineering status and remaining limits

- `./scripts/check` passed **71 tests**, formatting and Clippy after the correction.
  No dependency versions/lockfile changed. The separate advisory gate was not
  rerun; no local audit pass, release readiness or new CI result is claimed.
- Initial engineering tests were corrected to resize TestBackend itself and not
  demand overflow when a wide view fits. Post-correction compilation caught a
  moved test viewport vector; its test setup now clones the position list.
  These were engineering iterations, not author interventions.
- Persistent header fields still have a 60-cell authoring limit. The observed
  repair is evidence of that friction; HUD has not solved arbitrary header prose.
- No guarantee of factually self-contained fragments. Bottom disclosure versus
  always-whole fitted paragraphs remains a meaningful presentation tradeoff.
- One author, two positives and one negative do not establish broad selection,
  factual reliability, reader comprehension, or production architecture.
- No palette expansion, WIP integration, push, PR, merge, release, installation,
  remote-memory update, or Product Owner acceptance is implied.

## Post-correction independent review

A fresh local `hud-sequence-review-fix` reviewed the archived/current source and
entrypoint, raw/corrected facts, narrow renders, oversized paragraphs and captures.
It established no runtime/semantic defect, and independently ran the then-70-test
gate. Its two follow-ups were clearer archived-hash verification and actual
rendered-word coverage along page navigation, rather than relying only on legal
positions/progress. The commands above clarify verification without changing the
old manifest; a coordinator regression now checks every unique token across actual
PageUp/PageDown render paths at three sizes. That test brings the local gate to71;
no separate independent re-review of the added test is claimed.

Two overly broad synthetic model commands timed out after120 seconds and provide
no coverage. The coordinator bounded further exploration. A provider WebSocket
error interrupted reporting; a follow-up requested the final summary from completed
checks only. Neither intervention involved the fresh author. The reviewer made no
repository changes, did not consult Dottie or test a live TTY, and made no PO
acceptance claim. [Final review excerpt](assets/sequence/reviewer-fix-terminal-excerpt.txt).

## Live observation and prepared PO demo

The corrected intake ran in an owned Herdr pane at **90x17** content cells (`stty
size` verified). Repeated j moved from MORE at line1/22 to END at line12/22; k
returned to line1. Title/provenance/scope/relationship persisted. q returned to the
shell, verified by foreground process inspection before relaunch.
[Top](assets/sequence/reviewed/live-scroll-top.txt),
[end](assets/sequence/reviewed/live-scroll-end.txt),
[dimensions](assets/sequence/reviewed/live-scroll-size.txt).
PageUp/PageDown and Home/End are automated-test covered, not live-key verified.

The final paired demo is **90x35 chronology** and **88x35 procedure**, reflecting
the actual resized terminal, not a claim of a live72x35 inspection. Both reach END
in their initial view. Narrow72x35 and80x24 evidence above is headless. Captures:
[chronology](assets/sequence/reviewed/live-observatory.txt),
[procedure](assets/sequence/reviewed/live-intake.txt),
[layout](assets/sequence/reviewed/live-layout.json). No pixel screenshots or human
acceptance are claimed.

Operational state at capture: caller `w7:p1` retained focus/cwd; author `w7:pB` and
temporary sizing pane `w7:pE` were quit/verified at shell as applicable and closed.
Both review agents exited normally. Owned pane `w7:pC` was reused for chronology,
and owned `w7:pD` runs procedure; these two demos are intentionally left open for
PO inspection. Existing unrelated `w7:p2` in another tab is untouched. Rediscover
live topology and foreground processes before later control or cleanup.
