#!/usr/bin/env bash
set -u
root=/tmp/hud-fidelity-run1-rS7MUp/author
cd /tmp/hud-fidelity-run1-rS7MUp/checkout || exit 1
export CARGO_NET_OFFLINE=true
out=$root/evidence/final
mkdir -p "$root/final" "$out"
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
run a.copy cp "$root/revisions/r2/a.compare.json" "$root/final/a.compare.json"
run b.copy cp "$root/drafts/b.sequence.json" "$root/final/b.sequence.json"
run c.copy cp "$root/revisions/r2/c.compare.json" "$root/final/c.compare.json"
run d.copy cp "$root/drafts/d.sequence.json" "$root/final/d.sequence.json"
run e.copy cp "$root/drafts/e.prose.md" "$root/final/e.prose.md"
for pair in a:compare b:sequence c:compare d:sequence; do
  packet=${pair%:*}
  example=${pair#*:}
  input=$root/final/$packet.$example.json
  run "$packet.check" cargo run --quiet --example "$example" -- "$input" --check
  for size in 72x35 80x24; do
    cols=${size%x*}
    rows=${size#*x}
    case "$packet:$size" in
      a:72x35) offsets='0 20 30 65535';;
      a:80x24) offsets='0 15 25 35 65535';;
      c:72x35) offsets='0 20 35 45 65535';;
      c:80x24) offsets='0 15 25 35 45 50 65535';;
      *) offsets='0 10 65535';;
    esac
    for offset in $offsets; do
      run "$packet.$size.$offset" cargo run --quiet --example "$example" -- "$input" --preview "$cols" "$rows" "$offset"
    done
  done
done
run file-hashes sha256sum "$root"/final/* "$root"/drafts/* "$root"/revisions/r1/* "$root"/revisions/r2/*
