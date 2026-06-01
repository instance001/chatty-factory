# Primitive Catalog

This document defines the canonical reusable primitive vocabulary for bounded adaptive composition.

The design intent is:
- the host executes concrete machinery
- the GGUF reviews bounded plans beside the belt
- families adapt primitive classes into substrate-specific implementations

So we keep two layers:
- concrete primitive ids for exact execution
- canonical primitive classes/kinds for cross-family reasoning

## Helper Primitive Kinds

These are the reusable helper-side capability kinds currently recognized by the rebuild:

- `inbox_lane`
  - a bounded helper input lane
- `processed_output`
  - deterministic emitted helper output files
- `summary_emitter`
  - compact helper summary artifact emission
- `status_reporter`
  - compact helper status artifact emission
- `file_filter`
  - helper-side file acceptance/filter policy

Current examples:
- `module_assets_inbox_lane` -> `inbox_lane`
- `secondary_assets_inbox_lane` -> `inbox_lane`
- `local_inbox_processed_output` -> `processed_output`
- `local_inbox_summary_snapshot` -> `summary_emitter`
- `local_inbox_status_snapshot` -> `status_reporter`
- `module_assets_file_type_filter` -> `file_filter`

## Patch Primitive Classes

These are the reusable patch-side classes currently recognized by the rebuild:

- `summary_surface`
  - panel or surface that summarizes runtime/project/helper state
- `status_chip`
  - compact status indicator surface
- `info_chip`
  - compact informational chip such as lane count, types, or updated-at
- `metadata_row`
  - compact metadata summary row
- `notice_surface`
  - explanatory notice or empty-state surface
- `filter_rule`
  - patch that changes filtering/selection rules
- `inbox_lane_extension`
  - patch that adds or extends an inbox lane
- `selection_control`
  - patch that adds a selector/toggle style control
- `refresh_behavior`
  - patch that adds refresh or polling behavior
- `timestamp_row`
  - patch that shows recency/last-run time
- `badge_strip`
  - patch that adds badge-style summary cues
- `export_output`
  - patch that adds an export or outbound output artifact
- `patch_extension`
  - fallback class for known deterministic lanes that do not yet map to a narrower class

Current examples:
- `progress_banner` -> `summary_surface`
- `helper_summary_panel` -> `summary_surface`
- `processed_files_panel` -> `summary_surface`
- `helper_summary_status_chip` -> `status_chip`
- `helper_summary_lane_count_chip` -> `info_chip`
- `helper_summary_types_chip` -> `info_chip`
- `helper_summary_metadata_row` -> `metadata_row`
- `lane_scoped_metadata_row` -> `metadata_row`
- `helper_summary_filter_notice` -> `notice_surface`
- `lane_scoped_filter_notice` -> `notice_surface`
- `file_type_filter` -> `filter_rule`
- `secondary_inbox_lane` -> `inbox_lane_extension`
- `processed_file_selection` -> `selection_control`
- `auto_refresh_helper_panels` -> `refresh_behavior`
- `helper_last_run_stamp` -> `timestamp_row`
- `helper_summary_badges` -> `badge_strip`
- `json_output` -> `export_output`
- `email_sender` -> `export_output`

## Ownership Rules

- Families own concrete deterministic primitive ids and handlers.
- The host owns composition execution, reconciliation, and receipts.
- The planner may review primitive classes/kinds and concrete ids, but it does not invent new ones during bounded composition.
- New families should prefer mapping their concrete lanes into existing primitive classes before inventing new class names.
- New class names should be added sparingly and only when multiple families can benefit from them.

## Current Source Of Truth

Canonical primitive vocabulary currently lives in:
- [primitive_catalog.rs](C:/Users/User/Desktop/chattyfactory-module/chatty-factory/crates/chatty_factory_core/src/primitive_catalog.rs)

Family-declared patch primitive classes currently live in:
- [registry.rs](C:/Users/User/Desktop/chattyfactory-module/chatty-factory/crates/chatty_factory_families/src/registry.rs)

The intended split is:
- core owns the canonical class/kind names and fallback mappings
- families declare which concrete lanes provide which patch primitive classes
- the host uses family declarations first and falls back to the core catalog only for older or unannotated lanes

Current composition receipts surface both layers:
- concrete ids
- canonical classes/kinds

That is the intended long-term shape for cross-family composition reasoning.
