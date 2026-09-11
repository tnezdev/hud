#!/usr/bin/env bash
# Evidence collection only: invoke frozen examples, preserve original streams.
set -u
cd /tmp/hud-fidelity-run1-rS7MUp/checkout || exit 1
export CARGO_NET_OFFLINE=true
out=/tmp/hud-fidelity-run1-rS7MUp/author/evidence/initial
mkdir -p "$out"
printf 'cwd=%s\nCARGO_NET_OFFLINE=%s\n' "$PWD" "$CARGO_NET_OFFLINE" > "$out/environment.txt"
run() {
  local name=$1
  shift
  printf '%q ' "$@" > "$out/$name.command"
  printf '\n' >> "$out/$name.command"
  "$@" > "$out/$name.stdout" 2> "$out/$name.stderr"
  local status=$?
  printf '%s\n' "$status" > "$out/$name.exit"
  printf '%s exit=%s\n' "$name" "$status"
}
for pair in a:compare b:sequence c:compare d:sequence; do
  packet=${pair%:*}
  example=${pair#*:}
  input=/tmp/hud-fidelity-run1-rS7MUp/author/drafts/$packet.$example.json
  run "$packet.check" cargo run --quiet --example "$example" -- "$input" --check
  for size in 72x35 80x24; do
    cols=${size%x*}
    rows=${size#*x}
    for offset in 0 65535; do
      run "$packet.$size.$offset" cargo run --quiet --example "$example" -- "$input" --preview "$cols" "$rows" "$offset"
    done
  done
done
