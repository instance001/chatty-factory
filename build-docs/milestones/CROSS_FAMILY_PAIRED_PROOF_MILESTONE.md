# Cross-Family Paired Proof Milestone

## Why This Is Next

The rebuild can already:
- run a helper-backed monitoring proof on more than one family
- compare two finished projects for equivalent capability fulfillment
- auto-emit comparison receipts when a suitable counterpart already exists

That is strong, but it still leaves the most important proof one step short of being first-class host behavior.

The next milestone is to let the host intentionally orchestrate the pair as one proof flow.

## Goal

Run one shared helper-monitoring intent across two families and persist one linked proof bundle that records:
- both build requests
- both family choices
- both primitive execution plans
- the cross-family comparison receipt
- the final equivalent-capability outcome

## Proof Shape

One command should:
1. take one shared helper-monitoring request
2. run the `chattycog_webview_module` build path
3. run the `static_web_dashboard` build path
4. compare the resulting projects
5. persist one paired-proof receipt

## What Must Be Recorded

The paired-proof receipt should capture:
- shared request
- family-specific expanded requests
- left/right project names
- left/right request ids
- left/right family ids
- left/right composable route plan paths
- left/right primitive execution plan paths
- comparison receipt path
- equivalent capability fulfillment result

## Success Criteria

This milestone is successful when:
- one command can orchestrate both family proofs
- both builds still pass their own acceptance
- a comparison receipt is generated automatically
- a paired-proof receipt links the whole run together
- the final result expresses equivalent capability fulfillment, not file identity

## Why This Matters

This is the next clean step toward a real factory proof harness:
- host-owned orchestration
- family-agnostic primitive intent
- receipts that show the whole proof, not isolated parts
- less manual milestone choreography
