# Fresh HUD author trial report

Run evidence is under `/tmp/hud-compare-fresh-author/`. No repository files were edited.
This was one fresh author run; no post-coaching rerun occurred.

## Authored cases

| Case | Selected form | Rationale |
|---|---|---|
| `archive-services` | Compare | Lantern Cloud and Meadow Vault are independent offers answering the same supplied questions. The comparison keeps setup separate from recurring storage, does not total the amounts, does not rank them, and retains the checksum unknown. The supplied lack of equal service scope and other unknowns argues against a recommendation. |
| `exhibit-run-sheet` | Sequence | This is one supplied operational order, and order and times are the meaning. It is not a choice among alternatives. |
| `reimbursement-note` | Ordinary prose | There are no alternatives and no supplied procedure. A short status explanation keeps the missing receipt, missing approval record, and absent total together without calculating or implying approval. |
| `supplier-anchor` | Compare | Alder, Birch, and Cedar answer the same amount, fulfillment, and recycled-paper questions. The supplied ascending price order is retained as non-preference; Cedar's unknown delivered total is kept beside its quote, with no winner or score. |
| `venue-options` | Compare | North Hall and South Library are independent venue options with common supplied questions. Capacity, room booking, projector, and step-free entrance remain aligned, with unknown and unavailable states distinct and no ranking or booking decision. |

## Validation, failures, repairs, and revisions

- Initial Compare drafts were checked with the frozen `--check` invocation. All three failed because the provenance exceeded the 60-display-cell limit.
- Repair 1 shortened provenance and moved context into scope. The next checks failed because scope exceeded the 60-display-cell limit.
- Repair 2 compressed the three metadata fields. That edit accidentally omitted the JSON comma after `intent`; all three checks then failed with `expected \, or }`.
- Repair 3 restored the commas. Final shape checks passed for all three: `comparison OK (shape only; display data only)` with exit 0.
- The prose cases had no renderer/schema validation because prose was selected. Their complete final files were inspected; no failures or repairs occurred.
- Drafts and finals are preserved read-only under `drafts/` and `final/`; `meta/sha256.txt` records their hashes.

## Rendering inspection

Every Compare final was rendered at exactly 72x35 and 80x24 using only the frozen entrypoint invocation. The raw outputs, exit captures, and command lists are under `raw/` and `meta/`. The 80x24 `65535` end-view invocation was also captured to inspect continuation.

- `archive-services`: 32 rendered lines. Both sizes showed `MORE below` from the opening view; the end view reached `END` and exposed the checksum group. At 72x35 the opening view reached the recurring-storage group; at 80x24 it stopped during one-time setup. The end view starts after the header, so context is lost while scrolled, but continuation is explicit. The renderer repeats unknown/qualification wording (`UNKNOWN · not supplied` followed by `not supplied`).
- `supplier-anchor`: 24 rendered lines. The 72x35 view showed all content; the 80x24 opening view showed `MORE below`, and the end view showed the fulfillment and recycled-paper groups. Cedar's qualification wrapped but remained adjacent. Alder's unavailable state was visibly distinct. No clipping or unreadable ownership was observed.
- `venue-options`: 27 rendered lines. Both opening views showed `MORE below`; the end view exposed the complete projector and step-free groups. Alternative labels remained clear. Unknown/unavailable state labels plus qualifications repeat wording, but ownership and distinction remained readable.

The preview is a terminal capture only; these observations do not claim color perception, live-key behavior, human comprehension, or product acceptance.

## Complete prose inspection

`final/exhibit-run-sheet.md` and `final/reimbursement-note.md` were read in full. The sequence retains all five supplied steps, local times, and the inspection-time unknown plus the supplied non-claims. The reimbursement prose retains the trip date, 23.4 miles, 0.65 USD approved-mile rate, missing receipt, unrecorded approval, non-claims, requestable records, and absent total. No scroll or wrapping issue applies to these complete files.
