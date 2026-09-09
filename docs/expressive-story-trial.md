# Expressive story: demonstration and evidence

**Subsequent Product Owner feedback and decision:** intent orientation and timeline
scanability were praised; busy wrapped prose remains unresolved. The agreed next
move is the [semantic language contract](semantic-visual-language.md), beginning
with Sequence, not palette expansion or connector removal. The trial below is
preserved historical evidence; its original open questions are not the current
work queue. See the [next-session handoff](sequence-next-session.md).

## Bounded recommendation at the time of the trial

Continue exploring **semantic emphasis plus a data-only author/inspect loop**.
This probe makes a journey's departure or arrival genuinely primary and replaces
TOML + shell readers + payload files with one JSON document. It is an example,
not an approved HUD API. Keep its syntax disposable until a different kind of
story challenges it. Do not infer a need for a general component tree or renderer
rewrite. Higher-resolution graphics remain untested, not ruled out.

Product tradeoff: full-width flow makes sequential guidance more legible in a
small terminal, but does not preserve the simultaneous relationships of a canvas.
Decoration also costs space: the unchanged candidate has 26 body lines at 80x24;
the compact/plain treatment has 23. This is **not a color-only A/B test**.

The Product Owner evaluates demonstrated usefulness. No merge, publication,
installation, release, or architectural approval is implied by this experiment.
See the [pre-implementation contract](expressive-story-probe.md) and
[experimental authoring entrypoint](../skills/hud/expressive-probe.md).

## Demonstrate

Run from the checkout root, one command per authorized terminal:

```sh
# Two independently authored purposes; arrival includes the review correction below.
cargo run --quiet --example expressive_story -- examples/coach-departure.json
cargo run --quiet --example expressive_story -- examples/coach-arrival.json

# Compare unchanged baseline wording, not editorial improvements:
cargo run --quiet -- --config examples/coach-baseline.toml
cargo run --quiet --example expressive_story -- examples/coach-story.json
cargo run --quiet --example expressive_story -- examples/coach-story.json --plain
```

Example keys: j/k or arrows scroll, PgUp/PgDn page, Home/End jump, q/Esc/Ctrl-C
exit. Quit/relaunch after edits; no live reload or automatic publication. Production
HUD retains its existing card navigation. These are different experiences, not
interchangeable protocols.

## Controlled wording comparison

The three original text files from `/tmp/hud-route-trial-coach-journey/` were copied
to `examples/coach-baseline/` without edits. The portable TOML changes file readers
only. `coach-story.json` reorganizes the same lead, numbered route, and supporting
sentences. An automated equality assertion checks whitespace-normalized wording,
not byte-for-byte JSON equivalence. The persistent fictional provenance is an
additional label; the old `The chosen journey` heading is replaced by the existing
`Coach is the chosen route.` sentence. No factual content is newly verified.

Actual rendered cells, dimensions excluding terminal chrome:

| Treatment | 93x35 | 80x24 |
| --- | --- | --- |
| Original cards | [cells](assets/expressive-story/baseline-93x35.txt) | [cells](assets/expressive-story/baseline-80x24.txt) |
| Decorated flow | [cells](assets/expressive-story/candidate-93x35.txt) | [cells](assets/expressive-story/candidate-80x24.txt) |
| Compact/plain flow | [cells](assets/expressive-story/plain-93x35.txt) | [cells](assets/expressive-story/plain-80x24.txt) |

At 80x24 the cards clip the lead before the full arrival, fare, transfers, and
walking facts, and clip the route. Existing detail navigation can recover them;
we do not claim inaccessibility. The flowing candidate exposes the entire journey
in its opening view. Supporting notes still need scrolling, signaled by MORE below;
[the end view](assets/expressive-story/candidate-end-80x24.txt) retains provenance
and shows the uncertainty. At 93x35, the candidate fits completely.

The candidate has a real contrasting/bold headline and styled labels/connectors;
a buffer-style test checks this. Text captures do not establish perceived color,
contrast, or human comprehension. Route continuation lines return to the body
margin rather than a hanging indent: an unresolved readability limitation.

Reproduce cells:

```sh
cargo run --quiet --example story_preview -- journey 80 24
cargo run --quiet --example expressive_story -- examples/coach-story.json --preview 80 24
cargo run --quiet --example expressive_story -- examples/coach-story.json --preview 80 24 65535
cargo run --quiet --example expressive_story -- examples/coach-story.json --plain --preview 80 24
# Repeat at 93 35. The fixed baseline helper never executes its fixture commands.
```

## Fresh-author trial, before reviewer coaching

A fresh pi agent (`hud-story-author`, created Herdr pane `w7:p7`) received the
experimental entrypoint and an audience goal: help a traveler make the chosen
coach journey. Supplied fictional facts: one day, local times, 09:05 Cedar Quay bay
C departure, 10:15 Northbridge Museum east-gate arrival, 14 USD, zero transfers,
no additional walk to the entrance. Live clock, delays, seats, accessibility,
ticket validity, boarding deadline, and service number were explicitly unavailable.
No prescribed layout or schema coaching beyond the entrypoint. Linked examples
were accessible; this is not example-free invention. No other-agent consultation
or renderer/repository changes were allowed. Headless inspection was sufficient;
the coordinator, not this author, managed live demo panes.

Frozen during both authoring rounds:

- `skills/hud/expressive-probe.md` SHA-256:
  `377eefcc29263486fcf090bb41a162fe98e1966c6962f9a595ce4930658cffbb`
- `examples/expressive_story.rs` SHA-256 at trial:
  `7972b1e22b86ab0cae6f698939d229bc556c038eda6f4069904f021f48269144`
  A post-trial regression test was added; the non-test implementation is unchanged.

Round one managed one new JSON file, used `--check` and `--preview 93 35` / `80 24`,
then shortened its final step and moved no-additional-walk into support to remove
an orphan wrap. Final [JSON](assets/expressive-story/author-round-one/story.json),
[80x24](assets/expressive-story/author-round-one/80x24.txt), and
[93x35](assets/expressive-story/author-round-one/93x35.txt) are preserved.

The only semantic follow-up was: **“The traveler now wants reassurance about
arrival, not departure-led instructions.”** Operational constraints reiterated
preservation, the same facts/entrypoint, and inspection at the same sizes.
The author created a second file, led with arrival and no-walk reassurance,
retained departure as journey context, then shortened a wrapped disclaimer.
Frozen pre-review [JSON](assets/expressive-story/author-round-two/story.json),
[80x24](assets/expressive-story/author-round-two/80x24.txt), and
[93x35](assets/expressive-story/author-round-two/93x35.txt) remain unchanged.
Coordinator `cmp` confirmed round one unchanged after round two.

Both rounds reported initial and final validation success. Their visible trace
showed checks, previews, and one refinement per round; no validation repairs or
engineering interventions were observed/reported. The coordinator independently
read final JSON and rerendered both sizes. Intermediate drafts/full session history
are not archived, so this is not an exhaustive operation count or independence
proof. Do not turn one successful author into a reliability or speed benchmark.

**Effort evidence:** one file per purpose, with validation/inspection against that
same file, rather than a config plus three separate payloads and shell readers.
Repeated Cargo invocations remain. There is no matched baseline timing or error-rate
measurement, so claim lower plumbing and a viable loop—not proven overall speed.

## Independent review and factual correction

Dottie scoped the probe in consultation
`b1e9b488-7c48-4d61-9b0d-248fb3b4d241` and reviewed the pasted non-test runtime source,
JSON, and 80x24 evidence in `a39c2fe1-d96b-4cac-8a86-90bffe44f940`.
She gave qualified acceptance of the experiment, **not Product Owner approval**.
She did not access the local branch, execute tests/TUI, inspect the hashes, or
review 93x35 captures. Tests, skill contents and author trace were described rather
than independently retrieved in that review. The initial overlarge review request
was locally rejected before dispatch; the shortened request was submitted once.

Her material finding: round two retained the core journey facts but dropped the
explicit **one-day** scope. Static/fictional/not-live does not mean the same thing.
The coordinator copied the trial artifact to `examples/coach-arrival.json` and
changed just its second lead line to:

> Times are local; one fictional day only; not live.

The uncoached artifact stays frozen. The review-corrected
[80x24](assets/expressive-story/reviewed-arrival-80x24.txt) and
[93x35](assets/expressive-story/reviewed-arrival-93x35.txt) are separate evidence.
`coach-departure.json` is the unchanged final round-one file. A post-trial test checks
both demonstration files for visible facts/limits at 80x24, 93x35, and 71x35, and
checks the corrected arrival against the preserved trial content. This fix has
local automated verification; no second independent approval is claimed.

Dottie also noted the compulsory lead → route → support order is a narrow guided
story template, not demonstrated broad composition. Much of the benefit survives
in plain flow. These are limitations to carry forward, not hidden reasons to expand
the experiment now.

## Live evidence and engineering status

Both baseline and unchanged candidate ran in Herdr at 144x35 content cells (35x144
confirmed by `stty size`). A temporary 144x22 view exercised j/k from MORE at line 1
to END at line 5 and back, preserving the header/provenance:
[top](assets/expressive-story/live-scroll-top.txt),
[end](assets/expressive-story/live-scroll-end.txt). Quit returned to the shell and
allowed subsequent launches. Herdr rejected Home/End key names during this probe;
those transitions are unit-tested but **not live-verified**. No actual terminal
image or reader-comprehension test is claimed.

The final live demo places departure and reviewed arrival in two 72x35 views below
the caller, with focus preserved in the caller. Captures:
[departure](assets/expressive-story/live-departure.txt),
[arrival](assets/expressive-story/live-arrival.txt),
[layout](assets/expressive-story/live-layout.json). At this narrower size some notes
wrap but both views reach END with all facts/limits visible. Pane IDs are historical
handles, not instructions to reuse occupied panes in later sessions.

- `./scripts/check`: passes **60 tests**, formatting and Clippy. Eight tests were
  added for the new example; original examples remain green.
- Early test failures were assertions expecting unbroken sentences across actual
  wrapped lines. Assertions now normalize whitespace for those semantic checks;
  no renderer behavior was changed to hide a failure.
- `./scripts/audit`: unavailable because cargo-audit is not installed; no local
  advisory pass or CI result claimed. No dependency versions/lockfile changed.
- No production `src/` changes, interval/document WIP integration, global installs,
  remote writes, or publication. Example reads are byte-limited, not time-bounded
  for arbitrary filesystem objects; use regular local files, not devices/FIFOs.
  `--check` validates data shape, not truth, accessibility, or communicative quality.

## Product-facing acceptance question

Does the change from departure-first guidance to arrival reassurance feel like a
usefully different message? Is the decorated treatment worth its space over compact
flow? The demo is ready for that judgment; more engineering would not answer it.
