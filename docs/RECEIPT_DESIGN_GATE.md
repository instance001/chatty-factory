# Receipt Design Gate

Use this checklist before adding a new receipt family or expanding an existing
one.

The goal is to stop new receipts from quietly growing:

- `outcome_class`
- `normalized_failure_class`
- `recommended_next_action`
- `recommended_next_step`

without a clear ownership reason.

## Step 1. Decide What Kind Of Surface This Is

Pick one first:

1. operator-facing end-state surface
2. governed evidence or routing surface
3. narrow artifact/log/catalog surface
4. watchlist surface that may become routing later

If you cannot classify it, stop and clarify the receipt’s real job before
adding fields.

## Step 2. Apply The Field Rule

If it is an operator-facing end-state surface:

- allow `outcome_class`
- allow `recommended_next_action`
- allow `recommended_next_step`
- do not add `normalized_failure_class` unless the surface also acts as direct
  governed evidence

If it is a governed evidence or routing surface:

- allow `normalized_failure_class`
- allow `recommended_next_action`
- allow `recommended_next_step`
- do not add `outcome_class`

If it is a narrow artifact/log/catalog surface:

- do not add any of the four fields by default

If it is a watchlist surface:

- keep it narrow now
- only add `recommended_next_*` if the host will resume directly from it
- do not add `outcome_class`

## Step 3. Answer These Three Questions

Write down the answers in the PR, task note, or commit message:

1. Does this receipt answer "what happened", "what failed and why", or "what
   should happen next"?
2. Is this surface something an operator sees as an end result, or something
   the host uses as evidence?
3. Will later orchestration resume directly from this receipt?

If the answers are fuzzy, the receipt probably should stay narrower.

## Step 4. Check Existing Buckets

Use [Receipt Field Ownership Audit](./RECEIPT_FIELD_OWNERSHIP_AUDIT.md) to see
whether the family already belongs in:

- keep both outcome and continuation
- keep continuation only
- keep narrow
- watchlist

If your change would move a receipt to a different bucket, update the audit in
the same change.

## Step 5. Only Broaden On Real Need

A receipt should gain extra ownership fields only when one of these becomes
true:

- the host now surfaces it as an operator-facing end result
- the host now routes directly from it
- the host now classifies failures from it generically instead of only storing
  prose

If none of those are true, keep the receipt narrow.

## Anti-Patterns

Do not add both outcome and continuation fields just because:

- the receipt feels important
- the UI currently displays it
- another nearby receipt has them
- it might be useful later

Do not add `outcome_class` to:

- emitted artifact receipts
- execution logs
- runtime summaries
- governance reference summaries

unless they have explicitly become operator-facing end-state surfaces.

## Current Default

When in doubt:

- end-state summaries get outcome plus continuation
- evidence receipts get continuation without outcome
- artifact/log receipts stay narrow
