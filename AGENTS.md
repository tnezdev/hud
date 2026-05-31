# hud Agent Guide

## Project Shape

`hud` is a single-user local terminal cockpit. Simplicity and legibility win over general plugin-host architecture.

Read `README.md` and `docs/engineering-spec.md` before making non-trivial changes.

## Engineering Principles

- Keep side effects at the edges: command execution, filesystem access, terminal I/O, and time should be explicit boundaries.
- Prefer small, testable core logic over premature framework or plugin abstractions.
- Do not add async, background refresh, long-running plugin processes, or persistent state until the spec calls for them.
- Parse external data once at a boundary. Do not scatter defensive fallbacks through readers.
- Tests must not depend on real sleeps, wall-clock timing, real shell commands, or a real terminal.

## Quality Policy

- `./scripts/check` is the local definition of done for code changes. Run it before calling work done.
- `./scripts/audit` checks for known vulnerable dependencies. It is a separate security gate, not part of `./scripts/check`. CI runs `cargo audit` automatically; locally, install `cargo-audit` and run `./scripts/audit` or `cargo audit` directly.
- Behavior changes should include automated coverage: a unit test, an integration test (e.g., `tests/cli.rs`), example-config validation (`tests/examples.rs`), or a documented reason why no automated test fits.
- Docs-only changes must leave the repo green under `./scripts/check`.

## Current V1 Direction

- Rust binary.
- Local-first TUI, likely `ratatui` + `crossterm` when UI implementation starts.
- Static config, likely TOML.
- Command-backed panels.
- Plain text panel output first, with a structured-output boundary defined early.
- Manual refresh only in the first implementation slice.

## Workflow

- Run `./scripts/check` before calling work done.
- Run `./scripts/audit` before release or when dependency security matters. CI runs it on every push.
- Update `README.md`, `docs/engineering-spec.md`, or this file when behavior, structure, or workflow changes.
- For non-trivial implementation, write or update the relevant spec before code.
- Keep commits small and honest.
