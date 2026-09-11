# Bounded HUD submission

All paths below are relative to `/tmp/hud-fidelity-run1-rS7MUp/author/`.

## Final audience artifacts

| Packet | Final file | Choice and purpose |
|---|---|---|
| A | `final/a.compare.json` | Compare equipment-hire quotes, fulfillment and power-access terms on the supplied common basis; no recommendation. |
| B | `final/b.sequence.json` | Sequence / procedure: the complete supplied mandatory single path for the paper proof envelope; no execution claim. |
| C | `final/c.compare.json` | Compare bounded reading-collection terms, retaining undefined readiness, conditional services and unknown costs/start dates; no booking or all-in total. |
| D | `final/d.sequence.json` | Sequence / chronology: attributed selected notebook entries, not causation, verified readings or equipment instructions. |
| E | `final/e.prose.md` | Explain why different cost bases do not establish a lower-cost outing offer; no normalization or ranking. |

Intent and pre-composition reasoning for every packet: `intent-and-plan.md`. Choices were informed by the supplied discovery, not an uncued idiom-selection test.

## Validation and inspection

The actual frozen Cargo examples were invoked from `/tmp/hud-fidelity-run1-rS7MUp/checkout`, with `CARGO_NET_OFFLINE=true`. Only `--check` and in-memory `--preview` were used; no real terminal or live panes.

All four final JSON checks passed. All **33 final previews** passed with empty stderr and were read completely, including overlapping intermediate views at **72x35 and 80x24**. The final sources and E's prose were read in full. Coverage by requested offset and actual footer position is in `inspection-and-repairs.md`.

A/C initially failed because their provenance strings exceeded 60 cells. Original stderr and exit-1 results are preserved, including the failed preview attempts. R1 shortened provenance without dropping source status. R2 corrected two overly categorical absence-of-evidence sentences. B/D/E remain byte-identical to their drafts; inspection did not warrant revisions. C needed an extra 80x24 offset because two earlier requests snapped to the same view; the additional view exposed the full renewal group.

Overflow remains visible and requires scrolling: openings are not complete submissions on their own. In particular, some qualifications continue below an explicit footer warning. No qualification was removed to fit a single screen. Inspection found no further change warranted for this bounded submission.

## Evidence and history

- `drafts/`: all five initial drafts, unchanged.
- `revisions/r1/` and `revisions/r2/`: separate A/C revision snapshots.
- `evidence/initial/`, `evidence/r1/`, `evidence/middle/`, `evidence/final/`: each invocation has exact `.command`, original `.stdout`, original `.stderr`, and `.exit`; each stage records working directory/offline status.
- `evidence/run-initial.sh`, `run-r1.sh`, `run-middle.sh`, `submit-and-preview.sh`: exact invocation/capture scripts, with sibling collector stdout/stderr/exit records.
- `evidence/execution-ledger.md`: shell-call transcription and evidence conventions.
- `evidence/provenance-lengths.*`: contemporaneous local diagnostic of the author's own strings.
- `evidence/inspection/a.txt` through `e.txt`: fully inspected convenience collations of final data, commands and original streams. They are not newly rendered or reconstructed output.
- `evidence/inspection/stream-audit.txt`: inspected command/status/stderr collation across all stages.
- `evidence/final/file-hashes.stdout`: inspected SHA-256 manifest establishing final-to-draft/revision correspondence.
- `inspection-and-repairs.md`: actual inspection record, failures, self-directed repairs, coverage and remaining layout friction.

External reads were limited to the two supplied discovery files and packets A–E; no example fixtures were needed. Other reads were of this author's own artifacts. No renderer source, other repository documentation, historical archive, evaluator material, other sessions, live information or other agent was consulted. No git operations or author edits to the checkout, sources, discovery or packets were performed. The helper scripts collect evidence only; they do not add an idiom, renderer, schema or authoring framework.

Checks establish shape only. Headless text inspection does not establish Product Owner acceptance, human comprehension, perceived color, live-key behavior, production readiness or broad author reliability. Travis remains Product Owner; this submission makes no acceptance claim.
