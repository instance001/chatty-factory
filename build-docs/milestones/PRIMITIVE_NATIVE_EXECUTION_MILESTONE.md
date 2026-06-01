# Primitive-Native Execution Milestone

## Why This Is Next

The rebuild now has a strong composition story on paper and in receipts:
- direct deterministic lanes
- bounded composed builds
- bounded composed patches
- helper primitive ids and helper primitive kinds
- patch primitive classes
- family build primitive classes
- mixed `composition_bundle` lifecycle
- GGUF bounded review beside the belt

But execution still bottoms out mostly in:
- chosen family
- chosen patch kinds
- helper machinery inferred from the active family/project

That means the host can now describe composition in substrate-agnostic primitive terms better than it can execute it in those same terms.

This milestone closes that gap.

## Goal

Make bounded composition execution more primitive-native and less patch-kind-native.

That means:
- the host should build a primitive work order first
- families should declare how they satisfy primitive classes/kinds
- the host should execute the mapped primitive plan
- the GGUF should review primitive-level work orders, not mostly family-specific patch bundles

## Success Criteria

At the end of this milestone, ChattyFactory should be able to:

1. build a host-owned primitive work order from:
   - family build primitive classes
   - patch primitive classes
   - helper primitive kinds
   - composition layers

2. map that work order through family adapters instead of directly from patch kinds alone

3. persist an execution ledger that shows:
   - requested primitive intent
   - family adapter chosen per primitive
   - final primitive execution order
   - acceptance coverage per primitive group

4. prove one cross-family request that uses the same high-level primitive intent on two different families

## Architectural Direction

The design target is:
- host owns execution machinery
- families expose adapter metadata
- GGUF reviews bounded primitive work orders
- receipts stay host-owned

This should move the rebuild away from:
- "patch bundle first, primitives after"

and toward:
- "primitive work order first, family adapter mapping second"

## What To Add

### 1. Primitive Adapter Contract

Add a host-facing adapter declaration layer for families.

Each family should be able to say:
- which family build primitive classes it provides directly
- which patch primitive classes it can satisfy
- which helper primitive kinds it can host
- which combinations are native, optional, or unsupported

This should be declarative metadata first, not prompt logic.

### 2. Primitive Work Order Contract

Add a first-class primitive execution record, likely something like:
- `PrimitiveExecutionPlan`
- `PrimitiveExecutionStep`

At minimum it should record:
- work-order kind
- composition layers
- family target
- primitive classes/kinds requested
- adapter chosen per primitive
- execution order
- acceptance coverage notes

### 3. Primitive-Native Host Execution

Teach the host to:
- build the primitive work order
- reconcile GGUF-reviewed primitive choices
- choose family adapters
- execute the mapped work order
- persist step receipts against primitive steps, not only patch steps

### 4. Primitive-Aware Review Receipt

Extend bounded review so the host can journal:
- candidate primitive work order
- reviewed primitive work order
- final reconciled primitive work order
- adapter mapping summary

The GGUF should still not invent machinery.
It should only review and refine within host-exposed primitive options.

## First Proof Target

Use one high-level primitive intent across two families.

Recommended proof shape:
- "surface local inbox progress and status"

Run it across:
1. `static_web_dashboard`
2. `chattycog_webview_module`

The high-level primitive intent should stay the same:
- `summary_surface`
- `status_chip`
- `inbox_lane`
- `summary_emitter`

But the host should map that intent through different family adapters.

## Suggested Implementation Order

1. Add primitive adapter metadata for at least:
   - `static_web_dashboard`
   - `chattycog_webview_module`

2. Add primitive execution plan structs and persistence

3. Add host adapter-selection helpers

4. Execute one primitive-native composed build

5. Execute one primitive-native composed patch

6. Only after that, consider extending the same machinery to CLI families

## What Not To Do

Do not solve this milestone by:
- adding more micro patch lanes first
- hiding adapter choice inside prompt text
- letting the GGUF synthesize freeform execution logic
- collapsing primitive-native execution back into "pick patch kinds and hope"

That would weaken the architectural shift we just earned.

## Expected Outcome

If this milestone lands well, the rebuild will move one step closer to the original intent:
- the conveyor belt stays host-owned
- the GGUF stays beside the belt
- the host executes bounded primitive work orders
- family-specific differences become adapter mappings, not the whole planning language

That is the path from:
- reliable deterministic lane factory

to:
- host-owned adaptive factory with bounded, inspectable model supervision
