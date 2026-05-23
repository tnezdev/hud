# hud Engineering Spec

## Product Constraint

`hud` is a single-user local cockpit. Simplicity and legibility win over general plugin-host architecture.

The core should make local scripts and commands feel coherent, spacious, and action-oriented. It should not begin life as a full extension platform with complex process supervision, version negotiation, or remote lifecycle management.

## Fat-Marker Architecture

```text
static config
  -> app state builder
  -> panel runner boundary
  -> panel output parser
  -> dashboard state
  -> ratatui renderer
  -> input/action dispatcher
  -> command launcher boundary
```

## Core Invariants

1. A broken panel cannot crash the app.
2. Command execution is an edge effect, not mixed into rendering or state decisions.
3. Rendering consumes typed dashboard state, not raw command output.
4. Tests never depend on real time, real sleeps, real terminals, or real shell commands.
5. Plain text output is valid v1 panel content.
6. Structured output is parsed once at the boundary, not defensively interpreted everywhere.
7. The v1 architecture optimizes for one local user editing config and scripts directly.

## Initial Module Boundaries

```text
src/main.rs          // binary wiring only
src/app.rs           // event loop coordination
src/config.rs        // static config parse + validation
src/panel.rs         // panel model, panel states, output contract
src/command.rs       // command execution trait + real adapter
src/action.rs        // action model + dispatch decisions
src/ui.rs            // ratatui rendering from typed state
```

These boundaries are provisional. They should appear only when code needs them, not as empty architecture scaffolding.

## Testing Strategy

- Unit tests cover config validation, output parsing, focus movement, and action resolution.
- Command execution is tested through an injectable runner, not real shell commands.
- Contract tests appear when there is more than one command-runner implementation.
- Rendering tests wait until the UI structure stabilizes enough to make snapshots useful.
- One smoke spec should eventually run a tiny fixture config with a fake command runner.

## Initial Biases

- Prefer TOML for static config unless it proves awkward.
- Start with plain text panel output, but define the structured-output boundary early.
- Use `ratatui` and `crossterm` for the TUI once the core state/config/panel contract is testable.
- Start with manual refresh only.
- Avoid an async runtime until interval refresh, command execution, or input handling proves it is needed.
- Treat long-running plugins, generated UI, approvals, and agent supervision as later design directions.

## Refresh Model

V1 starts with manual refresh only.

- Manual refresh reruns either the focused panel or all panels, depending on the keybinding.
- There is no background polling in the first implementation slice.
- A slow panel should not block input handling or crash the dashboard.
- Long-running panel processes are out of scope for v1; each refresh is a bounded command invocation.
- Per-panel interval refresh remains a likely later extension. When added, time must enter through an injectable clock or tick source so tests can advance time deterministically.

## Open Decisions

1. What is the smallest useful config shape?
2. Should structured panel output be newline-delimited JSON in v1, or a single JSON document per refresh?
3. When should per-panel interval refresh be introduced?
4. What is the minimum semantic component set for the first useful dashboard?
5. What does a deploy mean initially: local binary, GitHub release artifact, or package manager path?
