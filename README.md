# hud

A programmable terminal cockpit for getting oriented, finding the next thing that needs attention, and jumping into action from the keyboard.

## Status

Pre-implementation scaffold. The accepted product PRD lives in `tnezdev/proposals` at `proposals/hud.md`.

## Development

```sh
cargo run
```

```sh
./scripts/check
```

## Engineering Questions

Before implementation, settle the first architecture slice:

- What is the minimal core loop for config, panel execution, rendering, input, and actions?
- What should the v1 panel output protocol guarantee?
- What test boundaries give fast confidence before every deploy?
- What belongs in core now, and what should stay as shell scripts or examples?
