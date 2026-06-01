# Helper/Service Milestone

This document defines the next major architectural milestone for the ChattyFactory rebuild.

The short version:

- the rebuild is now strong at deterministic build lanes
- strong at deterministic patch lanes
- strong at honest fallback and lane growth
- but still relatively weak at bounded helper/service behavior

That is the next gap to close.

Read this alongside:

- [REBUILD_PLAN.md](../plans/REBUILD_PLAN.md)
- [MILESTONE_CHECKPOINT.md](../checkpoints/MILESTONE_CHECKPOINT.md)
- [NEXT_WAVE_OPTIONS.md](../plans/NEXT_WAVE_OPTIONS.md)
- [docs/CONTRACT_INVENTORY.md](./docs/CONTRACT_INVENTORY.md)
- [../ARCHITECTURE_LEDGER.md](../ARCHITECTURE_LEDGER.md)
- [../RELIABILITY.md](../RELIABILITY.md)
- [../PARTS_MAP.md](../PARTS_MAP.md)
- [./CHATTYCOG_HOSTING_MILESTONE.md](./CHATTYCOG_HOSTING_MILESTONE.md)

## 1. Why This Is Next

The rebuild has already proven:

- plain-language request in
- real deterministic build out
- deterministic follow-up patching
- structured planner handoff when confidence is weak
- honest fallback when a lane does not exist
- host-owned scaffolding and promotion of new lanes into live capability

What it has not yet equally proven is this:

- a request that genuinely needs a bounded background helper
- a safe host-owned helper runtime contract
- helper-aware verification
- helper-aware lane growth

That matters because many real tool requests stop being “just files” at exactly this point.

Examples:

- watch a folder and process new files
- keep a small local inbox updated
- run a lightweight summarizer sidecar
- maintain a queue/state bridge for a hosted module
- run a bounded local service that a dashboard or module talks to

If we skip this milestone, the rebuild risks becoming:

- very good at static or batch-style deterministic builds
- less good at bounded live behavior

The reference material keeps pushing us toward solving this honestly rather than papering over it with more prompting.

## 2. Milestone Goal

Add the first real helper/service lane to the rebuild in a way that is:

- deterministic
- bounded
- inspectable
- host-supervised
- acceptance-backed
- compatible with fallback and lane-promotion workflows

The important architectural shift is:

- the model should choose when a helper lane is needed
- the host should define, launch, observe, and verify that helper lane

## 3. What Counts As A Helper/Service Lane

For this milestone, a helper/service lane means a bounded local process or sidecar that:

- has a typed purpose
- has a typed entrypoint
- operates inside declared workspace/runtime boundaries
- emits typed receipts or status files
- can be started and checked by the host
- does not become a freeform “just run arbitrary software” escape hatch

In scope:

- folder-watch helpers
- inbox processors
- local summarizer sidecars
- queue/bridge updaters
- helper-backed module state refreshers

Out of scope for the first pass:

- open-ended daemon ecosystems
- general plugin execution
- network-heavy helper services
- remote deployment
- auto-installed third-party infra

## 4. First Proof Recommendation

The first proof should be small, useful, and obviously real.

Recommended first proof:

- `local_inbox_helper`

Behavior:

- watch a declared folder or helper inbox
- detect newly dropped files
- write deterministic metadata/status outputs
- optionally generate a lightweight summary artifact
- expose those outputs to an existing family

Best host pairing candidates:

1. `chattycog_webview_module`
   - good because it naturally fits bridge/inbox concepts already present in the rebuild

2. `static_web_dashboard`
   - good because it gives a visible UI target for helper state

3. `python_cli_tool`
   - good because it is operationally simple and easy to acceptance-test

My recommendation is:

- prove the helper first against a ChattyCog webview or static web lane

because it best exercises the rebuild’s next missing behavior: a host-supervised live sidecar attached to a visible surface.

## 5. Deliverables

This milestone should produce:

### A. Contracts

Add typed contracts for:

- `HelperServiceSpec`
- `HelperRuntimeReceipt`
- `HelperLaunchPolicy`
- `HelperStatusSnapshot`
- `HelperAcceptancePlan`

At minimum, the helper spec should declare:

- helper id
- family attachment or scope
- purpose
- entrypoint
- working directory
- inputs
- outputs
- status files
- allowed commands or execution mode
- restart expectations

### B. Host Runtime Behavior

The host should be able to:

- recognize that a route needs a helper
- emit helper files/scaffold deterministically
- launch the helper in a bounded way
- verify that it started and wrote expected runtime artifacts
- stop or retire it cleanly
- persist runtime receipts

### C. Acceptance

Helper-aware acceptance should verify:

- files exist where expected
- helper status artifacts were written
- helper outputs match the declared contract
- helper behavior is bounded to the declared target paths

### D. Fallback/Growth Support

If a helper lane is requested but unsupported, fallback should now be able to scaffold:

- helper lane specs
- helper acceptance targets
- helper templates
- promotion-ready work bundles

### E. One Real Proof Lane

At least one lane should go all the way through:

- unsupported request
- fallback helper scaffold
- integrated starter files
- pending lane lifecycle
- host wiring
- fully live helper-backed capability

## 6. Proposed Contract Shape

The exact Rust shapes can evolve, but the host should eventually know something like:

```text
HelperServiceSpec
- helper_id
- helper_kind
- attached_family_id
- attached_tool_kind
- launch_command
- launch_args
- working_directory
- input_paths
- output_paths
- status_paths
- execution_policy
- expected_files
- restart_policy
- notes
```

And:

```text
HelperRuntimeReceipt
- helper_id
- request_id
- launched_at
- launch_status
- pid_or_handle
- observed_status_files
- observed_output_files
- failure_notes
```

This should remain host-owned and inspectable.

## 7. Safety Requirements

This milestone must preserve the rebuild’s current honesty and host control.

### Hard rules

- helper launch must stay confined to declared directories
- helper entrypoints must be explicit, not inferred from arbitrary user text
- helper commands must be policy-checked before execution
- helper runtime receipts must be persisted
- acceptance must fail closed if helper outputs are missing or malformed

### Important non-goal

We are not building a generic “run whatever background thing the model wants” mechanism.

We are building deterministic helper lanes with machine contracts.

## 8. Routing Implications

Once this milestone lands, routing should start recognizing helper-needing asks more explicitly.

Examples:

- “watch a folder”
- “keep an inbox updated”
- “process dropped files”
- “run a background summarizer”
- “monitor a directory and refresh the dashboard”

That does not mean the host should suddenly accept every one of those requests.

It means the route layer should be able to say:

- supported helper lane available
- helper lane required but unsupported
- helper lane incompatible with requested family

## 9. ChattyCog Implications

This milestone should line up directly with the ChattyCog bridge work already done.

Likely alignment points:

- `bridge/incoming_assets/<lane>/`
- helper-managed status snapshots
- helper-fed room or inbox refresh surfaces
- module-side bridge panels that reflect helper output

Good outcome:

- ChattyCog-hosted modules can gain bounded live-ish helper behavior
- without ChattyCog becoming the helper runtime brain

That matches the reference docs well:

- modules remain standalone first
- ChattyCog hosts and bridges them
- helpers stay explicit and bounded

## 10. Suggested Implementation Order

### Step 1

Add the docs and contract stubs:

- helper milestone doc
- contract inventory additions
- tentative helper spec/receipt structs

### Step 2

Add host policy and runtime receipts:

- helper launch policy
- helper runtime receipt persistence
- helper status check helpers

### Step 3

Add one deterministic helper scaffold:

- likely `local_inbox_helper`

### Step 4

Attach it to one family:

- preferred: `chattycog_webview_module` or `static_web_dashboard`

### Step 5

Add helper-aware acceptance

### Step 6

Add fallback scaffolding and extension lifecycle support for helper lanes

### Step 7

Prove one lane end to end and record that checkpoint

## 11. What Success Looks Like

By the end of this milestone, we should be able to point at one plain-language request and say:

- the host understood that this needed a helper
- the host generated the helper deterministically
- the host launched it safely
- the host verified its outputs
- the final project visibly used that helper behavior

That is the proof we want, not “a background process happened to start.”

## 12. What To Avoid

Avoid these traps:

- adding a giant general-purpose service framework first
- making helper behavior planner-only with weak contracts
- letting helper execution bypass route/policy/receipt layers
- burying helper state in invisible temp files
- treating helper lanes as special one-off hacks instead of first-class deterministic lanes

## 13. Recommendation

The best next move after this doc is:

1. add helper/service contracts to the rebuild
2. choose the first helper proof lane
3. wire helper receipts and policy
4. prove one helper-backed family end to end

If we want the cleanest possible first proof, I recommend:

- `local_inbox_helper` attached to `chattycog_webview_module`

because it naturally reuses the bridge/inbox work we have already earned, while pushing the rebuild into the next real complexity class.
