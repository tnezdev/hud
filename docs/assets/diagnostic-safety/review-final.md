# Focused re-review — PR #42 diagnostic safety

## Verdict

**No blocking findings. R1 is resolved, and original F1 is resolved within the documented handled-error scope.** This is a technical re-review, not product acceptance or merge authorization.

Reviewed the current delta against `6cbd8b8e3a7404269a7df6992e95c2a7dc2574e0`, the clarified diagnostic contract, shared helper and tests, all three live/run/main routes, and the preservation verifier. The first review's P2 was valid for that candidate; this correction closes it rather than retroactively making the initial review clean.

## R1 correction

At `examples/compare.rs:659`, `examples/sequence.rs:479` and `examples/expressive_story.rs:287`, the ordinary cleanup path now calls:

```rust
diagnostic::after_restore(result, ratatui::try_restore())
```

The loop result is already captured; argument evaluation invokes restoration regardless of whether that result succeeded. There is no short-circuit `?` between loop completion and cleanup. `try_restore()` returns its error instead of printing it. Initialization and loop behavior are unchanged.

`examples/support/diagnostic.rs:8–17` combines results without encoding or writing:

| Operation | Restoration | Returned result |
|---|---|---|
| success | success | success |
| failure | success | original error text unchanged |
| success | failure | labeled restoration failure |
| failure | failure | complete original error first, then labeled restoration failure |

Each example main sends that result through `finish` once. There is no remaining ordinary `ratatui::restore()` call or direct diagnostic print in these example routes. Argument, load, parse, validation, preview and returned live errors likewise flow to `finish`. The unchanged dependency panic hook remains the existing exclusion; no handled cleanup error is newly excluded.

### Independently reproduced cleanup behavior

The new captured probe links the actual resolved ratatui library and actual current helper. It never initializes a terminal or enables raw mode. A readerless stdout pipe forces real restoration-write failure; all other output is captured.

All six cases passed (`restore-probe.json`):

- Both succeed: exit 0, no stderr.
- Cleanup alone fails: exit 1, exactly `Failed to restore terminal: Broken pipe (os error 32)` plus one LF. This directly reverses the original probe's incorrect success exit.
- Operation alone fails: exit 1, original diagnostic safely encoded once.
- Both fail: exit 1, original diagnostic retained first, restoration label appended, exactly one final raw LF.
- Cleanup-only and combined failure with **stderr also connected to a readerless pipe**: exit 1, not a panic exit.

The synthetic operation message includes ESC, newline and printable Unicode. The captured successful restoration command bytes are recorded only as hex; nothing was sent to a real terminal. This probes the dependency/output edges, not an interactive execution of the examples.

## Original F1 and tests

The encoder still handles C0/DEL/C1, the specified bidi controls, and Unicode line/paragraph separators. Printable Unicode and literal escape notation survive unchanged. Parsing inputs are not sanitized, and `after_restore` retains raw error text until `finish` encodes the combined result. Writable sinks receive one trusted trailing LF; partial/failed sinks may receive only a prefix but still produce failure without a raw fallback.

The three new helper tests exercise all four operation/restoration combinations and encoded restoration-only/combined errors. They use only injected results, synthetic errors and byte vectors. All added repository tests remain in-memory: no new shell, terminal, sleep, time or runtime filesystem assumptions.

## Executed checks

Commands below ran from `/home/tnez/Work/tnezdev/hud`. Abbreviations: `R=/tmp/hud-diagnostic-fix-AjWPjR/review-final`, `B=/tmp/hud-diagnostic-fix-AjWPjR/before`.

1. `CARGO_TARGET_DIR=$R/target CARGO_NET_OFFLINE=true ./scripts/check` — exit 0; formatting, all-target/all-feature Clippy with warnings denied, and **109 passing tests**. Counts: 39 library, 8 CLI, 1 fixtures, 16 Compare, 17 Expressive Story, 24 Sequence, 4 story preview. See `check.log`.
2. `CARGO_TARGET_DIR=$R/target CARGO_NET_OFFLINE=true cargo build --offline --examples` — exit 0. See `build.log`.
3. `TMPDIR=$R python3 docs/assets/diagnostic-safety/verify.py $B $R/target/debug/examples $R/verification.json` — exit 0; **180 identical valid before/after invocations**, including **37 archived run-1 checks/previews (4 checks, 33 previews)**; **15 original injection cases reproduced and safely encoded**. Status, stdout and stderr were compared; archived stdout bytes also matched. See `verifier.log`, `verification.json`.
4. `python3 $R/independent_checks.py` — exit 0; source/manifest checks below and **284 supplemental captured before/after CLI rejections**. Coverage includes every specified control individually in unknown fields; invalid-type strings containing all controls; map-form invalid enum variants and wrong variant types; malformed, oversized and invalid-UTF-8 inputs; validation/argument failures; and actual open/read failures with hostile paths. Current stderr exactly matched independently encoded old stderr, with one final LF and no specified raw controls before it. See `independent-checks.json` and `.log`.
5. `rustc --edition=2024 --test $R/independent_helper.rs -o $R/independent_helper`, then `$R/independent_helper` — **14 passing tests**: eight current helper tests plus six independent tests. Additional checks cover every valid Unicode scalar against explicit numeric ranges; literal escapes/Unicode; interrupted and short writes; partial failures/write-zero; untouched success sinks; and all cleanup combinations with exact raw-text preservation before encoding. See `helper-tests.log`.
6. `rustc --edition=2024 $R/restore_probe.rs -L dependency=$R/target/debug/deps --extern ratatui=$R/target/debug/deps/libratatui-a037811e45d5268e.rlib -o $R/restore_probe`, then `python3 $R/restore_probe.py` — six cases passed as detailed above. See `restore-probe.log`, `restore-probe.json`.
7. `sha256sum -c` for each `docs/assets/authoring-fidelity/run-1/{frozen-inputs,author-submission,review-submission}.sha256` — all passed. Command 4 also independently parsed and hashed every entry. See `manifests.log`.
8. `git diff --check HEAD` — exit 0. Saved tracked diff and final status as `diff.patch` and `final-status.txt`; inspected output call sites with `rg`, saved in `output-paths.txt`.
9. Inline `python3` byte/hash comparisons confirmed the archived evidence copies listed below and matched all independently generated valid invocation records to the supplied final evidence. Results are in `summary.json`, along with checked exit statuses and the 109-test count.

The supplemental scripts were copied into this new review directory and adjusted there only; the original scripts/results were not modified.

## Preservation and historical evidence

- Independently checked each archived pre-fix example source against HEAD. For all three examples, all code before main is byte-identical after accounting for **exactly** the module declaration and cleanup-result delegation. Separately asserted the exact new main delegation and unchanged old test bodies. Therefore parsing, schemas, loading, init, loop, draw/style, wrapping, navigation and preview logic remain unchanged. Source/binary fingerprints are in `independent-checks.json`.
- The tracked delta contains only the engineering-spec addition and three example files. Production source, Cargo manifests/lockfile, fixtures and author entrypoints remain unchanged.
- Valid checks/previews preserve bytes across the verifier's fixtures, widths 64/72/80, heights 12/35/24, offsets 0/8/20/65535, and Expressive Story's ordinary/plain modes. Archived final captures also match. Unchanged style/render source and passing in-memory style tests supplement the plain-text comparisons.
- Manifest entries pass **29/29 frozen-inputs, 412/412 author-submission, 236/236 review-submission**. Manifest files themselves equal HEAD bytes. Current versus frozen source remains **18/20 equal**: the intentional differences are Compare and Sequence main/test/cleanup code. Frozen bytes and historical 20/20 evidence were not rewritten.
- `docs/assets/diagnostic-safety/review-initial.md` exactly equals the original `review/report.md`; the archived initial restore probe likewise exactly equals `review/restore-probe.json`. The separate initial check log/exit match the supplied originals, and committed-evidence `verification.json` exactly matches the supplied `verification-final.json`. Their hashes are in `summary.json`.

## Limits and authority

No real TUI, pane operation, author trial, network, installation, source modification, git mutation, commit, push or merge was performed. No raw hostile filenames or pre-fix control bytes were displayed. New report and execution artifacts were written only under `review-final/`; original review evidence remains intact.

The closed-pipe probe does not exercise interactive event handling or actual raw-mode restoration failure. Those returned-error routes were inspected in unchanged source. Supplied pre-fix binaries were executed and fingerprinted, not independently rebuilt/provenance-attested; current binaries were independently built offline.

Neither cargo-audit nor hosted CI was run. Hosted CI/audit on the eventual new commit remains outstanding. The review does not assert panic-output safety, broad Unicode/confusability protection or universal terminal containment. Accepted visual tradeoffs and the separate discovery-status erratum were not reopened.

**R1 closed; F1 resolved for this bounded contract. Travis retains product acceptance and merge authority.**
