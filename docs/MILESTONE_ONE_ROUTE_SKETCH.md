# Milestone One Route Sketch

This document sketches the first supported route graph for the rebuild.

Milestone one is intentionally narrow:
- one standalone web dashboard family
- one ChattyCog starter dashboard family
- deterministic route receipts
- deterministic acceptance
- minimal LLM role limited to ambiguity and QC

## Milestone-One Goal

Input:
- plain-language request for a basic dashboard or starter module

Output:
- deterministic scaffolded build
- optional ChattyCog wrapper layer
- project contract
- acceptance plan
- route receipt

## Supported Routes

### Route A: `static_web_dashboard`

Best for:
- browser dashboard asks
- local dashboard/tool asks that do not require native helper capability

High-level flow:
1. normalize request
2. classify as `new_build`
3. choose `static_web_dashboard`
4. resolve scaffold inputs
5. render templates
6. emit contract
7. build acceptance plan
8. run acceptance
9. stop success or emit failure report

### Route B: `chattycog_basic_dashboard`

Best for:
- ChattyCog starter module asks
- dashboard module starter asks

High-level flow:
1. normalize request
2. classify as `new_build`
3. detect module/wrapper intent
4. choose `chattycog_basic_dashboard`
5. resolve scaffold inputs
6. render dashboard templates
7. render wrapper templates
8. emit contract
9. build acceptance plan
10. run acceptance
11. stop success or emit failure report

## LLM Touch Points

Milestone one should allow LLM involvement only at these points:
- request ambiguity review
- route arbitration when both family routes look plausible
- final quality review when acceptance passes but user-facing fit is still uncertain

Everything else should be machine-owned.

## Control Nodes

Suggested first node set:
- `normalize_request`
- `classify_mode`
- `detect_wrapper_intent`
- `choose_family`
- `resolve_scaffold_inputs`
- `render_scaffold`
- `render_wrapper`
- `emit_project_spec`
- `build_acceptance_plan`
- `run_acceptance`
- `classify_failure`
- `stop_success`
- `stop_fail`

## Control Edges

Suggested edge reasons:
- `mode_is_new_build`
- `wrapper_requested`
- `dashboard_fit`
- `family_supported`
- `acceptance_passed`
- `acceptance_failed`
- `needs_llm_review`

## Failure Handling In Milestone One

Early failure outcomes:
- missing scaffold output
- invalid wrapper metadata
- missing expected markers
- schema validation failure
- route mismatch

Early repair choices:
- rerender scaffold
- rerender wrapper metadata
- rebuild acceptance plan
- escalate to LLM review if route was likely wrong

## What Milestone One Deliberately Excludes

- helper-backed families
- non-web families
- freeform broad planning
- AST-based source editing
- complex patch mode

That exclusion is healthy.
The point of milestone one is to prove the control/family/contract architecture on a narrow, high-confidence slice.

## What Milestone One Must Not Accidentally Do

- smuggle broad prototype behavior back in
- hide route choice in prompt-only logic
- treat ChattyCog wrapper emission as the whole product
- make the LLM write routine scaffold files

## Exit Criteria

Milestone one is successful when:
- the host can choose between the two web/dashboard families deterministically
- outputs are rendered from templates
- wrapper metadata is machine-generated
- route decisions are persisted as native data
- acceptance is deterministic
- failure reports are typed

If those conditions hold, the architecture is ready to expand outward into CLI and later helper-backed families.
