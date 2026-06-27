use std::path::Path;

use anyhow::Result;
use chatty_factory_core::{
    expand_operator_bundle_ids, operator_bundle_registry, patch_primitive_class,
    AcceptanceRecipeStatus, OperatorBundleStatus, PatchLaneStatus, PatchReceipt,
};

use crate::PatchArtifacts;

pub(crate) type PatchHandler =
    fn(&Path, &str, &str, &str) -> Result<(PatchArtifacts, PatchReceipt)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegistryCheckSpec {
    pub check_id: &'static str,
    pub kind: &'static str,
    pub target: &'static str,
    pub expected: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OperatorContributionSpec {
    pub substrate_kind: &'static str,
    pub operator_id: &'static str,
    pub feature_id: &'static str,
    pub check: RegistryCheckSpec,
    pub required_marker: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AcceptanceRecipeSpec {
    pub recipe_id: &'static str,
    pub substrate_kind: &'static str,
    pub tool_kind: Option<&'static str>,
    pub feature_id: &'static str,
    pub command_id: &'static str,
    pub expected_output: Option<&'static str>,
    pub required_markers: &'static [&'static str],
    pub checks: &'static [RegistryCheckSpec],
}

#[derive(Clone)]
pub(crate) struct PatchRecipeSpec {
    pub recipe_id: &'static str,
    pub substrate_kind: &'static str,
    pub tool_kind: Option<&'static str>,
    pub patch_kind: &'static str,
    pub primitive_classes: &'static [&'static str],
    pub dependency_mode: &'static str,
    pub requires_features: &'static [&'static str],
    pub provides_features: &'static [&'static str],
    pub request_match_any: &'static [&'static str],
    pub request_match_all: &'static [&'static str],
    pub handler: PatchHandler,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PatchStructuralGuardSpec {
    pub required_anchor_markers: &'static [&'static str],
    pub conflicting_anchor_markers: &'static [&'static str],
    pub expected_artifact_groups: &'static [&'static str],
    pub ownership_boundaries: &'static [&'static str],
}

const PATCH_CLASS_SUMMARY_SURFACE: &[&str] = &["summary_surface"];
const PATCH_CLASS_STATUS_CHIP: &[&str] = &["status_chip"];
const PATCH_CLASS_INFO_CHIP: &[&str] = &["info_chip"];
const PATCH_CLASS_METADATA_ROW: &[&str] = &["metadata_row"];
const PATCH_CLASS_NOTICE_SURFACE: &[&str] = &["notice_surface"];
const PATCH_CLASS_FILTER_RULE: &[&str] = &["filter_rule"];
const PATCH_CLASS_INBOX_LANE_EXTENSION: &[&str] = &["inbox_lane_extension"];
const PATCH_CLASS_SELECTION_CONTROL: &[&str] = &["selection_control"];
const PATCH_CLASS_REFRESH_BEHAVIOR: &[&str] = &["refresh_behavior"];
const PATCH_CLASS_TIMESTAMP_ROW: &[&str] = &["timestamp_row"];
const PATCH_CLASS_BADGE_STRIP: &[&str] = &["badge_strip"];
const PATCH_CLASS_EXPORT_OUTPUT: &[&str] = &["export_output"];
const PATCH_CLASS_PATCH_EXTENSION: &[&str] = &["patch_extension"];

pub(crate) fn patch_structural_guard_spec(
    substrate_kind: &str,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Option<PatchStructuralGuardSpec> {
    match (substrate_kind, tool_kind, patch_kind) {
        ("static_web", _, "progress_banner") => Some(PatchStructuralGuardSpec {
            required_anchor_markers: &["index.html::results-panel"],
            conflicting_anchor_markers: &[],
            expected_artifact_groups: &["entrypoints", "style_surfaces", "contract_files"],
            ownership_boundaries: &[
                "preserve static dashboard ownership split across index.html, app.js, and styles.css",
            ],
        }),
        ("cli", Some("csv_report"), "json_export")
        | ("cli", Some("csv_report"), "column_filter")
        | ("cli", Some("csv_report"), "email_sender")
        | ("cli", Some("csv_report"), "new_patch_lane") => Some(PatchStructuralGuardSpec {
            required_anchor_markers: &[
                "main.py::def build_parser()",
                "main.py::def main() -> int:",
            ],
            conflicting_anchor_markers: &[],
            expected_artifact_groups: &["entrypoints", "contract_files", "documentation_surfaces"],
            ownership_boundaries: &[
                "preserve python CLI entrypoint flow and contract-file synchronization",
            ],
        }),
        ("cli", Some("log_summary"), "severity_filter") => Some(PatchStructuralGuardSpec {
            required_anchor_markers: &[
                "src/main.rs::fn main()",
                "Cargo.toml::[dependencies]",
            ],
            conflicting_anchor_markers: &[],
            expected_artifact_groups: &["entrypoints", "contract_files", "documentation_surfaces"],
            ownership_boundaries: &[
                "preserve Rust CLI entrypoint flow and Cargo contract synchronization",
            ],
        }),
        ("cli", Some("log_summary"), "file_output") => Some(PatchStructuralGuardSpec {
            required_anchor_markers: &[
                "src/main.rs::fn main()",
                "Cargo.toml::[dependencies]",
            ],
            conflicting_anchor_markers: &[],
            expected_artifact_groups: &["entrypoints", "contract_files", "documentation_surfaces"],
            ownership_boundaries: &[
                "preserve Rust CLI entrypoint flow and Cargo contract synchronization",
            ],
        }),
        ("cli", Some("log_summary"), "json_output") => Some(PatchStructuralGuardSpec {
            required_anchor_markers: &[
                "src/main.rs::fn main()",
                "Cargo.toml::[dependencies]",
            ],
            conflicting_anchor_markers: &[],
            expected_artifact_groups: &["entrypoints", "contract_files", "documentation_surfaces"],
            ownership_boundaries: &[
                "preserve Rust CLI entrypoint flow and Cargo contract synchronization",
            ],
        }),
        ("cli", Some("log_summary"), "new_patch_lane") => Some(PatchStructuralGuardSpec {
            required_anchor_markers: &[
                "src/main.rs::fn main()",
                "Cargo.toml::[dependencies]",
            ],
            conflicting_anchor_markers: &[],
            expected_artifact_groups: &["entrypoints", "contract_files", "documentation_surfaces"],
            ownership_boundaries: &[
                "preserve Rust CLI entrypoint flow and Cargo contract synchronization",
            ],
        }),
        _ => None,
    }
}

pub(crate) fn patch_recipe_superseded_by(
    substrate_kind: &str,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> &'static [&'static str] {
    let _ = (substrate_kind, tool_kind, patch_kind);
    match () {
        _ => &[],
    }
}

const OPERATOR_CONTRIBUTIONS: &[OperatorContributionSpec] = &[
    OperatorContributionSpec {
        substrate_kind: "static_web",
        operator_id: "metric_card",
        feature_id: "metric_card",
        check: RegistryCheckSpec {
            check_id: "operator-metric-card",
            kind: "contains",
            target: "index.html",
            expected: Some("metric-card"),
        },
        required_marker: "metric-card",
    },
    OperatorContributionSpec {
        substrate_kind: "static_web",
        operator_id: "status_panel",
        feature_id: "status_panel",
        check: RegistryCheckSpec {
            check_id: "operator-status-panel",
            kind: "contains",
            target: "index.html",
            expected: Some("status-panel"),
        },
        required_marker: "status-panel",
    },
    OperatorContributionSpec {
        substrate_kind: "static_web",
        operator_id: "results_panel",
        feature_id: "results_panel",
        check: RegistryCheckSpec {
            check_id: "operator-results-panel",
            kind: "contains",
            target: "index.html",
            expected: Some("results-panel"),
        },
        required_marker: "results-panel",
    },
    OperatorContributionSpec {
        substrate_kind: "static_web",
        operator_id: "action_toolbar",
        feature_id: "action_toolbar",
        check: RegistryCheckSpec {
            check_id: "operator-action-toolbar",
            kind: "contains",
            target: "index.html",
            expected: Some("action-toolbar"),
        },
        required_marker: "action-toolbar",
    },
];

const DIRECTORY_AUDIT_RECIPE_CHECKS: &[RegistryCheckSpec] = &[RegistryCheckSpec {
    check_id: "recipe-directory-audit-readme",
    kind: "contains",
    target: "README.md",
    expected: Some("files=<n> ext:<extension>=<count> ..."),
}];

const DASHBOARD_OPERATOR_RECIPE_CHECKS: &[RegistryCheckSpec] = &[
    RegistryCheckSpec {
        check_id: "recipe-dashboard-operator-grid",
        kind: "contains",
        target: "index.html",
        expected: Some("operator-grid"),
    },
    RegistryCheckSpec {
        check_id: "recipe-dashboard-status-panel",
        kind: "contains",
        target: "index.html",
        expected: Some("status-panel"),
    },
    RegistryCheckSpec {
        check_id: "recipe-dashboard-action-toolbar",
        kind: "contains",
        target: "index.html",
        expected: Some("action-toolbar"),
    },
    RegistryCheckSpec {
        check_id: "recipe-dashboard-results-panel",
        kind: "contains",
        target: "index.html",
        expected: Some("results-panel"),
    },
];

const CSV_REPORT_RECIPE_CHECKS: &[RegistryCheckSpec] = &[RegistryCheckSpec {
    check_id: "recipe-csv-report-readme",
    kind: "contains",
    target: "README.md",
    expected: Some("rows=<n> columns=<n> header=<comma-separated-columns>"),
}];

const LOG_SUMMARY_RECIPE_CHECKS: &[RegistryCheckSpec] = &[RegistryCheckSpec {
    check_id: "recipe-log-summary-readme",
    kind: "contains",
    target: "README.md",
    expected: Some("errors=<n> warnings=<n> infos=<n>"),
}];

const ACCEPTANCE_RECIPES: &[AcceptanceRecipeSpec] = &[
    AcceptanceRecipeSpec {
        recipe_id: "directory_audit_contract",
        substrate_kind: "cli",
        tool_kind: Some("directory_audit"),
        feature_id: "directory_audit_contract",
        command_id: "planner_recipe_directory_audit_contract",
        expected_output: Some("README.md"),
        required_markers: &["files=<n> ext:<extension>=<count> ..."],
        checks: DIRECTORY_AUDIT_RECIPE_CHECKS,
    },
    AcceptanceRecipeSpec {
        recipe_id: "dashboard_operator_surface",
        substrate_kind: "static_web",
        tool_kind: None,
        feature_id: "dashboard_operator_surface",
        command_id: "planner_recipe_dashboard_operator_surface",
        expected_output: None,
        required_markers: &[
            "operator-grid",
            "status-panel",
            "action-toolbar",
            "results-panel",
        ],
        checks: DASHBOARD_OPERATOR_RECIPE_CHECKS,
    },
    AcceptanceRecipeSpec {
        recipe_id: "csv_report_stdout_contract",
        substrate_kind: "cli",
        tool_kind: Some("csv_report"),
        feature_id: "csv_report_stdout_contract",
        command_id: "planner_recipe_csv_report_stdout_contract",
        expected_output: None,
        required_markers: &["rows=<n> columns=<n> header=<comma-separated-columns>"],
        checks: CSV_REPORT_RECIPE_CHECKS,
    },
    AcceptanceRecipeSpec {
        recipe_id: "log_summary_stdout_contract",
        substrate_kind: "cli",
        tool_kind: Some("log_summary"),
        feature_id: "log_summary_stdout_contract",
        command_id: "planner_recipe_log_summary_stdout_contract",
        expected_output: None,
        required_markers: &["errors=<n> warnings=<n> infos=<n>"],
        checks: LOG_SUMMARY_RECIPE_CHECKS,
    },
];

pub(crate) fn operator_contribution_registry() -> &'static [OperatorContributionSpec] {
    OPERATOR_CONTRIBUTIONS
}

pub(crate) fn acceptance_recipe_registry() -> &'static [AcceptanceRecipeSpec] {
    ACCEPTANCE_RECIPES
}

pub(crate) fn patch_recipe_registry() -> Vec<PatchRecipeSpec> {
    vec![
        PatchRecipeSpec {
            recipe_id: "dashboard_progress_banner",
            substrate_kind: "static_web",
            tool_kind: None,
            patch_kind: "progress_banner",
            primitive_classes: PATCH_CLASS_SUMMARY_SURFACE,
            dependency_mode: "standalone",
            requires_features: &[],
            provides_features: &["progress_banner"],
            request_match_any: &["progress banner", "banner"],
            request_match_all: &[],
            handler: crate::patch_static_web_dashboard_progress_banner,
        },
        PatchRecipeSpec {
            recipe_id: "csv_report_json_export",
            substrate_kind: "cli",
            tool_kind: Some("csv_report"),
            patch_kind: "json_export",
            primitive_classes: PATCH_CLASS_EXPORT_OUTPUT,
            dependency_mode: "standalone",
            requires_features: &[],
            provides_features: &["json_export"],
            request_match_any: &["json export", "json output"],
            request_match_all: &["json", "export"],
            handler: crate::patch_python_csv_report_json_export,
        },
        PatchRecipeSpec {
            recipe_id: "csv_report_column_filter",
            substrate_kind: "cli",
            tool_kind: Some("csv_report"),
            patch_kind: "column_filter",
            primitive_classes: PATCH_CLASS_FILTER_RULE,
            dependency_mode: "standalone",
            requires_features: &[],
            provides_features: &["column_filter"],
            request_match_any: &["column filter", "select column", "only column"],
            request_match_all: &["column", "filter"],
            handler: crate::patch_python_csv_report_column_filter,
        },
        PatchRecipeSpec {
            recipe_id: "csv_report_email_sender",
            substrate_kind: "cli",
            tool_kind: Some("csv_report"),
            patch_kind: "email_sender",
            primitive_classes: PATCH_CLASS_EXPORT_OUTPUT,
            dependency_mode: "standalone",
            requires_features: &[],
            provides_features: &["email_sender"],
            request_match_any: &["email sender", "email report", "email export", "send email"],
            request_match_all: &["email"],
            handler: crate::patch_python_csv_report_email_sender,
        },
        PatchRecipeSpec {
            recipe_id: "log_summary_file_output",
            substrate_kind: "cli",
            tool_kind: Some("log_summary"),
            patch_kind: "file_output",
            primitive_classes: PATCH_CLASS_EXPORT_OUTPUT,
            dependency_mode: "standalone",
            requires_features: &[],
            provides_features: &["file_output"],
            request_match_any: &["file output", "write file", "save summary"],
            request_match_all: &["output", "file"],
            handler: crate::patch_rust_log_summary_file_output,
        },
        PatchRecipeSpec {
            recipe_id: "log_summary_severity_filter",
            substrate_kind: "cli",
            tool_kind: Some("log_summary"),
            patch_kind: "severity_filter",
            primitive_classes: PATCH_CLASS_FILTER_RULE,
            dependency_mode: "standalone",
            requires_features: &[],
            provides_features: &["severity_filter"],
            request_match_any: &["severity filter", "severity only", "select severity"],
            request_match_all: &["severity", "filter"],
            handler: crate::patch_rust_log_summary_severity_filter,
        },
        PatchRecipeSpec {
            recipe_id: "log_summary_json_output",
            substrate_kind: "cli",
            tool_kind: Some("log_summary"),
            patch_kind: "json_output",
            primitive_classes: PATCH_CLASS_EXPORT_OUTPUT,
            dependency_mode: "standalone",
            requires_features: &[],
            provides_features: &["json_output"],
            request_match_any: &[
                "json summary",
                "log json output",
                "json output for the summary",
            ],
            request_match_all: &["json", "summary"],
            handler: crate::patch_rust_log_summary_json_output,
        },
        PatchRecipeSpec {
            recipe_id: "python_cli_tool_csv_report_new_patch_lane",
            substrate_kind: "cli",
            tool_kind: Some("csv_report"),
            patch_kind: "new_patch_lane",
            primitive_classes: PATCH_CLASS_PATCH_EXTENSION,
            dependency_mode: "standalone",
            requires_features: &[],
            provides_features: &["email_sender"],
            request_match_any: &[
                "email delivery",
                "email draft",
                "delivery email",
                "mail draft",
            ],
            request_match_all: &["email"],
            handler: crate::patch_python_cli_tool_csv_report_new_patch_lane,
        },
        PatchRecipeSpec {
            recipe_id: "rust_cli_tool_log_summary_new_patch_lane",
            substrate_kind: "cli",
            tool_kind: Some("log_summary"),
            patch_kind: "new_patch_lane",
            primitive_classes: PATCH_CLASS_PATCH_EXTENSION,
            dependency_mode: "standalone",
            requires_features: &[],
            provides_features: &["markdown_export"],
            request_match_any: &[
                "markdown export",
                "markdown summary",
                "markdown report",
                "summary markdown",
            ],
            request_match_all: &["markdown"],
            handler: crate::patch_rust_cli_tool_log_summary_new_patch_lane,
        },
        // Add generated registry entry for `static_web_dashboard_helper_lane` here.
    ]
}

pub(crate) fn candidate_patch_recipe_ids_for(
    substrate_kind: &str,
    tool_kind: Option<&str>,
    project_features: &[String],
) -> Vec<String> {
    patch_recipe_registry()
        .into_iter()
        .filter(|spec| spec.substrate_kind == substrate_kind)
        .filter(|spec| spec.tool_kind.is_none() || spec.tool_kind == tool_kind)
        .filter(|spec| patch_recipe_is_applicable(spec, project_features))
        .map(|spec| spec.recipe_id.to_string())
        .collect()
}

pub(crate) fn candidate_acceptance_recipe_ids_for(
    substrate_kind: &str,
    tool_kind: Option<&str>,
) -> Vec<String> {
    acceptance_recipe_registry()
        .iter()
        .filter(|spec| spec.substrate_kind == substrate_kind)
        .filter(|spec| spec.tool_kind.is_none() || spec.tool_kind == tool_kind)
        .map(|spec| spec.recipe_id.to_string())
        .collect()
}

pub(crate) fn acceptance_recipe_statuses_for(
    substrate_kind: &str,
    tool_kind: Option<&str>,
    project_features: &[String],
) -> Vec<AcceptanceRecipeStatus> {
    acceptance_recipe_registry()
        .iter()
        .filter(|spec| spec.substrate_kind == substrate_kind)
        .filter(|spec| spec.tool_kind.is_none() || spec.tool_kind == tool_kind)
        .map(|spec| AcceptanceRecipeStatus {
            recipe_id: spec.recipe_id.to_string(),
            feature_id: spec.feature_id.to_string(),
            command_id: spec.command_id.to_string(),
            availability_status: if project_features
                .iter()
                .any(|feature| feature == spec.feature_id)
            {
                "already_applied".to_string()
            } else {
                "available".to_string()
            },
        })
        .collect()
}

pub(crate) fn operator_bundle_statuses_for(
    substrate_kind: &str,
    project_features: &[String],
) -> Vec<OperatorBundleStatus> {
    operator_bundle_registry()
        .iter()
        .filter(|bundle| {
            bundle
                .substrate_kinds
                .iter()
                .any(|candidate| candidate == &substrate_kind)
        })
        .map(|bundle| {
            let operator_ids = expand_operator_bundle_ids(&[bundle.bundle_id.to_string()]);
            let provides_features: Vec<String> = operator_ids
                .iter()
                .filter_map(|operator_id| {
                    operator_contribution_registry()
                        .iter()
                        .find(|contribution| {
                            contribution.substrate_kind == substrate_kind
                                && contribution.operator_id == operator_id
                        })
                        .map(|contribution| contribution.feature_id.to_string())
                })
                .collect();

            let applied_count = provides_features
                .iter()
                .filter(|feature| project_features.iter().any(|existing| existing == *feature))
                .count();
            let availability_status = if provides_features.is_empty() || applied_count == 0 {
                "available"
            } else if applied_count == provides_features.len() {
                "already_applied"
            } else {
                "partially_applied"
            };

            OperatorBundleStatus {
                bundle_id: bundle.bundle_id.to_string(),
                operator_ids,
                provides_features,
                availability_status: availability_status.to_string(),
            }
        })
        .collect()
}

pub(crate) fn patch_recipe_by_kind(
    substrate_kind: &str,
    tool_kind: Option<&str>,
    project_features: &[String],
    patch_kind: &str,
) -> Option<PatchRecipeSpec> {
    patch_recipe_registry().into_iter().find(|spec| {
        spec.substrate_kind == substrate_kind
            && (spec.tool_kind.is_none() || spec.tool_kind == tool_kind)
            && patch_recipe_is_dispatchable(spec, project_features)
            && spec.patch_kind == patch_kind
    })
}

pub(crate) fn patch_recipe_from_request_text(
    substrate_kind: &str,
    tool_kind: Option<&str>,
    project_features: &[String],
    lower_request: &str,
) -> Option<PatchRecipeSpec> {
    patch_recipe_registry().into_iter().find(|spec| {
        spec.substrate_kind == substrate_kind
            && (spec.tool_kind.is_none() || spec.tool_kind == tool_kind)
            && patch_recipe_is_dispatchable(spec, project_features)
            && (spec
                .request_match_any
                .iter()
                .any(|needle| lower_request.contains(needle))
                || (!spec.request_match_all.is_empty()
                    && spec
                        .request_match_all
                        .iter()
                        .all(|needle| lower_request.contains(needle))))
    })
}

pub(crate) fn patch_lane_statuses_for(
    substrate_kind: &str,
    tool_kind: Option<&str>,
    project_features: &[String],
) -> Vec<PatchLaneStatus> {
    patch_recipe_registry()
        .into_iter()
        .filter(|spec| spec.substrate_kind == substrate_kind)
        .filter(|spec| spec.tool_kind.is_none() || spec.tool_kind == tool_kind)
        .map(|spec| PatchLaneStatus {
            recipe_id: spec.recipe_id.to_string(),
            patch_kind: spec.patch_kind.to_string(),
            dependency_mode: spec.dependency_mode.to_string(),
            requires_features: spec
                .requires_features
                .iter()
                .map(|item| (*item).to_string())
                .collect(),
            provides_features: spec
                .provides_features
                .iter()
                .map(|item| (*item).to_string())
                .collect(),
            availability_status: patch_recipe_availability(&spec, project_features).to_string(),
            surgical_maturity: patch_recipe_surgical_maturity(
                spec.substrate_kind,
                spec.tool_kind,
                spec.patch_kind,
            )
            .to_string(),
            superseded_by_patch_kinds: patch_recipe_superseded_by(
                spec.substrate_kind,
                spec.tool_kind,
                spec.patch_kind,
            )
            .iter()
            .map(|item| (*item).to_string())
            .collect(),
            effective_preflight_readiness: String::new(),
            preflight_readiness_reason: String::new(),
        })
        .collect()
}

pub(crate) fn patch_recipe_surgical_maturity(
    substrate_kind: &str,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> &'static str {
    let Some(guard) = patch_structural_guard_spec(substrate_kind, tool_kind, patch_kind) else {
        return "uncontracted";
    };
    if !guard.conflicting_anchor_markers.is_empty() {
        return "legacy_shape_sensitive";
    }
    if !guard.expected_artifact_groups.is_empty() && guard.expected_artifact_groups.len() <= 3 {
        return "narrow_surface_contract";
    }
    if !guard.expected_artifact_groups.is_empty() {
        return "broad_surface_contract";
    }
    if !guard.required_anchor_markers.is_empty() {
        return "anchor_only_contract";
    }
    "uncontracted"
}

pub(crate) fn patch_primitive_classes_for(
    substrate_kind: &str,
    tool_kind: Option<&str>,
    patch_kinds: &[String],
) -> Vec<String> {
    let registry = patch_recipe_registry();
    let mut classes = Vec::new();
    for patch_kind in patch_kinds {
        let declared_classes = registry
            .iter()
            .find(|spec| {
                spec.substrate_kind == substrate_kind
                    && (spec.tool_kind.is_none() || spec.tool_kind == tool_kind)
                    && spec.patch_kind == patch_kind
            })
            .map(|spec| spec.primitive_classes)
            .unwrap_or(&[]);
        if declared_classes.is_empty() {
            let fallback = patch_primitive_class(patch_kind).to_string();
            if !classes.contains(&fallback) {
                classes.push(fallback);
            }
            continue;
        }
        for class in declared_classes {
            let class = (*class).to_string();
            if !classes.contains(&class) {
                classes.push(class);
            }
        }
    }
    classes
}

fn patch_recipe_availability(spec: &PatchRecipeSpec, project_features: &[String]) -> &'static str {
    let requirements_met = spec
        .requires_features
        .iter()
        .all(|feature| project_features.iter().any(|existing| existing == feature));
    let already_provided = spec
        .provides_features
        .iter()
        .all(|feature| project_features.iter().any(|existing| existing == feature));

    if already_provided {
        return "already_applied";
    }

    match spec.dependency_mode {
        "standalone" => "available",
        "requires_features" if requirements_met => "available",
        "requires_features" => "blocked_missing_features",
        _ if requirements_met => "available",
        _ => "blocked_missing_features",
    }
}

fn patch_recipe_is_applicable(spec: &PatchRecipeSpec, project_features: &[String]) -> bool {
    patch_recipe_availability(spec, project_features) == "available"
}

fn patch_recipe_is_dispatchable(spec: &PatchRecipeSpec, project_features: &[String]) -> bool {
    matches!(
        patch_recipe_availability(spec, project_features),
        "available" | "already_applied"
    )
}
