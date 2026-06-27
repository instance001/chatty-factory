# Factory Shape

This document preserves the actual shape ChattyFactory is working toward.

Use it as a decision filter before adding routing rules, planner logic, retry
behavior, or safety checks.

## What We Are Not Building

ChattyFactory is not:

- a template picker
- a nearest-family selector
- a "pick supported shape and fill blanks" generator
- an unconstrained agent where the model owns truth, execution, and excuses

## What We Are Building

ChattyFactory is:

> a receipt-owned, evidence-driven, host-governed factory where the model
> provides bounded method and the host preserves truth

The working equation is:

- the user carries the intent
- the LLM carries the method
- the host carries the funnel
- the output carries the artifact

## Host-Owned Truth

The host must own:

- frozen intent
- bounded execution posture
- file and tool boundaries
- verification
- receipts
- failure classification
- next-attempt posture
- patch truth

The model may propose:

- implementation strategy
- task decomposition ideas
- code or content changes
- likely toolchain usage inside a bounded lane

The model does not own truth.

## Gauntlet Rule

Fallback is not host selection.

Fallback is the gauntlet:

1. attempt under frozen intent
2. fail honestly
3. classify the failure
4. change the next attempt in a specific evidence-driven way
5. decompose or triangulate when needed
6. alter toolchain or model only when justified

The host should never smuggle product-shape authority back in by saying:

- this looks close to family X
- this template is near enough
- this easier scaffold counts as success

## Safety Rule

Safety does not come from forcing every request back into a familiar positive
shape.

Safety comes from:

- bounded attempts
- receipts
- permission boundaries
- verification
- honest requirement checks
- evidence-driven next-attempt changes

## Failure Rule

Failure is fuel, not shame.

The host traps evidence so the next attempt can differ for concrete reasons.

That means failures should become:

- decomposition evidence
- triangulation evidence
- retry evidence
- constraint-promotion evidence
- explicit next-attempt posture

They should not become excuses for reintroducing host-owned positive-lane truth.

## Short Mantra

- the host preserves truth
- the model provides method
- fallback is a gauntlet, not a substitution
- failure becomes evidence
- receipts matter more than optimism
