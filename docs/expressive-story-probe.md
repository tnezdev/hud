# Expressive story probe (preserved experiment, not a production contract)

**Status after Product Owner feedback:** intent orientation and timeline scanability
received positive feedback; busy wrapped prose remains a problem. Preserve the
timeline, including connectors. The next work is the [semantic language contract](semantic-visual-language.md),
not a calmer-treatment detour or palette expansion. The scope below is historical;
its required lead/route/support JSON is not the upcoming Sequence contract.

Implementation, fresh-author results, independent review, and demonstration:
[trial and evidence](expressive-story-trial.md).

## Question and bounded proposal

Can a stronger terminal-cell hierarchy and a simpler author/inspect loop help an
agent communicate intentionally? This follows the accepted journey demonstration,
not an approval of a new schema. Dottie's consultation
`b1e9b488-7c48-4d61-9b0d-248fb3b4d241` recommended separating visual and authoring
evidence and testing predictable flow against simultaneous card context.

Build an **example-only**, disposable composition: lead, ordered guidance, notes.
Use existing ratatui text, color, emphasis, connectors, and space—not pixel images,
a general component tree, coordinates, or a production renderer rewrite. The
tradeoff is predictable reading order and full-width wrapping versus side-by-side
relationships. Higher resolution remains unexplored, not rejected.

## Contract before implementation

- `cargo run --example expressive_story -- FILE.json [--plain]` opens a local
  read-only TUI. `--preview WIDTH HEIGHT [OFFSET]` emits rendered cells without a
  terminal; `--check` validates without rendering. These are example arguments,
  **not hud CLI flags**. Both views use the same typed data and renderer.
- One JSON document: title, provenance, lead (label, headline, lines), route
  (label, steps), support (label, lines). All required; no command, file inclusion,
  action, markup, or dynamic interpolation fields. Unknown fields fail closed.
- Parse/validate once at the filesystem edge. Enforce a 64 KiB input cap, bounded
  nonempty collections/strings, and reject control characters and bidi controls.
  Prose including shell syntax remains inert display data. No command-runner,
  shell, network, clock, or persistent-state boundary in this example.
- Lead occupies the full width with contrasting headline; numbered connected
  steps establish order; notes follow a separator. The same content can be viewed
  in plain mode to distinguish presentation from changed facts or wording.
- Content flows vertically and wraps. Scroll with j/k, arrows, PgUp/PgDn,
  Home/End; footer explicitly shows more content. q/Esc/Ctrl-C exits. No refresh:
  revise the file, inspect, then relaunch. Tiny viewports must not panic.
- Use ratatui's wrapped-line measurement behind its experimental feature, enabled
  only as a dev dependency. Do not invent a parallel word-wrapping algorithm.
- Preserve the prior three-panel journey's text verbatim as portable fixtures.
  Candidate may group those same sentences into roles; document any added labels
  or changed ordering. Preserve both baseline and candidate cells at 93x35 and
  80x24 (content cells, excluding Herdr borders). A plain rendering of the same
  candidate is a reference, not a third layout implementation.

## Evidence and stop conditions

1. Check parser errors, unknown executable fields, inert shell-looking prose,
   tiny sizes, wrapping/scrolling, visible fictional provenance, and all journey
   facts across overview + scrolled views using in-memory tests. Run scripts/check.
2. Inspect actual cells and live Herdr behavior. Keep evidence level explicit:
   buffer text does not establish color perception or comprehension.
3. A fresh author uses a frozen experimental entrypoint to compose and make one
   semantic revision without schema coaching or renderer edits. Record files,
   commands, validation/repair attempts and interventions. A single successful
   trial is not general reliability or a controlled authoring-speed comparison.
4. Dottie reviews actual source/evidence with verification limits stated. Product
   Owner sees the experience and decides whether it is useful enough to continue.

No merge, publication, release, global installation, remote access, memory writes,
background refresh, or absorption of the interval/document WIP. No claim that this
example is the chosen production authoring architecture. Stop after the bounded
prototype, review, and demonstration; bring meaningful tradeoffs, not code tasks.
