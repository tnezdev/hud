#!/usr/bin/env bash
set -u
cd /tmp/hud-fidelity-run1-rS7MUp/checkout || exit 1
export CARGO_NET_OFFLINE=true
out=/tmp/hud-fidelity-run1-rS7MUp/author/evidence/middle
mkdir -p "$out"
printf 'cwd=%s\nCARGO_NET_OFFLINE=%s\n' "$PWD" "$CARGO_NET_OFFLINE" > "$out/environment.txt"
for spec in a:72:35:20 a:72:35:30 a:80:24:15 a:80:24:25 a:80:24:35 c:72:35:20 c:72:35:35 c:72:35:45 c:80:24:15 c:80:24:25 c:80:24:35 c:80:24:45; do
  IFS=: read -r packet cols rows offset <<< "$spec"
  name=$packet.${cols}x${rows}.$offset
  cmd=(cargo run --quiet --example compare -- /tmp/hud-fidelity-run1-rS7MUp/author/revisions/r1/$packet.compare.json --preview "$cols" "$rows" "$offset")
  printf '%q ' "${cmd[@]}" > "$out/$name.command"
  printf '\n' >> "$out/$name.command"
  "${cmd[@]}" > "$out/$name.stdout" 2> "$out/$name.stderr"
  status=$?
  printf '%s\n' "$status" > "$out/$name.exit"
  printf '%s exit=%s\n' "$name" "$status"
done
