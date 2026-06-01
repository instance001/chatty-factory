# Proof Harness Manifests

This folder is the repo-extensible catalog layer for proof templates and capability comparison bundles.

Layout:
- `templates/`: `PrimitiveProofTemplate` JSON manifests
- `bundles/`: `CapabilityComparisonBundle` JSON manifests

Behavior:
- built-in proof templates and bundles remain available from `crates/chatty_factory_core/src/proof_harness.rs`
- manifests here are loaded on top of the built-ins
- a manifest with the same id overrides the built-in definition for host and UI catalog consumers

Extension lifecycle:
- `cargo run -p chatty_factory_cli -- register-proof-harness-bundle <template_manifest_json> <comparison_bundle_manifest_json>` registers a manifest pair into `operator_registry/pending_lanes.json` as a `proof_harness_bundle`
- that pending entry then moves through the same lifecycle as other deterministic machinery:
  - `implement-extension`
  - `validate-extension`
  - `prepare-extension-promotion`
  - `prepare-extension-apply-patch`
  - `consume-extension-apply-patch`
  - `validate-live-extension`
- for proof harness bundles, `host_wired` means the workspace proof catalog can resolve the template and bundle by id
- `fully_live` means the manifests still validate, resolve through the catalog, and pass compile-safe live validation

This is the next step toward making proof machinery repo-extensible instead of code-catalog-only.
