# Independent review — bounded HUD authoring trial, run 1

## Finding and scope

**All five submitted finals are fidelity-clean in this bounded evidence review.** All 49 predeclared fact obligations are accounted for below; no material final omission or unsupported assertion was found. Selection is suitable in each case, independently of fidelity. This is not a claim that the initial drafts passed: A/C initially failed validation and contained categorical wording subsequently repaired by the author.

The author's final captures—not additional reviewer-only views—cover all content at both requested sizes. Their actual reads are corroborated by the session, including the extra C renewal view. Independent execution reproduced all four final checks and all 33 final previews byte-for-byte. These conclusions concern supplied facts, text association and inspection evidence, not human comprehension, usefulness, acceptance or live-key behavior.

Travis remains Product Owner. No input or product repair, rerun of the author, repository mutation, new test suite, dependency change, live terminal, network access or other agent was used. Review outputs and compilation products are confined to `/tmp/hud-fidelity-run1-rS7MUp/review/`.

### Evidence notation

`E` = `/home/tnez/Work/tnezdev/hud/docs/assets/authoring-fidelity/run-1/`.
`F` = `E/author/final/`; `P` = `E/author/evidence/final/`.
`A80(35)`, for example, means the **author's original** `P/a.80x24.35.stdout`; parentheses are requested, zero-based offsets, not footer positions. Equivalent independent streams are in `review/independent/` with the same basenames. JSON evidence below identifies fields and named answers rather than relying on keyword counts.

## Per-case result

| Case | Selection / reason, recorded before composition | Actual invocation and syntax | Ledger fidelity | Rendered association and author inspection | Author effort |
|---|---|---|---|---|---|
| A | Compare: three independent hires share day, set, currency and price/fulfillment/power questions; no recommendation. | `a.compare.json`: actual Compare schema, 5 criteria × 3 alternatives. Real Cargo `compare --check` and `--preview`, not a Markdown label. Final check passes. | A01–A11 pass. Named unknown terms, explicit battery unavailability, half-brightness runtime and conditional delivery survive. | Stable supplier gutter and criterion groups; conditions remain in affected answers. Entire content covered at both sizes, 9 final captures. Opening 72×35 cuts a delivery qualification with a continuation warning; overlapping view completes it. | 3 checks + 22 preview attempts across stages; 5 initial failures. R1 provenance repair; R2 order-status wording repair. |
| B | Sequence / procedure: swapping the mandatory steps would misrepresent the supplied single path. | `b.sequence.json`: `relationship: procedure`, 5 items. Real Cargo `sequence` invocation. Final check passes. | B01–B09 pass. Identifier source, exact copying, named permission box, action gate, location and actual-time semantics survive. | Gate is in item 4's operative label, not only detail. All 5 items covered at both sizes, 6 final captures; 72×35 offset 10 duplicates End. | 2 checks + 12 previews; no failures or content revisions. Final is draft-identical. |
| C | Compare: same books, school, initial period and currency support comparison of bounded terms, not all-in cost or school receipt. | `c.compare.json`: 6 criteria × 2 alternatives. Real Cargo `compare` invocation. Final check passes. | C01–C12 pass. Undefined “ready,” calendar/working-day distinction, return/renewal availability and individually named gaps survive. | Local readiness qualifications retained; opening captures alone are incomplete. Entire content covered, 12 final captures; 80×24 requests 35/45 duplicate, additional 50 exposes Willow renewal. | 3 checks + 27 preview attempts; 5 initial failures. R1 provenance/intent repair; R2 conditional-service status repair; self-discovered inspection gap filled before submission. |
| D | Sequence / chronology: selected attributed notes have supplied temporal order, not an equipment procedure or causal chain. | `d.sequence.json`: `relationship: chronology`, 4 items plus 2 shared notes. Real Cargo `sequence` invocation. Final check passes. | D01–D09 pass. Approximation, source attribution, display-vs-air distinction, request-vs-completion and current-state limits survive. | Approximation and attribution are in labels. Global uncertainty stays in background, with all background inspected. Entire content covered, 6 final captures; 72×35 request 10 duplicates End. | 2 checks + 12 previews; no failures or revisions. Final is draft-identical. |
| E | Prose: one conclusion and reasons; incompatible monetary, passenger, route and journey bases make numeric alignment misleading. | `e.prose.md`, ordinary prose; no Cargo invocation claimed or needed. | E01–E08 pass. Both offers and every missing input retained; lower cost and outing total not established. | Whole prose read initially and again in final bundle; no renderer/scroll coverage claim applies. | One draft, unchanged final, no repair. |

Author totals: **83 Cargo invocations = 10 checks + 73 preview attempts**. Ten initial failures comprise 2 checks and 8 previews; 63 previews succeeded. Final subset: 4 successful checks + 33 successful previews. Collector exit 0 is not being substituted for individual statuses. There are also 5 final copy records and 1 hash-command record, giving 89 per-command records in the four stages.

## Obligation-to-output map

Every row below passes for the final artifact; the rendered citations refer to views inspected in full, with coverage established separately below. No obligation is waived merely because JSON parsed. Prohibited-inference findings come from reading the complete final, not merely locating the quoted negation.

### A — `F/a.compare.json`

| ID | Specific output evidence and association |
|---|---|
| A01 | Title identifies indoor repair day; `intent` addresses community workshop organizer inspecting “price, fulfillment and power access,” says no selected option; no recommendation/order action. A80(0). |
| A02 | Persistent `provenance`: “Fictional emails; not live or independently verified.” Supplier ownership is supplied by the named hire alternatives. `scope`: “18 October 2026 / one day / four-lamp set / GBP.” Alternatives remain Larch, Moss, Reed; `intent` rejects preference. All views retain header. |
| A03 | Larch / Quoted hire: `48 GBP`; criterion question supplies per-four-lamp-set, one-day basis. Fulfillment: depot, morning 18 October. A80(15). |
| A04 | Larch / Battery operation: `state: unavailable`, with “mains power is required.” Cable length: `unknown`, “Cable length is not supplied.” A80(35). |
| A05 | Moss / Quoted hire: `62 GBP` on the shared set/day basis. Fulfillment: depot evening 17 October; adjacent qualification “Hire covers 18 October only.” A80(15). |
| A06 | Moss battery value: “Available; quoted runtime up to 3 hours at half brightness.” Adjacent “This is not a full-brightness runtime.” Cable value “5 metres per lamp.” A80(35); A72(30) shows half/brightness wrapping without dropping the condition. |
| A07 | Reed fulfillment value: “Delivery on 18 October only if the organizer confirms by 12:00 local time on 15 October.” Condition is in the value itself; no confirmation asserted. A80(15), A72(20). |
| A08 | Reed `75 GBP` has its own qualification “Delivery charge is not supplied; final delivered total is unknown.” Repeated beside delivery, not treated as delivered total. A80(0), A80(15). |
| A09 | Reed battery `unknown`, “Whether this set supports battery operation is not supplied.” Cable “4 metres per lamp.” A80(35). |
| A10 | Each alternative / Other hire terms: `unknown` with “Deposit amount, damage excess and replacement-lamp availability are not supplied.” Generic criterion name does not replace the three named gaps. A80(65535). |
| A11 | `intent` names absent power-outlet survey and brightness measurements; rejects equal output, cheapest usable setup, free collection and established placed order. A80(0); no conflicting claim elsewhere. |

### B — `F/b.sequence.json`

| ID | Specific output evidence and association |
|---|---|
| B01 | Title “Release one paper proof envelope”; scope 21 October 2026, “complete supplied single-path procedure”; intent mandatory order for print-room volunteer, “not a production print run,” no alternate route. B80(0). |
| B02 | Persistent provenance “Fictional training; not live guidance; no step executed.” Generated landmark “Procedure · not execution status”; item 3 detail does not establish approval. B80(0), B80(65535). |
| B03 | Item 1 label copies identifier **from blue job card onto envelope**. Detail: “Copy it exactly; do not substitute the client's name. No actual identifier is supplied in this fictional packet.” B80(0). |
| B04 | Item 2 label counts enclosed proof sheets and writes count on “that same envelope”; detail includes blank and printed sheets and no expected count. B80(0), B80(10). |
| B05 | Item 3 label obtains supervisor initials in release box on blue job card; detail excludes comments-box initials and says packet does not say approval granted. B80(10), B80(65535). |
| B06 | Item 4 **label**: “Only after the release-box initials are present, place the envelope in the collection tray.” Detail additionally forbids placement before recorded permission. B72(0), B80(10), B80(65535). No unconditional operative label. |
| B07 | Item 4 detail: collection tray “inside the print room, not the public counter.” B80(10), B80(65535). |
| B08 | Item 5 label: actual placement time, paper handover log, local 24-hour time. Detail distinguishes tray placement from client collection and names both unsupplied times; no invented time. B72(10), B80(65535). |
| B09 | `items` order: copy → count → obtain initials → conditional placement → log. Rendered 01–05 preserve this order, with no current/completed step. B72(0,10), B80(0,10,65535). |

### C — `F/c.compare.json`

| ID | Specific output evidence and association |
|---|---|
| C01 | `intent` school librarian, “these written offers,” quoted loan/readiness/carriage/renewal, “not choose or book.” Provenance fictional, not live/independently verified; scope “2 Nov 2026 offers.” C80(0). |
| C02 | Scope same 60 **named** books, same school, EUR; intent repeats delivered to same school. Bracken precedes Willow throughout, expressly “not ranking.” C80(0,15). |
| C03 | Bracken / Quoted loan: “90 EUR for the first 28 calendar days only.” Criterion question is loan charge/period, not total; intent says no final all-in total established. C80(0,15). |
| C04 | Bracken / Readiness wording: quoted “ready on 9 November”; same answer's qualification says source does not define ready and date does not establish school receipt or permission to circulate. C80(25), C72(20). |
| C05 | Bracken outbound “Included.” Return carriage `unavailable`; expressly not offered by Bracken, school must arrange separately, cost not supplied. C80(25,35). |
| C06 | Bracken renewal value: “One renewal of up to 7 calendar days permitted only if no other school has reserved the collection.” Adjacent charge not supplied and renewal not established as secured. C80(50), C72(35). |
| C07 | Willow / Quoted loan: “110 EUR for the first 28 calendar days only.” C80(15). |
| C08 | Willow readiness value quotes “ready on 10 November”; its own adjacent qualification repeats undefined-ready, no school receipt and no circulation permission. C80(25), C72(20). Opening C72(0) is explicitly incomplete, not the sole evidence. |
| C09 | Willow return: “Offered for 18 EUR only when booked at least 2 working days before the loan ends.” Adjacent qualification names unknown ability to meet condition and no established secured carriage. C80(35), C72(35). No calendar-day conversion/assumed booking. |
| C10 | Willow / Renewal: `unavailable`; “Renewal is explicitly unavailable for this collection.” C80(50), C72(35). |
| C11 | Each / Quoted loan qualification: “The date on which the 28-day loan period starts is not supplied.” Ready dates never made loan-start dates. C80(15). |
| C12 | Both / Other terms: `unknown` and individually named “Lost-book charges, late fees and weekend contact arrangements.” Intent no final all-in total; no arithmetic added. C80(65535), C80(0). |

### D — `F/d.sequence.json`

| ID | Specific output evidence and association |
|---|---|
| D01 | Title greenhouse **notebook entries**, provenance fictional copied notes/not live or independently verified, scope 6 December 2026/local times/selected observations. In this greenhouse-specific context “local times” preserves greenhouse-local scope. Intent addresses coordinator and excludes complete incident record/equipment procedure. D80(0). |
| D02 | Item 1 label: “08:05 — Nia recorded a west-bench display reading of 12 degrees Celsius”; detail explicitly not independently checked air temperature. D80(0). |
| D03 | Item 2 label begins “Approximately 08:20 — Omar wrote that he closed the west vent.” Detail names unsupplied exact closure time and prior opening width; “report of an action, not proof of its effect.” D80(0,10). |
| D04 | Item 3 label: 08:50, Nia recorded **same** west-bench display reading 15 degrees Celsius. Detail: no comparison reading from another bench. D80(10,65535). |
| D05 | Item 4 label: 09:10, Eli recorded **requesting** sensor check for 7 December, next day. Detail: request does not establish scheduled, performed or completed check. D80(10,65535). |
| D06 | First shared note: not established why display changed, whether vent closure caused change, whether sensor accurate. Generated “Chronology · not causation,” and item 2 action/effect distinction, also constrain interpretation. D80(65535), D80(0). |
| D07 | Second shared note: not established current greenhouse temperature **or** vent position. D80(65535). |
| D08 | Same note: no target temperature, alarm threshold or instruction to repeat vent action supplied. D80(65535). |
| D09 | Items retain 08:05 → approximately 08:20 → 08:50 → 09:10 order; all attributed, no verified measurements, definite estimated timestamp or completed outcome substituted. D72(0,10), D80(0,10,65535). |

### E — `F/e.prose.md`

Paragraph numbering below excludes the title; final prose and `author/evidence/inspection/e.txt` were read completely.

| ID | Specific output evidence |
|---|---|
| E01 | Title “Transport costs: no lower-cost offer established”; paragraph 1 addresses youth-group treasurer and denies established lower cost/outing total. No live quote, recommendation, invented common basis or procedure follows. |
| E02 | Paragraph 1: fictional, copied 4 January 2027, not live or independently verified; independent offers, copied order not ranked; no booking and no travel date. |
| E03 | Paragraph 2: “420 GBP per coach,” “return trip from East Quay to Hill Camp,” driver included, tolls excluded. Not represented as per-passenger or all-in. |
| E04 | Paragraph 2 individually names absent coach capacity, coaches required and toll amounts. |
| E05 | Paragraph 3: “19 EUR per passenger,” “one-way ticket from Central Station to Valley Halt.” |
| E06 | Paragraph 3 individually names no return fare, group size, group discount, transfer between stops and coach endpoints, applicable exchange rate. |
| E07 | Paragraph 4 names lack of common currency, passenger count, route scope and return-journey basis; amounts not comparable outing totals. Paragraph 1 denies established lower-cost option or all-in total. |
| E08 | Paragraph 4 rejects unsupported doubling, conversion, per-person division and invented route; numerals cannot establish cheaper offer. No numeric ranking/calculation performed. No booking procedure or temporal order connecting offers. |

### Ledger versus supplied packets

I read all five actual packets, not just the ledger. No material ledger omission caused an untested final pass. Minor differences were resolved contextually rather than silently converted into stricter tests: A's supplier-email origin is expressed across provenance plus named supplier answers; D's notebook/local scope is expressed across title, provenance and scope. The packet's attendant/caretaker/coordinator roles, not individually demanded by D02–D05, also survive in details. C02 says “60 named books”; the packet supplies no actual book-name list to reproduce. E02 means the notes were **copied** on 4 January, not that the offers were necessarily issued then; the prose preserves copied-date wording. None is a new retroactive obligation.

## Author capture coverage, requested versus effective offsets

Both renderers reserve six header rows and one footer row: the body has 28 rows at 72×35, 17 at 80×24. Effective offset = printed first body line minus one. Snapping/clamping is from the actual frozen renderer, not an assumption that a requested offset was honored. Ranges below are one-based body lines actually exposed; end padding is not counted as additional content.

| Case / size | Requested offsets | Effective offsets | Actual body intervals, in matching order | Coverage |
|---|---|---|---|---|
| A 72×35 | 0,20,30,65535 | 0,18,30,42 | 1–28; 19–46; 31–58; 43–61 | 61/61 |
| A 80×24 | 0,15,25,35,65535 | 0,9,17,27,44 | 1–17; 10–26; 18–34; 28–44; 45–56 | 56/56 |
| B 72×35 | 0,10,65535 | 0,4,4 | 1–28; 5–32; 5–32 | 32/32 |
| B 80×24 | 0,10,65535 | 0,10,14 | 1–17; 11–27; 15–30 | 30/30 |
| C 72×35 | 0,20,35,45,65535 | 0,20,32,37,50 | 1–28; 21–48; 33–60; 38–65; 51–70 | 70/70 |
| C 80×24 | 0,15,25,35,45,50,65535 | 0,10,19,35,35,46,55 | 1–17; 11–27; 20–35; 36–52; 36–52; 47–63; 56–64 | 64/64 |
| D 72×35 | 0,10,65535 | 0,8,8 | 1–28; 9–34; 9–34 | 34/34 |
| D 80×24 | 0,10,65535 | 0,10,15 | 1–17; 11–27; 16–32 | 32/32 |

**Important exception to naive interval arithmetic:** C80(25) has a blank last body row, not visible body line 36. `compare.rs` defers a criterion heading when only its heading row fits; C80(35) supplies line 36. `coverage.py` explicitly accounts for this. All overlapping body rows agree, allowing for Sequence's first-line ownership marker. `coverage.txt` and `body-*.txt` are reviewer-derived accounting aids, not new author captures or replacements for originals.

The 33 captures contain **30 distinct views**: B/D 72×35 offset 10 repeats End, and C80(45) repeats C80(35). The fact that these are complete capture sets does not mean they are minimal scrolling paths. Author effort includes duplicate views and repeated validation after refinement, not measured live keystrokes.

### Association and incomplete excerpts

- A72(0) ends with “Delivery charge is not supplied; final delivered total” and `MORE below: answer/section continues`; A72(20) supplies “is unknown.” The amount answer already has the complete delivered-total caveat in the opening view. Reed's confirmation deadline stays in the delivery value. Moss's half-brightness condition wraps under the same supplier, and is complete together in A72(30)/A80(35).
- C72(0) ends at Willow's `Offer wording: "ready on 10 November".` with the answer-continuation footer. This is **not** a fully qualified standalone screenshot. C72(20) shows both ready dates with their own complete qualifications. C80(15) cuts Willow's qualification after “This date does not”; C80(25) restores the entire answer. Qualifier placement in the artifact is local; viewport boundaries still create incomplete excerpts.
- C80(35) and (45) stop after Bracken's renewal qualification. They do not expose Willow's renewal. The author noticed this before final inspection, included C80(50), and read it. Willow appears at body lines 53–54; End begins at 56, so a top/end or 35/45/end inspection would have missed it.
- B's conditional placement label remains qualified in views where the placement action is visible. End at 80×24 begins with `02 │` on a separator, then item 3; no invented current status. At 72×35 the opening cuts item 5's label before its time-format continuation, which the overlapping final view completes.
- D72 End begins at item 1's detail with `01 │`, identifying ownership even though its label is above. The opening and overlapping views jointly establish the whole attributed reading. D's shared uncertainty is offscreen initially at 80×24, but fully shown at End. The header keeps fictional/selected/local scope; chronology/cause distinction is not a live-status signal.

No permanent word loss or wrong item/alternative ownership was found at these sizes. Offscreen context remains a real cost. Complete capture coverage does not establish that a human will scroll, remember earlier context or interpret the warnings correctly.

## Verified author process, errors and independence

I inspected both compressed traces. Session `01a0881d-184d-7311-ba68-24395e807b4a` starts in the dedicated checkout at 2026-09-09 21:40:03Z, records `openai-codex / gpt-6-astra`, high thinking, and ends with one submission at 21:50:10Z (about 10 minutes elapsed, not a human-effort measurement).

The session contains one user prompt, 41 assistant messages and 91 tool results; no later user/coordinator message or compaction appears. All 91 tool calls match the JSON events' tool-execution starts by ID, name and arguments: **64 reads, 14 writes, 8 bash calls, 5 edits**. The trace first reads Compare (session line 6), then linked Sequence and A–E (line 8). Intent is written at line 15 before drafts (17–25). The remaining reads are the author's own artifacts. No renderer source, evaluator ledger, original repository, fixture, old-trial archive or other-agent read appears.

The eight bash calls are the four preserved evidence collectors, an own-string length diagnostic, two mkdir/copy preparations and the bundle collector. Their actual script contents only collect evidence/invoke frozen headless examples/copy author revisions. No git, network, live pane, source change or agent consultation is present. The recorded launch disables context-file, skill, extension, template and theme discovery; the observed reads and sole prompt are consistent with that launch. Those flags and instructional boundaries are not an OS sandbox or proof about hypothetical unrecorded external actors. This is one fresh recorded session, not a statistically independent sample of authors/models.

All 64 read-result texts match their archived file contents exactly, including the full final bundles; no truncated bundle read was counted as complete. Every trace-authored file and exact edit was reconstructed in memory and matched its archived counterpart (18 paths). Final bundles also match a fresh in-memory collation of their original streams exactly. These checks support preservation and actual exposure, not merely the author's claim to have inspected them.

Material historical issues, precisely:

1. A/C initial checks and each opening/end preview at both sizes exited **1**, empty stdout, exact stderr `provenance is limited to 60 display cells\n`. The saved diagnostic reports 62/61 characters. R1 shortened A's provenance, and shortened C's provenance while moving “written” into intent. Source-status meaning survives. Original failures remain archived.
2. A draft/R1 intent: **“no option is selected or order placed.”** The packet establishes no selection, but only forbids inferring a placed order. R2 changes this to **“no option is selected. No order is established as placed.”** This repairs an unsupported categorical absence claim.
3. C draft/R1 intent: **“conditional renewal or carriage is not secured.”** R2 changes it to **“conditional renewal or carriage is not established as secured.”** Unknown securing status is no longer strengthened into known non-securing.
4. C's earlier 80×24 inspection lacked Willow renewal. The author explicitly recorded the gap, added final offset 50 and read it; it is not an unresolved final inspection failure.

Session lines 54 and 101 contain the R1/R2 content edits; final collection is invoked at 106, bundle creation at 110, full bundle reads at 112/114/116, stream audit at 120, inspection record extension at 131, report at 135 and final submission at 137. No coordinator content/layout followup or post-submission content replacement appears. A/C finals equal R2; B/D/E finals equal drafts. `history-diffs.txt` records the only content changes and their effects on original previews. Earlier successful B/D streams are identical to corresponding finals; A/C differences are the documented wording/reflow/footer changes.

## Independent execution and preservation checks

### Actual commands and results

Independent Cargo execution was from `/tmp/hud-fidelity-run1-rS7MUp/checkout`, targeting **archived final paths**, not mutable author work files. `execute.py` invokes the following exact argument pattern for each case and every offset in the coverage table:

```sh
cd /tmp/hud-fidelity-run1-rS7MUp/checkout
CARGO_NET_OFFLINE=true \
CARGO_TARGET_DIR=/tmp/hud-fidelity-run1-rS7MUp/review/target \
TMPDIR=/tmp/hud-fidelity-run1-rS7MUp/review \
cargo run --locked --quiet --example compare -- \
/home/tnez/Work/tnezdev/hud/docs/assets/authoring-fidelity/run-1/author/final/a.compare.json --check
```

For previews, replace `--check` with `--preview WIDTH HEIGHT OFFSET`. B/D use `--example sequence` and their `.sequence.json` files; C uses `compare` and `c.compare.json`. All **37 fully expanded commands**, original independent stdout/stderr and exits are preserved in `review/independent/*.command|stdout|stderr|exit`. Invocation driver:

```sh
python3 /tmp/hud-fidelity-run1-rS7MUp/review/execute.py > /tmp/hud-fidelity-run1-rS7MUp/review/execute.stdout 2> /tmp/hud-fidelity-run1-rS7MUp/review/execute.stderr
```

All 37 exit 0, empty stderr, byte-identical stdout to corresponding author originals. Compare checks report `comparison OK (shape only; display data only)`; Sequence checks report `sequence OK (shape only; display data only)`. No extra viewport was needed to complete reviewer coverage. Reviewer source inspection, byte comparisons and body accounting are reviewer work, not credited to the author. The independent streams duplicate the complete original streams inspected above.

Other review execution: `python3 review/audit.py` verifies manifests, source copies and draft/final correspondence and extracts trace actions; `python3 review/coverage.py` verifies bundle collation and capture intervals. Both ran by their absolute paths with stdout/stderr/exit saved beside them. Inline Python produced `trace-actions.txt`, `trace-preservation.txt`, `history-diffs.txt` and `status-and-post-integrity.txt`: JSON/gzip reads, exact string/edit/copy comparisons, original command-stream status/difference inspection and repeated SHA-256 checks only. No story data was executed. Discovery used `pwd`, `ls`, `find`, `wc` and `rg`; files were examined with read tools.

One reviewer exploration command ended **2**: `ls -la /tmp/hud-fidelity-run1-rS7MUp/checkout/scripts` reported `No such file or directory`. No independent Cargo or audit execution failed. `./scripts/check` was **not rerun**: the allowed frozen checkout has no scripts directory, and this review made no repository documentation/evidence changes. The archived **pretrial** `check-pretrial.exit` is 0 and its log records passing checks; it is not a new reviewer/posttrial check claim. I did not copy a script/test tree or run a broader suite from the original repository to manufacture that evidence.

### Integrity, from the original repository root

Actual verification commands (cwd `/home/tnez/Work/tnezdev/hud`):

```sh
sha256sum -c /home/tnez/Work/tnezdev/hud/docs/assets/authoring-fidelity/run-1/frozen-inputs.sha256
sha256sum -c /home/tnez/Work/tnezdev/hud/docs/assets/authoring-fidelity/run-1/author-submission.sha256
```

Both exit 0, empty stderr: **29/29 frozen-manifest entries** and **412/412 submission-manifest entries** OK. Detailed streams are `frozen-inputs.sha256.stdout/.stderr` and `author-submission.sha256.stdout/.stderr`. Python SHA-256 comparison of every original-root path in `protected-checkpoint.json` finds **375/375 unchanged**, none missing. Repeated after independent execution: same counts, no mismatch (`status-and-post-integrity.txt`).

Additional read-only comparisons: 20 frozen runnable/discovery files equal their temporary-checkout counterparts; all 407 archived author files equal the original temporary author tree; five packets and prompt copies match. Trace/archived artifact comparisons described above go beyond trusting a newly generated manifest. These establish consistency with the supplied checkpoint and session; they do not independently authenticate the earlier push/remote hash or make a git/history/publication claim. No git operation was run. Author process exit is 0 and archived author stderr empty.

## One consequential PO tradeoff

**Faithful qualification can retain criterion/item ownership while spending more author effort and scrolling—and allowing individual screenshots to be incomplete.** A/C demonstrate the sharp end: 61/70 body lines at 72×35 and 56/64 at 80×24, two self-repair rounds each, and 9/12 final captures respectively. C needed an extra view after snapping duplicated the requested middle view; its opening 72×35 capture exposes Willow's ready wording without the local caveat yet visible. B/D retain a more compact connected structure; E preserves incompatible facts without forcing alignment.

The evidence supports retained grouping, reachable local qualifications and this concrete inspection cost. It does **not** establish whether the visual-parsing benefit is worth that cost to people, or whether people comprehend or act better. That acceptability judgment remains Travis's; this review makes no product decision and requests no implementation questionnaire.

**Review complete.** No broad reliability, production-readiness, merge, release, installation, publication or acceptance conclusion is made.
