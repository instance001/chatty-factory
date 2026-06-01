# Generalized Primitive Proof Harness Milestone

## Why This Is Next

The rebuild now proves something important:
- bounded composition can execute as host work
- primitive-native execution is real
- cross-family helper monitoring can be compared for equivalent capability fulfillment
- paired proof orchestration can be run and preserved as first-class factory work

That is a major shift.

But the strongest proof is still one capability cluster:
- helper-backed monitoring

If we stop here, the system risks becoming:
- very good at one proof shape
- with a polished shell around it

instead of becoming:
- a reusable proof harness for multiple primitive-native capability bundles

## Goal

Turn the current paired monitoring proof into a reusable host-owned proof harness.

That means the host should be able to run:
- a proof template
- against one or more family targets
- using declared primitive and capability bundles
- and persist comparable proof results

without hand-specializing every new proof type as a one-off.

## What This Milestone Should Add

### 1. Proof Template Contract

Add a host-readable proof template shape, something like:
- proof template id
- proof template kind
- shared request seed
- target families
- required primitive layers
- required primitive classes
- required helper primitive kinds
- required capability classes
- optional enrichment steps

This should let the host describe proofs in capability-native terms, not only in raw request strings.

### 2. Capability Comparison Bundle Contract

Add a reusable comparison bundle shape so equivalence is not only:
- helper monitoring specific

The bundle should define things like:
- required shared capability classes
- optional capability classes
- minimum equivalence threshold
- tolerated family-specific deltas

That makes proof comparison more formal and reusable.

### 3. Multi-Cluster Proof Support

The first new proof after helper monitoring should come from a different cluster.

Good candidates:
- helper-backed reporting
- CLI vs dashboard summary equivalence
- bridge/status/report equivalence

The key point is:
- same primitive-native capability bundle
- different substrate adapters
- host-owned comparison

### 4. Proof Harness Receipts

The host should persist proof harness receipts that clearly show:
- template used
- families targeted
- primitive bundle requested
- capability bundle required
- receipts produced
- final equivalence outcome

That should sit above individual paired-proof receipts as a more general orchestration record.

## First Recommended Proof Beyond Monitoring

Recommended next cluster:

### Summary / Reporting Equivalence

Example shape:
- one request for a local helper-backed summary/report capability
- one CLI/report family
- one dashboard family

The proof should compare capability classes like:
- summary generation
- filtered input handling
- status output
- report/export availability

This would move the rebuild beyond:
- monitoring-only equivalence

and toward:
- reusable capability proofing

## Suggested Implementation Order

1. Define proof template and comparison bundle contracts
2. Extract the existing helper-monitoring paired proof into a proof template
3. Add a generalized host proof runner
4. Keep the current monitoring proof as template `proof_helper_monitoring`
5. Add one second template from another capability cluster
6. Prove that both templates can run through the same host harness

## What Not To Do

- Do not hardcode each new proof as its own one-off orchestration command
- Do not collapse proof templates back into family-specific prompt text
- Do not make proof equivalence a file-by-file diff system
- Do not expand the UI further before the host proof harness generalizes

## Success Criteria

This milestone is successful when:
- proof execution is template-driven, not proof-name driven
- capability comparison is bundle-driven, not one-off monitoring logic
- at least two different capability clusters can run through the same proof harness
- receipts clearly show primitive and capability equivalence outcomes

## Why This Matters

This is the next step from:
- strong paired proof

to:
- reusable architectural proof system

That is what will let the rebuild keep moving toward the original intent:
- host-owned conveyor belt
- GGUF beside the belt
- primitive-native execution
- capability equivalence across different substrates
- proof work that scales beyond one successful demo
