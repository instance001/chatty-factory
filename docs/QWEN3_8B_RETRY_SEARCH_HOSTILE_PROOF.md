# Qwen3-8B Retry-Search Hostile Proof

This note captures the repeatable retry-search escalation proof currently
available in ChattyFactory and the fresh rerun performed on 2026-06-17.

## Why This Proof Exists

The purpose of this proof is not to show that Qwen3-8B can solve the task.

The purpose is to verify that the factory:

- can force rejection on the initial model
- can preserve that rejection in host-owned receipts
- can classify full ladder exhaustion honestly
- can treat the receipt, not shell timing alone, as the source of truth

This is a hostile proof in the sense that the host intentionally refuses to
accept the first model candidate even if it returns something parseable, so the
ladder and exhaustion logic are exercised on purpose.

## Repeatable Command

```powershell
cargo run -p chatty_factory_cli -- run-retry-search-model-proof --model Qwen3-8B-abliterated-q8_0.gguf
```

The UI also exposes a repeatable button for the same proof:

- `Run Retry-Search Ladder Proof`

## Host Implementation Shape

The proof runner lives in:

- [crates/chatty_factory_host/src/lib.rs](C:/Users/User/Desktop/github_portal/chatty-factory/crates/chatty_factory_host/src/lib.rs:7594)

It uses two retry postures:

1. `strict_json_contract`
2. `fieldwise_recovery_posture`

Both are asked for one very small bounded output:

- valid minified JSON
- exactly one key: `value`
- one short dashboard-ready phrase

The proof deliberately forces rejection on the initial model candidate to verify
ladder escalation behavior.

## Fresh Rerun On 2026-06-17

Fresh proof receipt:

- [retry-search-proof-1781654071233-0.json](C:/Users/User/Desktop/github_portal/chatty-factory/runtime/retry_search_proofs/retry-search-proof-1781654071233-0.json)

Fresh generation receipts:

- [retry-search-proof-1-strict_json_contract-generation.json](C:/Users/User/Desktop/github_portal/chatty-factory/runtime/model_task_generation_receipts/retry-search-proof-1-strict_json_contract-generation.json)
- [retry-search-proof-1-fieldwise_recovery_posture-generation.json](C:/Users/User/Desktop/github_portal/chatty-factory/runtime/model_task_generation_receipts/retry-search-proof-1-fieldwise_recovery_posture-generation.json)

Observed outcome:

- requested model selector: `Qwen3-8B-abliterated-q8_0.gguf`
- model candidate count: `1`
- retry posture count: `2`
- final outcome: `full_model_ladder_exhausted`
- forced initial rejection: `true`
- method space exhausted: `true`
- internal timeout observed: `false`

## What Happened

For both hostile proof methods, the generation receipt showed:

- `final_outcome = request_completed`
- `finish_reason = length`
- `response_content_mode = reasoning_fallback`
- `content_present = false`
- `reasoning_content_present = true`

That means the model responded at the transport/runtime level, but did not
return usable contract-following content for the bounded proof task.

The factory correctly treated that as:

- not a pass
- not an internal timeout
- not a shell-timeout artifact
- but full ladder exhaustion for the available candidate set

## Why This Matters

This proof validates several design requirements:

1. the host owns truth about pass/fail
2. retry-search is receipt-owned, not shell-owned
3. exhaustion is a first-class outcome, not an embarrassing edge case
4. model failure becomes evidence rather than a reason to disguise fallback

This is directly aligned with:

- [Factory Shape](./FACTORY_SHAPE.md)
- [Bounded Soft-Review Continuation](./BOUNDED_SOFT_REVIEW_CONTINUATION.md)
- [Current Architecture](./CURRENT_ARCHITECTURE.md)

## Practical Reading

This proof should be read as:

- "Qwen3-8B exhausted the current hostile proof ladder under the available
  method set"

not as:

- "the factory failed"
- "shell timeout killed the run"
- "the model was unsupported forever"

The useful artifact is the host-owned classification:

- one model candidate
- two hostile retry postures
- no usable contract output
- ladder exhausted honestly

## Next Use

This proof is now suitable as:

- a repeatable regression check for receipt-owned ladder behavior
- a reference example when discussing hostile bounded microtests
- evidence for future docs or UI notes about reasoning fallback versus usable
  structured output
