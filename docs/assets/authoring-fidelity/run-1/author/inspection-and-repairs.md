# Inspection and self-directed repairs

## Initial validation
All 20 initial Cargo invocations are preserved in evidence/initial/, each with .command, .stdout, .stderr and .exit. The collector's exit 0 means it finished collecting, not that the individual checks passed.

A and C: --check and both sizes' opening/end previews each exited 1. Original check stderr was exactly `provenance is limited to 60 display cells` followed by a newline. No rendered content was produced. A local length check of my own ASCII strings found 62 characters for A and 61 for C. No renderer source was inspected. B and D checks and previews exited 0; checks report shape/display-data only.

R1 keeps the original drafts untouched. A removes redundant `supplier` from the provenance line (the alternatives still identify hire suppliers). C moves `written` from provenance into the intent. Fictional, not live and not independently verified remain explicit. These two checks and all R1 previews passed.

## Actual inspection before R2
Read full initial B/D opening and end stdout at 72x35 and 80x24, and their offset-10 stdout in evidence/r1/. At 72x35 offset 10 clamps to the same final view. At 80x24 it provides an overlapping middle view. All five B steps, all four D observations and both D background paragraphs were read. B retains the release condition in step 4's label, comments-box exclusion, actual local 24-hour log semantics and no invented counts/times. D retains approximate time, attribution, display-versus-air distinction, requested-versus-completed check and causality/current-state limits. The footers visibly warn of continuation; item ownership remains marked when an end view starts in an earlier item's continuation. No B/D revision warranted by this inspection.

Read drafts/e.prose.md in full: copied order, two independent offers, distinct currencies/units/routes/directions, every missing input, no booking/date and no all-in or lower-cost conclusion are retained. No prose revision warranted.

Read full R1 A/C opening and end stdout at both sizes. Then read every stdout in evidence/middle/: A 72x35 offsets 20,30; A 80x24 offsets 15,25,35; C 72x35 offsets 20,35,45; C 80x24 offsets 15,25,35,45. These are actual requested offsets; snapping means the footer positions differ. The last two C 80x24 requests both start at line 36/64, so they do NOT cover Willow's renewal answer. An additional offset 50 is needed for that portion and will be included in R2 inspection. This gap was not treated as inspected.

A's entire R1 content was covered at both sizes: prices plus Reed's unknown delivered total, conditional delivery deadline, Moss's 18-October-only hire and half-brightness runtime, Larch unavailable versus Reed unknown battery state, all cable answers and shared unknown hire terms. C's 72x35 content was covered in full, including both undefined ready dates, unknown loan start, return conditions and renewal limitations. C's 80x24 content was covered except the renewal tail noted above. Qualification rows remain adjacent under the value column. Opening views alone do not expose all caveats; full inspection required overlapping views. C's opening 72x35 view stops immediately after Willow's ready-date value, with an explicit continuation footer; its matching qualification was read in the intermediate view.

## R2 fidelity refinement prompted by inspection
Two common-intent sentences were too categorical about absence of evidence:
- A originally said `no option is selected or order placed`. The packet establishes no selection but only forbids inferring an order. Replace with `no option is selected. No order is established as placed.`
- C originally said `conditional renewal or carriage is not secured`. Replace with `conditional renewal or carriage is not established as secured` to preserve unknown status rather than assert non-booking.

These are self-directed fidelity repairs, not a changed product decision. Save R2 separately and rerun --check plus all necessary headless views; no alteration of renderer, schema or discovery. The original R1 previews and original failure diagnostics remain unchanged.

## Final inspection completed
Read evidence/inspection/a.txt through e.txt completely. These are explicitly collated copies of the final source files and the actual final command/exit/stderr/stdout files, not reconstructed renderings. Read evidence/inspection/stream-audit.txt completely to inspect all saved per-command exit statuses and stderr, including all ten original failures. Also read final/file-hashes.stdout, submit-and-preview.stdout and its empty stderr, the three earlier collectors' empty stderr, and the bundle collector's stdout/empty stderr. The hashes match A/C finals to R2 and B/D/E finals to their preserved drafts.

All 33 final previews and all four final checks exited 0 with empty stderr. Both Compare checks say `comparison OK (shape only; display data only)`; both Sequence checks say `sequence OK (shape only; display data only)`. All successful earlier previews were read in full too; failed previews had no stdout to inspect.

Final requested offsets and observed footer starts (in matching order):

| Packet | Size | Requested offsets | Observed first body line / total |
|---|---|---|---|
| A | 72x35 | 0,20,30,65535 | 1,19,31,43 / 61 |
| A | 80x24 | 0,15,25,35,65535 | 1,10,18,28,45 / 56 |
| B | 72x35 | 0,10,65535 | 1,5,5 / 32 |
| B | 80x24 | 0,10,65535 | 1,11,15 / 30 |
| C | 72x35 | 0,20,35,45,65535 | 1,21,33,38,51 / 70 |
| C | 80x24 | 0,15,25,35,45,50,65535 | 1,11,20,36,36,47,56 / 64 |
| D | 72x35 | 0,10,65535 | 1,9,9 / 34 |
| D | 80x24 | 0,10,65535 | 1,11,16 / 32 |

The combined final views cover every body line, not merely opening/end. C's added 80x24 offset 50 shows the complete Renewal group, including Willow's explicit UNAVAILABLE and its qualification; the end view completes the other-terms tail. A's revised 72x35 opening now cuts Reed's fulfillment qualification at the bottom with an answer/section continuation warning; offset 20 shows that answer and unknown delivered total in full. R2's absence-of-evidence wording is visible in both sizes' opening views. Full source data and E's prose were also read in the final bundles.

No further refinement was warranted by this inspection: known, unknown and unavailable remain distinct; conditions remain attached to affected claims; no derived totals, recommendations, invented execution or causal conclusion appear. Overflow and snapping remain practical friction. Some end views leave blank rows, and some openings cut qualifications with an explicit footer. These are not standalone complete views; retain the data and overlapping previews rather than shortening away qualifications. This is headless textual inspection, not a live-terminal, color-perception or human-comprehension test.
