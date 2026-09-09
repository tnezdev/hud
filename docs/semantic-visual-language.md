# Semantic visual language: current product direction

## Current next slice: Compare (approved)

After accepting the demonstrated Sequence visual reference and its top padding,
Travis explicitly approved **Compare** as the next bounded semantic reference.
This supersedes the completed Sequence slice's no-expansion boundary for this one
idiom only; it does not authorize a component catalogue or production API overhaul.

Establish Compare's meaning, why/when, counterexamples, semantic inputs and
presentation obligations with Dottie before code. Use the supplier alternatives
that correctly did not fit Sequence as an anchor, retaining the prose baseline,
plus genuinely new material. A separate fresh author should choose among Sequence,
Compare and ordinary prose without selection/layout coaching. Evaluate factual
fidelity independently of valid syntax, correct selection and visual polish.

Product checkpoint: **Does Compare make alternatives easier to judge than prose
without losing important facts or forcing an inappropriate visual form?**

Travis remains Product Owner. Substantial work is delegated to fresh agents because
the coordinator session is near compaction; repository-local briefs and evidence
must survive a reset. Preserve accepted Sequence code, fixtures, discovery and
visual treatment. No merge, publication, release, installation, remote-memory
write, or wider palette expansion is authorized by this approval.

The [Compare brief](compare-reference.md) and [trial record](compare-trial.md) now
capture the delegated implementation, uncoached selections, fidelity failures and
post-trial discovery corrections. In the paired supplier/prose demonstration, Travis said the Compare view was
**“way easier to visually parse.”** This confirms a strong visual-parsing benefit
for the demonstrated case. Preserve this treatment as the demonstrated Compare
reference rather than continuing unsolicited visual changes. That feedback does
not establish better decisions, factual fidelity, dependable independent
authorship, a production API, or publication approval; those remain separate.

## Decision and ownership

Following the expressive-story demonstration, Travis approved pivoting toward the
**semantic language contract before expanding the palette**. HUD aims to establish
a visual language an agent can speak to communicate intentionally with people.
A dashboard is one possible expression, not the product boundary. The name HUD
may change later; no rename is currently requested.

Travis remains Product Owner: direction, principles, consequential tradeoffs, and
acceptance of demonstrated usefulness. Agents own routine design, implementation,
checks, and reviewer coordination. Continue collaborating with Dottie for continuity
and independent scrutiny; her advice is not additional implementation or publication
authority. GitHub remains the sole public project home.

This document records the approved direction and bounded next move. It does not
approve a production schema, broad renderer rewrite, or catalogue of new components.
The current executable remains the command-backed dashboard. The existing story
example is preserved experimental evidence, not the new language contract.

## Division of responsibility

> Agent: facts + intent → semantic composition.
>
> HUD: semantic composition + display constraints → visual presentation.

The agent chooses what to communicate, the appropriate visual idiom, emphasis,
relationships, and qualifications. HUD takes responsibility for making that
composition legible: layout, alignment, connectors, typography within the medium,
wrapping, spacing, and adaptation to the available terminal.

The agent can choose **“these are successive steps toward this outcome”** without
specifying character coordinates or drawing boxes and arrows. Its expressive
agency is retained; drawing mechanics are delegated. There is no current mandate
for a separate AI intent resolver or for interpreting arbitrary English inside HUD.
A discoverable structured semantic contract is the working hypothesis; its concrete
syntax still needs design.

Bring established information-design and UX principles to the terminal, not a
website rendered in character cells. Hierarchy, proximity, whitespace, readable line
length, progressive disclosure, and visual relationships all apply. ASCII/Unicode
lines, enclosures, and spatial arrangements can carry meaning rather than merely
decorate text. Higher-resolution graphics remain an open question, not the next
implementation commitment.

## Discovery must teach why and when

The Product Owner emphasized that HUD should describe not just what a component
is or how to invoke it, but **why and when to invoke it**, with examples of the
semantics it can express. Each reference idiom should teach:

- **Meaning:** the relationship it expresses.
- **Why:** what it helps a person perceive or understand.
- **When:** suitable contexts, counterexamples, and reasons not to use it.
- **Contract:** the semantic inputs the agent supplies and the presentation
  responsibilities HUD accepts.
- **Examples:** materially different communicative intentions using the idiom.

Invocation details follow this guidance; discovery is not merely a widget API
reference. Do not advertise geometry and styling knobs as though they were a
semantic language. Likewise, meaningful names alone do not establish dependable
visual behavior.

## Product feedback to preserve

Travis, after viewing departure-first and arrival-reassurance stories:

> “Semantically, I think these are oriented nicely around intents and actions.
> They clearly tell me what to put my attention on. Visually I have trouble
> keeping focus at places where the text wraps and gets a bit busy.”

He then clarified:

> “Place like the Journey timeline are absolutely great. I can quickly scan
> that, and easily get my 1-2-3 notes just in a quick glance”

This is positive acceptance of intent orientation and the timeline's scanability,
not blanket approval of the visual treatment, JSON schema, or architecture.
**Preserve the timeline's numbers, connectors, and reading rhythm.** A preliminary
agent suggestion to remove connector decoration was superseded by this explicit
feedback. No calmer-treatment implementation was made.

The unresolved problem is busy wrapped prose around that successful structure.
Continuation grouping and reading rhythm belong to HUD's presentation work; do not
rely on every author manually shortening prose to compensate. Keep uncertainty and
provenance visible or discoverable without disguising the cost of scrolling.
When comparing visual treatments, hold the story/wording constant. Comparing the
departure and arrival stories demonstrates changed intent, not a controlled visual
comparison.

## Subsequent Sequence feedback: section navigation

After the reference trial and scroll-boundary correction, Travis said:

> “Almost. What looks off to me is what in Web UX world I think we would call
> vertical rythym. My eyes have trouble navigating because they don't know how to
> break-up the sections. I think that's what it is.”

The difficulty locating sections is product feedback; vertical rhythm is a
working diagnosis, not approval of a specific layout. The bounded
[section-rhythm comparison](sequence-rhythm.md) holds story wording and the
connected timeline constant while strengthening section boundaries. Do not shift
that presentation work onto the author or interpret this as palette expansion.

Travis subsequently said “They are much better. But it is still busy at the top,”
with a screenshot highlighting the opening title/metadata/purpose region. This
accepts the improved section boundaries, not the entire presentation. The bounded
[opening-hierarchy refinement](sequence-header.md) keeps the improved body while
separating the title and its qualifications from renderer-owned sequence labeling.

After seeing that refinement, Travis called it “much easier to read” and said it
was looking like what he hoped for. He requested one blank row before the first
line, comparing the effect to web padding or margins. Preserve that hierarchy and
add the single outer top-padding row; do not translate “boxes” into a requirement
for enclosing borders. This is acceptance of the demonstrated reading direction,
not production API or publication approval.

After the single-row top-padding adjustment, Travis said **“That looks great!”**
The demonstrated Sequence presentation is now the accepted visual reference:
clear title/context hierarchy, one outer top-padding row, distinct section spacing,
compact label/detail grouping, and the numbered connected spine. Preserve this
checkpoint rather than continuing unsolicited visual iteration. This accepts the
shown experience, not a production schema, broader palette, merge or publication.

## Agreed next slice: Sequence as a reference idiom

1. Write a small repository-local Sequence brief with Dottie before implementation:
   meaning, why/when, counterexamples, semantic inputs, presentation obligations,
   and examples from different kinds of stories.
2. Use the successful existing timeline as evidence and a starting point, not a
   requirement to retain the prototype's compulsory lead → route → support schema.
   No final Sequence fields or invocation syntax have been approved yet.
3. Freeze the trial entrypoint and test with a fresh author: **two suitable stories
   and one deliberately unsuitable case**, without telling it which should use
   Sequence. Choosing not to use Sequence must be a valid outcome; do not build
   another idiom merely to accommodate the negative case.
4. Demonstrate appropriate selection and composition without layout coaching, plus
   HUD's presentation at narrow terminal sizes. Preserve facts, unknowns, and
   temporal qualifiers. Capture actual render/inspect/refine behavior, errors,
   interventions, and limitations separately from claims of authoring ease.
5. Obtain independent review and run `./scripts/check`. Bring the Product Owner
   the experience and meaningful tradeoffs, not implementation questionnaires.

Proposed acceptance question, agreed as the next checkpoint:

> Does teaching this idiom help an agent communicate well, rather than merely
> invoke a component correctly?

Selection quality, semantic fidelity, and legible presentation are separate
observations. A valid document or one successful invocation is insufficient.
Specific stories, syntax, tests, and rendering changes remain engineering work to
resolve together, not settled Product Owner requirements. The subsequent
[Sequence reference brief](sequence-reference.md) records the bounded engineering
hypothesis and implementation plan; its [entrypoint](../skills/hud/sequence.md)
teaches supplied procedure/chronology selection without committing a production API.

## Scope guardrails and preserved evidence

- No palette expansion, general resolver framework, plugin host, automatic
  publishing, or production API commitment in this next slice.
- Keep display data separate from executable command configuration. Do not create
  a convenience authoring interface that executes story content.
- Keep filesystem, command, terminal, and time boundaries explicit and testable.
  Do not absorb the separate interval/document WIP.
- Preserve prior fixtures, captures, reviews, and trial imperfections; do not
  rewrite history to imply broader acceptance or a flawless fresh-author trial.
- No merge, release, global installation, remote-memory writes, or public
  publication is authorized by this pivot. Local documentation and feature-branch
  preparation are authorized.

Historical experiments:

- [Film-night vocabulary and intent change](agent-canvas.md): experimental Lead,
  Compare, Explain, Direct, Measure over the existing panel API.
- [Flowing-story scope](expressive-story-probe.md) and
  [trial/evidence](expressive-story-trial.md): one-file authoring, prominent lead,
  connected route, supporting notes, fresh-author revision, and review correction.
- `31ab9cf` preserves the entire expressive-story experiment before this
  direction-setting cleanup. Both its runtime and its syntax remain experimental.
