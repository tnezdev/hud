# Primary verification and completed local checkpoint

## Authority and publication

Travis authorized pushing the existing reference checkpoint and running the bounded
fidelity trial, retaining his Product Owner role. `6f5e9c5a124fcb00ce209dcad870085e06af917a`
on `feat/sequence-reference` was pushed to `origin` (GitHub) and reverified with
`git ls-remote --heads origin feat/sequence-reference` after the trial.

The first push failed because HTTPS Git had no credential helper. The successful
retry used the already-authenticated `gh` account via command-local configuration:

```sh
git -c credential.helper= -c 'credential.helper=!gh auth git-credential' \
  push -u origin feat/sequence-reference
```

No credential was printed or stored in repository files; no global Git/auth
configuration was changed. Normal `-u` branch tracking was established. No merge,
PR, release, global install or further publication was performed. The new trial
files and three current-document updates remain local/uncommitted; no staging or
commit was performed. GitHub remains the public home.

## Evidence scrutiny

One fresh non-interactive author and one fresh independent evaluator completed,
both exit 0 with empty process stderr. Exact launches/prompts and losslessly
compressed sessions/events are archived. No Herdr pane control occurred; earlier
retired pane handles were not reused. No author coaching, rerun, coordinator
content/layout repair, Dottie consultation or remote-memory/project write.

The reviewer found all five finals faithful across 49 predeclared obligations,
reproduced four checks and 33 previews byte-for-byte, and independently checked
trace exposure, draft history and actual full-content capture coverage. The
primary read all packets and final artifacts, the independent report, coverage
script, derived complete 80x24 bodies, original C opening/renewal captures, and
author process/repair records. This is not a second independent full renderer
execution by the primary or a human comprehension test.

A/C failures and self-repairs remain preserved, including unsupported categorical
absence statements and C's initially missed renewal tail. The independent final
review does not retroactively make those drafts correct. No corrected demo copy
was needed or authored after submission.

The PO tradeoff is qualification fidelity versus scrolling/inspection cost, not
whether to remove accepted visual grouping. C's 72x35 opening ends before one
answer's qualification; the footer marks continuation and another inspected view
completes it. A complete artifact/capture set is not a self-contained screenshot.

## Gates and invariants

`./scripts/check` passed after documentation/results were written: **80 tests**,
formatting and Clippy. Original output/status: `check-final.log` and
`check-final.exit`. The pretrial gate is separately preserved. No dependency
versions or lockfile changed; no new local advisory or CI pass is claimed.

No runtime behavior changed, so no new code test is appropriate. Existing tests
remain green; the archived author's and reviewer's real validation/render commands
supply automated example-data evidence, while semantic fidelity is explicitly
reviewed rather than asserted by a new keyword-test framework.

Primary `primary-integrity.txt` records:

- frozen-input manifest: 29/29;
- author submission manifest: 412/412;
- review submission manifest: 236/236;
- 372/375 baseline checkpoint files byte-identical; intentional changes are only
  `README.md`, `docs/reference-next-session.md`, `docs/semantic-visual-language.md`;
- all 20 frozen runnable/discovery files equal current sources;
- remote branch still at the published checkpoint above.

Both accepted renderers, actual entrypoints, fixtures, original evidence,
production files and dependencies are unchanged. The reviewer previously verified
375/375 before the intentional current-document updates; these statements refer
to different checkpoints, not a hidden mismatch. Existing root `.gitattributes`
is unchanged. Evidence-local attributes exempt only original command recordings,
collations and derived diff/row outputs' trailing spaces, preserving exact raw
bytes without loosening source-code checks.

Whitespace verification covers tracked changes with `git diff --check`, and each
new file with `git diff --no-index --check /dev/null FILE`, without staging or
changing the actual index. See `primary-whitespace.txt` for the completed result.

## Resume

Start at `docs/authoring-fidelity-trial.md` and the independent `review/report.md`.
All trial work is complete; no active author/reviewer or pending code repair.
Bring the concrete scrolling/inspection cost to Travis. Any next experiment,
visual change, merge or further publication needs its own bounded decision.
