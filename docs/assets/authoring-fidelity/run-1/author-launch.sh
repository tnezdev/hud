#!/usr/bin/env bash
# Recorded invocation for this one run; do not rerun over preserved evidence.
set -u
cd /tmp/hud-fidelity-run1-rS7MUp/checkout || exit 1
env -u PI_SESSION_FILE -u PI_SESSION_ID -u HERDR_ENV \
  PI_OFFLINE=1 CARGO_NET_OFFLINE=true \
  pi --mode json -p --provider openai-codex --model gpt-6-astra --thinking high \
  --no-context-files --no-extensions --no-skills --no-prompt-templates \
  --no-themes --no-approve --tools read,bash,write,edit \
  --session-dir /tmp/hud-fidelity-run1-rS7MUp/sessions \
  --name hud-fidelity-author-1 \
  @/tmp/hud-fidelity-run1-rS7MUp/prompt.md \
  > /tmp/hud-fidelity-run1-rS7MUp/author-events.jsonl \
  2> /tmp/hud-fidelity-run1-rS7MUp/author-stderr.log
result=$?
printf '%s\n' "$result" > /tmp/hud-fidelity-run1-rS7MUp/author.exit
exit "$result"
