# Primary Product Owner comparison

The primary coordinator retrieved the delegated implementation/trial report,
reviewed the trial and discovery, and requested two explicitly post-trial fixes:
clarify that the run sheet was numbered Markdown rather than a HUD Sequence
invocation, and document the complete Compare shape/limits plus the Sequence
entrypoint link. Raw author evidence and frozen source/discovery copies remain
separate; this is not a rerun or product acceptance.

Final visible comparison uses the preserved supplier facts:

- Left `w7:pC`: `examples/sequence-suppliers.txt` printed with `fold -s -w 67` in
  a cleared shell. The shell prompt remains visible. This is static prose, not
  an interactive HUD view.
- Right `w7:pD`: `cargo run --quiet --example compare -- examples/compare-suppliers.json`.
  j/k and paging are available here; q quits the example.
- Both actual PTYs measured71columns x35rows. Left is temporarily set to71columns
  although its physical width at setup is72; restore current physical dimensions
  during cleanup, not historical values after a resize.
- Caller `w7:p1` retained focus/cwd. Trial lead `w7:pG` was exited, shell-verified,
  and closed after report retrieval. No author/builder/reviewer worker remains.

`prose-71x35.txt`, `compare-71x35.txt`, size files and `layout.json` capture this
actual final state. The Compare capture matches the delegated live capture at
that size. All supplier facts/qualifications are preserved in the demonstration
fixture; it is the builder's checked anchor, not the flawed fresh-author supplier
artifact. This compares semantic recomposition of the same facts, not an
identical-wording color-only A/B. Cells/shell output are not pixel screenshots,
reader comprehension, or Product Owner acceptance.

The demonstration is ready for judgment: does grouping by common criteria make
the choices easier to judge than prose? The cost is repeated labels, questions,
state wording and vertical space. Fresh-author selection looked appropriate, but
fidelity still required review; see `docs/compare-trial.md` for omissions and limits.
