# Project Patch Readiness Governance Milestone

## Goal

Take the new patch-lane readiness model:
- `ready`
- `already_present`
- `dependency_blocked`
- `structurally_blocked`
- `surface_mismatch`

and promote it from a per-view runtime calculation into a host-owned per-project governance artifact.

This is the next layer after patch diagnosis and freeze:
- diagnosis explains one surgery
- readiness explains one lane against one project
- project patch readiness governance explains the patchability posture of the whole project over time

## Why This Is Next

We now have:
- declarative patch-lane structural guards
- diagnosis receipts
- freeze receipts
- postcheck receipts
- lane maturity and contract-confidence signals
- browser triage for blocked lanes

What we do not yet have is project-level memory of patch safety drift.

Without that, ChattyFactory can say:
- "this lane is blocked right now"

but not yet:
- "this project has gotten riskier to patch since its last healthy state"
- "this project has improved patchability over time"
- "this project has accumulated legacy-shape-sensitive blockers"

## First Slice

Add a host-owned receipt per generated project, for example under:
- `runtime/project_patch_readiness_receipts/<project_name>.json`

The first receipt should carry:
- project identity
- family/tool identity
- `ProjectSpec.json` path and hash
- patch-lane readiness counts
- patch-lane surgical maturity mix
- blocked-lane reasons
- a stable readiness signature hash
- change-vs-baseline classification

And a bulk refresh status artifact:
- `runtime/project_patch_readiness_refresh_status.json`

## Initial Baseline Model

For the first slice, compare current project patchability against the previously recorded baseline and classify:
- `baseline_recorded`
- `stable_since_patchability_baseline`
- `changed_since_patchability_baseline`
- `improved_since_patchability_baseline`
- `regressed_since_patchability_baseline`

This does not need to be perfect yet.

The important threshold is:
- ChattyFactory starts remembering project patchability posture over time
- and that memory is host-owned rather than UI-local

## Host Integration

Add:
- bulk refresh command
- receipt persistence
- refresh on project-browser refresh
- refresh after patch execution and patch freeze skip

That gives the model a low-friction update path before we decide whether every build path should also stamp it immediately.

## UI Follow-On

Once the receipt layer is real, the project browser can grow from:
- lane-level blocked triage

to:
- project-level badges like:
  - `patch-ready`
  - `patch-drifted`
  - `patch-regressed`
  - `patch-improved`

## First Proof Targets

- `build_me_a_tau_helper_backed`
- `build_me_a_local_operations_dashboard`
- `build_me_a_python_csv_report`

Those already exercise:
- ready lanes
- already-present lanes
- structurally blocked lanes
- richer-family surface mismatch risk

## Architectural Intention

This is not a separate patch-execution mechanism.

It is a project-level governance layer that sits above:
- patch lane contracts
- diagnosis
- freeze
- postcheck

and answers:
- how patchable is this project right now
- how has that changed over time
- where should an operator expect surgery to be safe or risky
