# HUD sequence authoring record

## Case A — Observatory briefing

- **Audience/intent:** The incoming shift lead needs to distinguish what the selected entries establish from what remains unknown.
- **Choice:** `chronology` Sequence. The entries have a supplied temporal order, while the reference says chronology must not imply cause, duration, completeness or current status.
- **Draft:** `case-a-draft.json`.
- **Repair:** The first check of the draft failed because its scope exceeded the 60-display-cell field limit. The draft is preserved. The final scope was shortened without dropping the date, local-time qualification or incomplete-log qualification.
- **Final:** `case-a-final.json`.
- **Validation command:** `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-a-final.json --check`
- **Validation result:** `sequence OK (shape only; display data only)`.
- **Initial failure output:** `case-a-initial-check-error.txt`.
- **Inspection commands and captures:**
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-a-final.json --preview 72 35` → `captures/case-a-72x35-offset0.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-a-final.json --preview 72 35 12` → `captures/case-a-72x35-offset12.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-a-final.json --preview 80 24` → `captures/case-a-80x24-offset0.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-a-final.json --preview 80 24 8` → `captures/case-a-80x24-offset8.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-a-final.json --preview 80 24 16` → `captures/case-a-80x24-offset16.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-a-final.json --preview 80 24 65535` → `captures/case-a-80x24-end.txt`
- **What I saw:** At 72x35 all 21 body lines and the END footer were visible. At 80x24 the full title, provenance, scope, relationship, intent, three chronology items, and notes were visible across the initial and clamped/end views; the approximate time remained attached to its event, and the footer reported END. No silent shortening or misleading progress/current-state display appeared.
- **Revision after inspection:** None warranted. The explicit notes preserve the unknown adjustment, cause, camera restoration, current roof state and safety clearance.

## Case B — Print studio choice

- **Audience/intent:** A workshop organizer needs to choose one supplier for this one batch of 100 handouts; no choice has been made.
- **Choice:** Ordinary prose comparison, not Sequence. The quote numbers are ascending price rather than preference, and the suppliers are independent alternatives; Sequence would falsely suggest a ranked path or a procedure through all three.
- **Draft:** `case-b-draft.txt`.
- **Final:** `case-b-final.txt` (unchanged; no refinement was warranted).
- **Rendered validation:** None. This was intentionally kept as prose because Sequence is inappropriate; there is no rendered composition to inspect.
- **Coverage:** The final text retains the supply date and one-batch scope, all quote details, Cedar's unknown delivery charges, and the absence of quality ratings, opening hours and budget ceiling. It does not select a winner or require buying from all three.

## Case C — Archive volunteer handoff

- **Audience/intent:** A volunteer should follow the complete supplied intake procedure for today's donated photograph batch without assuming permission to scan.
- **Choice:** `procedure` Sequence. The supplied order is operational and conditional; Sequence makes the order visible while explicitly stating that it does not execute actions.
- **Draft:** `case-c-draft.json`.
- **Final:** `case-c-final.json` (unchanged after inspection).
- **Validation command:** `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-c-final.json --check`
- **Validation result:** `sequence OK (shape only; display data only)`.
- **Inspection commands and captures:**
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-c-final.json --preview 72 35` → `captures/case-c-72x35-offset0.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-c-final.json --preview 72 35 12` → `captures/case-c-72x35-offset12.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-c-final.json --preview 80 24` → `captures/case-c-80x24-offset0.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-c-final.json --preview 80 24 8` → `captures/case-c-80x24-offset8.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-c-final.json --preview 80 24 16` → `captures/case-c-80x24-offset16.txt`
  - `cargo run --quiet --example sequence -- /tmp/hud-sequence-author/case-c-final.json --preview 80 24 65535` → `captures/case-c-80x24-end.txt`
- **What I saw:** At 72x35 all 25 body lines and the END footer were visible. At 80x24 the initial view showed the top of item 1 through the prerequisite item; offset 8 clamped to the last view and exposed the continuation of item 1 plus items 2–3 and all notes, with the footer reporting END. The mandatory permission condition remained in the item label, and no action/completion state was implied.
- **Revision after inspection:** None warranted. The final note and item detail preserve the queue-placement boundary, unknown permission/count/date, annotation handling, and non-execution status.

All artifacts are under `/tmp/hud-sequence-author/`.
