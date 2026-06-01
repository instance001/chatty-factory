# Patch Diagnosis X-Ray Milestone

This milestone defines the next reliability layer for ChattyFactory patching:

- before patching an existing project
- produce a host-owned diagnostic X-ray of the project
- freeze the patch plan against that diagnosis
- execute the patch against known structure and known invariants
- validate the result against the same diagnosis afterward

## Why This Milestone Exists

We are now largely past the phase of asking:

- can ChattyFactory build something real?
- can ChattyFactory patch something real?

Within its supported envelope, the answer is increasingly yes.

The next architectural pressure is different:

- can it patch safely?
- can it patch predictably against existing code?
- can it avoid collateral damage?
- can it explain where and why it will operate before it edits anything?

That means the next serious host-owned layer is not another patch lane count increase.
It is a diagnosis layer before surgery.

## Problem Statement

Right now the patch pipeline has:

- request interpretation
- deterministic or bounded patch selection
- governance and acceptance visibility

What it does not yet have strongly enough is a pre-patch structural model of the target project that says:

- what the project looks like
- where the relevant insertion points are
- which files and surfaces matter
- which invariants must remain true
- which areas are risky
- what should be re-checked after the patch

Without that, even a correct patch lane can still be too locally informed.

## Design Goal

Add a host-owned project diagnosis artifact that becomes part of the patch flow.

The intent is:

1. understand the request
2. diagnose the project
3. bind the patch plan to that diagnosis
4. patch the project
5. validate against both acceptance and diagnosis-derived invariants

This should feel like:

- X-ray before surgery
- surgical plan before incision
- post-op check against pre-op structure

## Core New Artifact

Recommended primary artifact:

- `ProjectPatchDiagnosis`

Alternative acceptable names if the implementation reveals a better fit:

- `PatchDiagnosisReceipt`
- `ProjectXrayReceipt`
- `PatchSurgeryPlan`

The important part is the role, not the exact name.

## What The Diagnosis Should Capture

At minimum, the diagnosis artifact should record:

- target project name
- family id
- tool kind
- substrate
- request summary
- relevant files for this patch
- detected entrypoints
- likely insertion points
- risky files or risky regions
- local conventions or patterns detected
- inferred invariants to preserve
- pre-patch artifact hashes or summaries
- expected post-patch recheck surfaces

Examples of useful invariants:

- keep exported symbol set stable
- preserve existing helper bundle contract
- do not break known route/bootstrap file
- do not duplicate an already-present surface
- maintain existing lane wiring pattern

## New Patch Flow

The intended patch flow becomes:

1. interpret user request
2. select deterministic or bounded patch route
3. produce `ProjectPatchDiagnosis`
4. produce or refine a `PatchIntentFreeze`
   - the freeze should now reference diagnosis findings
5. execute patch
6. run compile / acceptance / governance validation
7. run diagnosis-aware post-patch validation
8. persist receipts

This does not replace acceptance.

It complements acceptance by making the patch structurally aware before and after execution.

## Relationship To Existing Mechanisms

### Request Interpretation Freeze

The existing freeze says:

- what the user meant

The diagnosis should say:

- what the project is
- where the change belongs
- what must remain true

Both are needed.

### Governance

Governance currently tracks:

- drift
- baseline
- last-known-good
- lifecycle state

Diagnosis should strengthen governance later by providing:

- richer pre/post structural comparison
- patch-specific “what changed where” context

### Acceptance

Acceptance should remain the main behavior check.

Diagnosis adds:

- structural safety
- insertion-point sanity
- local invariant preservation

## First Implementation Slice

The first slice should stay deliberately bounded.

Recommended scope:

- patch flow only
- existing generated projects only
- one or two strong families first
  - `static_web_dashboard`
  - `chattycog_webview_module`

Recommended first patch proof:

- a patch against an existing generated project that inserts a new surface into a known UI area
- the diagnosis should identify:
  - target files
  - target insertion region
  - invariants to preserve

Good early proof shapes:

- add a metadata row
- add a summary panel
- add a status surface near an existing summary area

## Suggested Receipt Shape

The first receipt should likely include sections like:

- `request_summary`
- `project_surface_summary`
- `candidate_target_files`
- `candidate_insertion_points`
- `preserve_invariants`
- `risk_notes`
- `post_patch_checks`

If useful, it can also persist:

- compact content previews
- file role classification
- local ownership summaries

## Suggested Host Responsibilities

The host should own:

- diagnosis generation
- structural file discovery
- invariant derivation
- diagnosis receipt persistence
- post-patch diagnosis-aware verification

The model may still help rank or review bounded choices later, but the host should own the machinery wherever possible.

## What This Milestone Is Not

This is not:

- arbitrary whole-repo static analysis
- a full language server
- free-form patching of any codebase
- model-only “I read the files and hope”

It is a bounded, deterministic reliability layer for the projects ChattyFactory already owns or generated.

## Success Criteria

This milestone is successful when:

1. a patch request against an existing generated project emits a diagnosis receipt before editing
2. the patch execution references diagnosis-selected target surfaces
3. the post-patch validation checks diagnosis-derived invariants
4. receipts clearly show:
   - what was diagnosed
   - what was intended
   - what changed
   - what was preserved

## Best Next Step After This Doc

The first implementation step should be:

1. add the new diagnosis contract
2. persist diagnosis receipts under `runtime/`
3. thread diagnosis into one existing deterministic patch path
4. prove it on a real patch against an existing generated project

That is the smallest honest slice that moves ChattyFactory from:

- patching something real

to:

- patching something real with pre-op structure awareness
