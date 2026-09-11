#!/usr/bin/env bash
set -u
cd /tmp/hud-fidelity-run1-rS7MUp/checkout || exit 1
export CARGO_NET_OFFLINE=true
out=/tmp/hud-fidelity-run1-rS7MUp/author/evidence/r1
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
for packet in a c; do
  input=/tmp/hud-fidelity-run1-rS7MUp/author/revisions/r1/$packet.compare.json
  run "$packet.check" cargo run --quiet --example compare -- "$input" --check
  for size in 72x35 80x24; do
    cols=${size%x*}
    rows=${size#*x}
    for offset in 0 65535; do
      run "$packet.$size.$offset" cargo run --quiet --example compare -- "$input" --preview "$cols" "$rows" "$offset"
    done
  done
done
for packet in b d; do
  input=/tmp/hud-fidelity-run1-rS7MUp/author/drafts/$packet.sequence.json
  for size in 72x35 80x24; do
    run "$packet.$size.10" cargo run --quiet --example sequence -- "$input" --preview "${size%x*}" "${size#*x}" 10
  done
done
