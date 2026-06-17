# UI Module Structure

This doc tracks the current intentional module split inside `crates/chatty_factory_ui/src/`.

It is not a full UI architecture manifesto. It is a maintenance map for the current extraction wave, especially around the governance surface.

## Current Files

### `main.rs`

Still owns:

- app state
- task orchestration
- runtime loading and refresh
- extension registry flow
- proof-panel composition
- broader lane composition root behavior
- lane note and lifecycle action orchestration
- activity/history wiring outside the extracted panels

`main.rs` remains the composition root for the UI.

### `governance_ui.rs`

Owns shared governance presentation helpers:

- drift summary wording
- baseline summary wording
- note summary wording
- refresh status wording
- stale / never-refreshed / cooldown wording
- metric-strip rendering
- refresh-state rendering
- governance detail-block rendering

This module should stay lightweight and reusable.

### `catalog_governance_panels.rs`

Owns the catalog-backed governance panels:

- family governance
- template governance

These panels are catalog-like rather than extension-row-like, so they share a different shape from the governed lane detail panel.

### `extension_governance_panel.rs`

Owns the governance-specific subsection of extension detail rendering for:

- proof harness bundles
- composition bundles
- patch recipes
- helper lanes
- bridge lanes
- unresolved-layer summaries

This is the current home for governed lane evidence and governance-specific lane status rendering.

### `extension_workbench_panel.rs`

Owns the lane workbench and artifact-preview subsection for extension detail rendering:

- scaffold / source-spec reveal actions
- integrated / promotion / apply-patch artifact lists
- blocker display
- notes and acceptance-target reveal actions
- source spec / implementation notes / acceptance target previews
- promotion and apply-patch compare views
- first integrated file preview

This is the current home for non-governance lane evidence and preview-heavy workbench rendering.

### `patch_xray_panel.rs`

Owns the patch diagnosis / surgery-evidence surface:

- latest-result patch X-ray rendering
- selected-project recent patch X-ray history
- diagnosis / freeze / postcheck receipt loading
- outcome classification and badges
- blocked-only triage toggle
- first-line blocked / duplicate / applied reason summaries
- receipt reveal actions for patch surgery evidence

This is the current home for pre-op / post-op patch reliability evidence in the UI.

### `proof_run_panel.rs`

Owns the proof-run control surface:

- proof profile selection
- save / duplicate / delete / reset profile actions
- proof template picker
- template and comparison-bundle explainer block
- template / bundle contract reveal actions
- shared request input
- run-proof action
- retry-search ladder proof launch action
- proof history filter controls

This is the current home for proof-launch setup and template-facing proof control flow.
It is also where repeatable hostile ladder proofs are launched when the goal is
to validate receipt-owned retry-search behavior rather than a happy-path proof result.

### `proof_history_panel.rs`

Owns the paired-proof results and history surface:

- latest proof status
- proof notes
- proof export history and diff
- proof timeline
- favorite proofs
- recent paired proofs
- proof history rendering

This is the current home for proof evidence, export, and historical proof inspection.

### `request_action_panel.rs`

Owns the general request/build/patch control surface:

- request input
- planner controls
- build request action
- patch selected project action
- current target project summary

This is the current home for the non-proof request/action work area.

The request/result surface is also where the operator now sees host-owned
fallback guidance such as:

- outcome class
- recommended next action
- recommended next step
- build verification linkage

That presentation is still rooted in `main.rs`, but it is now part of the
intentional receipt-first operator surface rather than incidental status text.

The proof and patch evidence surfaces now follow the same pattern too:

- the proof runtime panel shows retry-search ladder next action and next step
- the patch X-ray panel shows postcheck failure class, next action, and next
  step when present

## Extraction Rule

The current rule is:

- extract by honest subsystem boundary
- do not split only to reduce line count

Good extraction candidates:

- shared governance presentation
- catalog governance panels
- governed lane evidence sections
- proof control surfaces
- proof results/history surfaces
- request/action control surfaces

Bad extraction candidates:

- one-off widget fragments with weak cohesion
- helpers that are still tightly bound to one narrow call site

## Likely Next Splits

The next likely module boundaries are:

1. registry-list row rendering
2. command output / task result surface
3. project browser panel / selection work
4. remaining lane action rows outside the extracted panels

Those should only move once they feel as naturally shared as the governance surface now does.
