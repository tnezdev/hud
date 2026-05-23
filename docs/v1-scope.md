# hud V1 Scope

## Goal

V1 proves the cockpit loop:

```text
open -> orient -> move focus -> refresh -> act
```

The product should be useful as a local terminal home surface before it becomes a broader platform. A user should be able to install `hud`, configure a handful of command-backed panels, open it in a terminal or tmux popup, understand what needs attention, and trigger simple local actions from the keyboard.

## In Scope

- Rust binary.
- Static TOML config.
- Config discovery at `$XDG_CONFIG_HOME/.hud/config.toml`.
- One home dashboard.
- Command-backed panels.
- Plain text panel output.
- Manual refresh for focused panel and all panels.
- Vim-like focus navigation between panels.
- Configured fire-and-forget actions.
- Visible error states for broken panel commands.
- Bounded command execution with a default timeout.
- A source-install path from GitHub.
- Documentation sufficient to install, configure, run, and dogfood V1.

## Out of Scope

- Background refresh.
- Long-running plugin processes.
- General plugin-host architecture.
- Persistent state.
- Durable dismissals, action history, or caching.
- Agent workflows.
- Embedded browser surfaces.
- Free-form dashboard input flows.
- Multiple dashboard pages.
- User accounts, auth, secrets, or external service ownership.

## Product Constraints

`hud` is a single-user local cockpit. Simplicity and legibility win over generality.

Command execution, filesystem access, terminal I/O, and time are edge effects. Core logic should be testable without a real terminal, real shell commands, real sleeps, or wall-clock time.

## V1 Milestones

### 1. Core Model

Define the data model for config, panels, actions, dashboard state, command results, panel errors, and focus movement.

Acceptance criteria:

- Core types describe the cockpit loop without terminal or shell dependencies.
- Focus movement can be tested with plain unit tests.
- Panel state supports at least idle, loading, ready, and error outcomes.

### 2. Config Boundary

Load and validate static TOML config from `$XDG_CONFIG_HOME/.hud/config.toml`.

Acceptance criteria:

- Missing config produces a clear user-facing error.
- Invalid config produces a clear user-facing error with enough context to fix it.
- Config parsing happens once at the boundary.
- Readers consume typed config, not loose TOML values.

### 3. Command Boundary

Add an injectable command runner for panel refreshes and actions.

Acceptance criteria:

- Unit tests use a fake runner, not real shell commands.
- Real command execution is wired at the binary edge.
- Command results capture stdout, stderr, exit status, timeout, and execution failure distinctly.
- Default panel command timeout is decided and documented before implementation.

### 4. First TUI Shell

Add `ratatui` and `crossterm` only when there is typed dashboard state to render.

Acceptance criteria:

- App opens in a terminal.
- App can quit cleanly from the keyboard.
- App renders fake dashboard state before real commands are wired through.
- Focused panel is visually distinct.

### 5. Live Panels

Wire config-loaded panels to command execution and manual refresh.

Acceptance criteria:

- Initial load runs configured panel commands or clearly indicates panels need refresh.
- User can refresh focused panel.
- User can refresh all panels.
- Broken commands render error states instead of crashing the app.
- Slow commands are bounded by timeout behavior.

### 6. Actions

Support configured fire-and-forget actions from the focused panel.

Acceptance criteria:

- Actions are visible enough for the user to discover available keys.
- Action commands run through an injectable boundary.
- Failed action launches produce visible feedback or a clear error path.
- Actions do not require Rust changes.

### 7. Dogfood Dashboard

Create a real local config that proves the cockpit loop with useful panels.

Candidate panels:

- tmux sessions.
- dirty repos.
- GitHub PRs or notifications.
- Taskwarrior tasks.
- running or recent agents.

Acceptance criteria:

- The dashboard is useful enough to open at the start of a work session.
- The layout is spacious rather than a dense wall of text.
- At least one configured action jumps into a real workflow.

### 8. Source Install

Provide an install path from GitHub.

Acceptance criteria:

- A user can install from the repository with a documented terminal command or GitHub-driven flow.
- Installation does not require unpublished local state.
- README documents install, config path, first run, and tmux popup usage.

## Semantic Components

The PRD names this minimum semantic component set:

- text
- markdown
- list
- table
- status cards
- tree
- log tail
- diff
- action list

V1 should not require all of these to be polished before the cockpit loop works. Plain text is the first supported panel output. The structured-output boundary should be defined early enough that semantic components can be added deliberately instead of by parsing ad hoc text throughout the app.

## Decisions

- V1 optimizes for a single-user local cockpit, not a general plugin host.
- V1 uses manual refresh only.
- V1 starts with static TOML config.
- V1 config discovery starts at `$XDG_CONFIG_HOME/.hud/config.toml`.
- V1 panel sources are command-backed.
- V1 actions are fire-and-forget local commands.
- V1 deploy means a source-install path from GitHub.

## Open Scope Questions

1. What is the exact TOML config shape?
2. Does V1 implement structured output, or only reserve and document the boundary?
3. Is the default command timeout 2 minutes, 5 minutes, or something panel-type-specific?
4. Which semantic component comes immediately after plain text?
5. Should initial panel commands run automatically on app open, or should the first screen wait for manual refresh?
