# Remaining Legacy-Sensitive Patch Lanes Review

## Why This Review Exists

The recent contract-widening and selective-modernization waves materially improved patch reliability:

- many ChattyCog helper-summary lanes are now `narrow_surface_contract`
- the old Rust CLI legacy lanes were rewritten onto the composed renderer path
- project patchability governance can now measure a cleaner lane landscape

That means the remaining `legacy_shape_sensitive` set is now small enough to inspect directly and intentionally.

## Current Remaining Legacy-Sensitive Shape

The live remaining set is effectively:

### ChattyCog helper-summary historical lanes

- `helper_summary_badges`
- `helper_status_chip`

These still depend on older helper-summary insertion shapes and are correctly blocked when newer evolved helper-summary chip surfaces already exist.

Those blocks are now explicit product guidance, not just structural accidents:

- `helper_summary_badges` is superseded by:
  - `helper_summary_count_delta`
  - `helper_summary_lane_count_chip`
  - `helper_summary_types_chip`
- `helper_status_chip` is superseded by:
  - `helper_summary_status_chip`

### Rust CLI legacy lanes

This cluster is now closed.

The old Rust log-summary legacy lanes were selectively modernized onto the shared composed-main renderer, so:

- `file_output`
- `severity_filter`
- `json_output`

now behave as `narrow_surface_contract` lanes instead of legacy-sensitive ones.

## Recommended Classification

### Keep as legacy-sensitive historical variants

- `helper_summary_badges`
- `helper_status_chip`

Reason:

- each one encodes an earlier helper-summary shape
- the project can now evolve beyond that shape
- newer lanes already express the underlying user outcome better
- safe blocking plus named replacements is better than forcing the old insertion pattern

### Already in a good place

- most ChattyCog panel and helper-summary support lanes
- Python CSV report lanes
- Rust log-summary lanes after selective modernization

These are already narrow enough, or safe enough, that they do not need immediate architectural attention.

## Best Next Decision

The best next move remains selective curation, not more blanket coverage changes.

Two healthy options:

1. leave a remaining legacy-sensitive lane as a historical blocked variant

This is appropriate if:

- newer lanes already cover the same user outcome
- or the old lane shape no longer matches the modern project architecture

2. create a modern replacement lane

This is appropriate if:

- the user-facing capability is still valuable
- but the implementation should now target the newer evolved surfaces directly

## Recommendation

Do not keep widening contracts blindly.

Use the remaining legacy-sensitive set as a design queue:

- decide which capabilities still matter
- then either:
  - modernize them into current project shapes
  - or explicitly accept them as intentionally blocked historical shapes

The repo is now much closer to that curated state:

- Rust legacy-sensitive lanes were modernized and kept alive
- the remaining ChattyCog pair is intentionally historical and explicitly superseded

That is a better use of the current architecture than chasing “zero legacy-sensitive lanes” as a number.
