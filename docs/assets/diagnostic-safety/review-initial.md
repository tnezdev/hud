# Independent review — PR #42 bounded diagnostic-safety delta

## Verdict

**One P2 finding: a handled cleanup-error path still bypasses the new diagnostic boundary.** The demonstrated Serde unknown-field/enum and supplied-path injection mechanisms are fixed, but I would not mark the complete handled-diagnostic contract/F1 follow-up resolved until R1 is addressed or explicitly excepted by the PO.

Reviewed against `6cbd8b8e3a7404269a7df6992e95c2a7dc2574e0`, including `AGENTS.md`, the diagnostic contract/spec section, all tracked changes and new files, each example's parser/load/live/preview/run/main output paths, all added tests, and `docs/assets/diagnostic-safety/verify.py`. No repository files were edited; no commit, push, merge, pane operation, agent consultation, installation, network access, or real TUI was performed. All review artifacts are in this directory.

## R1 — P2: ordinary terminal restoration errors print outside `finish`

**Locations:** `examples/compare.rs:659`, `examples/sequence.rs:479`, `examples/expressive_story.rs:287`; these execute before the newly changed mains at lines 704, 518 and 326 respectively.

Each `live()` still calls `ratatui::restore()`. In the resolved dependency, `ratatui-0.30.0/src/init.rs:457–461`, this function handles `try_restore()` failure itself:

```rust
if let Err(err) = try_restore() {
    std::eprintln!("Failed to restore terminal: {err}");
}
```

This is an **ordinary handled error**, not the explicitly excluded panic-hook path. `try_restore()` can fail while disabling raw mode or writing/flushing `LeaveAlternateScreen` to stdout. Its error is printed raw and discarded before `run()` returns and before `diagnostic::finish` can encode anything. Consequently:

- This diagnostic does not pass through the specified single encoding boundary.
- A cleanup failure can emit a diagnostic even when the enclosing operation returns `Ok(())`; the new helper then returns success.
- When the live operation also fails, cleanup can add a separate diagnostic before the encoded run error, rather than the promised single diagnostic line.

**Independent evidence:** `restore_probe.rs` links the actual resolved ratatui library and invokes this exact cleanup call, followed by the actual `finish(Ok(()), ...)`. `restore_probe.py` supplies a stdout pipe with its reader already closed and captures stderr. No terminal is initialized, no raw mode is enabled, and no real terminal receives output. The process returned **0**, with captured stderr:

```text
Failed to restore terminal: Broken pipe (os error 32)
```

See `restore-probe.json`, `restore-probe.log`, and the copied dependency source `ratatui-0.30.0-init.rs`. This is a dependency cleanup-path probe, not a claim to have exercised the examples interactively.

**Impact qualification:** The cleanup call predates this delta. I did not establish attacker-controlled source/path text in these concrete OS cleanup errors, so this is not a newly demonstrated terminal-injection exploit through restoration. It is a functional/output-boundary coverage defect in the advertised fix. The original JSON/path attack paths tested below are safely encoded. The exception for unchanged panic output does not cover this normal `Err` handler.

**Requested bounded correction:** Use the non-printing `ratatui::try_restore()` edge and incorporate its error into the result sent to the existing `finish` boundary. Specify precedence/combination when both the live operation and restoration fail, without losing the original failure. Add in-memory tests for operation-success/cleanup-failure and both-fail cases; keep parsing, drawing, wrapping, scrolling, schemas, dependencies and frozen inputs unchanged. This needs no renderer or runtime redesign. Rerun the bounded checks afterward.

## What passed

### Encoding and handled paths that reach `finish`

- All three mains delegate their `run()` results once to the shared example-only helper. Argument/number/dimension failures, load/open/read failures, JSON syntax/schema/type/variant failures, validation failures, preview errors, and returned live init/draw/event-read errors reach that boundary. R1 is the separate cleanup side output.
- The helper encodes C0, DEL, C1, U+061C, U+200E/F, U+2028–E and U+2066–9 with `escape_default`; ordinary text and existing literal escape notation are preserved. No parser/input sanitization or repeated call-site encoding was introduced.
- On a writable sink, the helper writes exactly one raw final LF, with embedded CR/LF and separators escaped. Empty errors still produce one LF. On writer failure it returns failure without an unsafe fallback; a failing sink can naturally retain only a prefix, so complete delivery/newline cannot be guaranteed in that case.
- The added repository tests are in-memory: parser values, compile-time fixtures, byte vectors, synthetic I/O errors and an injected broken writer. They introduce no shell, terminal, sleep, clock or runtime filesystem assumptions. Unknown-field tests cover all struct object levels in all three schemas; enum tests cover Compare and Sequence. Expressive Story has no enum.

### Independently executed verification

All commands ran from `/home/tnez/Work/tnezdev/hud` unless the paths themselves specify otherwise. For brevity, `R=/tmp/hud-diagnostic-fix-AjWPjR/review` and `B=/tmp/hud-diagnostic-fix-AjWPjR/before` below.

1. `CARGO_TARGET_DIR=$R/target CARGO_NET_OFFLINE=true ./scripts/check` — exit 0: formatting, all-target/all-feature Clippy with warnings denied, and **100 passing tests**. Logs: `check.log`, `check.exit`.
2. `CARGO_TARGET_DIR=$R/target CARGO_NET_OFFLINE=true cargo build --offline --examples` — exit 0. Logs: `build.log`, `build.exit`.
3. `TMPDIR=$R python3 docs/assets/diagnostic-safety/verify.py $B $R/target/debug/examples $R/verification.json` — exit 0: **180 identical valid before/after invocations**, including **37 archived run-1 checks/previews (4 checks + 33 previews)**, and **15 reproduced pre-fix hostile JSON/path diagnostics now escaped**. This compares status, stdout and stderr, and checks archived stdout bytes. Pre-fix control bytes were captured, not displayed. Logs: `verifier.log`, `verification.json`.
4. `rustc --edition=2024 --test $R/independent_helper.rs -o $R/independent_helper`, then `$R/independent_helper` — **10 passing tests**, comprising the helper's five existing tests plus five independent tests. Independently covered every valid Rust Unicode scalar against explicit numeric ranges; literal escapes/Unicode preservation; interrupted and one-byte writes; partial failures and write-zero; and a success sink that panics if touched. Logs: `helper-tests.log`.
5. `python3 $R/independent_checks.py` — exit 0: independent source preservation and manifest checks, plus **284 supplemental captured before/after CLI rejections**. These cover every specified control individually in unknown field names, invalid-type strings containing all controls, map-form enum variants and invalid variant types, malformed/oversized/invalid-UTF-8 input, validation errors, argument failures, and actual open/read failures with hostile Unicode paths. Each current stderr exactly matches a separately encoded old diagnostic, contains one trailing LF and no specified raw controls before it. See `independent-checks.json`. The initial scratch run stopped because OS argv cannot contain NUL; the corrected rerun excludes NUL only from argv, retaining NUL coverage in JSON and helper tests. The initial log is retained as `independent-checks-initial.log`.
6. `sha256sum -c` on each `docs/assets/authoring-fidelity/run-1/{frozen-inputs,author-submission,review-submission}.sha256` — all entries passed; additionally independently parsed/hashed by command 5: **29/29, 412/412, 236/236**. The three manifest files themselves equal HEAD bytes. Log: `manifests.log`.
7. Cleanup probe compilation: `rustc --edition=2024 $R/restore_probe.rs -L dependency=$R/target/debug/deps --extern ratatui=$R/target/debug/deps/libratatui-a037811e45d5268e.rlib -o $R/restore_probe`; then `python3 $R/restore_probe.py` — reproduced R1 as described above.
8. `git diff --check HEAD` — exit 0. Saved full tracked diff as `diff.patch`, new-file inventory as `untracked.txt`, and final worktree status as `final-status.txt`.

### Preservation

- The archived pre-fix source for all three examples exactly matches HEAD. Independently compared **all source before main**, excluding only the newly inserted module declaration: byte-identical. Thus schemas, validation, load, renderer/style, wrapping, scrolling, preview, live and run implementations were not modified. All old test bodies remain exact suffixes; new tests were prepended.
- The only tracked changes are the engineering-spec addition and three example files. New files are the diagnostic helper, bounded contract document and verifier. Production `src/`, Cargo manifests/lockfile, fixtures, entrypoints and frozen evidence are unchanged from HEAD.
- Rechecked current versus the 20 frozen run-1 files: **18/20 equal**. The two expected differences are `examples/compare.rs` and `examples/sequence.rs`; their current main/test changes do not invalidate the preserved frozen hashes. Historical 20/20 claims were not rewritten or represented as current.
- Valid preview bytes are unchanged across representative fixtures at 64×12, 72×35 and 80×24 and offsets 0, 8, 20 and 65535, including both expressive/plain modes where supported. Archived run-1 captures also match. Source equality covers style logic not visible in text captures, and existing in-memory render/style tests pass.

## Verification limits and authority

- No real-terminal/live-session fault injection or visual acceptance trial was run. Returned live I/O routing was inspected in source; restoration was separately probed without terminal initialization. Captured previews plus unchanged source do not constitute a new human visual acceptance.
- Supplied pre-fix binaries were executed and fingerprinted; their companion source was verified against HEAD, but the old binaries were not independently rebuilt or provenance-attested. Current examples were independently built offline in the separate review target directory.
- Neither cargo-audit nor hosted CI was run. Cargo-audit is unavailable locally; hosted CI/audit on the eventual new commit remains a later gate. No audit/CI success is claimed.
- No broad Unicode/confusability or panic-output guarantee is asserted. Accepted visual tradeoffs, the F2 discovery-status erratum, and unrelated runtime behavior were not reopened.
- Travis retains product acceptance and merge authority. This review performs and authorizes no merge.
