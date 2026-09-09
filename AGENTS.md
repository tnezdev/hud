# hud Agent Guide

## Project Shape

HUD is moving toward a single-user, local-first semantic visual language for
agent communication; dashboards are one application. The agent selects meaningful
visual idioms, and HUD resolves their presentation. The current runtime remains a
command-backed terminal cockpit. Simplicity and legibility win over general
plugin-host architecture.

Read `README.md`, `docs/engineering-spec.md`, and
`docs/semantic-visual-language.md` before making non-trivial changes. Establish the
Sequence reference contract (including why/when and counterexamples) before
expanding the palette. Preserve the demonstrated timeline; busy prose wrapping is
not a mandate to remove its connectors. Prior example schemas are evidence, not
approved production architecture.

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

## Existing V1 Runtime (Not the Full Product Boundary)

- Rust binary with `ratatui` + `crossterm`.
- Static TOML config and command-backed panels.
- Text, table JSON, and metrics JSON outputs parsed at an explicit boundary.
- Manual refresh; the interval/document experiment remains separate WIP.
- The example-only flowing-story JSON and preview are not the new semantic contract.

## Workflow

- Run `./scripts/check` before calling work done.
- Run `./scripts/audit` before release or when dependency security matters. CI runs it on every push.
- Update `README.md`, `docs/engineering-spec.md`, or this file when behavior, structure, or workflow changes.
- For non-trivial implementation, write or update the relevant spec before code.
- Keep commits small and honest.
