# Architecture Checkpoint 23

## What Changed

The selective curation pass is now materially complete.

After the Rust CLI modernization wave and the ChattyCog supersession work:

- the Rust legacy-sensitive cluster is gone
- the remaining legacy-sensitive lanes are intentionally historical
- their modern replacement paths are explicit in lane metadata and UI surfaces

The live remaining set is effectively:

- `helper_summary_badges`
- `helper_status_chip`

and both are now documented as superseded rather than “still waiting to be fixed.”

## Why This Matters

This closes an important architectural loop.

The system is no longer treating legacy-sensitive status as one undifferentiated bucket.

We now have three honest outcomes for old lanes:

1. modernize and keep

Example:

- Rust `file_output`
- Rust `severity_filter`
- Rust `json_output`

2. block safely and point to the replacement

Example:

- `helper_summary_badges`
- `helper_status_chip`

3. leave broad or narrow lanes alone if they are already honest

Example:

- most modern ChattyCog helper-summary support lanes
- Python CLI report lanes

That is a healthier product posture than endlessly widening guard metadata.

## Architectural Read

The patch catalog has crossed from:

- contract hardening

into:

- capability curation

That means the next reliability gains are less about hunting for stray handler crashes and more about deliberately deciding:

- which old capabilities still deserve a modern lane
- which capabilities are better represented by newer surfaces

## Best Next Move

The next strongest move is not another general sweep.

It is to apply this same curation standard when new legacy-sensitive cases appear:

- modernize them if the capability still matters
- supersede them if the newer surface already tells the better story

So from here, the best next work is probably not more patch-catalog cleanup by default, but either:

- deeper diagnosis/readiness governance, or
- the next product capability wave that benefits from the same contract discipline.
