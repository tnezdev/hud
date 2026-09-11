# Example diagnostic safety — PR #42 F1

## Authority and bounded contract (before code)

Travis authorized addressing the pre-merge review finding after CI passed on
`6cbd8b8`. The [full-branch review](https://github.com/tnezdev/hud/pull/42#issuecomment-5627789754)
found that all three data-only examples reject unknown JSON fields but print
Serde's decoded field names directly to stderr. An escaped ESC in an otherwise
valid JSON member therefore reaches the terminal through an error, before display
text validation. The primary independently reproduced Compare with captured
stderr, without emitting the control bytes to a real terminal.

This fix covers `examples/compare.rs`, `examples/sequence.rs` and
`examples/expressive_story.rs` only. It is **diagnostic-output encoding**, not a
content-sanitizing resolver or a change to the accepted visuals/semantic contract.
No production `src/`, dependencies, lockfile, schemas, parsing rules, fixtures,
entrypoints, draw/wrap/scroll behavior or interval/document WIP changes. No merge,
release, installation or further palette expansion is authorized.

### Output boundary

- All handled `Result` errors from an example's `run` function pass through one
  example-only stderr boundary. This includes parse/validation/argument errors,
  filesystem errors containing a supplied path, and handled terminal/I/O errors.
- Keep the underlying error strings and rejection rules intact; encode once just
  before writing. Do not sanitize source JSON, replace bad input with fallbacks,
  or scatter escaping across parser/loader call sites.
- Escape every Rust `char::is_control()` character (C0, DEL, C1), Unicode bidi
  controls U+061C, U+200E/F, U+202A–E and U+2066–9, and Unicode line/paragraph
  separators U+2028/9. Use visible Rust `escape_default` notation (`\n`, `\t`,
  `\u{1b}`, etc.) instead of emitting or silently deleting those characters.
- Preserve other printable Unicode, punctuation and ordinary diagnostic text.
  Write exactly one trusted trailing newline, independent of embedded newlines.
  This protects handled diagnostics, not all possible dependency panic output or
  every Unicode visual-confusability issue; no universal terminal sandbox claim.
- Return failure even if stderr itself cannot be written. Never fall back to an
  unescaped error/panic message. Successful results emit no diagnostic and keep
  their current success status/stdout untouched.
- **Review-driven clarification, before the cleanup change:** the ordinary live
  exit must call non-printing `ratatui::try_restore()`, not `restore()` (which
  prints and discards its own error). Always attempt restoration after the live
  loop, even when the operation failed. If restoration alone fails, return a
  failure labeled `Failed to restore terminal`. If both fail, keep the original
  operation error first and append the labeled restoration error, then encode
  the combined diagnostic once. A successful restoration preserves the operation
  result. The existing dependency panic hook remains outside this handled-error
  contract; no live loop, draw or navigation logic changes.

A tiny `examples/support/diagnostic.rs` module shared by the three examples owns
this concrete output rule and an injected `Write` sink. This is not a shared
renderer, plugin abstraction, natural-language interpreter or production API.

## Regression and preservation plan

Use in-memory writers and parser inputs; no shell commands, real terminals,
sleeps, wall-clock values or filesystem assumptions in the new unit tests:

1. Hostile unknown fields at top and nested object boundaries for each parser;
   invalid enum variants where the existing schema has an enum.
2. Assert both rejection/failure and the actual safe bytes from the same finishing
   function used by `main`, not merely that the parser rejects the payload.
3. Exhaust C0/DEL/C1 and the named bidi/separator controls in the output helper;
   preserve printable Unicode/normal diagnostics and exactly one final newline.
4. Exercise a synthetic path-bearing I/O diagnostic, successful completion and a
   failing writer; no raw-error fallback when writing fails. Test all combinations
   of operation/restoration success/failure using injected results, including
   both errors' preservation and a single encoded output line.
5. Capture the pre-fix binaries/source identities before implementation; compare
   valid checks and headless rendering afterward, including all 33 run-1 final
   previews and representative fixtures at supported narrow widths/scroll positions.
   Verify draw/wrap/scroll code is byte-identical, including style logic not
   established by plain-text captures alone.
6. Verify the frozen-input/author/review manifests without changing their bytes
   or expected hashes. Current source hashes intentionally change for the three
   output boundaries/tests; earlier “20/20 current equals frozen” reports remain
   historical, not a claim about the fixed sources. No author trial rerun.
7. Run `./scripts/check`, independent focused review, and PR CI including audit on
   the new commit. Preserve the original full-branch finding as review evidence.

The separate F2 discovery-status note remains an erratum in the PR: the active
Compare banner predates run-1, which exercised those exact frozen discovery bytes.
This diagnostic fix does not silently edit or replace that trialed entrypoint.

## Review history

The first candidate passed 100 tests, 180 before/after valid-output comparisons
and 15 hostile-field/variant/path cases. Independent review corroborated the
encoding/preservation results but found that unchanged `ratatui::restore()` still
printed handled cleanup errors before the new finishing function. A captured
closed-pipe probe returned success with a cleanup diagnostic; no attacker-controlled
cleanup message or live-terminal exploit was established. This was a remaining
handled-output/exit-status defect, not a rendering defect. The original review is
preserved separately; it is not being rewritten into a clean first review.

The clarified cleanup rule above was specified before addressing that finding.
All three ordinary live cleanup paths now use `try_restore()`, combine operation
and restoration results without losing either error, then pass the result through
the existing diagnostic boundary. The dependency's panic hook is unchanged.

## Completed implementation and focused re-review

The [initial review](assets/diagnostic-safety/review-initial.md) retains R1 as an
open finding on the first candidate. The [focused re-review](assets/diagnostic-safety/review-final.md)
found **no blocking findings**, resolving original F1 and the cleanup follow-up
within the handled-error contract. This was re-review of the same candidate work,
not a new author trial or a retroactive clean first review.

- `./scripts/check`: **109 passing tests**, formatting and Clippy. The 29 additional
  compiled cases are eight helper tests in each of the three examples plus five
  parser/schema diagnostic tests. Shared helper tests are not 24 independent
  feature scenarios. All new repository tests are in-memory.
- Primary and reviewer each verified **180 identical valid before/after CLI
  invocations**, including all 4 archived final checks and 33 trial previews, plus
  **15** reproduced hostile-field/variant/path cases now rejected and escaped.
  Status, stdout and stderr were compared, not only renderer labels.
- The reviewer independently exercised **284** additional captured CLI rejections,
  all valid Unicode scalars against the specified numeric escape ranges, short /
  interrupted / partial-failure writers, and six no-real-terminal cleanup probes.
  A readerless stdout pipe now produces failure for cleanup-only failure; both
  errors remain visible when the operation also fails. Broken stderr still returns
  failure rather than panicking. No real terminal was initialized for these probes.
- The three pre-fix example sources match `6cbd8b8`. Accounting only for the new
  module reference, main output delegation and cleanup-result delegation, all
  non-test code remains byte-identical. This preserves style/draw/wrap/navigation
  logic, not merely plain-text preview bytes.
- Frozen-input, author-submission and review-submission manifests still verify
  **29/29, 412/412 and 236/236** entries. Current versus the 20 run-1 frozen source /
  discovery files is now **18/20**: Compare and Sequence intentionally differ in
  diagnostic/cleanup plumbing and tests. The old sources and hashes are unchanged
  historical evidence, not silently replaced with fixed implementations. Older
  manifests naming live example sources can therefore differ intentionally;
  validate their archived/versioned inputs, not rewritten expected hashes.

Evidence is under [docs/assets/diagnostic-safety/](assets/diagnostic-safety/):
`verify.py`, `verification.json`, separate first-candidate/complete check logs,
initial and final review reports, and captured cleanup-probe results (unsafe
original bytes are represented as hex, not executable terminal output). Temporary
execution paths in reports are historical pointers. The supplemental reviewer
artifacts remain under `/tmp/hud-diagnostic-fix-AjWPjR/`; they are not all archived
or promised to persist. The initial review's supplemental argv probe excluded
NUL after a recorded OS argv rejection, retaining NUL coverage in JSON/helper
checks. That review iteration is not an application failure or hidden test pass.

The controlled before/after verifier can be rerun from the repository root:

```sh
python3 docs/assets/diagnostic-safety/verify.py BEFORE_BINARY_DIR AFTER_BINARY_DIR OUTPUT.json
sha256sum -c docs/assets/authoring-fidelity/run-1/frozen-inputs.sha256
sha256sum -c docs/assets/authoring-fidelity/run-1/author-submission.sha256
sha256sum -c docs/assets/authoring-fidelity/run-1/review-submission.sha256
./scripts/check
```

Build the three pre-fix binaries from `6cbd8b8` in a separate checkout and the
current binaries from this fix; do not replace historical evidence or display raw
pre-fix stderr from hostile probes. No production source, Cargo/dependency version,
fixture or author entrypoint changed. New PR CI/audit must pass on the fix commit;
old green checks on `6cbd8b8` do not cover this change. No local advisory pass,
interactive cleanup fault injection, broad Unicode-confusability/panic-output
safety, merge, release or installation is claimed.
