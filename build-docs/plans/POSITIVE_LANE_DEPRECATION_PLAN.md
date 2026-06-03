# Positive Lane Deprecation Plan

This plan makes the current architecture pivot explicit:

- the older positive build lanes proved that ChattyFactory could build, patch,
  diagnose, and govern real outputs
- they are no longer the long-term product center
- template-first skeleton builds plus the negative constraint shelf are now the
  main route to broader reliability

The point is not to delete useful history blindly.
The point is to stop treating old lanes as sacred and migrate deliberately.

## Why This Plan Exists

The older positive-lane strategy gave us:

- deterministic build routes
- bounded patch routes
- x-ray / diagnosis / freeze / postcheck loops
- project patchability governance
- real proof that the factory can produce and update artifacts

But if we keep scaling through more and more narrow positive lanes, we get:

- lane sprawl
- more routing heuristics than product truth
- harder maintenance
- more risk that old convenience paths override better substrate choices

The new posture is:

- let the LLM interpret and plan broad intent
- let the host own boilerplate, templates, acceptance, and verification
- let the negative bookshelf reject known bad implementation methods
- start from stronger skeletons instead of endlessly adding more narrow lanes

## Current Family Inventory

Current built-in families:

- `static_web_dashboard`
- `chattycog_webview_module`
- `chattycog_native_window_module`
- `chattycog_workspace_module`
- `python_cli_tool`
- `rust_cli_tool`

These are not all equal going forward.
Some should remain as useful substrates.
Some should be treated as transitional proving grounds.

## Decision Buckets

### Keep For Now

Keep temporarily where a family still provides one of these:

- a strong regression fixture
- a practical deterministic baseline
- a useful acceptance substrate while a better skeleton path is still young

Current likely temporary keeps:

- `rust_cli_tool`
- `python_cli_tool`
- `static_web_dashboard`

Reason:
- they are still useful as baseline deterministic families and still teach the
  verification/governance layers something real

### Freeze Now

Freeze means:

- no new major capability growth
- no “just one more lane” expansion
- only maintenance, migration, or bug fixes

Current freeze candidates:

- `chattycog_webview_module`
- `chattycog_workspace_module`
- any further broad family growth that exists mainly to avoid creating better
  skeletons

Reason:
- they were valuable proving grounds, but they should not keep pulling the
  architecture back toward family proliferation

### Prefer As Primary Skeleton Paths

These are the kinds of outputs that should lead new work:

- standalone skeletons with optional ecosystem plug files
- ecosystem-native skeletons whose contract files are explicit and removable
- template-first families whose acceptance and host contracts are easy to verify

Current primary forward target:

- `chattycog_native_window_module`

Reason:
- it matches the real ChattyCog direction
- it is standalone by nature
- it can carry removable compatibility files instead of being ecosystem-only

### Remove First

Remove only after replacement paths are proven, but these are the first classes
of behavior to retire:

- routing heuristics that force requests into old families because of historical
  convenience
- ecosystem compatibility paths that rely on wrapper layering instead of
  skeleton-first contracts
- dead helper routing that exists only to preserve old family logic

## Migration Waves

### Wave 1: Demote Architecturally

- mark old positive lanes as transitional in docs
- stop growing them
- stop making new ecosystem work depend on them
- prefer skeleton-first work for new platform-facing outputs

Success condition:
- no new roadmap work depends on expanding family count as the main solution

### Wave 2: Prove Replacement Skeletons

- prove the standalone native Rust dashboard + ChattyCog plug-file shell
- verify emitted file contracts
- verify acceptance and self-test
- verify module-host compatibility surfaces

Success condition:
- at least one skeleton-first path is stronger than the historical family path
  it replaces

### Wave 3: Cut Old Routing Heuristics

- remove or narrow heuristics that still silently push work into deprecated
  family shapes
- prefer substrate truth over historical success-rate shortcuts

Success condition:
- request routing matches current architecture intent, not past convenience

### Wave 4: Prune Dead Code

- remove family-specific logic that survives only by inertia
- remove obsolete tests that enforce deprecated routing
- keep only fixtures that still teach the negative shelf something useful

Success condition:
- smaller family/routing surface without loss of confidence

## Practical Keep / Freeze / Remove Read

### Keep Temporarily

- `rust_cli_tool`
- `python_cli_tool`
- `static_web_dashboard`

### Freeze Immediately

- `chattycog_webview_module`
- `chattycog_workspace_module`
- family growth that is really compensating for weak skeletons

### Prefer Going Forward

- `chattycog_native_window_module`
- future standalone skeletons with optional ecosystem plug files

### Remove Later

- heuristic routes that only exist to preserve historical family choices
- any family logic that becomes redundant once skeleton-first builds are proven

## Guardrails Before Removal

Before removing any legacy lane or route, confirm:

- a newer skeleton-first path exists
- that replacement builds successfully
- it has acceptance coverage
- docs point to the replacement
- any useful historical learning is preserved in tests, fixtures, or the
  negative shelf

## Current Recommendation

Do not do a mass deletion pass yet.

Do this next:

1. keep proving the `chattycog_native_window_module` skeleton path
2. treat older positive families as frozen unless maintenance truly needs them
3. start removing the most misleading old routing assumptions first
4. only then prune family code in waves

That gives us a cleaner transition:

- less architecture thrash
- less dead-weight accumulation
- less chance that old family heuristics quietly fight the new model
