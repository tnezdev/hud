# Semantic visual language: current product direction

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
resolve together, not settled Product Owner requirements.

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
