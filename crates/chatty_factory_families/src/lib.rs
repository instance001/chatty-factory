use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use chatty_factory_core::{
    AcceptanceCheck, AcceptancePlan, BuildReceipt, ChattyCogVisualLoadSpec,
    HelperLaunchPolicy, HelperPrimitiveSpec, HelperServiceSpec, HelperStatusSnapshot,
    BuildSeedInputs, PatchLaneStatus, PatchReceipt, ProjectSpec, RequestPlan,
};
use minijinja::Environment;
use serde_json::to_string_pretty;

mod registry;

use registry::{
    acceptance_recipe_registry, acceptance_recipe_statuses_for,
    candidate_acceptance_recipe_ids_for, candidate_patch_recipe_ids_for,
    operator_bundle_statuses_for, operator_contribution_registry, patch_lane_statuses_for,
    patch_primitive_classes_for, patch_recipe_by_kind, patch_recipe_from_request_text,
    patch_recipe_registry, patch_recipe_surgical_maturity, patch_structural_guard_spec,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildArtifacts {
    pub project_dir: PathBuf,
    pub emitted_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchArtifacts {
    pub project_dir: PathBuf,
    pub modified_files: Vec<PathBuf>,
    pub patch_kind: String,
}

#[derive(Debug, Clone)]
struct WebBuildBundle {
    index_html: String,
    app_js: String,
    styles_css: String,
    readme_md: String,
}

fn base_template_environment() -> Result<Environment<'static>> {
    let mut env = Environment::new();
    env.add_template(
        "substrates/static_web/index.html",
        include_str!("../../../templates/substrates/static_web/index.html.j2"),
    )?;
    env.add_template(
        "substrates/static_web/app.js",
        include_str!("../../../templates/substrates/static_web/app.js.j2"),
    )?;
    env.add_template(
        "substrates/static_web/styles.css",
        include_str!("../../../templates/substrates/static_web/styles.css.j2"),
    )?;
    env.add_template(
        "substrates/static_web/README.md",
        include_str!("../../../templates/substrates/static_web/README.md.j2"),
    )?;
    env.add_template(
        "substrates/python_cli/main.py",
        include_str!("../../../templates/substrates/python_cli/main.py.j2"),
    )?;
    env.add_template(
        "substrates/python_cli/README.md",
        include_str!("../../../templates/substrates/python_cli/README.md.j2"),
    )?;
    env.add_template(
        "substrates/rust_cli/Cargo.toml",
        include_str!("../../../templates/substrates/rust_cli/Cargo.toml.j2"),
    )?;
    env.add_template(
        "substrates/rust_cli/main.rs",
        include_str!("../../../templates/substrates/rust_cli/main.rs.j2"),
    )?;
    env.add_template(
        "substrates/rust_cli/README.md",
        include_str!("../../../templates/substrates/rust_cli/README.md.j2"),
    )?;
    Ok(env)
}

fn render_named<S: serde::Serialize>(template_name: &str, context: S) -> Result<String> {
    let env = base_template_environment()?;
    let template = env.get_template(template_name)?;
    Ok(template.render(context)?)
}

#[derive(Debug, Clone)]
struct LocalInboxHelperBundle {
    helper_service: HelperServiceSpec,
    helper_status_snapshot: HelperStatusSnapshot,
    helper_summary_json: String,
    helper_readme: String,
}

pub fn candidate_patch_recipe_ids(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
    project_features: &[String],
) -> Vec<String> {
    substrate_kind
        .map(|substrate_kind| {
            candidate_patch_recipe_ids_for(substrate_kind, tool_kind, project_features)
        })
        .unwrap_or_default()
}

pub fn patch_lane_statuses(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
    project_features: &[String],
) -> Vec<PatchLaneStatus> {
    substrate_kind
        .map(|substrate_kind| patch_lane_statuses_for(substrate_kind, tool_kind, project_features))
        .unwrap_or_default()
}

pub fn patch_primitive_classes(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
    patch_kinds: &[String],
) -> Vec<String> {
    substrate_kind
        .map(|substrate_kind| patch_primitive_classes_for(substrate_kind, tool_kind, patch_kinds))
        .unwrap_or_default()
}

pub fn patch_required_anchor_markers(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Vec<String> {
    substrate_kind
        .and_then(|substrate_kind| {
            patch_structural_guard_spec(substrate_kind, tool_kind, patch_kind)
        })
        .map(|spec| {
            spec.required_anchor_markers
                .iter()
                .map(|marker| (*marker).to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn patch_conflicting_anchor_markers(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Vec<String> {
    substrate_kind
        .and_then(|substrate_kind| {
            patch_structural_guard_spec(substrate_kind, tool_kind, patch_kind)
        })
        .map(|spec| {
            spec.conflicting_anchor_markers
                .iter()
                .map(|marker| (*marker).to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn patch_expected_artifact_groups(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Vec<String> {
    substrate_kind
        .and_then(|substrate_kind| {
            patch_structural_guard_spec(substrate_kind, tool_kind, patch_kind)
        })
        .map(|spec| {
            spec.expected_artifact_groups
                .iter()
                .map(|group| (*group).to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn patch_ownership_boundaries(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Vec<String> {
    substrate_kind
        .and_then(|substrate_kind| {
            patch_structural_guard_spec(substrate_kind, tool_kind, patch_kind)
        })
        .map(|spec| {
            spec.ownership_boundaries
                .iter()
                .map(|note| (*note).to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn patch_surgical_maturity(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Option<String> {
    substrate_kind.map(|substrate_kind| {
        patch_recipe_surgical_maturity(substrate_kind, tool_kind, patch_kind).to_string()
    })
}

pub fn patch_preflight_readiness(
    project_dir: &Path,
    spec: &ProjectSpec,
    lane: &PatchLaneStatus,
) -> (String, String) {
    let superseded_note = patch_superseded_note(lane);
    match lane.availability_status.as_str() {
        "already_applied" => {
            return (
                "already_present".into(),
                "all lane-provided features are already present in the project".into(),
            );
        }
        "available" => {}
        other => {
            return (
                "dependency_blocked".into(),
                format!("lane dependencies are not yet satisfied (`{other}`)"),
            );
        }
    }

    let required_markers = patch_required_anchor_markers(
        registry_substrate_kind(spec).as_deref(),
        spec.tool_kind.as_deref(),
        &lane.patch_kind,
    );
    let conflicting_markers = patch_conflicting_anchor_markers(
        registry_substrate_kind(spec).as_deref(),
        spec.tool_kind.as_deref(),
        &lane.patch_kind,
    );
    let expected_groups = patch_expected_artifact_groups(
        registry_substrate_kind(spec).as_deref(),
        spec.tool_kind.as_deref(),
        &lane.patch_kind,
    );

    let missing_required = required_markers
        .iter()
        .filter(|marker| !structural_marker_present(project_dir, marker))
        .cloned()
        .collect::<Vec<_>>();
    if !missing_required.is_empty() {
        return (
            "structurally_blocked".into(),
            format_with_superseded_note(
                format!(
                    "required structural anchors are missing: {}",
                    missing_required
                        .into_iter()
                        .take(2)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                superseded_note.as_deref(),
            ),
        );
    }

    let present_conflicts = conflicting_markers
        .iter()
        .filter(|marker| structural_marker_present(project_dir, marker))
        .cloned()
        .collect::<Vec<_>>();
    if !present_conflicts.is_empty() {
        return (
            "structurally_blocked".into(),
            format_with_superseded_note(
                format!(
                    "conflicting evolved structural markers are already present: {}",
                    present_conflicts
                        .into_iter()
                        .take(2)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                superseded_note.as_deref(),
            ),
        );
    }

    let missing_groups = expected_groups
        .iter()
        .filter(|group| !artifact_group_present(project_dir, spec, group))
        .cloned()
        .collect::<Vec<_>>();
    if !missing_groups.is_empty() {
        return (
            "surface_mismatch".into(),
            format_with_superseded_note(
                format!(
                    "declared surface groups are not exposed in the current project shape: {}",
                    missing_groups.join(", ")
                ),
                superseded_note.as_deref(),
            ),
        );
    }

    (
        "ready".into(),
        "declared anchors and surface groups are present for preflight".into(),
    )
}

fn patch_superseded_note(lane: &PatchLaneStatus) -> Option<String> {
    if lane.superseded_by_patch_kinds.is_empty() {
        None
    } else {
        Some(format!(
            "superseded by modern lanes: {}",
            lane.superseded_by_patch_kinds.join(", ")
        ))
    }
}

fn format_with_superseded_note(reason: String, superseded_note: Option<&str>) -> String {
    if let Some(note) = superseded_note {
        format!("{reason}; {note}")
    } else {
        reason
    }
}

fn structural_marker_present(project_dir: &Path, marker: &str) -> bool {
    let Some((relative_path, snippet)) = marker.split_once("::") else {
        return false;
    };
    let file_path = project_dir.join(relative_path);
    let Ok(contents) = fs::read_to_string(file_path) else {
        return false;
    };
    contents.contains(snippet)
}

fn artifact_group_present(project_dir: &Path, spec: &ProjectSpec, group: &str) -> bool {
    match group {
        "entrypoints" => spec
            .entrypoints
            .iter()
            .any(|path| project_dir.join(path).exists()),
        "contract_files" => {
            project_dir.join("ProjectSpec.json").exists()
                && project_dir.join("AcceptancePlan.json").exists()
        }
        "bridge_surfaces" => {
            project_dir.join("bridge").exists()
                || spec
                    .expected_files
                    .iter()
                    .any(|path| path.starts_with("bridge/") && project_dir.join(path).exists())
                || project_dir.join("chattycog_bridge.js").exists()
        }
        "helper_service_surfaces" => {
            !spec.helper_services.is_empty()
                && spec.helper_services.iter().any(|helper| {
                    !helper.entrypoint.trim().is_empty()
                        || helper
                            .status_paths
                            .iter()
                            .any(|path| project_dir.join(path).exists())
                        || helper
                            .output_paths
                            .iter()
                            .any(|path| project_dir.join(path).exists())
                })
        }
        "style_surfaces" => {
            project_dir.join("styles.css").exists()
                || spec
                    .expected_files
                    .iter()
                    .any(|path| path.ends_with(".css") && project_dir.join(path).exists())
        }
        "logic_surfaces" => {
            project_dir.join("app.js").exists()
                || project_dir.join("main.py").exists()
                || project_dir.join("src").join("main.rs").exists()
                || spec.entrypoints.iter().any(|path| {
                    (path.ends_with(".js") || path.ends_with(".py") || path.ends_with(".rs"))
                        && project_dir.join(path).exists()
                })
        }
        "documentation_surfaces" => {
            project_dir.join("README.md").exists()
                || spec
                    .expected_files
                    .iter()
                    .any(|path| path.ends_with("README.md") && project_dir.join(path).exists())
        }
        _ => true,
    }
}

pub fn candidate_acceptance_recipe_ids(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
) -> Vec<String> {
    substrate_kind
        .map(|substrate_kind| candidate_acceptance_recipe_ids_for(substrate_kind, tool_kind))
        .unwrap_or_default()
}

pub fn refresh_project_contract_views_for_project(
    project_dir: Option<&Path>,
    spec: &mut ProjectSpec,
) {
    let Some(substrate_kind) = registry_substrate_kind(spec) else {
        spec.patch_lanes.clear();
        spec.acceptance_recipes.clear();
        spec.operator_bundles.clear();
        return;
    };

    spec.patch_lanes = patch_lane_statuses_for(
        substrate_kind.as_str(),
        spec.tool_kind.as_deref(),
        &spec.features,
    );
    if let Some(project_dir) = project_dir.filter(|path| path.exists()) {
        let lane_statuses = spec
            .patch_lanes
            .iter()
            .cloned()
            .map(|mut lane| {
                let (readiness, reason) = patch_preflight_readiness(project_dir, spec, &lane);
                lane.effective_preflight_readiness = readiness;
                lane.preflight_readiness_reason = reason;
                lane
            })
            .collect::<Vec<_>>();
        spec.patch_lanes = lane_statuses;
    } else {
        for lane in &mut spec.patch_lanes {
            lane.effective_preflight_readiness = "unknown".into();
            lane.preflight_readiness_reason =
                "project-aware preflight readiness was not computed for this contract snapshot"
                    .into();
        }
    }
    spec.acceptance_recipes = acceptance_recipe_statuses_for(
        substrate_kind.as_str(),
        spec.tool_kind.as_deref(),
        &spec.features,
    );
    spec.operator_bundles =
        operator_bundle_statuses_for(substrate_kind.as_str(), &spec.features);
}

fn refresh_project_contract_views(spec: &mut ProjectSpec) {
    let inferred_project_dir = PathBuf::from("output").join(&spec.project_name);
    refresh_project_contract_views_for_project(Some(&inferred_project_dir), spec);
}

fn request_wants_local_inbox_helper(inputs: &BuildSeedInputs) -> bool {
    let lower = inputs.summary.to_ascii_lowercase();
    lower.contains("helper")
        || lower.contains("inbox")
        || lower.contains("watch")
        || lower.contains("monitor")
        || lower.contains("filtered file")
        || lower.contains("local file")
}

fn registry_substrate_kind(spec: &ProjectSpec) -> Option<String> {
    match spec.substrate.trim() {
        "static_web" => Some("static_web".into()),
        "python_cli" | "rust_cli" | "cli" => Some("cli".into()),
        "" => None,
        other => Some(other.to_string()),
    }
}

fn build_local_inbox_helper_bundle(
    project_name: &str,
    purpose: &str,
    attachment_note: &str,
) -> Result<LocalInboxHelperBundle> {
    let helper_status_snapshot = HelperStatusSnapshot {
        helper_id: "local_inbox_helper".into(),
        helper_kind: "local_inbox_helper".into(),
        status: "prepared".into(),
        summary: "Local inbox helper bundle is present but not yet launched.".into(),
        observed_inputs: vec!["bridge/incoming_assets/module_assets".into()],
        observed_outputs: vec![
            "bridge/helpers/local_inbox/processed".into(),
            "bridge/helpers/local_inbox/summary.json".into(),
        ],
        observed_status_files: vec!["bridge/helpers/local_inbox/status.json".into()],
        observed_primitive_ids: vec![
            "module_assets_inbox_lane".into(),
            "local_inbox_processed_output".into(),
            "local_inbox_summary_snapshot".into(),
            "local_inbox_status_snapshot".into(),
        ],
        updated_at: None,
    };
    let helper_service = HelperServiceSpec {
        helper_spec_id: "chattyfactory.helper_service.v1".into(),
        helper_id: "local_inbox_helper".into(),
        helper_kind: "local_inbox_helper".into(),
        attached_tool_kind: None,
        attached_project_name: Some(project_name.to_string()),
        purpose: purpose.into(),
        entrypoint: "helpers/local_inbox_helper/README.md".into(),
        working_directory: ".".into(),
        input_paths: vec!["bridge/incoming_assets/module_assets".into()],
        output_paths: vec![
            "bridge/helpers/local_inbox/processed".into(),
            "bridge/helpers/local_inbox/summary.json".into(),
        ],
        status_paths: vec!["bridge/helpers/local_inbox/status.json".into()],
        launch_policy: Some(HelperLaunchPolicy {
            helper_id: "local_inbox_helper".into(),
            helper_kind: "local_inbox_helper".into(),
            allowed_root: ".".into(),
            program: "host_local_inbox_helper".into(),
            args: vec![],
            working_directory: Some(".".into()),
            status_paths: vec!["bridge/helpers/local_inbox/status.json".into()],
            expected_files: vec![
                "bridge/incoming_assets/module_assets".into(),
                "bridge/incoming_assets/module_assets/sample-inbox-note.txt".into(),
                "bridge/helpers/local_inbox/status.json".into(),
                "bridge/helpers/local_inbox/summary.json".into(),
            ],
            created_at: None,
        }),
        allowed_extensions: Vec::new(),
        lane_allowed_extensions: Default::default(),
        expected_files: vec![
            "helpers/local_inbox_helper/README.md".into(),
            "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
            "bridge/incoming_assets/module_assets/sample-inbox-note.txt".into(),
            "bridge/helpers/local_inbox/status.json".into(),
            "bridge/helpers/local_inbox/summary.json".into(),
            "bridge/helpers/local_inbox/processed/sample-inbox-note.txt".into(),
        ],
        primitives: vec![
            HelperPrimitiveSpec {
                primitive_id: "module_assets_inbox_lane".into(),
                primitive_kind: "inbox_lane".into(),
                purpose: "Observe the primary module asset inbox lane.".into(),
                input_paths: vec!["bridge/incoming_assets/module_assets".into()],
                output_paths: Vec::new(),
                status_paths: Vec::new(),
                dependency_mode: "standalone".into(),
                requires_primitives: Vec::new(),
                notes: vec![
                    "Seeds the bounded helper with the primary incoming asset lane.".into(),
                ],
                created_at: None,
            },
            HelperPrimitiveSpec {
                primitive_id: "local_inbox_processed_output".into(),
                primitive_kind: "processed_output".into(),
                purpose: "Emit bounded processed helper artifacts for accepted inbox files.".into(),
                input_paths: vec!["bridge/incoming_assets/module_assets".into()],
                output_paths: vec!["bridge/helpers/local_inbox/processed".into()],
                status_paths: Vec::new(),
                dependency_mode: "requires_primitives".into(),
                requires_primitives: vec!["module_assets_inbox_lane".into()],
                notes: vec![
                    "Copies accepted helper inputs into the deterministic processed output root."
                        .into(),
                ],
                created_at: None,
            },
            HelperPrimitiveSpec {
                primitive_id: "local_inbox_summary_snapshot".into(),
                primitive_kind: "summary_emitter".into(),
                purpose:
                    "Write the deterministic helper summary snapshot for bounded inbox processing."
                        .into(),
                input_paths: vec!["bridge/incoming_assets/module_assets".into()],
                output_paths: vec!["bridge/helpers/local_inbox/summary.json".into()],
                status_paths: Vec::new(),
                dependency_mode: "requires_primitives".into(),
                requires_primitives: vec![
                    "module_assets_inbox_lane".into(),
                    "local_inbox_processed_output".into(),
                ],
                notes: vec![
                    "Summarizes accepted, discovered, and filtered helper files for host/UI use."
                        .into(),
                ],
                created_at: None,
            },
            HelperPrimitiveSpec {
                primitive_id: "local_inbox_status_snapshot".into(),
                primitive_kind: "status_reporter".into(),
                purpose: "Write the compact helper status snapshot for bounded host supervision."
                    .into(),
                input_paths: vec!["bridge/incoming_assets/module_assets".into()],
                output_paths: Vec::new(),
                status_paths: vec!["bridge/helpers/local_inbox/status.json".into()],
                dependency_mode: "requires_primitives".into(),
                requires_primitives: vec![
                    "module_assets_inbox_lane".into(),
                    "local_inbox_processed_output".into(),
                    "local_inbox_summary_snapshot".into(),
                ],
                notes: vec![
                    "Provides a compact current-state contract for the bounded helper runner."
                        .into(),
                ],
                created_at: None,
            },
        ],
        notes: vec![
            attachment_note.into(),
            "Launch/runtime behavior is host-supervised through a bounded local helper runner."
                .into(),
        ],
        created_at: None,
    };
    let helper_summary_json = serde_json::to_string_pretty(&serde_json::json!({
        "helper_id": "local_inbox_helper",
        "status": "pending_host_run",
        "updated_at": helper_status_snapshot.updated_at,
        "observed_lane": "module_assets",
        "observed_file_count": 0,
        "discovered_file_count": 0,
        "filtered_out_file_count": 0,
        "allowed_extensions": [],
        "lane_allowed_extensions": {},
        "observed_primitive_ids": helper_status_snapshot.observed_primitive_ids,
        "notes": [
            "This file is a deterministic placeholder before the host helper runner executes.",
            "The bounded helper runner will replace this with an observed inbox summary."
        ]
    }))?;
    let helper_readme = format!(
        "# Local Inbox Helper\n\nThis is a bounded helper/service bundle for ChattyFactory.\n\nPurpose:\n- observe the module asset inbox lane\n- emit deterministic helper status and summary artifacts\n- provide a bounded seam for host-supervised helper runtime work\n\nAttached surface:\n- {attachment_note}\n"
    );

    Ok(LocalInboxHelperBundle {
        helper_service,
        helper_status_snapshot,
        helper_summary_json,
        helper_readme,
    })
}

pub fn build_static_web_seed(
    output_root: &Path,
    inputs: &BuildSeedInputs,
) -> Result<BuildArtifacts> {
    if inputs.project_name.trim().is_empty() {
        bail!("project_name is required");
    }

    let project_dir = output_root.join(&inputs.project_name);
    fs::create_dir_all(&project_dir)?;

    let bundle = render_static_web_dashboard_bundle(inputs)?;
    let helper_bundle = if request_wants_local_inbox_helper(inputs) {
        Some(build_local_inbox_helper_bundle(
            &inputs.project_name,
            "Observe a local inbox lane and emit deterministic helper status/output artifacts for a static dashboard.",
            "Cross-surface helper primitive bundle attached to a static web dashboard.",
        )?)
    } else {
        None
    };

    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        substrate: "static_web".into(),
        tool_kind: None,
        request_summary: Some(inputs.summary.clone()),
        entrypoints: vec!["index.html".into(), "app.js".into(), "styles.css".into()],
        expected_files: {
            let mut files = vec![
                "index.html".into(),
                "app.js".into(),
                "styles.css".into(),
                "README.md".into(),
                "ProjectSpec.json".into(),
                "AcceptancePlan.json".into(),
            ];
            if let Some(helper_bundle) = &helper_bundle {
                files.extend(helper_bundle.helper_service.expected_files.clone());
                ensure_marker(&mut files, "bridge/incoming_assets/module_assets/.keep");
                ensure_marker(
                    &mut files,
                    "bridge/helpers/local_inbox/processed/sample-inbox-note.txt",
                );
            }
            files
        },
        features: inputs.feature_tokens.clone(),
        acceptance_commands: vec!["static_file_check".into()],
        acceptance_checks: {
            let mut checks = vec![
                AcceptanceCheck {
                    check_id: "file-index".into(),
                    kind: "exists".into(),
                    target: "index.html".into(),
                    expected: None,
                },
                AcceptanceCheck {
                    check_id: "file-app".into(),
                    kind: "exists".into(),
                    target: "app.js".into(),
                    expected: None,
                },
                AcceptanceCheck {
                    check_id: "file-readme".into(),
                    kind: "exists".into(),
                    target: "README.md".into(),
                    expected: None,
                },
                AcceptanceCheck {
                    check_id: "marker-substrate".into(),
                    kind: "contains".into(),
                    target: "index.html".into(),
                    expected: Some("data-substrate=\"static_web\"".into()),
                },
            ];
            if let Some(helper_bundle) = &helper_bundle {
                checks.push(AcceptanceCheck {
                    check_id: "helper-spec".into(),
                    kind: "helper_service_spec".into(),
                    target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
                    expected: Some(helper_bundle.helper_service.helper_id.clone()),
                });
                checks.push(AcceptanceCheck {
                    check_id: "helper-status".into(),
                    kind: "helper_status_snapshot".into(),
                    target: "bridge/helpers/local_inbox/status.json".into(),
                    expected: Some("completed".into()),
                });
                checks.push(AcceptanceCheck {
                    check_id: "helper-summary".into(),
                    kind: "helper_summary_snapshot".into(),
                    target: "bridge/helpers/local_inbox/summary.json".into(),
                    expected: Some("1".into()),
                });
                checks.push(AcceptanceCheck {
                    check_id: "static-dashboard-helper-monitoring-surface".into(),
                    kind: "static_dashboard_helper_monitoring_surface_contract".into(),
                    target: "index.html".into(),
                    expected: Some("summary,preview".into()),
                });
            }
            checks
        },
        wrapper_metadata: Vec::new(),
        chattycog_hosting_mode: None,
        chattycog_ui_owner: None,
        chattycog_bridge_capabilities: None,
        helper_services: helper_bundle
            .as_ref()
            .map(|bundle| vec![bundle.helper_service.clone()])
            .unwrap_or_default(),
        supported_patch_kinds: vec!["progress_banner".into()],
        patch_lanes: Vec::new(),
        acceptance_recipes: Vec::new(),
        operator_bundles: Vec::new(),
        updated_at: None,
    };
    let mut project_spec = project_spec;
    refresh_project_contract_views(&mut project_spec);

    let acceptance_plan = AcceptancePlan {
        acceptance_id: format!("acceptance-{}", inputs.project_name),
        request_id: format!("request-{}", inputs.project_name),
        checks: vec![
            AcceptanceCheck {
                check_id: "index-exists".into(),
                kind: "exists".into(),
                target: "index.html".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "js-exists".into(),
                kind: "exists".into(),
                target: "app.js".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "css-exists".into(),
                kind: "exists".into(),
                target: "styles.css".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "readme-exists".into(),
                kind: "exists".into(),
                target: "README.md".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "substrate-marker".into(),
                kind: "contains".into(),
                target: "index.html".into(),
                expected: Some("data-substrate=\"static_web\"".into()),
            },
            AcceptanceCheck {
                check_id: "asset-link".into(),
                kind: "contains".into(),
                target: "index.html".into(),
                expected: Some("app.js".into()),
            },
        ],
        required_files: {
            let mut files = vec![
                "index.html".into(),
                "app.js".into(),
                "styles.css".into(),
                "README.md".into(),
                "ProjectSpec.json".into(),
                "AcceptancePlan.json".into(),
            ];
            if let Some(helper_bundle) = &helper_bundle {
                files.extend(helper_bundle.helper_service.expected_files.clone());
                ensure_marker(&mut files, "bridge/incoming_assets/module_assets/.keep");
                ensure_marker(
                    &mut files,
                    "bridge/helpers/local_inbox/processed/sample-inbox-note.txt",
                );
            }
            files
        },
        required_markers: vec![
            "data-substrate=\"static_web\"".into(),
            "Dashboard build loaded.".into(),
        ],
        commands: vec!["static_file_check".into()],
        expected_outputs: vec!["index.html".into()],
        helper_checks: helper_bundle
            .as_ref()
            .map(|bundle| vec![bundle.helper_service.helper_id.clone()])
            .unwrap_or_default(),
        schema_checks: Vec::new(),
    };
    let mut acceptance_plan = acceptance_plan;
    if let Some(helper_bundle) = &helper_bundle {
        ensure_check(
            &mut acceptance_plan.checks,
            AcceptanceCheck {
                check_id: "helper-spec".into(),
                kind: "helper_service_spec".into(),
                target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
                expected: Some(helper_bundle.helper_service.helper_id.clone()),
            },
        );
        ensure_check(
            &mut acceptance_plan.checks,
            AcceptanceCheck {
                check_id: "helper-status".into(),
                kind: "helper_status_snapshot".into(),
                target: "bridge/helpers/local_inbox/status.json".into(),
                expected: Some("completed".into()),
            },
        );
        ensure_check(
            &mut acceptance_plan.checks,
            AcceptanceCheck {
                check_id: "helper-summary".into(),
                kind: "helper_summary_snapshot".into(),
                target: "bridge/helpers/local_inbox/summary.json".into(),
                expected: Some("1".into()),
            },
        );
        ensure_check(
            &mut acceptance_plan.checks,
            AcceptanceCheck {
                check_id: "static-dashboard-helper-monitoring-surface".into(),
                kind: "static_dashboard_helper_monitoring_surface_contract".into(),
                target: "index.html".into(),
                expected: Some("summary,preview".into()),
            },
        );
        ensure_marker(
            &mut acceptance_plan.required_markers,
            "helper-monitor-panel",
        );
        ensure_marker(
            &mut acceptance_plan.required_markers,
            "helper-preview-panel",
        );
    }

    let project_spec_json = to_string_pretty(&project_spec)?;
    let acceptance_plan_json = to_string_pretty(&acceptance_plan)?;

    let mut files = vec![
        ("index.html", bundle.index_html),
        ("app.js", bundle.app_js),
        ("styles.css", bundle.styles_css),
        ("README.md", bundle.readme_md),
        ("ProjectSpec.json", project_spec_json),
        ("AcceptancePlan.json", acceptance_plan_json),
    ];
    if let Some(helper_bundle) = helper_bundle {
        files.push((
            "bridge/incoming_assets/module_assets/.keep",
            "module_assets inbox lane\n".into(),
        ));
        files.push((
            "bridge/incoming_assets/module_assets/sample-inbox-note.txt",
            "helper inbox sample emitted by the rebuild\n".into(),
        ));
        files.push((
            "bridge/helpers/local_inbox/status.json",
            to_string_pretty(&helper_bundle.helper_status_snapshot)?,
        ));
        files.push((
            "bridge/helpers/local_inbox/summary.json",
            helper_bundle.helper_summary_json,
        ));
        files.push((
            "helpers/local_inbox_helper/README.md",
            helper_bundle.helper_readme,
        ));
        files.push((
            "helpers/local_inbox_helper/HelperServiceSpec.json",
            to_string_pretty(&helper_bundle.helper_service)?,
        ));
    }

    write_project_files(&project_dir, files)
}


pub fn build_python_cli_seed(
    output_root: &Path,
    inputs: &BuildSeedInputs,
) -> Result<BuildArtifacts> {
    if inputs.project_name.trim().is_empty() {
        bail!("project_name is required");
    }

    let project_dir = output_root.join(&inputs.project_name);
    fs::create_dir_all(&project_dir)?;

    let title = title_or_default(inputs, "ChattyFactory Python CLI");
    let summary = summary_or_default(
        inputs,
        "A deterministic Python CLI build emitted by the ChattyFactory rebuild.",
    );
    let tool_kind =
        config_value(&inputs.entrypoint_config, "tool_kind").unwrap_or("directory_audit");

    let fixture_files = if tool_kind == "file_sorter" {
        vec![
            (
                "fixtures/input/sample.txt",
                "sample text file generated by the rebuild\n".to_string(),
            ),
            ("fixtures/output/.gitkeep", "".to_string()),
        ]
    } else if tool_kind == "csv_report" {
        vec![(
            "fixtures/input/sample.csv",
            "name,score,team\nAda,98,red\nLinus,87,blue\nGrace,91,green\n".to_string(),
        )]
    } else if tool_kind == "log_summary" {
        vec![(
            "fixtures/input/app.log",
            "INFO startup complete\nWARN cache miss\nERROR database unavailable\nERROR retry failed\n".to_string(),
        )]
    } else if tool_kind == "text_stats" {
        vec![(
            "fixtures/input/sample.txt",
            "alpha beta\ncharlie delta\n".to_string(),
        )]
    } else if tool_kind == "directory_audit" {
        vec![
            (
                "fixtures/input/notes.txt",
                "build pipeline notes\n".to_string(),
            ),
            (
                "fixtures/input/script.py",
                "print('audit fixture')\n".to_string(),
            ),
            ("fixtures/input/readme.md", "# fixture\n".to_string()),
        ]
    } else {
        Vec::new()
    };

    let context = serde_json::json!({
        "title": title,
        "summary": summary,
        "tool_kind": tool_kind,
    });

    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        substrate: "python_cli".into(),
        tool_kind: Some(tool_kind.to_string()),
        request_summary: Some(inputs.summary.clone()),
        entrypoints: vec!["main.py".into()],
        expected_files: vec![
            "main.py".into(),
            "README.md".into(),
            "ProjectSpec.json".into(),
            "AcceptancePlan.json".into(),
        ],
        features: inputs.feature_tokens.clone(),
        acceptance_commands: if tool_kind == "file_sorter" {
            vec!["python_cli_sorter".into()]
        } else if tool_kind == "csv_report" {
            vec!["python_cli_csv_report".into()]
        } else if tool_kind == "log_summary" {
            vec!["python_cli_log_summary".into()]
        } else if tool_kind == "text_stats" {
            vec!["python_cli_text_stats".into()]
        } else {
            vec!["python_cli_directory_audit".into()]
        },
        acceptance_checks: vec![
            AcceptanceCheck {
                check_id: "main-exists".into(),
                kind: "exists".into(),
                target: "main.py".into(),
                expected: None,
            },
            if tool_kind == "file_sorter" {
                AcceptanceCheck {
                    check_id: "python-sorter-run".into(),
                    kind: "python_run_success".into(),
                    target: "main.py --input fixtures/input --output fixtures/output".into(),
                    expected: None,
                }
            } else if tool_kind == "csv_report" {
                AcceptanceCheck {
                    check_id: "python-csv-run".into(),
                    kind: "python_output_contains".into(),
                    target: "main.py --input fixtures/input/sample.csv".into(),
                    expected: Some("rows=3 columns=3 header=name,score,team".into()),
                }
            } else if tool_kind == "log_summary" {
                AcceptanceCheck {
                    check_id: "python-log-run".into(),
                    kind: "python_output_contains".into(),
                    target: "main.py --input fixtures/input/app.log".into(),
                    expected: Some("errors=2 warnings=1 infos=1".into()),
                }
            } else if tool_kind == "text_stats" {
                AcceptanceCheck {
                    check_id: "python-stats-run".into(),
                    kind: "python_output_contains".into(),
                    target: "main.py --input fixtures/input/sample.txt".into(),
                    expected: Some("lines=2 words=4 chars=25".into()),
                }
            } else {
                AcceptanceCheck {
                    check_id: "python-audit-run".into(),
                    kind: "python_output_contains".into(),
                    target: "main.py --input fixtures/input".into(),
                    expected: Some("files=3".into()),
                }
            },
        ],
        wrapper_metadata: Vec::new(),
        chattycog_hosting_mode: None,
        chattycog_ui_owner: None,
        chattycog_bridge_capabilities: None,
        helper_services: Vec::new(),
        supported_patch_kinds: if tool_kind == "csv_report" {
            vec![
                "json_export".into(),
                "column_filter".into(),
                "email_sender".into(),
            ]
        } else {
            Vec::new()
        },
        patch_lanes: Vec::new(),
        acceptance_recipes: Vec::new(),
        operator_bundles: Vec::new(),
        updated_at: None,
    };
    let mut project_spec = project_spec;
    refresh_project_contract_views(&mut project_spec);

    let acceptance_plan = AcceptancePlan {
        acceptance_id: format!("acceptance-{}", inputs.project_name),
        request_id: format!("request-{}", inputs.project_name),
        checks: vec![
            AcceptanceCheck {
                check_id: "main-exists".into(),
                kind: "exists".into(),
                target: "main.py".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "readme-exists".into(),
                kind: "exists".into(),
                target: "README.md".into(),
                expected: None,
            },
            if tool_kind == "file_sorter" {
                AcceptanceCheck {
                    check_id: "sorter-run".into(),
                    kind: "python_run_success".into(),
                    target: "main.py --input fixtures/input --output fixtures/output".into(),
                    expected: None,
                }
            } else if tool_kind == "csv_report" {
                AcceptanceCheck {
                    check_id: "csv-run".into(),
                    kind: "python_output_contains".into(),
                    target: "main.py --input fixtures/input/sample.csv".into(),
                    expected: Some("rows=3 columns=3 header=name,score,team".into()),
                }
            } else if tool_kind == "log_summary" {
                AcceptanceCheck {
                    check_id: "log-run".into(),
                    kind: "python_output_contains".into(),
                    target: "main.py --input fixtures/input/app.log".into(),
                    expected: Some("errors=2 warnings=1 infos=1".into()),
                }
            } else if tool_kind == "text_stats" {
                AcceptanceCheck {
                    check_id: "stats-run".into(),
                    kind: "python_output_contains".into(),
                    target: "main.py --input fixtures/input/sample.txt".into(),
                    expected: Some("lines=2 words=4 chars=25".into()),
                }
            } else {
                AcceptanceCheck {
                    check_id: "audit-run".into(),
                    kind: "python_output_contains".into(),
                    target: "main.py --input fixtures/input".into(),
                    expected: Some("files=3".into()),
                }
            },
            if tool_kind == "file_sorter" {
                AcceptanceCheck {
                    check_id: "sorted-output".into(),
                    kind: "exists".into(),
                    target: "fixtures/output/txt/sample.txt".into(),
                    expected: None,
                }
            } else if tool_kind == "csv_report" {
                AcceptanceCheck {
                    check_id: "csv-marker".into(),
                    kind: "contains".into(),
                    target: "main.py".into(),
                    expected: Some("header=".into()),
                }
            } else if tool_kind == "log_summary" {
                AcceptanceCheck {
                    check_id: "log-marker".into(),
                    kind: "contains".into(),
                    target: "main.py".into(),
                    expected: Some("warnings=".into()),
                }
            } else if tool_kind == "text_stats" {
                AcceptanceCheck {
                    check_id: "stats-arg-marker".into(),
                    kind: "contains".into(),
                    target: "main.py".into(),
                    expected: Some("--input".into()),
                }
            } else {
                AcceptanceCheck {
                    check_id: "main-marker".into(),
                    kind: "contains".into(),
                    target: "main.py".into(),
                    expected: Some("ext:".into()),
                }
            },
        ],
        required_files: if tool_kind == "file_sorter" {
            vec![
                "main.py".into(),
                "README.md".into(),
                "fixtures/input/sample.txt".into(),
                "ProjectSpec.json".into(),
                "AcceptancePlan.json".into(),
            ]
        } else if tool_kind == "csv_report" {
            vec![
                "main.py".into(),
                "README.md".into(),
                "fixtures/input/sample.csv".into(),
                "ProjectSpec.json".into(),
                "AcceptancePlan.json".into(),
            ]
        } else if tool_kind == "log_summary" {
            vec![
                "main.py".into(),
                "README.md".into(),
                "fixtures/input/app.log".into(),
                "ProjectSpec.json".into(),
                "AcceptancePlan.json".into(),
            ]
        } else if tool_kind == "text_stats" {
            vec![
                "main.py".into(),
                "README.md".into(),
                "fixtures/input/sample.txt".into(),
                "ProjectSpec.json".into(),
                "AcceptancePlan.json".into(),
            ]
        } else {
            vec![
                "main.py".into(),
                "README.md".into(),
                "fixtures/input/notes.txt".into(),
                "fixtures/input/script.py".into(),
                "fixtures/input/readme.md".into(),
                "ProjectSpec.json".into(),
                "AcceptancePlan.json".into(),
            ]
        },
        required_markers: if tool_kind == "file_sorter" {
            vec!["--input".into(), "--output".into()]
        } else if tool_kind == "csv_report" {
            vec!["rows=".into(), "columns=".into(), "header=".into()]
        } else if tool_kind == "log_summary" {
            vec!["errors=".into(), "warnings=".into(), "infos=".into()]
        } else if tool_kind == "text_stats" {
            vec!["lines=".into(), "words=".into(), "chars=".into()]
        } else {
            vec!["files=".into(), "ext:".into()]
        },
        commands: if tool_kind == "file_sorter" {
            vec!["python_cli_sorter".into()]
        } else if tool_kind == "csv_report" {
            vec!["python_cli_csv_report".into()]
        } else if tool_kind == "log_summary" {
            vec!["python_cli_log_summary".into()]
        } else if tool_kind == "text_stats" {
            vec!["python_cli_text_stats".into()]
        } else {
            vec!["python_cli_directory_audit".into()]
        },
        expected_outputs: vec!["main.py".into()],
        helper_checks: Vec::new(),
        schema_checks: Vec::new(),
    };

    let mut project_spec = project_spec;
    refresh_project_contract_views(&mut project_spec);

    let mut files = vec![
        (
            "main.py",
            render_named("substrates/python_cli/main.py", &context)?,
        ),
        (
            "README.md",
            render_named("substrates/python_cli/README.md", &context)?,
        ),
        ("ProjectSpec.json", to_string_pretty(&project_spec)?),
        ("AcceptancePlan.json", to_string_pretty(&acceptance_plan)?),
    ];
    for (path, contents) in fixture_files {
        files.push((path, contents));
    }

    write_project_files(&project_dir, files)
}

pub fn build_rust_cli_seed(output_root: &Path, inputs: &BuildSeedInputs) -> Result<BuildArtifacts> {
    if inputs.project_name.trim().is_empty() {
        bail!("project_name is required");
    }

    let project_dir = output_root.join(&inputs.project_name);
    let src_dir = project_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let title = title_or_default(inputs, "ChattyFactory Rust CLI");
    let summary = summary_or_default(
        inputs,
        "A deterministic Rust CLI build emitted by the ChattyFactory rebuild.",
    );
    let package_name = inputs.project_name.replace('-', "_");
    let tool_kind =
        config_value(&inputs.entrypoint_config, "tool_kind").unwrap_or("directory_audit");

    let fixture_files = if tool_kind == "file_sorter" {
        vec![
            (
                "fixtures/input/sample.txt",
                "sample text file generated by the rebuild\n".to_string(),
            ),
            ("fixtures/output/.gitkeep", "".to_string()),
        ]
    } else if tool_kind == "csv_report" {
        vec![(
            "fixtures/input/sample.csv",
            "name,score,team\nAda,98,red\nLinus,87,blue\nGrace,91,green\n".to_string(),
        )]
    } else if tool_kind == "log_summary" {
        vec![(
            "fixtures/input/app.log",
            "INFO startup complete\nWARN cache miss\nERROR database unavailable\nERROR retry failed\n".to_string(),
        )]
    } else if tool_kind == "text_stats" {
        vec![(
            "fixtures/input/sample.txt",
            "alpha beta\ncharlie delta\n".to_string(),
        )]
    } else if tool_kind == "directory_audit" {
        vec![
            (
                "fixtures/input/notes.txt",
                "build pipeline notes\n".to_string(),
            ),
            ("fixtures/input/script.py", "fn main() {}\n".to_string()),
            ("fixtures/input/readme.md", "# fixture\n".to_string()),
        ]
    } else {
        Vec::new()
    };

    let context = serde_json::json!({
        "title": title,
        "summary": summary,
        "package_name": package_name,
        "tool_kind": tool_kind,
    });
    let helper_bundle = if request_wants_local_inbox_helper(inputs) {
        Some(build_local_inbox_helper_bundle(
            &inputs.project_name,
            "Observe a local inbox lane and emit deterministic helper status/output artifacts for a Rust CLI surface.",
            "Cross-surface helper primitive bundle attached to a Rust CLI surface.",
        )?)
    } else {
        None
    };

    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        substrate: "rust_cli".into(),
        tool_kind: Some(tool_kind.to_string()),
        request_summary: Some(inputs.summary.clone()),
        entrypoints: vec!["Cargo.toml".into(), "src/main.rs".into()],
        expected_files: {
            let mut files = vec![
                "Cargo.toml".into(),
                "src/main.rs".into(),
                "README.md".into(),
                "ProjectSpec.json".into(),
                "AcceptancePlan.json".into(),
            ];
            if let Some(helper_bundle) = &helper_bundle {
                files.extend(helper_bundle.helper_service.expected_files.clone());
                ensure_marker(&mut files, "bridge/incoming_assets/module_assets/.keep");
                ensure_marker(
                    &mut files,
                    "bridge/helpers/local_inbox/processed/sample-inbox-note.txt",
                );
            }
            files
        },
        features: inputs.feature_tokens.clone(),
        acceptance_commands: if tool_kind == "file_sorter" {
            vec!["cargo_check".into(), "cargo_run_sorter".into()]
        } else if tool_kind == "csv_report" {
            vec!["cargo_check".into(), "cargo_run_csv_report".into()]
        } else if tool_kind == "log_summary" {
            vec!["cargo_check".into(), "cargo_run_log_summary".into()]
        } else if tool_kind == "text_stats" {
            vec!["cargo_check".into(), "cargo_run_text_stats".into()]
        } else {
            vec!["cargo_check".into(), "cargo_run_directory_audit".into()]
        },
        acceptance_checks: vec![
            AcceptanceCheck {
                check_id: "cargo-manifest".into(),
                kind: "exists".into(),
                target: "Cargo.toml".into(),
                expected: None,
            },
            if tool_kind == "file_sorter" {
                AcceptanceCheck {
                    check_id: "cargo-sorter-run".into(),
                    kind: "cargo_run_success".into(),
                    target: "-- --input fixtures/input --output fixtures/output".into(),
                    expected: None,
                }
            } else if tool_kind == "csv_report" {
                AcceptanceCheck {
                    check_id: "cargo-csv-run".into(),
                    kind: "cargo_run_output_contains".into(),
                    target: "-- --input fixtures/input/sample.csv".into(),
                    expected: Some("rows=3 columns=3 header=name,score,team".into()),
                }
            } else if tool_kind == "log_summary" {
                AcceptanceCheck {
                    check_id: "cargo-log-run".into(),
                    kind: "cargo_run_output_contains".into(),
                    target: "-- --input fixtures/input/app.log".into(),
                    expected: Some("errors=2 warnings=1 infos=1".into()),
                }
            } else if tool_kind == "text_stats" {
                AcceptanceCheck {
                    check_id: "cargo-stats-run".into(),
                    kind: "cargo_run_output_contains".into(),
                    target: "-- --input fixtures/input/sample.txt".into(),
                    expected: Some("lines=2 words=4 chars=25".into()),
                }
            } else {
                AcceptanceCheck {
                    check_id: "cargo-audit-run".into(),
                    kind: "cargo_run_output_contains".into(),
                    target: "-- --input fixtures/input".into(),
                    expected: Some("files=3".into()),
                }
            },
        ],
        wrapper_metadata: Vec::new(),
        chattycog_hosting_mode: None,
        chattycog_ui_owner: None,
        chattycog_bridge_capabilities: None,
        helper_services: helper_bundle
            .as_ref()
            .map(|bundle| vec![bundle.helper_service.clone()])
            .unwrap_or_default(),
        supported_patch_kinds: if tool_kind == "log_summary" {
            vec![
                "file_output".into(),
                "severity_filter".into(),
                "json_output".into(),
            ]
        } else {
            Vec::new()
        },
        patch_lanes: Vec::new(),
        acceptance_recipes: Vec::new(),
        operator_bundles: Vec::new(),
        updated_at: None,
    };
    let mut project_spec = project_spec;
    if let Some(helper_bundle) = &helper_bundle {
        ensure_check(
            &mut project_spec.acceptance_checks,
            AcceptanceCheck {
                check_id: "helper-spec".into(),
                kind: "helper_service_spec".into(),
                target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
                expected: Some(helper_bundle.helper_service.helper_id.clone()),
            },
        );
        ensure_check(
            &mut project_spec.acceptance_checks,
            AcceptanceCheck {
                check_id: "helper-status".into(),
                kind: "helper_status_snapshot".into(),
                target: "bridge/helpers/local_inbox/status.json".into(),
                expected: Some("completed".into()),
            },
        );
        ensure_check(
            &mut project_spec.acceptance_checks,
            AcceptanceCheck {
                check_id: "helper-summary".into(),
                kind: "helper_summary_snapshot".into(),
                target: "bridge/helpers/local_inbox/summary.json".into(),
                expected: Some("1".into()),
            },
        );
    }
    refresh_project_contract_views(&mut project_spec);

    let acceptance_plan = AcceptancePlan {
        acceptance_id: format!("acceptance-{}", inputs.project_name),
        request_id: format!("request-{}", inputs.project_name),
        checks: vec![
            AcceptanceCheck {
                check_id: "cargo-exists".into(),
                kind: "exists".into(),
                target: "Cargo.toml".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "main-exists".into(),
                kind: "exists".into(),
                target: "src/main.rs".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "cargo-check".into(),
                kind: "cargo_check".into(),
                target: ".".into(),
                expected: None,
            },
            if tool_kind == "file_sorter" {
                AcceptanceCheck {
                    check_id: "cargo-sorter-run".into(),
                    kind: "cargo_run_success".into(),
                    target: "-- --input fixtures/input --output fixtures/output".into(),
                    expected: None,
                }
            } else if tool_kind == "csv_report" {
                AcceptanceCheck {
                    check_id: "cargo-csv-run".into(),
                    kind: "cargo_run_output_contains".into(),
                    target: "-- --input fixtures/input/sample.csv".into(),
                    expected: Some("rows=3 columns=3 header=name,score,team".into()),
                }
            } else if tool_kind == "log_summary" {
                AcceptanceCheck {
                    check_id: "cargo-log-run".into(),
                    kind: "cargo_run_output_contains".into(),
                    target: "-- --input fixtures/input/app.log".into(),
                    expected: Some("errors=2 warnings=1 infos=1".into()),
                }
            } else if tool_kind == "text_stats" {
                AcceptanceCheck {
                    check_id: "cargo-stats-run".into(),
                    kind: "cargo_run_output_contains".into(),
                    target: "-- --input fixtures/input/sample.txt".into(),
                    expected: Some("lines=2 words=4 chars=25".into()),
                }
            } else {
                AcceptanceCheck {
                    check_id: "cargo-audit-run".into(),
                    kind: "cargo_run_output_contains".into(),
                    target: "-- --input fixtures/input".into(),
                    expected: Some("files=3".into()),
                }
            },
            if tool_kind == "file_sorter" {
                AcceptanceCheck {
                    check_id: "cargo-sorted-output".into(),
                    kind: "exists".into(),
                    target: "fixtures/output/txt/sample.txt".into(),
                    expected: None,
                }
            } else if tool_kind == "csv_report" {
                AcceptanceCheck {
                    check_id: "cargo-csv-marker".into(),
                    kind: "contains".into(),
                    target: "src/main.rs".into(),
                    expected: Some("header=".into()),
                }
            } else if tool_kind == "log_summary" {
                AcceptanceCheck {
                    check_id: "cargo-log-marker".into(),
                    kind: "contains".into(),
                    target: "src/main.rs".into(),
                    expected: Some("warnings=".into()),
                }
            } else if tool_kind == "text_stats" {
                AcceptanceCheck {
                    check_id: "cargo-stats-marker".into(),
                    kind: "contains".into(),
                    target: "src/main.rs".into(),
                    expected: Some("--input".into()),
                }
            } else {
                AcceptanceCheck {
                    check_id: "cargo-main-marker".into(),
                    kind: "contains".into(),
                    target: "src/main.rs".into(),
                    expected: Some("ext:".into()),
                }
            },
        ],
        required_files: {
            let mut files = if tool_kind == "file_sorter" {
                vec![
                    "Cargo.toml".into(),
                    "src/main.rs".into(),
                    "README.md".into(),
                    "fixtures/input/sample.txt".into(),
                    "ProjectSpec.json".into(),
                    "AcceptancePlan.json".into(),
                ]
            } else if tool_kind == "csv_report" {
                vec![
                    "Cargo.toml".into(),
                    "src/main.rs".into(),
                    "README.md".into(),
                    "fixtures/input/sample.csv".into(),
                    "ProjectSpec.json".into(),
                    "AcceptancePlan.json".into(),
                ]
            } else if tool_kind == "log_summary" {
                vec![
                    "Cargo.toml".into(),
                    "src/main.rs".into(),
                    "README.md".into(),
                    "fixtures/input/app.log".into(),
                    "ProjectSpec.json".into(),
                    "AcceptancePlan.json".into(),
                ]
            } else if tool_kind == "text_stats" {
                vec![
                    "Cargo.toml".into(),
                    "src/main.rs".into(),
                    "README.md".into(),
                    "fixtures/input/sample.txt".into(),
                    "ProjectSpec.json".into(),
                    "AcceptancePlan.json".into(),
                ]
            } else {
                vec![
                    "Cargo.toml".into(),
                    "src/main.rs".into(),
                    "README.md".into(),
                    "fixtures/input/notes.txt".into(),
                    "fixtures/input/script.py".into(),
                    "fixtures/input/readme.md".into(),
                    "ProjectSpec.json".into(),
                    "AcceptancePlan.json".into(),
                ]
            };
            if let Some(helper_bundle) = &helper_bundle {
                files.extend(helper_bundle.helper_service.expected_files.clone());
                ensure_marker(&mut files, "bridge/incoming_assets/module_assets/.keep");
                ensure_marker(
                    &mut files,
                    "bridge/helpers/local_inbox/processed/sample-inbox-note.txt",
                );
            }
            files
        },
        required_markers: if tool_kind == "file_sorter" {
            vec!["--input".into(), "--output".into()]
        } else if tool_kind == "csv_report" {
            vec!["rows=".into(), "columns=".into(), "header=".into()]
        } else if tool_kind == "log_summary" {
            vec!["errors=".into(), "warnings=".into(), "infos=".into()]
        } else if tool_kind == "text_stats" {
            vec!["lines=".into(), "words=".into(), "chars=".into()]
        } else {
            vec!["files=".into(), "ext:".into()]
        },
        commands: if tool_kind == "file_sorter" {
            vec!["cargo_run_sorter".into()]
        } else if tool_kind == "csv_report" {
            vec!["cargo_check".into(), "cargo_run_csv_report".into()]
        } else if tool_kind == "log_summary" {
            vec!["cargo_check".into(), "cargo_run_log_summary".into()]
        } else if tool_kind == "text_stats" {
            vec!["cargo_run_text_stats".into()]
        } else {
            vec!["cargo_check".into(), "cargo_run_directory_audit".into()]
        },
        expected_outputs: vec!["src/main.rs".into()],
        helper_checks: helper_bundle
            .as_ref()
            .map(|bundle| vec![bundle.helper_service.helper_id.clone()])
            .unwrap_or_default(),
        schema_checks: Vec::new(),
    };
    let mut acceptance_plan = acceptance_plan;
    if let Some(helper_bundle) = &helper_bundle {
        ensure_check(
            &mut acceptance_plan.checks,
            AcceptanceCheck {
                check_id: "helper-spec".into(),
                kind: "helper_service_spec".into(),
                target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
                expected: Some(helper_bundle.helper_service.helper_id.clone()),
            },
        );
        ensure_check(
            &mut acceptance_plan.checks,
            AcceptanceCheck {
                check_id: "helper-status".into(),
                kind: "helper_status_snapshot".into(),
                target: "bridge/helpers/local_inbox/status.json".into(),
                expected: Some("completed".into()),
            },
        );
        ensure_check(
            &mut acceptance_plan.checks,
            AcceptanceCheck {
                check_id: "helper-summary".into(),
                kind: "helper_summary_snapshot".into(),
                target: "bridge/helpers/local_inbox/summary.json".into(),
                expected: Some("1".into()),
            },
        );
    }

    let mut project_spec = project_spec;
    refresh_project_contract_views(&mut project_spec);

    let mut files = vec![
        (
            "Cargo.toml",
            render_named("substrates/rust_cli/Cargo.toml", &context)?,
        ),
        (
            "src/main.rs",
            render_named("substrates/rust_cli/main.rs", &context)?,
        ),
        (
            "README.md",
            render_named("substrates/rust_cli/README.md", &context)?,
        ),
        ("ProjectSpec.json", to_string_pretty(&project_spec)?),
        ("AcceptancePlan.json", to_string_pretty(&acceptance_plan)?),
    ];
    if let Some(helper_bundle) = helper_bundle {
        files.push((
            "bridge/incoming_assets/module_assets/.keep",
            "module_assets inbox lane\n".into(),
        ));
        files.push((
            "bridge/incoming_assets/module_assets/sample-inbox-note.txt",
            "helper inbox sample emitted by the rebuild\n".into(),
        ));
        files.push((
            "bridge/helpers/local_inbox/status.json",
            to_string_pretty(&helper_bundle.helper_status_snapshot)?,
        ));
        files.push((
            "bridge/helpers/local_inbox/summary.json",
            helper_bundle.helper_summary_json,
        ));
        files.push((
            "helpers/local_inbox_helper/README.md",
            helper_bundle.helper_readme,
        ));
        files.push((
            "helpers/local_inbox_helper/HelperServiceSpec.json",
            to_string_pretty(&helper_bundle.helper_service)?,
        ));
    }
    for (path, contents) in fixture_files {
        files.push((path, contents));
    }

    write_project_files(&project_dir, files)
}

fn write_project_files(project_dir: &Path, files: Vec<(&str, String)>) -> Result<BuildArtifacts> {
    let mut emitted_files = Vec::new();
    for (name, contents) in files {
        let path = project_dir.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, contents)?;
        emitted_files.push(path);
    }

    let project_spec_path = project_dir.join("ProjectSpec.json");
    if project_spec_path.exists() {
        let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&project_spec_path)?)?;
        refresh_project_contract_views_for_project(Some(project_dir), &mut spec);
        fs::write(&project_spec_path, to_string_pretty(&spec)?)?;
    }

    Ok(BuildArtifacts {
        project_dir: project_dir.to_path_buf(),
        emitted_files,
    })
}

fn render_chattycog_visual_load_json(spec: &ChattyCogVisualLoadSpec) -> Result<String> {
    let mut value = serde_json::Map::new();
    value.insert("kind".into(), serde_json::Value::String(spec.kind.clone()));
    value.insert(
        "auto_launch".into(),
        serde_json::Value::Bool(spec.auto_launch),
    );
    if let Some(title) = &spec.title {
        value.insert("title".into(), serde_json::Value::String(title.clone()));
    }
    if let Some(notes) = &spec.notes {
        value.insert("notes".into(), serde_json::Value::String(notes.clone()));
    }
    if let Some(file) = &spec.file {
        value.insert("file".into(), serde_json::Value::String(file.clone()));
    }
    if let Some(url) = &spec.url {
        value.insert("url".into(), serde_json::Value::String(url.clone()));
    }
    if let Some(window_title_contains) = &spec.window_title_contains {
        value.insert(
            "window_title_contains".into(),
            serde_json::Value::String(window_title_contains.clone()),
        );
    }
    if let Some(command) = &spec.build_command {
        value.insert("build".into(), serde_json::to_value(command)?);
    }
    if let Some(command) = &spec.launch_command {
        value.insert("launch".into(), serde_json::to_value(command)?);
    }
    if let Some(command) = &spec.serve_command {
        value.insert("serve".into(), serde_json::to_value(command)?);
    }
    if let Some(wait_ms) = spec.serve_wait_ms {
        value.insert(
            "serve_wait_ms".into(),
            serde_json::Value::Number(wait_ms.into()),
        );
    }
    Ok(to_string_pretty(&serde_json::Value::Object(value))?)
}

pub fn build_receipt(
    request_id: &str,
    inputs: &BuildSeedInputs,
    artifacts: &BuildArtifacts,
) -> BuildReceipt {
    BuildReceipt {
        receipt_id: format!("build-{}", request_id),
        request_id: request_id.to_string(),
        project_name: inputs.project_name.clone(),
        project_dir: artifacts.project_dir.display().to_string(),
        substrate: config_value(&inputs.entrypoint_config, "substrate")
            .unwrap_or("unknown_substrate")
            .to_string(),
        tool_kind: config_value(&inputs.entrypoint_config, "tool_kind").map(|v| v.to_string()),
        emitted_files: artifacts
            .emitted_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        created_at: None,
    }
}

pub fn apply_request_plan_enrichments(project_dir: &Path, plan: &RequestPlan) -> Result<()> {
    if plan.planner_acceptance_checks.is_empty()
        && plan.planner_required_markers.is_empty()
        && plan.planner_acceptance_commands.is_empty()
        && plan.planner_expected_outputs.is_empty()
        && plan.planner_suggested_patch_kinds.is_empty()
        && plan.planner_suggested_features.is_empty()
        && plan.planner_operator_ids.is_empty()
        && plan.planner_acceptance_recipe_ids.is_empty()
    {
        return Ok(());
    }

    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");
    if !spec_path.exists() || !acceptance_path.exists() {
        bail!("planner enrichments require ProjectSpec.json and AcceptancePlan.json");
    }

    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    apply_operator_contributions(&mut spec, &mut acceptance, &plan.planner_operator_ids);
    apply_acceptance_recipe_contributions(
        &mut spec,
        &mut acceptance,
        &plan.planner_acceptance_recipe_ids,
    );

    for check in &plan.planner_acceptance_checks {
        ensure_check(&mut spec.acceptance_checks, check.clone());
        ensure_check(&mut acceptance.checks, check.clone());
    }
    for marker in &plan.planner_required_markers {
        ensure_marker(&mut acceptance.required_markers, marker);
    }
    for command in &plan.planner_acceptance_commands {
        ensure_command(&mut spec.acceptance_commands, command);
        ensure_command(&mut acceptance.commands, command);
    }
    for output in &plan.planner_expected_outputs {
        ensure_marker(&mut acceptance.expected_outputs, output);
    }
    for patch_kind in &plan.planner_suggested_patch_kinds {
        ensure_marker(&mut spec.supported_patch_kinds, patch_kind);
    }
    for feature in &plan.planner_suggested_features {
        ensure_marker(&mut spec.features, feature);
    }

    refresh_project_contract_views(&mut spec);
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;
    Ok(())
}

pub fn dispatch_patch_request(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    raw_request: &str,
    spec: &ProjectSpec,
    preferred_patch_recipe_ids: &[String],
    preferred_patch_kind: Option<&str>,
) -> Result<Option<(PatchArtifacts, PatchReceipt)>> {
    let lower_request = raw_request.to_ascii_lowercase();
    let substrate_kind = registry_substrate_kind(spec);
    let tool_kind = spec.tool_kind.as_deref();
    let project_features = spec.features.as_slice();

    if let Some(out) = dispatch_patch_recipe_request(
        project_dir,
        project_name,
        request_id,
        raw_request,
        spec,
        preferred_patch_recipe_ids,
    )? {
        return Ok(Some(out));
    }

    if let (Some(substrate_kind), Some(patch_kind)) = (substrate_kind.as_deref(), preferred_patch_kind) {
        if let Some(recipe) =
            patch_recipe_by_kind(substrate_kind, tool_kind, project_features, patch_kind)
        {
            return Ok(Some((recipe.handler)(
                project_dir,
                project_name,
                request_id,
                raw_request,
            )?));
        }
    }

    if let Some(substrate_kind) = substrate_kind.as_deref() {
        if let Some(recipe) =
            patch_recipe_from_request_text(
                substrate_kind,
                tool_kind,
                project_features,
                &lower_request,
            )
        {
            return Ok(Some((recipe.handler)(
                project_dir,
                project_name,
                request_id,
                raw_request,
            )?));
        }
    }

    Ok(None)
}

fn dispatch_patch_recipe_request(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    raw_request: &str,
    spec: &ProjectSpec,
    preferred_patch_recipe_ids: &[String],
) -> Result<Option<(PatchArtifacts, PatchReceipt)>> {
    let Some(substrate_kind) = registry_substrate_kind(spec) else {
        return Ok(None);
    };
    for recipe_id in preferred_patch_recipe_ids {
        let project_features = spec.features.as_slice();
        for recipe in patch_recipe_registry() {
            if let Some(resolved) = patch_recipe_by_kind(
                substrate_kind.as_str(),
                spec.tool_kind.as_deref(),
                project_features,
                recipe.patch_kind,
            ) {
                if resolved.recipe_id == recipe_id {
                    let out =
                        (resolved.handler)(project_dir, project_name, request_id, raw_request)?;
                    if out.0.patch_kind == resolved.patch_kind {
                        return Ok(Some(out));
                    }
                }
            }
        }
    }

    Ok(None)
}

pub fn patch_static_web_dashboard_progress_banner(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("progress-banner") {
        let banner = "\n    <section class=\"panel progress-banner\" data-operator=\"progress_banner\">\n      <h2>Progress</h2>\n      <p id=\"progress-text\">Host-applied progress banner patch is active.</p>\n    </section>\n";
        let needle = "    <section class=\"panel results-panel\">";
        if let Some(pos) = index.find(needle) {
            index.insert_str(pos, banner);
        } else {
            bail!("results panel marker missing in index.html");
        }
    }

    if !styles.contains(".progress-banner") {
        styles.push_str(
            "\n.progress-banner {\n  border-style: dashed;\n  background: linear-gradient(135deg, rgba(198, 93, 46, 0.12), rgba(53, 92, 125, 0.08));\n}\n",
        );
    }

    if !spec
        .features
        .iter()
        .any(|feature| feature == "progress_banner")
    {
        spec.features.push("progress_banner".into());
    }
    if !spec
        .supported_patch_kinds
        .iter()
        .any(|kind| kind == "progress_banner")
    {
        spec.supported_patch_kinds.push("progress_banner".into());
    }
    if !spec
        .acceptance_checks
        .iter()
        .any(|check| check.check_id == "progress-banner-marker")
    {
        spec.acceptance_checks.push(AcceptanceCheck {
            check_id: "progress-banner-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("progress-banner".into()),
        });
    }

    if !acceptance
        .checks
        .iter()
        .any(|check| check.check_id == "progress-banner-marker")
    {
        acceptance.checks.push(AcceptanceCheck {
            check_id: "progress-banner-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("progress-banner".into()),
        });
    }
    if !acceptance
        .required_markers
        .iter()
        .any(|marker| marker == "progress-banner")
    {
        acceptance.required_markers.push("progress-banner".into());
    }

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        project_name: project_name.to_string(),
        substrate: spec.substrate.clone(),
        patch_kind: "progress_banner".into(),
        request_summary: request_summary.to_string(),
        modified_files: modified_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        created_at: None,
    };

    Ok((
        PatchArtifacts {
            project_dir: project_dir.to_path_buf(),
            modified_files,
            patch_kind: "progress_banner".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_python_csv_report_json_export(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let main_path = project_dir.join("main.py");
    let readme_path = project_dir.join("README.md");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut main = fs::read_to_string(&main_path)?;
    let mut readme = fs::read_to_string(&readme_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !main.contains("import json") {
        main = main.replacen("import argparse\n", "import argparse\nimport json\n", 1);
    }
    if !main.contains("--json-out") {
        main = main.replacen(
            "    parser.add_argument(\"--input\", required=True, help=\"csv file to inspect\")\n",
            "    parser.add_argument(\"--input\", required=True, help=\"csv file to inspect\")\n    parser.add_argument(\"--json-out\", type=Path, help=\"optional json output path\")\n",
            1,
        );
    }
    if !main.contains("\"header\": header") {
        let old = "    if not rows:\n        print(\"rows=0 columns=0 header=\")\n        return 0\n    header = rows[0].split(\",\")\n    data_rows = rows[1:]\n    print(\n        f\"rows={len(data_rows)} columns={len(header)} header={','.join(part.strip() for part in header)}\"\n    )\n    return 0\n";
        let new = "    if not rows:\n        summary = {\"rows\": 0, \"columns\": 0, \"header\": []}\n        if args.json_out:\n            args.json_out.parent.mkdir(parents=True, exist_ok=True)\n            args.json_out.write_text(json.dumps(summary, indent=2), encoding=\"utf-8\")\n        print(\"rows=0 columns=0 header=\")\n        return 0\n    header = [part.strip() for part in rows[0].split(\",\")]\n    data_rows = rows[1:]\n    summary = {\"rows\": len(data_rows), \"columns\": len(header), \"header\": header}\n    if args.json_out:\n        args.json_out.parent.mkdir(parents=True, exist_ok=True)\n        args.json_out.write_text(json.dumps(summary, indent=2), encoding=\"utf-8\")\n    print(\n        f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    )\n    return 0\n";
        if main.contains(old) {
            main = main.replacen(old, new, 1);
        } else {
            bail!("csv_report main.py did not match expected patch shape");
        }
    }

    if !readme.contains("--json-out") {
        readme.push_str(
            "\n## Patch Lane\n\n`python main.py --input fixtures/input/sample.csv --json-out fixtures/output/report.json`\n\nThis writes the computed CSV summary to `fixtures/output/report.json`.\n",
        );
    }

    if !spec.features.iter().any(|feature| feature == "json_export") {
        spec.features.push("json_export".into());
    }
    ensure_marker(&mut spec.supported_patch_kinds, "json_export");
    ensure_marker(&mut spec.supported_patch_kinds, "column_filter");
    ensure_marker(&mut spec.supported_patch_kinds, "email_sender");
    if !spec
        .acceptance_checks
        .iter()
        .any(|check| check.check_id == "csv-json-export-run")
    {
        spec.acceptance_checks.push(AcceptanceCheck {
            check_id: "csv-json-export-run".into(),
            kind: "python_run_success".into(),
            target:
                "main.py --input fixtures/input/sample.csv --json-out fixtures/output/report.json"
                    .into(),
            expected: None,
        });
    }
    if !spec
        .acceptance_commands
        .iter()
        .any(|cmd| cmd == "python_cli_csv_report_json_export")
    {
        spec.acceptance_commands
            .push("python_cli_csv_report_json_export".into());
    }

    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "csv-json-export-run".into(),
            kind: "python_run_success".into(),
            target:
                "main.py --input fixtures/input/sample.csv --json-out fixtures/output/report.json"
                    .into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "csv-json-export-file".into(),
            kind: "exists".into(),
            target: "fixtures/output/report.json".into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "csv-json-export-marker".into(),
            kind: "contains".into(),
            target: "main.py".into(),
            expected: Some("--json-out".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "--json-out");
    ensure_command(
        &mut acceptance.commands,
        "python_cli_csv_report_json_export",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&main_path, main)?;
    fs::write(&readme_path, readme)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        main_path.clone(),
        readme_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        project_name: project_name.to_string(),
        substrate: spec.substrate.clone(),
        patch_kind: "json_export".into(),
        request_summary: request_summary.to_string(),
        modified_files: modified_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        created_at: None,
    };

    Ok((
        PatchArtifacts {
            project_dir: project_dir.to_path_buf(),
            modified_files,
            patch_kind: "json_export".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_python_csv_report_column_filter(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let main_path = project_dir.join("main.py");
    let readme_path = project_dir.join("README.md");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut main = fs::read_to_string(&main_path)?;
    let mut readme = fs::read_to_string(&readme_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !main.contains("--column") {
        main = main.replacen(
            "    parser.add_argument(\"--input\", required=True, help=\"csv file to inspect\")\n",
            "    parser.add_argument(\"--input\", required=True, help=\"csv file to inspect\")\n    parser.add_argument(\"--column\", help=\"optional column name to focus on\")\n",
            1,
        );
    }
    if !main.contains("column_name = args.column.strip()") {
        let old_base = "    if not rows:\n        print(\"rows=0 columns=0 header=\")\n        return 0\n    header = rows[0].split(\",\")\n    data_rows = rows[1:]\n    print(\n        f\"rows={len(data_rows)} columns={len(header)} header={','.join(part.strip() for part in header)}\"\n    )\n    return 0\n";
        let new_base = "    if not rows:\n        print(\"rows=0 columns=0 header=\")\n        return 0\n    header = [part.strip() for part in rows[0].split(\",\")]\n    data_rows = rows[1:]\n    summary = {\"rows\": len(data_rows), \"columns\": len(header), \"header\": header}\n    if args.column:\n        column_name = args.column.strip()\n        if column_name not in header:\n            raise SystemExit(f\"unknown column: {column_name}\")\n        index = header.index(column_name)\n        values = []\n        for row in data_rows:\n            cells = [part.strip() for part in row.split(\",\")]\n            if index < len(cells) and cells[index]:\n                values.append(cells[index])\n        focused = f\"column={column_name} values={','.join(values)}\"\n        print(focused)\n        return 0\n    print(\n        f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    )\n    return 0\n";
        let old_json = "    header = [part.strip() for part in rows[0].split(\",\")]\n    data_rows = rows[1:]\n    summary = {\"rows\": len(data_rows), \"columns\": len(header), \"header\": header}\n    if args.json_out:\n        args.json_out.parent.mkdir(parents=True, exist_ok=True)\n        args.json_out.write_text(json.dumps(summary, indent=2), encoding=\"utf-8\")\n    print(\n        f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    )\n    return 0\n";
        let new_json = "    header = [part.strip() for part in rows[0].split(\",\")]\n    data_rows = rows[1:]\n    summary = {\"rows\": len(data_rows), \"columns\": len(header), \"header\": header}\n    if args.column:\n        column_name = args.column.strip()\n        if column_name not in header:\n            raise SystemExit(f\"unknown column: {column_name}\")\n        index = header.index(column_name)\n        values = []\n        for row in data_rows:\n            cells = [part.strip() for part in row.split(\",\")]\n            if index < len(cells) and cells[index]:\n                values.append(cells[index])\n        focused = f\"column={column_name} values={','.join(values)}\"\n        if args.json_out:\n            args.json_out.parent.mkdir(parents=True, exist_ok=True)\n            args.json_out.write_text(json.dumps({**summary, \"column\": column_name, \"values\": values}, indent=2), encoding=\"utf-8\")\n        print(focused)\n        return 0\n    if args.json_out:\n        args.json_out.parent.mkdir(parents=True, exist_ok=True)\n        args.json_out.write_text(json.dumps(summary, indent=2), encoding=\"utf-8\")\n    print(\n        f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    )\n    return 0\n";
        if main.contains(old_base) {
            main = main.replacen(old_base, new_base, 1);
        } else if main.contains(old_json) {
            main = main.replacen(old_json, new_json, 1);
        } else if !main.contains("focused = f\"column={column_name} values={','.join(values)}\"") {
            bail!("csv_report main.py did not match expected column filter patch shape");
        }
    }

    if !readme.contains("--column team") {
        readme.push_str(
            "\n## Column Filter\n\n`python main.py --input fixtures/input/sample.csv --column team`\n\nThis prints only the values from the selected CSV column.\n",
        );
    }

    if !spec
        .features
        .iter()
        .any(|feature| feature == "column_filter")
    {
        spec.features.push("column_filter".into());
    }
    ensure_marker(&mut spec.supported_patch_kinds, "json_export");
    ensure_marker(&mut spec.supported_patch_kinds, "column_filter");
    ensure_marker(&mut spec.supported_patch_kinds, "email_sender");
    if !spec
        .acceptance_checks
        .iter()
        .any(|check| check.check_id == "csv-column-filter-run")
    {
        spec.acceptance_checks.push(AcceptanceCheck {
            check_id: "csv-column-filter-run".into(),
            kind: "python_output_contains".into(),
            target: "main.py --input fixtures/input/sample.csv --column team".into(),
            expected: Some("column=team values=red,blue,green".into()),
        });
    }
    if !spec
        .acceptance_commands
        .iter()
        .any(|cmd| cmd == "python_cli_csv_report_column_filter")
    {
        spec.acceptance_commands
            .push("python_cli_csv_report_column_filter".into());
    }

    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "csv-column-filter-run".into(),
            kind: "python_output_contains".into(),
            target: "main.py --input fixtures/input/sample.csv --column team".into(),
            expected: Some("column=team values=red,blue,green".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "csv-column-filter-marker".into(),
            kind: "contains".into(),
            target: "main.py".into(),
            expected: Some("--column".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "--column");
    ensure_command(
        &mut acceptance.commands,
        "python_cli_csv_report_column_filter",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&main_path, main)?;
    fs::write(&readme_path, readme)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        main_path.clone(),
        readme_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        project_name: project_name.to_string(),
        substrate: spec.substrate.clone(),
        patch_kind: "column_filter".into(),
        request_summary: request_summary.to_string(),
        modified_files: modified_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        created_at: None,
    };

    Ok((
        PatchArtifacts {
            project_dir: project_dir.to_path_buf(),
            modified_files,
            patch_kind: "column_filter".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_python_csv_report_email_sender(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let main_path = project_dir.join("main.py");
    let readme_path = project_dir.join("README.md");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut main = fs::read_to_string(&main_path)?;
    let mut readme = fs::read_to_string(&readme_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !main.contains("--email-to") {
        main = main.replacen(
            "    return parser\n",
            "    parser.add_argument(\"--email-to\", help=\"email recipient for a generated report draft\")\n    parser.add_argument(\"--email-out\", type=Path, help=\"output path for a generated email draft\")\n    return parser\n",
            1,
        );
    }

    if !main.contains("def maybe_write_email(") {
        main = main.replacen(
            "\n\ndef main() -> int:\n",
            "\n\ndef maybe_write_email(email_out, email_to, subject: str, body: str) -> None:\n    if not email_out and not email_to:\n        return\n    if not email_out or not email_to:\n        raise SystemExit(\"--email-to and --email-out must be provided together\")\n    email_out.parent.mkdir(parents=True, exist_ok=True)\n    email_out.write_text(\n        f\"To: {email_to}\\nSubject: {subject}\\n\\n{body}\\n\",\n        encoding=\"utf-8\",\n    )\n\n\ndef main() -> int:\n",
            1,
        );
    }

    if !main.contains("maybe_write_email(args.email_out, args.email_to") {
        if main.contains(
            "    print(\n        f\"rows={len(data_rows)} columns={len(header)} header={','.join(part.strip() for part in header)}\"\n    )\n    return 0\n"
        ) {
            main = main.replacen(
                "    print(\n        f\"rows={len(data_rows)} columns={len(header)} header={','.join(part.strip() for part in header)}\"\n    )\n    return 0\n",
                "    report_line = f\"rows={len(data_rows)} columns={len(header)} header={','.join(part.strip() for part in header)}\"\n    maybe_write_email(args.email_out, args.email_to, \"CSV Report Summary\", report_line)\n    print(report_line)\n    return 0\n",
                1,
            );
        } else if main.contains(
            "        focused = f\"column={column_name} values={','.join(values)}\"\n        print(focused)\n        return 0\n    print(\n        f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    )\n    return 0\n"
        ) {
            main = main.replacen(
                "        focused = f\"column={column_name} values={','.join(values)}\"\n        print(focused)\n        return 0\n    print(\n        f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    )\n    return 0\n",
                "        focused = f\"column={column_name} values={','.join(values)}\"\n        maybe_write_email(args.email_out, args.email_to, \"CSV Report Column Summary\", focused)\n        print(focused)\n        return 0\n    report_line = f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    maybe_write_email(args.email_out, args.email_to, \"CSV Report Summary\", report_line)\n    print(report_line)\n    return 0\n",
                1,
            );
        } else if main.contains(
            "        focused = f\"column={column_name} values={','.join(values)}\"\n        if args.json_out:\n            args.json_out.parent.mkdir(parents=True, exist_ok=True)\n            args.json_out.write_text(json.dumps({**summary, \"column\": column_name, \"values\": values}, indent=2), encoding=\"utf-8\")\n        print(focused)\n        return 0\n    if args.json_out:\n        args.json_out.parent.mkdir(parents=True, exist_ok=True)\n        args.json_out.write_text(json.dumps(summary, indent=2), encoding=\"utf-8\")\n    print(\n        f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    )\n    return 0\n"
        ) {
            main = main.replacen(
                "        focused = f\"column={column_name} values={','.join(values)}\"\n        if args.json_out:\n            args.json_out.parent.mkdir(parents=True, exist_ok=True)\n            args.json_out.write_text(json.dumps({**summary, \"column\": column_name, \"values\": values}, indent=2), encoding=\"utf-8\")\n        print(focused)\n        return 0\n    if args.json_out:\n        args.json_out.parent.mkdir(parents=True, exist_ok=True)\n        args.json_out.write_text(json.dumps(summary, indent=2), encoding=\"utf-8\")\n    print(\n        f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    )\n    return 0\n",
                "        focused = f\"column={column_name} values={','.join(values)}\"\n        if args.json_out:\n            args.json_out.parent.mkdir(parents=True, exist_ok=True)\n            args.json_out.write_text(json.dumps({**summary, \"column\": column_name, \"values\": values}, indent=2), encoding=\"utf-8\")\n        maybe_write_email(args.email_out, args.email_to, \"CSV Report Column Summary\", focused)\n        print(focused)\n        return 0\n    if args.json_out:\n        args.json_out.parent.mkdir(parents=True, exist_ok=True)\n        args.json_out.write_text(json.dumps(summary, indent=2), encoding=\"utf-8\")\n    report_line = f\"rows={summary['rows']} columns={summary['columns']} header={','.join(summary['header'])}\"\n    maybe_write_email(args.email_out, args.email_to, \"CSV Report Summary\", report_line)\n    print(report_line)\n    return 0\n",
                1,
            );
        } else {
            bail!("csv_report main.py did not match expected email sender patch shape");
        }
    }

    if !readme.contains("--email-to ops@example.com --email-out") {
        readme.push_str(
            "\n## Email Sender\n\n`python main.py --input fixtures/input/sample.csv --email-to ops@example.com --email-out fixtures/output/report_email.eml`\n\nThis writes an email-ready report draft to `fixtures/output/report_email.eml`.\n",
        );
    }

    if !spec
        .features
        .iter()
        .any(|feature| feature == "email_sender")
    {
        spec.features.push("email_sender".into());
    }
    ensure_marker(&mut spec.supported_patch_kinds, "json_export");
    ensure_marker(&mut spec.supported_patch_kinds, "column_filter");
    ensure_marker(&mut spec.supported_patch_kinds, "email_sender");
    if !spec
        .acceptance_checks
        .iter()
        .any(|check| check.check_id == "csv-email-sender-run")
    {
        spec.acceptance_checks.push(AcceptanceCheck {
            check_id: "csv-email-sender-run".into(),
            kind: "python_run_success".into(),
            target: "main.py --input fixtures/input/sample.csv --email-to ops@example.com --email-out fixtures/output/report_email.eml".into(),
            expected: None,
        });
    }
    if !spec
        .acceptance_commands
        .iter()
        .any(|cmd| cmd == "python_cli_csv_report_email_sender")
    {
        spec.acceptance_commands
            .push("python_cli_csv_report_email_sender".into());
    }

    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "csv-email-sender-run".into(),
            kind: "python_run_success".into(),
            target: "main.py --input fixtures/input/sample.csv --email-to ops@example.com --email-out fixtures/output/report_email.eml".into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "csv-email-sender-file".into(),
            kind: "exists".into(),
            target: "fixtures/output/report_email.eml".into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "csv-email-sender-recipient".into(),
            kind: "contains".into(),
            target: "fixtures/output/report_email.eml".into(),
            expected: Some("To: ops@example.com".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "csv-email-sender-marker".into(),
            kind: "contains".into(),
            target: "main.py".into(),
            expected: Some("--email-out".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "--email-to");
    ensure_marker(&mut acceptance.required_markers, "--email-out");
    ensure_command(
        &mut acceptance.commands,
        "python_cli_csv_report_email_sender",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&main_path, main)?;
    fs::write(&readme_path, readme)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        main_path.clone(),
        readme_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        project_name: project_name.to_string(),
        substrate: spec.substrate.clone(),
        patch_kind: "email_sender".into(),
        request_summary: request_summary.to_string(),
        modified_files: modified_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        created_at: None,
    };

    Ok((
        PatchArtifacts {
            project_dir: project_dir.to_path_buf(),
            modified_files,
            patch_kind: "email_sender".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_rust_log_summary_file_output(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let main_path = project_dir.join("src/main.rs");
    let readme_path = project_dir.join("README.md");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut readme = fs::read_to_string(&readme_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;
    let (_, has_severity_filter, has_json_output, has_markdown_output) =
        rust_log_summary_feature_flags(&spec);
    let main = render_rust_log_summary_main(
        true,
        has_severity_filter,
        has_json_output,
        has_markdown_output,
    );

    if !readme.contains("--output") {
        readme.push_str(
            "\n## Patch Lane\n\n`cargo run -- --input fixtures/input/app.log --output fixtures/output/summary.txt`\n\nThis writes the computed log summary to `fixtures/output/summary.txt`.\n",
        );
    }

    if !spec.features.iter().any(|feature| feature == "file_output") {
        spec.features.push("file_output".into());
    }
    ensure_marker(&mut spec.supported_patch_kinds, "file_output");
    ensure_marker(&mut spec.supported_patch_kinds, "severity_filter");
    ensure_marker(&mut spec.supported_patch_kinds, "json_output");
    if !spec
        .acceptance_checks
        .iter()
        .any(|check| check.check_id == "log-file-output-run")
    {
        spec.acceptance_checks.push(AcceptanceCheck {
            check_id: "log-file-output-run".into(),
            kind: "cargo_run_success".into(),
            target: "-- --input fixtures/input/app.log --output fixtures/output/summary.txt".into(),
            expected: None,
        });
    }
    if !spec
        .acceptance_commands
        .iter()
        .any(|cmd| cmd == "cargo_run_log_summary_file_output")
    {
        spec.acceptance_commands
            .push("cargo_run_log_summary_file_output".into());
    }

    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-file-output-run".into(),
            kind: "cargo_run_success".into(),
            target: "-- --input fixtures/input/app.log --output fixtures/output/summary.txt".into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-file-output-file".into(),
            kind: "exists".into(),
            target: "fixtures/output/summary.txt".into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-file-output-marker".into(),
            kind: "contains".into(),
            target: "src/main.rs".into(),
            expected: Some("--output".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "--output");
    ensure_command(
        &mut acceptance.commands,
        "cargo_run_log_summary_file_output",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&main_path, main)?;
    fs::write(&readme_path, readme)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        main_path.clone(),
        readme_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        project_name: project_name.to_string(),
        substrate: spec.substrate.clone(),
        patch_kind: "file_output".into(),
        request_summary: request_summary.to_string(),
        modified_files: modified_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        created_at: None,
    };

    Ok((
        PatchArtifacts {
            project_dir: project_dir.to_path_buf(),
            modified_files,
            patch_kind: "file_output".into(),
        },
        patch_receipt,
    ))
}

fn render_rust_log_summary_main(
    has_file_output: bool,
    has_severity_filter: bool,
    has_json_output: bool,
    has_markdown_output: bool,
) -> String {
    let mut out = String::new();
    out.push_str("use std::env;\n");
    out.push_str("use std::fs;\n");
    out.push_str("use std::path::PathBuf;\n\n");
    out.push_str("fn main() {\n\n");
    out.push_str("    let args = env::args().skip(1).collect::<Vec<_>>();\n");
    out.push_str("    let mut input: Option<PathBuf> = None;\n");
    if has_file_output {
        out.push_str("    let mut output: Option<PathBuf> = None;\n");
    }
    if has_json_output {
        out.push_str("    let mut json_out: Option<PathBuf> = None;\n");
    }
    if has_markdown_output {
        out.push_str("    let mut markdown_out: Option<PathBuf> = None;\n");
    }
    if has_severity_filter {
        out.push_str("    let mut severity: Option<String> = None;\n");
    }
    out.push_str("    let mut i = 0usize;\n");
    out.push_str("    while i < args.len() {\n");
    out.push_str("        match args[i].as_str() {\n");
    out.push_str("            \"--input\" if i + 1 < args.len() => {\n");
    out.push_str("                input = Some(PathBuf::from(&args[i + 1]));\n");
    out.push_str("                i += 2;\n");
    out.push_str("            }\n");
    if has_severity_filter {
        out.push_str("            \"--severity\" if i + 1 < args.len() => {\n");
        out.push_str("                severity = Some(args[i + 1].to_ascii_lowercase());\n");
        out.push_str("                i += 2;\n");
        out.push_str("            }\n");
    }
    if has_file_output {
        out.push_str("            \"--output\" if i + 1 < args.len() => {\n");
        out.push_str("                output = Some(PathBuf::from(&args[i + 1]));\n");
        out.push_str("                i += 2;\n");
        out.push_str("            }\n");
    }
    if has_json_output {
        out.push_str("            \"--json-out\" if i + 1 < args.len() => {\n");
        out.push_str("                json_out = Some(PathBuf::from(&args[i + 1]));\n");
        out.push_str("                i += 2;\n");
        out.push_str("            }\n");
    }
    if has_markdown_output {
        out.push_str("            \"--markdown-out\" if i + 1 < args.len() => {\n");
        out.push_str("                markdown_out = Some(PathBuf::from(&args[i + 1]));\n");
        out.push_str("                i += 2;\n");
        out.push_str("            }\n");
    }
    out.push_str("            _ => {\n");
    out.push_str("                i += 1;\n");
    out.push_str("            }\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("    let input_file = input.expect(\"--input is required\");\n");
    out.push_str("    let text = fs::read_to_string(&input_file).expect(\"read input file\");\n");
    out.push_str("    let lower = text.to_ascii_lowercase();\n");
    out.push_str("    let errors = lower.matches(\"error\").count();\n");
    out.push_str("    let warnings = lower.matches(\"warn\").count();\n");
    out.push_str("    let infos = lower.matches(\"info\").count();\n");
    if has_severity_filter {
        out.push_str("    let rendered = if let Some(severity_name) = severity.as_deref() {\n");
        out.push_str("        let count = match severity_name {\n");
        out.push_str("            \"error\" | \"errors\" => errors,\n");
        out.push_str("            \"warn\" | \"warning\" | \"warnings\" => warnings,\n");
        out.push_str("            \"info\" | \"infos\" => infos,\n");
        out.push_str("            other => panic!(\"unknown severity: {}\", other),\n");
        out.push_str("        };\n");
        out.push_str("        format!(\"severity={} count={}\", severity_name, count)\n");
        out.push_str("    } else {\n");
        out.push_str(
            "        format!(\"errors={} warnings={} infos={}\", errors, warnings, infos)\n",
        );
        out.push_str("    };\n");
    } else {
        out.push_str("    let rendered = format!(\"errors={} warnings={} infos={}\", errors, warnings, infos);\n");
    }
    out.push_str("    println!(\"{}\", rendered);\n");
    if has_file_output {
        out.push_str("    if let Some(output_path) = output {\n");
        out.push_str("        if let Some(parent) = output_path.parent() {\n");
        out.push_str("            fs::create_dir_all(parent).expect(\"create output parent\");\n");
        out.push_str("        }\n");
        out.push_str("        fs::write(&output_path, format!(\"{}\\n\", rendered)).expect(\"write output report\");\n");
        out.push_str("    }\n");
    }
    if has_json_output {
        if has_severity_filter {
            out.push_str(
                "    let json_payload = if let Some(severity_name) = severity.as_deref() {\n",
            );
            out.push_str("        let count = match severity_name {\n");
            out.push_str("            \"error\" | \"errors\" => errors,\n");
            out.push_str("            \"warn\" | \"warning\" | \"warnings\" => warnings,\n");
            out.push_str("            \"info\" | \"infos\" => infos,\n");
            out.push_str("            other => panic!(\"unknown severity: {}\", other),\n");
            out.push_str("        };\n");
            out.push_str("        format!(\"{{\\\"severity\\\":\\\"{}\\\",\\\"count\\\":{}}}\", severity_name, count)\n");
            out.push_str("    } else {\n");
            out.push_str("        format!(\"{{\\\"errors\\\":{},\\\"warnings\\\":{},\\\"infos\\\":{}}}\", errors, warnings, infos)\n");
            out.push_str("    };\n");
        } else {
            out.push_str("    let json_payload = format!(\"{{\\\"errors\\\":{},\\\"warnings\\\":{},\\\"infos\\\":{}}}\", errors, warnings, infos);\n");
        }
        out.push_str("    if let Some(json_path) = json_out {\n");
        out.push_str("        if let Some(parent) = json_path.parent() {\n");
        out.push_str(
            "            fs::create_dir_all(parent).expect(\"create json output parent\");\n",
        );
        out.push_str("        }\n");
        out.push_str("        fs::write(&json_path, format!(\"{}\\n\", json_payload)).expect(\"write json output\");\n");
        out.push_str("    }\n");
    }
    if has_markdown_output {
        if has_severity_filter {
            out.push_str(
                "    let markdown_payload = if let Some(severity_name) = severity.as_deref() {\n",
            );
            out.push_str("        let count = match severity_name {\n");
            out.push_str("            \"error\" | \"errors\" => errors,\n");
            out.push_str("            \"warn\" | \"warning\" | \"warnings\" => warnings,\n");
            out.push_str("            \"info\" | \"infos\" => infos,\n");
            out.push_str("            other => panic!(\"unknown severity: {}\", other),\n");
            out.push_str("        };\n");
            out.push_str("        format!(\"# Log Summary\\n\\n- Severity: {}\\n- Count: {}\\n\", severity_name, count)\n");
            out.push_str("    } else {\n");
            out.push_str("        format!(\"# Log Summary\\n\\n- Errors: {}\\n- Warnings: {}\\n- Infos: {}\\n\", errors, warnings, infos)\n");
            out.push_str("    };\n");
        } else {
            out.push_str("    let markdown_payload = format!(\"# Log Summary\\n\\n- Errors: {}\\n- Warnings: {}\\n- Infos: {}\\n\", errors, warnings, infos);\n");
        }
        out.push_str("    if let Some(markdown_path) = markdown_out {\n");
        out.push_str("        if let Some(parent) = markdown_path.parent() {\n");
        out.push_str(
            "            fs::create_dir_all(parent).expect(\"create markdown output parent\");\n",
        );
        out.push_str("        }\n");
        out.push_str("        fs::write(&markdown_path, markdown_payload).expect(\"write markdown output\");\n");
        out.push_str("    }\n");
    }
    out.push_str("\n}\n");
    out
}

pub fn patch_rust_log_summary_severity_filter(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let main_path = project_dir.join("src/main.rs");
    let readme_path = project_dir.join("README.md");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut readme = fs::read_to_string(&readme_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;
    let (has_file_output, _, has_json_output, has_markdown_output) =
        rust_log_summary_feature_flags(&spec);
    let main =
        render_rust_log_summary_main(has_file_output, true, has_json_output, has_markdown_output);

    if !readme.contains("--severity error") {
        readme.push_str(
            "\n## Severity Filter\n\n`cargo run -- --input fixtures/input/app.log --severity error`\n\nThis prints only the count for the selected severity.\n",
        );
    }

    if !spec
        .features
        .iter()
        .any(|feature| feature == "severity_filter")
    {
        spec.features.push("severity_filter".into());
    }
    ensure_marker(&mut spec.supported_patch_kinds, "file_output");
    ensure_marker(&mut spec.supported_patch_kinds, "severity_filter");
    ensure_marker(&mut spec.supported_patch_kinds, "json_output");
    if !spec
        .acceptance_checks
        .iter()
        .any(|check| check.check_id == "log-severity-filter-run")
    {
        spec.acceptance_checks.push(AcceptanceCheck {
            check_id: "log-severity-filter-run".into(),
            kind: "cargo_run_output_contains".into(),
            target: "-- --input fixtures/input/app.log --severity error".into(),
            expected: Some("severity=error count=2".into()),
        });
    }
    if !spec
        .acceptance_commands
        .iter()
        .any(|cmd| cmd == "cargo_run_log_summary_severity_filter")
    {
        spec.acceptance_commands
            .push("cargo_run_log_summary_severity_filter".into());
    }

    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-severity-filter-run".into(),
            kind: "cargo_run_output_contains".into(),
            target: "-- --input fixtures/input/app.log --severity error".into(),
            expected: Some("severity=error count=2".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-severity-filter-marker".into(),
            kind: "contains".into(),
            target: "src/main.rs".into(),
            expected: Some("--severity".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "--severity");
    ensure_command(
        &mut acceptance.commands,
        "cargo_run_log_summary_severity_filter",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&main_path, main)?;
    fs::write(&readme_path, readme)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        main_path.clone(),
        readme_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        project_name: project_name.to_string(),
        substrate: spec.substrate.clone(),
        patch_kind: "severity_filter".into(),
        request_summary: request_summary.to_string(),
        modified_files: modified_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        created_at: None,
    };

    Ok((
        PatchArtifacts {
            project_dir: project_dir.to_path_buf(),
            modified_files,
            patch_kind: "severity_filter".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_rust_log_summary_json_output(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let main_path = project_dir.join("src/main.rs");
    let readme_path = project_dir.join("README.md");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut readme = fs::read_to_string(&readme_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    let (has_file_output, has_severity_filter, _, has_markdown_output) =
        rust_log_summary_feature_flags(&spec);
    let main = render_rust_log_summary_main(
        has_file_output,
        has_severity_filter,
        true,
        has_markdown_output,
    );

    if !readme.contains("--json-out fixtures/output/summary.json") {
        readme.push_str(
            "\n## JSON Output\n\n`cargo run -- --input fixtures/input/app.log --json-out fixtures/output/summary.json`\n\nThis writes the computed log summary to `fixtures/output/summary.json`.\n",
        );
    }

    if !spec.features.iter().any(|feature| feature == "json_output") {
        spec.features.push("json_output".into());
    }
    ensure_marker(&mut spec.supported_patch_kinds, "file_output");
    ensure_marker(&mut spec.supported_patch_kinds, "severity_filter");
    ensure_marker(&mut spec.supported_patch_kinds, "json_output");
    if !spec
        .acceptance_checks
        .iter()
        .any(|check| check.check_id == "log-json-output-run")
    {
        spec.acceptance_checks.push(AcceptanceCheck {
            check_id: "log-json-output-run".into(),
            kind: "cargo_run_success".into(),
            target: "-- --input fixtures/input/app.log --json-out fixtures/output/summary.json"
                .into(),
            expected: None,
        });
    }
    if !spec
        .acceptance_commands
        .iter()
        .any(|cmd| cmd == "cargo_run_log_summary_json_output")
    {
        spec.acceptance_commands
            .push("cargo_run_log_summary_json_output".into());
    }

    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-json-output-run".into(),
            kind: "cargo_run_success".into(),
            target: "-- --input fixtures/input/app.log --json-out fixtures/output/summary.json"
                .into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-json-output-file".into(),
            kind: "exists".into(),
            target: "fixtures/output/summary.json".into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-json-output-content".into(),
            kind: "contains".into(),
            target: "fixtures/output/summary.json".into(),
            expected: Some("\"errors\":2".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-json-output-marker".into(),
            kind: "contains".into(),
            target: "src/main.rs".into(),
            expected: Some("--json-out".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "--json-out");
    ensure_command(
        &mut acceptance.commands,
        "cargo_run_log_summary_json_output",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&main_path, main)?;
    fs::write(&readme_path, readme)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        main_path.clone(),
        readme_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        project_name: project_name.to_string(),
        substrate: spec.substrate.clone(),
        patch_kind: "json_output".into(),
        request_summary: request_summary.to_string(),
        modified_files: modified_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        created_at: None,
    };

    Ok((
        PatchArtifacts {
            project_dir: project_dir.to_path_buf(),
            modified_files,
            patch_kind: "json_output".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_rust_log_summary_markdown_export(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let main_path = project_dir.join("src/main.rs");
    let readme_path = project_dir.join("README.md");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut readme = fs::read_to_string(&readme_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    let (has_file_output, has_severity_filter, has_json_output, _) =
        rust_log_summary_feature_flags(&spec);
    let main =
        render_rust_log_summary_main(has_file_output, has_severity_filter, has_json_output, true);

    if !readme.contains("--markdown-out fixtures/output/summary.md") {
        readme.push_str(
            "\n## Markdown Export\n\n`cargo run -- --input fixtures/input/app.log --markdown-out fixtures/output/summary.md`\n\nThis writes the computed log summary to `fixtures/output/summary.md` in markdown form.\n",
        );
    }

    if !spec
        .features
        .iter()
        .any(|feature| feature == "markdown_export")
    {
        spec.features.push("markdown_export".into());
    }
    ensure_marker(&mut spec.supported_patch_kinds, "file_output");
    ensure_marker(&mut spec.supported_patch_kinds, "severity_filter");
    ensure_marker(&mut spec.supported_patch_kinds, "json_output");
    ensure_marker(&mut spec.supported_patch_kinds, "new_patch_lane");
    if !spec
        .acceptance_checks
        .iter()
        .any(|check| check.check_id == "log-markdown-export-run")
    {
        spec.acceptance_checks.push(AcceptanceCheck {
            check_id: "log-markdown-export-run".into(),
            kind: "cargo_run_success".into(),
            target: "-- --input fixtures/input/app.log --markdown-out fixtures/output/summary.md"
                .into(),
            expected: None,
        });
    }
    if !spec
        .acceptance_commands
        .iter()
        .any(|cmd| cmd == "cargo_run_log_summary_markdown_export")
    {
        spec.acceptance_commands
            .push("cargo_run_log_summary_markdown_export".into());
    }

    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-markdown-export-run".into(),
            kind: "cargo_run_success".into(),
            target: "-- --input fixtures/input/app.log --markdown-out fixtures/output/summary.md"
                .into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-markdown-export-file".into(),
            kind: "exists".into(),
            target: "fixtures/output/summary.md".into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-markdown-export-content".into(),
            kind: "contains".into(),
            target: "fixtures/output/summary.md".into(),
            expected: Some("# Log Summary".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "log-markdown-export-marker".into(),
            kind: "contains".into(),
            target: "src/main.rs".into(),
            expected: Some("--markdown-out".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "--markdown-out");
    ensure_command(
        &mut acceptance.commands,
        "cargo_run_log_summary_markdown_export",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&main_path, main)?;
    fs::write(&readme_path, readme)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        main_path.clone(),
        readme_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        project_name: project_name.to_string(),
        substrate: spec.substrate.clone(),
        patch_kind: "new_patch_lane".into(),
        request_summary: request_summary.to_string(),
        modified_files: modified_files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        created_at: None,
    };

    Ok((
        PatchArtifacts {
            project_dir: project_dir.to_path_buf(),
            modified_files,
            patch_kind: "new_patch_lane".into(),
        },
        patch_receipt,
    ))
}

fn rust_log_summary_feature_flags(spec: &ProjectSpec) -> (bool, bool, bool, bool) {
    let has_file_output = spec.features.iter().any(|feature| feature == "file_output");
    let has_severity_filter = spec
        .features
        .iter()
        .any(|feature| feature == "severity_filter");
    let has_json_output = spec.features.iter().any(|feature| feature == "json_output");
    let has_markdown_output = spec
        .features
        .iter()
        .any(|feature| feature == "markdown_export");
    (
        has_file_output,
        has_severity_filter,
        has_json_output,
        has_markdown_output,
    )
}


fn render_static_web_dashboard_bundle(inputs: &BuildSeedInputs) -> Result<WebBuildBundle> {
    let helper_monitoring = request_wants_local_inbox_helper(inputs);
    let context = serde_json::json!({
        "title": title_or_default(inputs, "ChattyFactory Dashboard"),
        "summary": summary_or_default(inputs, "A deterministic dashboard build emitted by the ChattyFactory rebuild."),
        "primary_metric_label": "Build ready",
        "route_label": "static_web_dashboard",
        "status_text": "Host-rendered dashboard build is ready.",
        "primary_action_label": "Show Build State",
        "secondary_action_label": "Show Route",
        "results_heading": "Results",
        "results_empty": "No actions have been triggered yet.",
        "interactive_message": "Host-rendered dashboard interactions are available.",
        "helper_monitoring": helper_monitoring,
    });

    Ok(WebBuildBundle {
        index_html: render_named("substrates/static_web/index.html", &context)?,
        app_js: render_named("substrates/static_web/app.js", &context)?,
        styles_css: render_named("substrates/static_web/styles.css", &context)?,
        readme_md: render_named("substrates/static_web/README.md", &context)?,
    })
}

fn title_or_default<'a>(inputs: &'a BuildSeedInputs, fallback: &'a str) -> &'a str {
    if inputs.title.trim().is_empty() {
        fallback
    } else {
        &inputs.title
    }
}

fn summary_or_default<'a>(inputs: &'a BuildSeedInputs, fallback: &'a str) -> &'a str {
    if inputs.summary.trim().is_empty() {
        fallback
    } else {
        &inputs.summary
    }
}

fn config_value<'a>(items: &'a [String], key: &str) -> Option<&'a str> {
    items.iter().find_map(|item| {
        let (head, tail) = item.split_once('=')?;
        if head == key {
            Some(tail)
        } else {
            None
        }
    })
}

fn ensure_check(checks: &mut Vec<AcceptanceCheck>, check: AcceptanceCheck) {
    if !checks
        .iter()
        .any(|existing| existing.check_id == check.check_id)
    {
        checks.push(check);
    }
}

fn ensure_marker(markers: &mut Vec<String>, marker: &str) {
    if !markers.iter().any(|existing| existing == marker) {
        markers.push(marker.to_string());
    }
}

fn upsert_helper_primitive(helper_spec: &mut HelperServiceSpec, primitive: HelperPrimitiveSpec) {
    if let Some(existing) = helper_spec
        .primitives
        .iter_mut()
        .find(|existing| existing.primitive_id == primitive.primitive_id)
    {
        *existing = primitive;
    } else {
        helper_spec.primitives.push(primitive);
    }
}

fn update_check_expected(
    checks: &mut Vec<AcceptanceCheck>,
    check_id: &str,
    expected: Option<String>,
) {
    if let Some(existing) = checks
        .iter_mut()
        .find(|existing| existing.check_id == check_id)
    {
        existing.expected = expected;
    }
}

fn ensure_command(commands: &mut Vec<String>, command: &str) {
    if !commands.iter().any(|existing| existing == command) {
        commands.push(command.to_string());
    }
}

fn apply_operator_contributions(
    spec: &mut ProjectSpec,
    acceptance: &mut AcceptancePlan,
    operator_ids: &[String],
) {
    let Some(substrate_kind) = registry_substrate_kind(spec) else {
        return;
    };
    for operator_id in operator_ids {
        for contribution in operator_contribution_registry() {
            if contribution.substrate_kind == substrate_kind.as_str()
                && contribution.operator_id == operator_id
            {
                ensure_marker(&mut spec.features, contribution.feature_id);
                let check = AcceptanceCheck {
                    check_id: contribution.check.check_id.into(),
                    kind: contribution.check.kind.into(),
                    target: contribution.check.target.into(),
                    expected: contribution.check.expected.map(str::to_string),
                };
                ensure_check(&mut acceptance.checks, check);
                ensure_marker(
                    &mut acceptance.required_markers,
                    contribution.required_marker,
                );
            }
        }
    }
}

fn apply_acceptance_recipe_contributions(
    spec: &mut ProjectSpec,
    acceptance: &mut AcceptancePlan,
    recipe_ids: &[String],
) {
    let Some(substrate_kind) = registry_substrate_kind(spec) else {
        return;
    };
    for recipe_id in recipe_ids {
        for recipe in acceptance_recipe_registry() {
            if recipe.recipe_id != recipe_id || recipe.substrate_kind != substrate_kind.as_str() {
                continue;
            }
            if recipe.tool_kind.is_some() && recipe.tool_kind != spec.tool_kind.as_deref() {
                continue;
            }

            ensure_marker(&mut spec.features, recipe.feature_id);
            ensure_command(&mut spec.acceptance_commands, recipe.command_id);
            ensure_command(&mut acceptance.commands, recipe.command_id);
            if let Some(output) = recipe.expected_output {
                ensure_marker(&mut acceptance.expected_outputs, output);
            }
            for marker in recipe.required_markers {
                ensure_marker(&mut acceptance.required_markers, marker);
            }
            for check_spec in recipe.checks {
                let check = AcceptanceCheck {
                    check_id: check_spec.check_id.into(),
                    kind: check_spec.kind.into(),
                    target: check_spec.target.into(),
                    expected: check_spec.expected.map(str::to_string),
                };
                ensure_check(&mut spec.acceptance_checks, check.clone());
                ensure_check(&mut acceptance.checks, check);
            }
        }
    }
}

pub(crate) fn patch_python_cli_tool_csv_report_new_patch_lane(
    project_dir: &std::path::Path,
    project_name: &str,
    request: &str,
    request_id: &str,
) -> anyhow::Result<(PatchArtifacts, chatty_factory_core::PatchReceipt)> {
    patch_python_csv_report_email_sender(project_dir, project_name, request_id, request)
}

pub(crate) fn patch_rust_cli_tool_log_summary_new_patch_lane(
    project_dir: &std::path::Path,
    project_name: &str,
    request: &str,
    request_id: &str,
) -> anyhow::Result<(PatchArtifacts, chatty_factory_core::PatchReceipt)> {
    patch_rust_log_summary_markdown_export(project_dir, project_name, request_id, request)
}
