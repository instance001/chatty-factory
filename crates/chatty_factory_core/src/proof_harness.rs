use crate::contracts::{
    CapabilityComparisonBundle, CapabilityComparisonPolicy, PrimitiveProofEnrichmentBinding,
    PrimitiveProofExecutionRecipe, PrimitiveProofSubstrateRequestBinding, PrimitiveProofTemplate,
};
use crate::ids::SubstrateKind;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn built_in_proof_templates() -> Vec<PrimitiveProofTemplate> {
    vec![
        PrimitiveProofTemplate {
            template_id: "proof_summary_reporting".into(),
            template_kind: "cross_surface_summary_reporting".into(),
            display_label: "Summary Reporting".into(),
            description: "Checks whether reporting-oriented surfaces can fulfill the same helper-backed summary, status, and report-output bundle.".into(),
            shared_request_seed: "build me a helper-backed summary reporting surface that filters inputs and exposes status plus report output".into(),
            target_substrate_kinds: vec![SubstrateKind::StaticWeb, SubstrateKind::Cli],
            required_composition_layers: vec![
                "base_build".into(),
                "patch".into(),
                "helper".into(),
            ],
            required_base_build_primitive_classes: vec![
                "summary_surface".into(),
                "status_chip".into(),
            ],
            required_patch_primitive_classes: vec![
                "summary_surface".into(),
                "export_output".into(),
                "filter_rule".into(),
            ],
            required_helper_primitive_kinds: vec![
                "inbox_lane".into(),
                "file_filter".into(),
                "summary_emitter".into(),
                "status_reporter".into(),
            ],
            required_capability_classes: vec![
                "helper_bundle".into(),
                "helper_summary".into(),
                "lane_filter_rules".into(),
                "report_output".into(),
                "status_surface".into(),
            ],
            optional_capability_classes: vec![
                "helper_contract".into(),
                "export_surface".into(),
            ],
            optional_enrichment_steps: vec![
                "json_output".into(),
                "progress_banner".into(),
            ],
            execution_recipe: PrimitiveProofExecutionRecipe {
                comparison_bundle_id: "bundle_summary_reporting_equivalence".into(),
                request_generation_kind: "summary_reporting".into(),
                enrichment_kind: "summary_reporting".into(),
                substrate_request_bindings: vec![
                    PrimitiveProofSubstrateRequestBinding {
                        substrate_kind: Some(SubstrateKind::StaticWeb),
                        substrate_label: "static web".into(),
                        request_template:
                            "build me a static web dashboard with a local inbox helper that {request_tail}".into(),
                        empty_request_fallback: Some(
                            "build me a static web dashboard with a local inbox helper that exposes status plus report output".into(),
                        ),
                    },
                    PrimitiveProofSubstrateRequestBinding {
                        substrate_kind: Some(SubstrateKind::Cli),
                        substrate_label: "cli".into(),
                        request_template:
                            "build me a rust cli log summary tool with a local inbox helper that {request_tail}".into(),
                        empty_request_fallback: Some(
                            "build me a rust cli log summary tool with a local inbox helper that filters inputs and exposes status plus report output".into(),
                        ),
                    },
                ],
                enrichment_bindings: vec![
                    PrimitiveProofEnrichmentBinding {
                        substrate_kind: Some(SubstrateKind::Cli),
                        missing_capability_classes: vec!["export_surface".into()],
                        patch_requests: vec!["add json output".into()],
                    },
                    PrimitiveProofEnrichmentBinding {
                        substrate_kind: Some(SubstrateKind::StaticWeb),
                        missing_capability_classes: vec!["status_surface".into()],
                        patch_requests: vec!["add progress banner".into()],
                    },
                ],
            },
            created_at: None,
        },
        PrimitiveProofTemplate {
            template_id: "proof_status_reporting".into(),
            template_kind: "cross_surface_status_reporting".into(),
            display_label: "Status Reporting".into(),
            description: "Checks whether different surfaces can fulfill the same status-forward reporting bundle with shared summary and report output behavior.".into(),
            shared_request_seed: "build me a helper-backed status reporting surface that filters inputs and exposes status plus report output".into(),
            target_substrate_kinds: vec![SubstrateKind::StaticWeb, SubstrateKind::Cli],
            required_composition_layers: vec![
                "base_build".into(),
                "patch".into(),
                "helper".into(),
            ],
            required_base_build_primitive_classes: vec![
                "summary_surface".into(),
                "status_chip".into(),
            ],
            required_patch_primitive_classes: vec![
                "summary_surface".into(),
                "export_output".into(),
            ],
            required_helper_primitive_kinds: vec![
                "inbox_lane".into(),
                "file_filter".into(),
                "summary_emitter".into(),
                "status_reporter".into(),
            ],
            required_capability_classes: vec![
                "helper_summary".into(),
                "report_output".into(),
                "status_surface".into(),
            ],
            optional_capability_classes: vec![
                "helper_bundle".into(),
                "helper_contract".into(),
                "lane_filter_rules".into(),
                "export_surface".into(),
            ],
            optional_enrichment_steps: vec![
                "json_output".into(),
                "progress_banner".into(),
            ],
            execution_recipe: PrimitiveProofExecutionRecipe {
                comparison_bundle_id: "bundle_status_reporting_equivalence".into(),
                request_generation_kind: "summary_reporting".into(),
                enrichment_kind: "summary_reporting".into(),
                substrate_request_bindings: vec![
                    PrimitiveProofSubstrateRequestBinding {
                        substrate_kind: Some(SubstrateKind::StaticWeb),
                        substrate_label: "static web".into(),
                        request_template:
                            "build me a static web dashboard with a local inbox helper that {request_tail}".into(),
                        empty_request_fallback: Some(
                            "build me a static web dashboard with a local inbox helper that exposes status plus report output".into(),
                        ),
                    },
                    PrimitiveProofSubstrateRequestBinding {
                        substrate_kind: Some(SubstrateKind::Cli),
                        substrate_label: "cli".into(),
                        request_template:
                            "build me a rust cli log summary tool with a local inbox helper that {request_tail}".into(),
                        empty_request_fallback: Some(
                            "build me a rust cli log summary tool with a local inbox helper that filters inputs and exposes status plus report output".into(),
                        ),
                    },
                ],
                enrichment_bindings: vec![
                    PrimitiveProofEnrichmentBinding {
                        substrate_kind: Some(SubstrateKind::Cli),
                        missing_capability_classes: vec!["export_surface".into()],
                        patch_requests: vec!["add json output".into()],
                    },
                    PrimitiveProofEnrichmentBinding {
                        substrate_kind: Some(SubstrateKind::StaticWeb),
                        missing_capability_classes: vec!["status_surface".into()],
                        patch_requests: vec!["add progress banner".into()],
                    },
                ],
            },
            created_at: None,
        },
        PrimitiveProofTemplate {
            template_id: "proof_filtered_reporting".into(),
            template_kind: "cross_surface_filtered_reporting".into(),
            display_label: "Filtered Reporting".into(),
            description: "Checks whether different reporting-oriented surfaces fulfill the same helper-backed filtered reporting bundle.".into(),
            shared_request_seed: "build me a helper-backed filtered reporting surface that filters inputs and emits report output".into(),
            target_substrate_kinds: vec![SubstrateKind::StaticWeb, SubstrateKind::Cli],
            required_composition_layers: vec![
                "base_build".into(),
                "patch".into(),
                "helper".into(),
            ],
            required_base_build_primitive_classes: vec![
                "summary_surface".into(),
                "status_chip".into(),
            ],
            required_patch_primitive_classes: vec![
                "summary_surface".into(),
                "export_output".into(),
                "filter_rule".into(),
            ],
            required_helper_primitive_kinds: vec![
                "inbox_lane".into(),
                "file_filter".into(),
                "summary_emitter".into(),
                "status_reporter".into(),
            ],
            required_capability_classes: vec![
                "helper_summary".into(),
                "lane_filter_rules".into(),
                "report_output".into(),
            ],
            optional_capability_classes: vec![
                "status_surface".into(),
                "helper_contract".into(),
                "export_surface".into(),
            ],
            optional_enrichment_steps: vec![
                "json_output".into(),
                "progress_banner".into(),
            ],
            execution_recipe: PrimitiveProofExecutionRecipe {
                comparison_bundle_id: "bundle_filtered_reporting_equivalence".into(),
                request_generation_kind: "summary_reporting".into(),
                enrichment_kind: "summary_reporting".into(),
                substrate_request_bindings: vec![
                    PrimitiveProofSubstrateRequestBinding {
                        substrate_kind: Some(SubstrateKind::StaticWeb),
                        substrate_label: "static web".into(),
                        request_template:
                            "build me a static web dashboard with a local inbox helper that {request_tail}".into(),
                        empty_request_fallback: Some(
                            "build me a static web dashboard with a local inbox helper that filters inputs and emits report output".into(),
                        ),
                    },
                    PrimitiveProofSubstrateRequestBinding {
                        substrate_kind: Some(SubstrateKind::Cli),
                        substrate_label: "cli".into(),
                        request_template:
                            "build me a rust cli log summary tool with a local inbox helper that {request_tail}".into(),
                        empty_request_fallback: Some(
                            "build me a rust cli log summary tool with a local inbox helper that filters inputs and emits report output".into(),
                        ),
                    },
                ],
                enrichment_bindings: vec![
                    PrimitiveProofEnrichmentBinding {
                        substrate_kind: Some(SubstrateKind::Cli),
                        missing_capability_classes: vec!["export_surface".into()],
                        patch_requests: vec!["add json output".into()],
                    },
                    PrimitiveProofEnrichmentBinding {
                        substrate_kind: Some(SubstrateKind::StaticWeb),
                        missing_capability_classes: vec!["status_surface".into()],
                        patch_requests: vec!["add progress banner".into()],
                    },
                ],
            },
            created_at: None,
        },
    ]
}

pub fn proof_template_by_id(
    template_id: &str,
    shared_request_seed_override: Option<&str>,
) -> Option<PrimitiveProofTemplate> {
    built_in_proof_templates()
        .into_iter()
        .find(|template| template.template_id == template_id)
        .map(|mut template| {
            if let Some(seed) = shared_request_seed_override {
                template.shared_request_seed = seed.to_string();
            }
            template
        })
}

pub fn built_in_capability_comparison_bundles() -> Vec<CapabilityComparisonBundle> {
    vec![
        CapabilityComparisonBundle {
            bundle_id: "bundle_helper_monitoring_equivalence".into(),
            bundle_kind: "legacy_comparison_equivalence".into(),
            required_shared_capability_classes: vec![
                "helper_bundle".into(),
                "helper_monitoring_surface".into(),
                "helper_preview_surface".into(),
                "helper_summary".into(),
                "lane_filter_rules".into(),
                "multi_lane_coverage".into(),
            ],
            optional_shared_capability_classes: vec![
                "helper_contract".into(),
                "helper_status".into(),
            ],
            tolerated_left_only_capability_classes: vec![
                "helper_processed_files_surface".into(),
                "helper_processed_output".into(),
            ],
            tolerated_right_only_capability_classes: vec![
                "helper_contract".into(),
                "helper_status".into(),
                "helper_processed_files_surface".into(),
                "helper_processed_output".into(),
            ],
            minimum_shared_capability_count: 6,
            equivalence_mode: "required_shared_bundle".into(),
            policy: CapabilityComparisonPolicy {
                comparison_receipt_prefix: "legacy-comparison-monitoring".into(),
                comparison_label: "helper monitoring".into(),
                shared_note_label: "shared helper monitoring capabilities".into(),
                left_only_note_label: "left-only capabilities".into(),
                right_only_note_label: "right-only capabilities".into(),
                required_bundle_note_label: "required shared bundle".into(),
                success_note_template:
                    "both projects satisfy the shared {comparison_label} capability bundle".into(),
                failure_note_template:
                    "shared {comparison_label} capability bundle is incomplete across the compared projects".into(),
            },
            created_at: None,
        },
        CapabilityComparisonBundle {
            bundle_id: "bundle_summary_reporting_equivalence".into(),
            bundle_kind: "cross_surface_equivalence".into(),
            required_shared_capability_classes: vec![
                "helper_summary".into(),
                "lane_filter_rules".into(),
                "report_output".into(),
                "status_surface".into(),
            ],
            optional_shared_capability_classes: vec![
                "helper_contract".into(),
                "export_surface".into(),
            ],
            tolerated_left_only_capability_classes: Vec::new(),
            tolerated_right_only_capability_classes: vec![
                "helper_contract".into(),
            ],
            minimum_shared_capability_count: 4,
            equivalence_mode: "required_shared_bundle".into(),
            policy: CapabilityComparisonPolicy {
                comparison_receipt_prefix: "legacy-comparison-summary-reporting".into(),
                comparison_label: "summary reporting".into(),
                shared_note_label: "shared summary reporting capabilities".into(),
                left_only_note_label: "left-only capabilities".into(),
                right_only_note_label: "right-only capabilities".into(),
                required_bundle_note_label: "required shared bundle".into(),
                success_note_template:
                    "both projects satisfy the shared {comparison_label} capability bundle".into(),
                failure_note_template:
                    "shared {comparison_label} capability bundle is incomplete across the compared projects".into(),
            },
            created_at: None,
        },
        CapabilityComparisonBundle {
            bundle_id: "bundle_status_reporting_equivalence".into(),
            bundle_kind: "cross_surface_equivalence".into(),
            required_shared_capability_classes: vec![
                "helper_summary".into(),
                "report_output".into(),
                "status_surface".into(),
            ],
            optional_shared_capability_classes: vec![
                "helper_bundle".into(),
                "helper_contract".into(),
                "lane_filter_rules".into(),
                "export_surface".into(),
            ],
            tolerated_left_only_capability_classes: Vec::new(),
            tolerated_right_only_capability_classes: vec![
                "helper_contract".into(),
                "export_surface".into(),
            ],
            minimum_shared_capability_count: 3,
            equivalence_mode: "required_shared_bundle".into(),
            policy: CapabilityComparisonPolicy {
                comparison_receipt_prefix: "legacy-comparison-status-reporting".into(),
                comparison_label: "status reporting".into(),
                shared_note_label: "shared status reporting capabilities".into(),
                left_only_note_label: "left-only capabilities".into(),
                right_only_note_label: "right-only capabilities".into(),
                required_bundle_note_label: "required shared bundle".into(),
                success_note_template:
                    "both projects satisfy the shared {comparison_label} capability bundle".into(),
                failure_note_template:
                    "shared {comparison_label} capability bundle is incomplete across the compared projects".into(),
            },
            created_at: None,
        },
        CapabilityComparisonBundle {
            bundle_id: "bundle_filtered_reporting_equivalence".into(),
            bundle_kind: "cross_surface_equivalence".into(),
            required_shared_capability_classes: vec![
                "helper_summary".into(),
                "lane_filter_rules".into(),
                "report_output".into(),
            ],
            optional_shared_capability_classes: vec![
                "status_surface".into(),
                "helper_bundle".into(),
                "helper_contract".into(),
                "helper_status".into(),
                "multi_lane_coverage".into(),
                "export_surface".into(),
            ],
            tolerated_left_only_capability_classes: vec![
                "helper_monitoring_surface".into(),
                "helper_preview_surface".into(),
            ],
            tolerated_right_only_capability_classes: vec![
                "helper_contract".into(),
                "export_surface".into(),
            ],
            minimum_shared_capability_count: 3,
            equivalence_mode: "required_shared_bundle".into(),
            policy: CapabilityComparisonPolicy {
                comparison_receipt_prefix: "legacy-comparison-filtered-reporting".into(),
                comparison_label: "filtered reporting".into(),
                shared_note_label: "shared filtered reporting capabilities".into(),
                left_only_note_label: "left-only capabilities".into(),
                right_only_note_label: "right-only capabilities".into(),
                required_bundle_note_label: "required shared bundle".into(),
                success_note_template:
                    "both projects satisfy the shared {comparison_label} capability bundle".into(),
                failure_note_template:
                    "shared {comparison_label} capability bundle is incomplete across the compared projects".into(),
            },
            created_at: None,
        },
    ]
}

pub fn capability_comparison_bundle_by_id(bundle_id: &str) -> Option<CapabilityComparisonBundle> {
    built_in_capability_comparison_bundles()
        .into_iter()
        .find(|bundle| bundle.bundle_id == bundle_id)
}

fn load_json_manifests<T: DeserializeOwned>(root: &Path) -> Vec<(PathBuf, T)> {
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut files = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    files.sort();
    files
        .into_iter()
        .filter_map(|path| {
            let contents = fs::read_to_string(&path).ok()?;
            let manifest = serde_json::from_str::<T>(&contents).ok()?;
            Some((path, manifest))
        })
        .collect()
}

pub fn repo_proof_template_manifest_root(workspace_root: &Path) -> PathBuf {
    workspace_root.join("proof_harness").join("templates")
}

pub fn repo_capability_comparison_bundle_manifest_root(workspace_root: &Path) -> PathBuf {
    workspace_root.join("proof_harness").join("bundles")
}

pub fn repo_proof_templates(workspace_root: &Path) -> Vec<PrimitiveProofTemplate> {
    load_json_manifests::<PrimitiveProofTemplate>(&repo_proof_template_manifest_root(
        workspace_root,
    ))
    .into_iter()
    .map(|(_, template)| template)
    .collect()
}

pub fn repo_capability_comparison_bundles(
    workspace_root: &Path,
) -> Vec<CapabilityComparisonBundle> {
    load_json_manifests::<CapabilityComparisonBundle>(
        &repo_capability_comparison_bundle_manifest_root(workspace_root),
    )
    .into_iter()
    .map(|(_, bundle)| bundle)
    .collect()
}

pub fn proof_templates_from_root(workspace_root: &Path) -> Vec<PrimitiveProofTemplate> {
    let mut merged = BTreeMap::new();
    for template in built_in_proof_templates() {
        merged.insert(template.template_id.clone(), template);
    }
    for template in repo_proof_templates(workspace_root) {
        merged.insert(template.template_id.clone(), template);
    }
    merged.into_values().collect()
}

pub fn capability_comparison_bundles_from_root(
    workspace_root: &Path,
) -> Vec<CapabilityComparisonBundle> {
    let mut merged = BTreeMap::new();
    for bundle in built_in_capability_comparison_bundles() {
        merged.insert(bundle.bundle_id.clone(), bundle);
    }
    for bundle in repo_capability_comparison_bundles(workspace_root) {
        merged.insert(bundle.bundle_id.clone(), bundle);
    }
    merged.into_values().collect()
}

pub fn proof_template_by_id_from_root(
    workspace_root: &Path,
    template_id: &str,
    shared_request_seed_override: Option<&str>,
) -> Option<PrimitiveProofTemplate> {
    proof_templates_from_root(workspace_root)
        .into_iter()
        .find(|template| template.template_id == template_id)
        .map(|mut template| {
            if let Some(seed) = shared_request_seed_override {
                template.shared_request_seed = seed.to_string();
            }
            template
        })
}

pub fn capability_comparison_bundle_by_id_from_root(
    workspace_root: &Path,
    bundle_id: &str,
) -> Option<CapabilityComparisonBundle> {
    capability_comparison_bundles_from_root(workspace_root)
        .into_iter()
        .find(|bundle| bundle.bundle_id == bundle_id)
}

pub fn proof_template_manifest_path(workspace_root: &Path, template_id: &str) -> Option<PathBuf> {
    load_json_manifests::<PrimitiveProofTemplate>(&repo_proof_template_manifest_root(
        workspace_root,
    ))
    .into_iter()
    .find(|(_, template)| template.template_id == template_id)
    .map(|(path, _)| path)
}

pub fn capability_comparison_bundle_manifest_path(
    workspace_root: &Path,
    bundle_id: &str,
) -> Option<PathBuf> {
    load_json_manifests::<CapabilityComparisonBundle>(
        &repo_capability_comparison_bundle_manifest_root(workspace_root),
    )
    .into_iter()
    .find(|(_, bundle)| bundle.bundle_id == bundle_id)
    .map(|(path, _)| path)
}
