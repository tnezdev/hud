# TUI Design Spike

Issue: `tnezdev/hud#12`

## Direction

`hud` should feel less like a generic terminal dashboard and more like a compact cockpit: one clear place to orient, one obvious focus target, and fast keyboard paths into action.

## Aesthetic Lab Experiment

`examples/aesthetic-lab.toml` is the visual pressure fixture for the next spike. It uses the existing four-panel mission-control composition, typed metrics and tables, row drill-in, and actions to explore a more authored terminal identity without adding a theme or component framework.

The experimental treatment should remain terminal-native:

- Inherit the terminal background instead of painting a web-style surface.
- Use one restrained ANSI accent for orientation and focus, with text, symbols, border weight, or reverse video carrying the same meaning when color is unavailable.
- Give focused panels stronger geometry and keep ready panels visually quiet.
- Use compact geometric symbols with an ASCII fallback; do not require emoji, Nerd Fonts, or private-use glyphs.
- Preserve labels and numeric values in metrics instead of encoding meaning in bar color alone.
- Fall back from the three-up / one-wide composition when the terminal is too narrow for three legible instruments.
- Respect `NO_COLOR`; use `HUD_ASCII_UI=1` to exercise the portable glyph treatment.

The spike is successful when the fixture feels intentional at the documented 100×30 comfort size, remains usable at 80×24, and its dashboard, panel detail, row detail, loading, ready, and action-help states remain legible without color.

For the next implementation slice, prefer a focused home surface over tabs, modes, or a component framework. The useful experiment is not "more widgets"; it is whether a small set of Ratatui primitives can make command-backed panels feel spatial, calm, and actionable.

## Phase 1: Cockpit Surface

Use `examples/kitchen-sink.toml` as the local fixture for exploratory UI work. It should remain a normal `hud` config rather than a special command-line mode, so design ideas are exercised through the same path users run.

Start with a card-grid home plus drill-in detail:

- A thin orientation band at the top for title, current intent, and global status.
- A card grid of configured panels for broad situational awareness.
- Spatial focus movement with `h`/`j`/`k`/`l` matching left/down/up/right.
- `Enter` drills into the focused card and gives it the full detail surface.
- `q`, `x`, or `Esc` returns from detail to the card grid.
- In detail mode, `j`/`k` scroll panel output. Other direction keys should remain available for future panel-local navigation instead of hidden card focus movement.
- A help/action overlay opened with `?` instead of a dense permanent footer. It should show global keys for the active view and actions for the focused panel.

## View Tree Direction

The desired interaction model is a rendering tree / view stack, not a fixed pair of modes.

Example target flow:

```text
Dashboard card grid
  -> Panel detail
    -> Selected row detail
      -> Deeper configured views as needed
```

Each active view should define:

- What it renders.
- What keyboard input means in that view.
- What actions are available.
- What data it needs and whether that data is loading.
- How the user returns to the parent view.

The current `Grid` / `Detail` behavior is useful spike scaffolding, but it should evolve toward an explicit active-view model and eventually a stack of views. Input should be routed to the active view only: `h`/`j`/`k`/`l` in the dashboard move between cards, while the same keys in a panel detail can scroll or select rows. `Enter` activates the selected item in the current view. `q`, `x`, or `Esc` should pop one view back toward the dashboard.

Future row drill-in should be user-configured. A panel may expose rows; selecting a row may enter a configured detail view that runs an on-demand command, shows a loading state, then renders the result. Fetching belongs at view-entry / command boundaries, not inside rendering.

Near-term implementation direction:

- Rename the current mode concept toward an active `View` enum.
- Keep rendering pure from app state.
- Route input by active view.
- Add a stack only when a second drill-in level exists.
- Add row selection before configurable row-detail commands.

Ratatui components to lean on first:

- `Block`: visual hierarchy through border style, title placement, padding, and focus treatment.
- `Paragraph`: default plain-text panel renderer for V1 command output.
- `List` and `ListState`: panel navigation, launcher-like surfaces, and action menus.
- `Table` and `TableState`: structured panel experiments once command output has a typed boundary.
- `Scrollbar`: large output affordance when panel content exceeds its viewport.
- `Clear`: modal overlays for help, actions, or focused detail without growing the footer.

Design constraints:

- Keep side effects at the edges; rendering should consume typed dashboard state.
- Do not introduce a plugin/component architecture for the spike.
- Do not introduce background refresh or persistent state.
- Keep plain text as valid panel content.
- Add selection or scroll state only where the interaction requires it.

## Phase 2: Data And Aggregates

Data widgets are likely valuable after the cockpit surface has a stable shape. Usage, counts, trends, and other aggregations should get visual affordances, but they should not drive the first redesign.

Good Phase 2 candidates:

- `Gauge` / `LineGauge`: budget usage, quota, command progress, or status fullness.
- `Sparkline`: compact trends such as usage over time, issue flow, or task throughput.
- `BarChart`: small categorical aggregates such as issue counts, repo states, or task buckets.
- `Chart`: only if there is a real graphing use case; likely too heavy for V1.
- `Canvas`: reserve for custom cockpit visuals if stock widgets cannot express the product shape.

Phase 2 should start only after structured panel output is defined at the parser boundary. Avoid parsing arbitrary text in UI code to feed charts.

## Open Questions For The Live Spike

- Does a panel list plus focused detail feel faster than the current card grid?
- Should secondary panels be visible at all, or should `hud` commit to one focused surface?
- Is the action overlay discoverable enough to replace the footer-heavy model?
- What minimum scroll behavior is needed for long command output?
- Which panel outputs deserve typed summaries before charts enter the product?
