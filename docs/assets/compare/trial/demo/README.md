# Actual Compare/prose demo

This is a small coordinator-owned PTY demo, not a screenshot or acceptance
claim. The same supplier facts were shown in two forms at the same actual PTY
size, 71 columns × 35 rows:

- `compare-suppliers-live-71x35.txt` — the example Compare renderer, captured
  while live in owned pane `w7:pD`; all 28 body lines fit and the footer says
  `END`.
- `supplier-prose-live-71x35.txt` — the retained
  `examples/sequence-suppliers.txt`, displayed with `fold -s -w 67` in the same
  pane after clearing it; the shell prompt remains visible as an honest terminal
  capture.
- `/tmp/hud-compare-demo-size.txt` and
  `/tmp/hud-compare-prose-demo-size.txt` record `35 71` from `stty size`.

Compare makes criterion correspondence and Cedar's qualification easier to scan
at the cost of repeated alternative labels and a renderer-owned landmark. Prose
is shorter in conceptual structure and keeps the supplier narrative together,
but correspondence requires scanning each bullet. This is a coordinator
observation, not a human-comprehension result or Product Owner approval.
