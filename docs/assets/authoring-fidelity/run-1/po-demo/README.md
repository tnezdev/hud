# Reading-collection demo — subsequent PO feedback

Travis asked the primary to drive the live case. This demonstration happened
after the frozen author trial and independent review; those raw artifacts are
unchanged. It used the actual author final, not a reviewer-corrected demo copy.

## Setup and observed navigation

- Original prose: `../fact-packets/c.txt`, displayed with `fold -s -w 67` piped
  into `less -+F -+X`. Actual PTY 72 columns x 35 rows. All text fit, `(END)`.
- Compare: `target/debug/examples/compare` on `../author/final/c.compare.json`
  (paths above are relative to this directory; the real command used repo-root
  paths). Actual PTY 71 columns x 35 rows. No dimension override or content edit.
- Initial Compare view: line 1/70, with Willow's quoted ready date at the bottom
  and its qualification below the cut. PO was given time for the opening view.
- After Travis said “I'm ready,” primary sent one terminal PageDown. Compare
  moved to line 21/70: both readiness qualifications fully visible, then outbound
  delivery and most return carriage. Willow's final return qualification still
  continued below the footer. Primary paused for feedback. Renewal was not yet
  shown live; no complete human walkthrough is claimed.
- Herdr rejected `pagedown` and `page-down` aliases without UI changes. The
  successful key input was the standard terminal CSI 6~ sequence through
  `herdr pane send-text w7:pJ $'\033[6~'`.

Different composition and one-column PTY difference mean this is not a controlled
identical-wording/geometry A/B. Captures are terminal cells, not pixel screenshots
or measured comprehension. The source prose has only whitespace wrapping changed.

## PO statement and decision boundary

> The grouping is definitely easier to follow for me. The scrolling is not much
> of a detriment IMO. Well worth the tradeoff of being easier to scan and parse
> the information.

This accepts the demonstrated grouping/scrolling tradeoff for Travis on this
case. Preserve the visuals; don't treat scrolling alone as permission to redesign
or shorten qualifications. The feedback is consistent with the earlier supplier
visual-parsing benefit and now explicitly accepts the observed scrolling cost.

It is not a claim of improved decisions, measured comprehension, every group
being inspected by the PO, broad author reliability, a production API, merge,
release, installation, further publication or another idiom. Full-content
factual/capture review is separate evidence in `../review/report.md`.

## Capture and process state

Files copied byte-for-byte from `/tmp/hud-fidelity-demo-e9WtzH/`:
`prose-live.txt`, `compare-opening-live.txt`, `compare-readiness-live.txt`,
`prose-size.txt`, `compare-size.txt`, `layout.json`.

Last verified layout: caller `w7:p1` above, still focused with its cwd unchanged;
owned `w7:pH` lower-left source pager; owned `w7:pJ` lower-right live Compare.
These demo processes were left open for the PO, not cleaned up or restarted.
Rediscover live topology/processes before any subsequent control. No unrelated
pane was touched. Later reading of this record does not prove those handles
remain current.

No renderer, author data, discovery, dependency, source or historical evidence
was modified. This is a docs/capture-only follow-up. `check-po-feedback.log` and
`check-po-feedback.exit` preserve the subsequent `./scripts/check` result.
