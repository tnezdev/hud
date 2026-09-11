#!/usr/bin/env bash
# Recorded independent review invocation; do not overwrite completed evidence.
set -u
cd /home/tnez/Work/tnezdev/hud || exit 1
env -u PI_SESSION_FILE -u PI_SESSION_ID -u HERDR_ENV \
  PI_OFFLINE=1 CARGO_NET_OFFLINE=true \
  pi --mode json -p --provider openai-codex --model gpt-6-astra --thinking high \
  --no-context-files --no-extensions --no-skills --no-prompt-templates \
  --no-themes --no-approve --tools read,bash,write,edit \
  --session-dir /tmp/hud-fidelity-run1-rS7MUp/review-sessions \
  --name hud-fidelity-review-1 \
  @/home/tnez/Work/tnezdev/hud/docs/assets/authoring-fidelity/run-1/reviewer-prompt.md \
  > /tmp/hud-fidelity-run1-rS7MUp/reviewer-events.jsonl \
  2> /tmp/hud-fidelity-run1-rS7MUp/reviewer-stderr.log
result=$?
printf '%s\n' "$result" > /tmp/hud-fidelity-run1-rS7MUp/reviewer.exit
exit "$result"
