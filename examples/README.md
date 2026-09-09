# hud Examples

These files are valid `hud` configs. They are both dogfood fixtures for design work and living documentation for users.

Run one with:

```sh
cargo run -- --config examples/kitchen-sink.toml
```

## Configs

- `starter.toml`: safe first-run config with no external tool dependencies.
- `dogfood.toml`: local working cockpit for real tools like `tmux`, `gh`, and `task`.
- `kitchen-sink.toml`: static design fixture for panel density, action discovery, long output, typed table output, metrics widgets, and row drill-in.
- `aesthetic-lab.toml`: stylized, dependency-free mission-control fixture for typography, geometric signals, spatial hierarchy, typed instruments, and drill-in.
- `film-night-organizer.toml`: fictional decision brief; recommendation, reasons, and venue comparison.
- `film-night-guests.toml`: the same event recomposed for arriving guests; destination, reassurance, and route, without the obsolete comparison.

## Agent authoring trial

Read [the HUD skill](../skills/hud/SKILL.md) for purpose-first composition guidance.
Run the film-night configs from the checkout root: their fixed commands read files
under `examples/film-night/`. They need a POSIX shell and `cat`, but no network,
live clock, credentials, or user configuration. The data is entirely fictional.

For a fixture-only text inspection of the production renderer, without starting
a terminal or executing commands:

```sh
cargo run --quiet --example story_preview -- organizer 120 40
cargo run --quiet --example story_preview -- organizer 80 24 compare
cargo run --quiet --example story_preview -- guests 80 24
```

The optional last argument opens a panel detail by its ID. This tool accepts only
these two fixtures, not arbitrary configs. It injects their exact committed data
at the command-result boundary and prints rendered cells; it does not verify real
command execution, colors, keyboard events, or human comprehension. Its tests run
in `./scripts/check`. See [the experiment contract](../docs/agent-canvas.md).
