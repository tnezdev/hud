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
