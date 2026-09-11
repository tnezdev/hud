# Next session: semantic references checkpoint

## PR #42 diagnostic-safety follow-up

The complete branch was committed through `6cbd8b8` and published as
[PR #42](https://github.com/tnezdev/hud/pull/42). Its full-branch review found
terminal-control injection through decoded Serde error text despite green CI.
Travis authorized the bounded fix; see
[example-diagnostic-safety.md](example-diagnostic-safety.md).

The three example CLIs now encode handled diagnostics at one example-only output
boundary. Ordinary cleanup uses non-printing restoration and preserves both
operation/cleanup errors. Focused re-review found no blockers after catching and
correcting that cleanup bypass in the first candidate. The local gate passes
109 tests plus formatting/Clippy; verify current commit and hosted CI/audit on
GitHub rather than reusing the old head's green checks. No merge was authorized.

Accepted rendering/style/navigation, fixtures, actual discovery bytes, production
and dependencies remain unchanged. Only the three examples' diagnostic/cleanup
plumbing and tests intentionally differ from the prior source checkpoint. Frozen
trial sources and all original manifests remain unchanged; current equality with
the 20 run-1 frozen source/discovery files is now 18/20, not the historical 20/20.
The Compare discovery banner's “has not been trialed” wording predates run-1; the
current-status erratum is in the PR and diagnostic brief, not a replaced trial input.

## Publication decision — one PR for the complete branch

Travis subsequently authorized committing the trial/PO feedback, pushing the same
`feat/sequence-reference` branch and opening one PR against `main`. Its scope
includes the expressive-story exploration, semantic-language pivot, Sequence and
Compare references, and fidelity evidence. No prerequisite PR remains open:
PR #41 (the earlier authoring-skill prototype) is already merged. Keep the
existing commits and failed-trial evidence; do not imply this is production API
promotion. Full-diff review and passing CI/audit are required before merge.
This authorizes publication of this work, not merging, releasing, installing,
a third idiom or a broader product slice. Discover exact commit/PR/check status
with Git and GitHub rather than treating older operational notes as current.

## Latest update — bounded fidelity trial completed

Travis authorized pushing `6f5e9c5` on `feat/sequence-reference` and proceeding
with the bounded fidelity trial, retaining his PO role. The remote branch hash
was verified after push. No merge was performed.

Read [authoring-fidelity-trial.md](authoring-fidelity-trial.md) for the new
checkpoint. One fresh author submitted two Compare documents, two actual
Sequence documents and one prose decline on five new packets. An independent
review found all five finals faithful to the 49 predeclared obligations, after
recorded self-repairs; no coordinator content/layout coaching or repair occurred.
Both processes exited. No live panes were created or controlled in this trial.

All accepted renderers, fixtures, discovery and old evidence are unchanged.
The new trial evidence/docs were local and uncommitted at trial completion;
the subsequent publication decision above includes them. Its frozen pre-run
brief, raw artifacts and review live under
`docs/assets/authoring-fidelity/run-1/`. See `primary-verification.md` there for
final checks and the intentional current-document changes versus the baseline.

Subsequently Travis requested a live paired reading-collection demo. After the
opening and readiness/carriage view he said the grouping was “definitely easier
to follow” and the scrolling “well worth the tradeoff” for easier scanning and
parsing. This accepts the demonstrated tradeoff for this case. Preserve the
visuals; do not reopen scrolling as an unsolicited redesign task. Renewal had
not yet been shown live; full-content coverage is separate reviewer evidence.
See the trial's `po-demo/` record and `/tmp/hud-next-session.md` for owned demo
panes; rediscover before control. Merge and the next product slice remain
separate decisions beyond the authorized branch push/PR. No measured comprehension or general reliability claim.
Historical “not started” recommendations and no-push status below describe the
preceding checkpoint and are superseded only by this explicit authorization/run.

## Start here

Read `AGENTS.md`, `README.md`, `engineering-spec.md` and
`semantic-visual-language.md`, then `compare-reference.md`, `compare-trial.md`
and `skills/hud/compare.md` (the skill path is relative to the repository root).
Sequence detail is in `sequence-reference.md`, `sequence-trial.md`,
`sequence-rhythm.md` and `sequence-header.md`.

The accepted reference work is being preserved as one honest local checkpoint:
**Sequence and Compare experiments**, not a production API or release. Identify
its exact commit/branch with Git; `/tmp/hud-next-session.md` records the completed
checkpoint state after commit. Older handoffs' active-worker and uncommitted
statuses describe their earlier moments, not instructions to restart that work.

## Product decisions and limits

- Travis remains Product Owner. Agents own implementation, tests and review.
- Sequence's demonstrated visual treatment was accepted, including one outer top
  padding row, title/context hierarchy, larger section gaps, compact item interiors
  and the numbered connected spine. Do not redesign it unsolicited.
- Travis said Compare was **“way easier to visually parse”** than the supplier
  prose. Preserve that demonstrated visual benefit; it is not proof of better
  decisions, factual fidelity or general author reliability.
- Compare means alternatives answering common named questions, not equivalence,
  rankings, scores, totals, normalization or recommendations. Unknown/unavailable
  states and consequential qualifications stay explicit.
- Both are isolated, inert-data Cargo examples. Production `src/`, dependencies,
  the old expressive prototype and the interval/document WIP are unchanged.
- No third idiom, general resolver, production schema, merge, public push, release,
  installation or remote-memory/project write is authorized by these checkpoints.

## What the trials actually established

Sequence selected suitable procedure/chronology and declined alternatives, but
lost facts and exposed a clipped-negation bug. Raw trials, corrections and final
presentation history remain preserved. Do not erase the failures or call corrected
copies unassisted successes.

Compare's five-packet run selected Compare for three alternatives packets (one
was the supplied anchor), numbered Markdown for a run sheet, and prose for a
reimbursement. **The run sheet was not HUD Sequence JSON or a renderer invocation.**
Compare artifacts omitted or compressed important qualifications. The archive
service demonstration has a separately labeled correction; raw author evidence
is immutable. Selection/syntax success did not establish fidelity.

The frozen Compare discovery also omitted exact limits, a complete JSON example,
and a link to actual Sequence authoring. Those were corrected **after** the trial.
The improved entrypoint has not been independently trialed. Preserve original
frozen copies/manifests separately from the corrected current entrypoint.

## Recommended next product slice — not started

Focus on **authoring fidelity and discovery**, not more visual components:

1. Agree the bounded question with the PO: can a fresh author use the final
   entrypoints and preserve every decision-relevant fact/qualification without
   coordinator repair?
2. Delegate fresh work from this handoff rather than extending the old near-full
   coordinator context. Freeze the final entrypoints; use genuinely new supplied
   facts and separate evaluator expectations from author guidance.
3. Exercise actual selected entrypoints (including Sequence when selected), not
   only semantic labels in Markdown. Prose remains a legitimate outcome.
4. Distinguish missing data from unavailable answers, preserve units/time/source
   scope and interpretive caveats, and record every repair/intervention honestly.
5. Return evidence and one consequential tradeoff to the PO. Do not invent a
   framework, schema expansion or content-sanitizing resolver to hide omissions.

This is a recommendation for the fresh thread, not permission to start a third
component or claim the final authoring path is already reliable.

## Evidence, checks and operations

- `docs/assets/sequence/` preserves Sequence history and checkpoints.
- `docs/assets/compare/trial/` holds packets, raw author artifacts, reviews,
  corrected demo and frozen/current discovery manifests. Follow verification
  commands in `compare-trial.md`; old live-path manifests intentionally refer to
  pre-correction files, not current changed paths.
- `docs/assets/compare/po-demo/` records the primary's paired supplier/prose demo.
- Last completed gate before packaging: `./scripts/check`, **80 tests**, formatting
  and Clippy. Run it again for changes. No new advisory or CI pass is claimed;
  dependency versions/lockfile did not change. Captured check logs retain their
  tool-emitted final blank line; `.gitattributes` exempts only that EOF whitespace
  in `docs/assets/**/check*.log`, without rewriting evidence or relaxing code checks.
- All builder, author and trial-lead agents finished and exited. Do not restart
  them from historical pane IDs. Demo panes are being gracefully cleaned up for
  the fresh thread; rediscover topology before control. Exact final state is in
  `/tmp/hud-next-session.md` after cleanup/commit.
- Consultation IDs, actual verification limits and context are in the trial docs.
  Consult Dottie read-only for relevant gaps; current local PO decisions outrank
  older advice. Never scrape her personal composer or automatically write memory.

GitHub remains the sole public home. Local checkpointing is not publication.
