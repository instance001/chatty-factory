use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use chatty_factory_core::{
    AcceptanceCheck, AcceptancePlan, BuildReceipt, ChattyCogBridgeCapabilities,
    ChattyCogBridgeSpec, ChattyCogCommandSpec, ChattyCogModuleSpec, ChattyCogVisualLoadSpec,
    ChattyEduModuleSpec, DesiredSurface, ExoskeletonTarget, FamilyId, HelperLaunchPolicy,
    HelperPrimitiveSpec, HelperServiceSpec, HelperStatusSnapshot, PatchLaneStatus, PatchReceipt,
    ProjectSpec, RequestPlan, ScaffoldInputs,
};
use chatty_factory_templates::render_named;
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
pub enum FamilyPriority {
    Tier1,
    Tier2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamilyDescriptor {
    pub id: FamilyId,
    pub status: &'static str,
    pub priority: FamilyPriority,
    pub primary_substrate: DesiredSurface,
    pub supports_chattycog_wrapper: bool,
    pub supports_standalone: bool,
}

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

#[derive(Debug, Clone)]
struct LocalInboxHelperBundle {
    helper_service: HelperServiceSpec,
    helper_status_snapshot: HelperStatusSnapshot,
    helper_summary_json: String,
    helper_readme: String,
}

pub fn built_in_families() -> Vec<FamilyDescriptor> {
    vec![
        FamilyDescriptor {
            id: FamilyId::StaticWebDashboard,
            status: "planned",
            priority: FamilyPriority::Tier1,
            primary_substrate: DesiredSurface::Web,
            supports_chattycog_wrapper: true,
            supports_standalone: true,
        },
        FamilyDescriptor {
            id: FamilyId::ChattycogWebviewModule,
            status: "planned",
            priority: FamilyPriority::Tier1,
            primary_substrate: DesiredSurface::Web,
            supports_chattycog_wrapper: true,
            supports_standalone: true,
        },
        FamilyDescriptor {
            id: FamilyId::ChattycogNativeWindowModule,
            status: "planned",
            priority: FamilyPriority::Tier1,
            primary_substrate: DesiredSurface::Desktop,
            supports_chattycog_wrapper: true,
            supports_standalone: true,
        },
        FamilyDescriptor {
            id: FamilyId::ChattyeduNativeWindowModule,
            status: "planned",
            priority: FamilyPriority::Tier1,
            primary_substrate: DesiredSurface::Desktop,
            supports_chattycog_wrapper: false,
            supports_standalone: true,
        },
        FamilyDescriptor {
            id: FamilyId::ChattycogChattyeduNativeWindowModule,
            status: "planned",
            priority: FamilyPriority::Tier1,
            primary_substrate: DesiredSurface::Desktop,
            supports_chattycog_wrapper: true,
            supports_standalone: true,
        },
        FamilyDescriptor {
            id: FamilyId::ChattycogWorkspaceModule,
            status: "planned",
            priority: FamilyPriority::Tier1,
            primary_substrate: DesiredSurface::Unknown,
            supports_chattycog_wrapper: true,
            supports_standalone: false,
        },
        FamilyDescriptor {
            id: FamilyId::PythonCliTool,
            status: "planned",
            priority: FamilyPriority::Tier2,
            primary_substrate: DesiredSurface::Cli,
            supports_chattycog_wrapper: false,
            supports_standalone: true,
        },
        FamilyDescriptor {
            id: FamilyId::RustCliTool,
            status: "planned",
            priority: FamilyPriority::Tier2,
            primary_substrate: DesiredSurface::Cli,
            supports_chattycog_wrapper: false,
            supports_standalone: true,
        },
    ]
}

pub fn families_for_target(target: ExoskeletonTarget) -> Vec<FamilyDescriptor> {
    built_in_families()
        .into_iter()
        .filter(|family| match target {
            ExoskeletonTarget::None => family.supports_standalone,
            ExoskeletonTarget::ChattyCog => family.supports_chattycog_wrapper,
        })
        .collect()
}

pub fn candidate_patch_recipe_ids(
    family_id: Option<&FamilyId>,
    tool_kind: Option<&str>,
    project_features: &[String],
) -> Vec<String> {
    family_id
        .map(FamilyId::as_str)
        .map(|family_id| candidate_patch_recipe_ids_for(family_id, tool_kind, project_features))
        .unwrap_or_default()
}

pub fn patch_lane_statuses(
    family_id: Option<&FamilyId>,
    tool_kind: Option<&str>,
    project_features: &[String],
) -> Vec<PatchLaneStatus> {
    family_id
        .map(FamilyId::as_str)
        .map(|family_id| patch_lane_statuses_for(family_id, tool_kind, project_features))
        .unwrap_or_default()
}

pub fn patch_primitive_classes(
    family_id: Option<&FamilyId>,
    tool_kind: Option<&str>,
    patch_kinds: &[String],
) -> Vec<String> {
    family_id
        .map(FamilyId::as_str)
        .map(|family_id| patch_primitive_classes_for(family_id, tool_kind, patch_kinds))
        .unwrap_or_default()
}

pub fn patch_required_anchor_markers(
    family_id: Option<&FamilyId>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Vec<String> {
    family_id
        .map(FamilyId::as_str)
        .and_then(|family_id| patch_structural_guard_spec(family_id, tool_kind, patch_kind))
        .map(|spec| {
            spec.required_anchor_markers
                .iter()
                .map(|marker| (*marker).to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn patch_conflicting_anchor_markers(
    family_id: Option<&FamilyId>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Vec<String> {
    family_id
        .map(FamilyId::as_str)
        .and_then(|family_id| patch_structural_guard_spec(family_id, tool_kind, patch_kind))
        .map(|spec| {
            spec.conflicting_anchor_markers
                .iter()
                .map(|marker| (*marker).to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn patch_expected_artifact_groups(
    family_id: Option<&FamilyId>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Vec<String> {
    family_id
        .map(FamilyId::as_str)
        .and_then(|family_id| patch_structural_guard_spec(family_id, tool_kind, patch_kind))
        .map(|spec| {
            spec.expected_artifact_groups
                .iter()
                .map(|group| (*group).to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn patch_ownership_boundaries(
    family_id: Option<&FamilyId>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Vec<String> {
    family_id
        .map(FamilyId::as_str)
        .and_then(|family_id| patch_structural_guard_spec(family_id, tool_kind, patch_kind))
        .map(|spec| {
            spec.ownership_boundaries
                .iter()
                .map(|note| (*note).to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub fn patch_surgical_maturity(
    family_id: Option<&FamilyId>,
    tool_kind: Option<&str>,
    patch_kind: &str,
) -> Option<String> {
    family_id.map(FamilyId::as_str).map(|family_id| {
        patch_recipe_surgical_maturity(family_id, tool_kind, patch_kind).to_string()
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
        spec.family_id.as_ref(),
        spec.tool_kind.as_deref(),
        &lane.patch_kind,
    );
    let conflicting_markers = patch_conflicting_anchor_markers(
        spec.family_id.as_ref(),
        spec.tool_kind.as_deref(),
        &lane.patch_kind,
    );
    let expected_groups = patch_expected_artifact_groups(
        spec.family_id.as_ref(),
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
    family_id: Option<&FamilyId>,
    tool_kind: Option<&str>,
) -> Vec<String> {
    family_id
        .map(FamilyId::as_str)
        .map(|family_id| candidate_acceptance_recipe_ids_for(family_id, tool_kind))
        .unwrap_or_default()
}

pub fn refresh_project_contract_views_for_project(
    project_dir: Option<&Path>,
    spec: &mut ProjectSpec,
) {
    let Some(family_id) = spec.family_id.as_ref().map(FamilyId::as_str) else {
        spec.patch_lanes.clear();
        spec.acceptance_recipes.clear();
        spec.operator_bundles.clear();
        return;
    };

    spec.patch_lanes =
        patch_lane_statuses_for(family_id, spec.tool_kind.as_deref(), &spec.features);
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
    spec.acceptance_recipes =
        acceptance_recipe_statuses_for(family_id, spec.tool_kind.as_deref(), &spec.features);
    spec.operator_bundles = operator_bundle_statuses_for(family_id, &spec.features);
}

fn refresh_project_contract_views(spec: &mut ProjectSpec) {
    let inferred_project_dir = PathBuf::from("output").join(&spec.project_name);
    refresh_project_contract_views_for_project(Some(&inferred_project_dir), spec);
}

fn request_wants_local_inbox_helper(inputs: &ScaffoldInputs) -> bool {
    let lower = inputs.summary.to_ascii_lowercase();
    lower.contains("helper")
        || lower.contains("inbox")
        || lower.contains("watch")
        || lower.contains("monitor")
        || lower.contains("filtered file")
        || lower.contains("local file")
}

fn build_local_inbox_helper_bundle(
    project_name: &str,
    attached_family_id: FamilyId,
    purpose: &str,
    family_note: &str,
) -> Result<LocalInboxHelperBundle> {
    let helper_status_snapshot = HelperStatusSnapshot {
        helper_id: "local_inbox_helper".into(),
        helper_kind: "local_inbox_helper".into(),
        status: "scaffolded".into(),
        summary: "Local inbox helper scaffold is present but not yet launched.".into(),
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
        attached_family_id: Some(attached_family_id),
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
            family_note.into(),
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
        "# Local Inbox Helper\n\nThis is a bounded helper/service scaffold for ChattyFactory.\n\nPurpose:\n- observe the module asset inbox lane\n- emit deterministic helper status and summary artifacts\n- provide a bounded seam for host-supervised helper runtime work\n\nAttached family:\n- {family_note}\n"
    );

    Ok(LocalInboxHelperBundle {
        helper_service,
        helper_status_snapshot,
        helper_summary_json,
        helper_readme,
    })
}

pub fn build_static_web_dashboard(
    output_root: &Path,
    inputs: &ScaffoldInputs,
) -> Result<BuildArtifacts> {
    if !matches!(inputs.family_id, Some(FamilyId::StaticWebDashboard)) {
        bail!("static web dashboard builder requires family_id static_web_dashboard");
    }
    if inputs.project_name.trim().is_empty() {
        bail!("project_name is required");
    }

    let project_dir = output_root.join(&inputs.project_name);
    fs::create_dir_all(&project_dir)?;

    let bundle = render_static_web_dashboard_bundle(inputs)?;
    let helper_bundle = if request_wants_local_inbox_helper(inputs) {
        Some(build_local_inbox_helper_bundle(
            &inputs.project_name,
            FamilyId::StaticWebDashboard,
            "Observe a local inbox lane and emit deterministic helper status/output artifacts for a static dashboard.",
            "Cross-family helper primitive bundle attached to a static web dashboard.",
        )?)
    } else {
        None
    };

    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        family_id: Some(FamilyId::StaticWebDashboard),
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
                    check_id: "marker-family".into(),
                    kind: "contains".into(),
                    target: "index.html".into(),
                    expected: Some("data-family=\"static_web_dashboard\"".into()),
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
        family_id: Some(FamilyId::StaticWebDashboard),
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
                check_id: "family-marker".into(),
                kind: "contains".into(),
                target: "index.html".into(),
                expected: Some("data-family=\"static_web_dashboard\"".into()),
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
            "data-family=\"static_web_dashboard\"".into(),
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

pub fn build_chattycog_webview_module(
    output_root: &Path,
    inputs: &ScaffoldInputs,
) -> Result<BuildArtifacts> {
    if !matches!(inputs.family_id, Some(FamilyId::ChattycogWebviewModule)) {
        bail!("chattycog webview builder requires family_id chattycog_webview_module");
    }
    if inputs.project_name.trim().is_empty() {
        bail!("project_name is required");
    }

    let project_dir = output_root.join(&inputs.project_name);
    fs::create_dir_all(&project_dir)?;

    let title = title_or_default(inputs, "ChattyCog Webview Module");
    let summary = summary_or_default(
        inputs,
        "A deterministic ChattyCog-compatible webview module emitted by the ChattyFactory rebuild.",
    );
    let display_name = title.to_string();
    let module_id = inputs.project_name.replace('-', "_");

    let family_context = serde_json::json!({
        "title": title,
        "summary": summary,
        "module_state": "Module build ready",
        "bridge_status": "Bridge adapter ready",
        "results_heading": "Module Results",
        "results_empty": "Module build is ready for feature patches and integration work.",
        "interactive_message": "ChattyCog module dashboard loaded.",
    });
    let wrapper_context = serde_json::json!({
        "module_id": module_id,
        "display_name": display_name,
        "icon": "dashboard",
        "description": summary,
        "visual_notes": "ChattyFactory rebuild dashboard module",
        "handshake_notes": "Extend this module through deterministic family and operator lanes as the rebuild grows.",
    });
    let visual_load_spec = ChattyCogVisualLoadSpec {
        kind: "webview".into(),
        auto_launch: true,
        title: Some(display_name.clone()),
        notes: Some("ChattyFactory rebuild dashboard module".into()),
        file: Some("index.html".into()),
        url: None,
        window_title_contains: None,
        build_command: None,
        launch_command: None,
        serve_command: None,
        serve_wait_ms: None,
    };
    let module_spec = ChattyCogModuleSpec {
        module_spec_id: "chattyfactory.chattycog_module_spec.v1".into(),
        project_name: inputs.project_name.clone(),
        module_id: module_id.clone(),
        display_name: display_name.clone(),
        description: summary.to_string(),
        visual_kind: "webview".into(),
        visual_title: display_name.clone(),
        visual_file: "index.html".into(),
        handshake_path: "HANDSHAKE.md".into(),
        manifest_path: "manifest.json".into(),
        visual_load_path: Some("visual_load.json".into()),
        visual_load: Some(visual_load_spec),
        bridge: ChattyCogBridgeSpec {
            status_path: "bridge/status.json".into(),
            script_path: Some("chattycog_bridge.js".into()),
            log_sources_path: Some("bridge/log_sources.json".into()),
            capabilities: ChattyCogBridgeCapabilities {
                status_enabled: true,
                log_sources_enabled: true,
                shared_room_state_enabled: true,
                shared_room_events_enabled: true,
                outgoing_room_events_enabled: true,
                incoming_asset_lanes: vec!["module_assets".into()],
            },
            recommended_runtime_files: vec![
                "bridge/status.json".into(),
                "bridge/log_sources.json".into(),
                "bridge/shared_room_state.json".into(),
                "bridge/shared_room_events.json".into(),
                "bridge/outgoing_room_events.json".into(),
                "bridge/incoming_assets/module_assets".into(),
            ],
        },
        created_at: None,
    };
    let helper_bundle = build_local_inbox_helper_bundle(
        &inputs.project_name,
        FamilyId::ChattycogWebviewModule,
        "Observe the module asset inbox and emit deterministic helper status/output artifacts.",
        "First helper/service milestone scaffold attached to a ChattyCog webview module.",
    )?;
    let helper_status_snapshot = helper_bundle.helper_status_snapshot.clone();
    let helper_service = helper_bundle.helper_service.clone();

    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        substrate: "static_web_with_chattycog_wrapper".into(),
        tool_kind: None,
        request_summary: Some(inputs.summary.clone()),
        entrypoints: vec![
            "index.html".into(),
            "app.js".into(),
            "styles.css".into(),
            "chattycog_bridge.js".into(),
            "bridge/status.json".into(),
        ],
        expected_files: vec![
            "index.html".into(),
            "app.js".into(),
            "styles.css".into(),
            "chattycog_bridge.js".into(),
            "bridge/status.json".into(),
            "bridge/log_sources.json".into(),
            "bridge/shared_room_state.json".into(),
            "bridge/shared_room_events.json".into(),
            "bridge/outgoing_room_events.json".into(),
            "bridge/incoming_assets/module_assets/.keep".into(),
            "bridge/incoming_assets/module_assets/sample-inbox-note.txt".into(),
            "bridge/helpers/local_inbox/status.json".into(),
            "bridge/helpers/local_inbox/summary.json".into(),
            "bridge/helpers/local_inbox/processed/sample-inbox-note.txt".into(),
            "helpers/local_inbox_helper/README.md".into(),
            "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
            "ChattyCogModuleSpec.json".into(),
            "manifest.json".into(),
            "visual_load.json".into(),
            "HANDSHAKE.md".into(),
            "ProjectSpec.json".into(),
            "AcceptancePlan.json".into(),
        ],
        features: inputs.feature_tokens.clone(),
        acceptance_commands: vec!["static_file_check".into(), "wrapper_metadata_check".into()],
        acceptance_checks: vec![
            AcceptanceCheck {
                check_id: "chattycog-module-contract".into(),
                kind: "chattycog_module_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "bridge-link".into(),
                kind: "contains".into(),
                target: "index.html".into(),
                expected: Some("chattycog_bridge.js".into()),
            },
            AcceptanceCheck {
                check_id: "family-marker".into(),
                kind: "contains".into(),
                target: "index.html".into(),
                expected: Some("data-family=\"chattycog_webview_module\"".into()),
            },
            AcceptanceCheck {
                check_id: "helper-spec".into(),
                kind: "helper_service_spec".into(),
                target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
                expected: Some("local_inbox_helper".into()),
            },
            AcceptanceCheck {
                check_id: "helper-status".into(),
                kind: "helper_status_snapshot".into(),
                target: "bridge/helpers/local_inbox/status.json".into(),
                expected: Some("completed".into()),
            },
            AcceptanceCheck {
                check_id: "helper-summary".into(),
                kind: "helper_summary_snapshot".into(),
                target: "bridge/helpers/local_inbox/summary.json".into(),
                expected: Some("1".into()),
            },
        ],
        wrapper_metadata: vec![
            "ChattyCogModuleSpec.json".into(),
            "manifest.json".into(),
            "visual_load.json".into(),
            "HANDSHAKE.md".into(),
            "bridge/status.json".into(),
            "bridge/log_sources.json".into(),
            "bridge/shared_room_state.json".into(),
            "bridge/shared_room_events.json".into(),
            "bridge/outgoing_room_events.json".into(),
            "bridge/incoming_assets/module_assets/.keep".into(),
        ],
        chattycog_hosting_mode: Some("hosted_webview".into()),
        chattycog_ui_owner: Some("module".into()),
        chattycog_bridge_capabilities: Some(module_spec.bridge.capabilities.clone()),
        helper_services: vec![helper_service.clone()],
        supported_patch_kinds: vec![
            "bridge_activity_panel".into(),
            "metric_strip".into(),
            "asset_inbox_panel".into(),
            "helper_summary_panel".into(),
            "helper_summary_badges".into(),
            "helper_summary_empty_state".into(),
            "helper_last_run_stamp".into(),
            "helper_summary_metadata_row".into(),
            "helper_summary_count_delta".into(),
            "helper_summary_lane_count_chip".into(),
            "helper_summary_types_chip".into(),
            "helper_summary_status_chip".into(),
            "helper_summary_updated_at_chip".into(),
            "helper_summary_filter_notice".into(),
            "lane_scoped_filter_notice".into(),
            "lane_scoped_metadata_row".into(),
            "helper_summary_discovered_notice".into(),
            "secondary_inbox_lane".into(),
            "helper_status_chip".into(),
            "processed_files_panel".into(),
            "auto_refresh_helper_panels".into(),
            "processed_file_preview_panel".into(),
            "processed_file_selection".into(),
            "file_type_filter".into(),
        ],
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
        family_id: Some(FamilyId::ChattycogWebviewModule),
        checks: vec![
            AcceptanceCheck {
                check_id: "index-exists".into(),
                kind: "exists".into(),
                target: "index.html".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "bridge-exists".into(),
                kind: "exists".into(),
                target: "chattycog_bridge.js".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "shared-room-events-exists".into(),
                kind: "exists".into(),
                target: "bridge/shared_room_events.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "module-contract".into(),
                kind: "chattycog_module_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "visual-load-contract".into(),
                kind: "chattycog_visual_load_contract".into(),
                target: "visual_load.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "bridge-contract".into(),
                kind: "chattycog_bridge_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "visual-load-exists".into(),
                kind: "exists".into(),
                target: "visual_load.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "bridge-reference".into(),
                kind: "contains".into(),
                target: "index.html".into(),
                expected: Some("chattycog_bridge.js".into()),
            },
            AcceptanceCheck {
                check_id: "manifest-module-id".into(),
                kind: "contains".into(),
                target: "manifest.json".into(),
                expected: Some(inputs.project_name.replace('-', "_")),
            },
            AcceptanceCheck {
                check_id: "family-marker".into(),
                kind: "contains".into(),
                target: "index.html".into(),
                expected: Some("data-family=\"chattycog_webview_module\"".into()),
            },
        ],
        required_files: vec![
            "index.html".into(),
            "app.js".into(),
            "styles.css".into(),
            "chattycog_bridge.js".into(),
            "bridge/status.json".into(),
            "bridge/log_sources.json".into(),
            "bridge/shared_room_events.json".into(),
            "bridge/helpers/local_inbox/status.json".into(),
            "bridge/helpers/local_inbox/summary.json".into(),
            "bridge/helpers/local_inbox/processed/sample-inbox-note.txt".into(),
            "helpers/local_inbox_helper/README.md".into(),
            "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
            "bridge/incoming_assets/module_assets/sample-inbox-note.txt".into(),
            "ChattyCogModuleSpec.json".into(),
            "manifest.json".into(),
            "visual_load.json".into(),
            "HANDSHAKE.md".into(),
            "ProjectSpec.json".into(),
            "AcceptancePlan.json".into(),
        ],
        required_markers: vec![
            "data-family=\"chattycog_webview_module\"".into(),
            "chattycog_bridge.js".into(),
        ],
        commands: vec!["static_file_check".into(), "wrapper_metadata_check".into()],
        expected_outputs: vec!["manifest.json".into(), "index.html".into()],
        helper_checks: Vec::new(),
        schema_checks: Vec::new(),
    };

    let helper_spec_json = to_string_pretty(&helper_service)?;
    let helper_status_json = to_string_pretty(&helper_status_snapshot)?;
    let helper_summary_json = helper_bundle.helper_summary_json;
    let helper_readme = helper_bundle.helper_readme;

    let files = vec![
        (
            "index.html",
            render_named(
                "families/chattycog_basic_dashboard/index.html",
                &family_context,
            )?,
        ),
        (
            "app.js",
            render_named("families/chattycog_basic_dashboard/app.js", &family_context)?,
        ),
        (
            "styles.css",
            render_named(
                "families/chattycog_basic_dashboard/styles.css",
                &family_context,
            )?,
        ),
        (
            "chattycog_bridge.js",
            render_named("wrappers/chattycog/chattycog_bridge.js", &wrapper_context)?,
        ),
        (
            "bridge/status.json",
            format!(
                "{{\n  \"module_id\": \"{}\",\n  \"event_type\": \"suspend_rundown\",\n  \"summary\": \"\",\n  \"snapshot\": \"\",\n  \"tags\": [],\n  \"payload\": {{}},\n  \"updated_at_unix_ms\": 0\n}}\n",
                module_id
            ),
        ),
        (
            "bridge/log_sources.json",
            "{\n  \"sources\": []\n}\n".into(),
        ),
        (
            "bridge/shared_room_state.json",
            "{\n  \"room_active\": false,\n  \"room_policy\": \"general\"\n}\n".into(),
        ),
        (
            "bridge/shared_room_events.json",
            "{\n  \"events\": []\n}\n".into(),
        ),
        (
            "bridge/outgoing_room_events.json",
            "{\n  \"events\": []\n}\n".into(),
        ),
        (
            "bridge/incoming_assets/module_assets/.keep",
            "module_assets inbox lane\n".into(),
        ),
        (
            "bridge/incoming_assets/module_assets/sample-inbox-note.txt",
            "helper inbox sample emitted by the rebuild\n".into(),
        ),
        (
            "bridge/helpers/local_inbox/status.json",
            helper_status_json,
        ),
        (
            "bridge/helpers/local_inbox/summary.json",
            helper_summary_json,
        ),
        ("helpers/local_inbox_helper/README.md", helper_readme),
        (
            "helpers/local_inbox_helper/HelperServiceSpec.json",
            helper_spec_json,
        ),
        (
            "manifest.json",
            render_named("wrappers/chattycog/manifest.json", &wrapper_context)?,
        ),
        (
            "visual_load.json",
            render_chattycog_visual_load_json(
                module_spec.visual_load.as_ref().expect("webview visual load"),
            )?,
        ),
        (
            "HANDSHAKE.md",
            render_named("wrappers/chattycog/HANDSHAKE.md", &wrapper_context)?,
        ),
        ("ChattyCogModuleSpec.json", to_string_pretty(&module_spec)?),
        ("ProjectSpec.json", to_string_pretty(&project_spec)?),
        ("AcceptancePlan.json", to_string_pretty(&acceptance_plan)?),
    ];

    write_project_files(&project_dir, files)
}

pub fn build_chattycog_native_window_module(
    output_root: &Path,
    inputs: &ScaffoldInputs,
) -> Result<BuildArtifacts> {
    if !matches!(
        inputs.family_id,
        Some(FamilyId::ChattycogNativeWindowModule)
    ) {
        bail!("chattycog native window builder requires family_id chattycog_native_window_module");
    }

    let project_dir = output_root.join(&inputs.project_name);
    fs::create_dir_all(project_dir.join("src"))?;

    let title = title_or_default(inputs, "ChattyCog Native Window Module");
    let summary = summary_or_default(
        inputs,
        "A deterministic standalone Rust GUI dashboard with removable Chatty-Cog compatibility plug files emitted by the ChattyFactory rebuild.",
    );
    let display_name = title.to_string();
    let module_id = inputs.project_name.replace('-', "_");
    let self_test_message = format!("{} native_window_self_test_ok", module_id);
    let package_name = inputs.project_name.clone();
    let executable_name = format!("{}.exe", package_name);
    let executable_path = format!("target/debug/{}", executable_name);
    let native_context = serde_json::json!({
        "package_name": package_name,
        "title": title,
        "summary": summary,
        "module_id": module_id,
        "display_name": display_name,
        "title_literal": serde_json::to_string(title)?,
        "summary_literal": serde_json::to_string(summary)?,
        "module_id_literal": serde_json::to_string(&module_id)?,
        "display_name_literal": serde_json::to_string(&display_name)?,
        "self_test_message_literal": serde_json::to_string(&self_test_message)?,
        "self_test_message": self_test_message,
    });

    let visual_load = ChattyCogVisualLoadSpec {
        kind: "native_window".into(),
        auto_launch: true,
        title: Some(display_name.clone()),
        notes: Some("ChattyFactory rebuild native Rust dashboard module".into()),
        file: None,
        url: None,
        window_title_contains: Some(display_name.clone()),
        build_command: Some(ChattyCogCommandSpec {
            program: "cargo".into(),
            args: vec!["build".into()],
            cwd: Some(".".into()),
        }),
        launch_command: Some(ChattyCogCommandSpec {
            program: executable_path.clone(),
            args: Vec::new(),
            cwd: Some(".".into()),
        }),
        serve_command: None,
        serve_wait_ms: None,
    };
    let module_spec = ChattyCogModuleSpec {
        module_spec_id: "chattyfactory.chattycog_module_spec.v1".into(),
        project_name: inputs.project_name.clone(),
        module_id: module_id.clone(),
        display_name: display_name.clone(),
        description: summary.to_string(),
        visual_kind: "native_window".into(),
        visual_title: display_name.clone(),
        visual_file: executable_path.clone(),
        handshake_path: "HANDSHAKE.md".into(),
        manifest_path: "manifest.json".into(),
        visual_load_path: Some("visual_load.json".into()),
        visual_load: Some(visual_load),
        bridge: ChattyCogBridgeSpec {
            status_path: "bridge/status.json".into(),
            script_path: None,
            log_sources_path: Some("bridge/log_sources.json".into()),
            capabilities: ChattyCogBridgeCapabilities {
                status_enabled: true,
                log_sources_enabled: true,
                shared_room_state_enabled: true,
                shared_room_events_enabled: true,
                outgoing_room_events_enabled: true,
                incoming_asset_lanes: vec!["module_assets".into()],
            },
            recommended_runtime_files: vec![
                "bridge/status.json".into(),
                "bridge/log_sources.json".into(),
                "bridge/shared_room_state.json".into(),
                "bridge/shared_room_events.json".into(),
                "bridge/outgoing_room_events.json".into(),
                "bridge/incoming_assets/module_assets".into(),
            ],
        },
        created_at: None,
    };
    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        family_id: Some(FamilyId::ChattycogNativeWindowModule),
        substrate: "rust_native_window_with_chattycog_wrapper".into(),
        tool_kind: Some("native_window_starter".into()),
        request_summary: Some(inputs.summary.clone()),
        entrypoints: vec!["Cargo.toml".into(), "src/main.rs".into()],
        expected_files: vec![
            "Cargo.toml".into(),
            "src/main.rs".into(),
            "README.md".into(),
            "STATE_TEMPLATE.md".into(),
            "manifest.json".into(),
            "visual_load.json".into(),
            "HANDSHAKE.md".into(),
            "bridge/status.json".into(),
            "bridge/log_sources.json".into(),
            "bridge/shared_room_state.json".into(),
            "bridge/shared_room_events.json".into(),
            "bridge/outgoing_room_events.json".into(),
            "bridge/incoming_assets/module_assets/.keep".into(),
            "network_capabilities.json".into(),
            "ChattyCogModuleSpec.json".into(),
            "ProjectSpec.json".into(),
            "AcceptancePlan.json".into(),
        ],
        features: inputs.feature_tokens.clone(),
        acceptance_commands: vec![
            "cargo_check".into(),
            "cargo_run_native_window_self_test".into(),
        ],
        acceptance_checks: vec![
            AcceptanceCheck {
                check_id: "native-cargo-manifest".into(),
                kind: "exists".into(),
                target: "Cargo.toml".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "native-main-entrypoint".into(),
                kind: "exists".into(),
                target: "src/main.rs".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "native-self-test-marker".into(),
                kind: "contains".into(),
                target: "src/main.rs".into(),
                expected: Some("native_window_self_test_ok".into()),
            },
            AcceptanceCheck {
                check_id: "native-module-contract".into(),
                kind: "chattycog_module_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "native-visual-load".into(),
                kind: "chattycog_visual_load_contract".into(),
                target: "visual_load.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "native-bridge-contract".into(),
                kind: "chattycog_bridge_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
        ],
        wrapper_metadata: vec![
            "manifest.json".into(),
            "visual_load.json".into(),
            "HANDSHAKE.md".into(),
            "bridge/status.json".into(),
            "bridge/log_sources.json".into(),
            "bridge/shared_room_state.json".into(),
            "bridge/shared_room_events.json".into(),
            "bridge/outgoing_room_events.json".into(),
            "bridge/incoming_assets/module_assets/.keep".into(),
            "network_capabilities.json".into(),
            "ChattyCogModuleSpec.json".into(),
        ],
        chattycog_hosting_mode: Some("hosted_native_window".into()),
        chattycog_ui_owner: Some("module".into()),
        chattycog_bridge_capabilities: Some(module_spec.bridge.capabilities.clone()),
        helper_services: Vec::new(),
        supported_patch_kinds: vec!["bridge_status_panel".into(), "ready_toggle".into()],
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
        family_id: Some(FamilyId::ChattycogNativeWindowModule),
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
            AcceptanceCheck {
                check_id: "native-self-test".into(),
                kind: "cargo_run_output_contains".into(),
                target: "-- --self-test".into(),
                expected: Some(self_test_message.clone()),
            },
            AcceptanceCheck {
                check_id: "module-contract".into(),
                kind: "chattycog_module_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "visual-load-contract".into(),
                kind: "chattycog_visual_load_contract".into(),
                target: "visual_load.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "bridge-contract".into(),
                kind: "chattycog_bridge_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
        ],
        required_files: project_spec.expected_files.clone(),
        required_markers: vec![
            "native_window_self_test_ok".into(),
            "// bridge_status_panel_anchor".into(),
            "// ready_toggle_anchor".into(),
        ],
        commands: vec![
            "cargo_check".into(),
            "cargo_run_native_window_self_test".into(),
        ],
        expected_outputs: vec!["visual_load.json".into(), "src/main.rs".into()],
        helper_checks: Vec::new(),
        schema_checks: Vec::new(),
    };

    let wrapper_context = serde_json::json!({
        "module_id": module_id,
        "display_name": display_name,
        "icon": "window",
        "description": summary,
        "visual_notes": "ChattyFactory rebuild native Rust dashboard module",
        "handshake_notes": "This family emits a standalone Rust GUI dashboard with removable Chatty-Cog compatibility files and a native-window hosting contract.",
    });
    let handshake_md = format!(
        "# {display_name}\n\nThis module was built by the ChattyFactory rebuild as a standalone Rust GUI dashboard with removable Chatty-Cog compatibility files.\n\n## Module identity\n\n- **module_id**: `{module_id}`\n- **display_name**: `{display_name}`\n\n## What this module is for\n\n{summary}\n\n## Inputs this module expects\n\n- plain-language module or dashboard requests\n- follow-up feature and patch requests\n- optional hosted Chatty-Cog bridge state when the module is docked inside Chatty-Cog\n\n## Outputs this module produces\n\n- a standalone Rust GUI dashboard\n- Chatty-Cog discovery and hosting metadata\n- a portable bridge status lane\n- deterministic project and acceptance contracts\n\n## Module Surface\n\n- standalone native window UI\n- Chatty-Cog manifest and visual load plug files\n- bridge stubs for status and room/event handoff\n- project contract and acceptance lane\n\n## Suspend rundown template\n\n> **Status:** blank native Rust dashboard shell is live and ready for feature work\n> **What changed:** standalone GUI substrate and Chatty-Cog compatibility plug files were emitted\n> **Open questions:** confirm the next module capability or patch lane to add\n> **Next action:** apply the next accepted deterministic patch or module feature bundle\n> **Artifacts:** `ProjectSpec.json`, `AcceptancePlan.json`, `bridge/status.json`, `network_capabilities.json`\n\n## Notes\n\nRemove the Chatty-Cog plug files if you want to keep the dashboard standalone-only.\n"
    );

    let files = vec![
        (
            "Cargo.toml",
            render_named(
                "families/chattycog_native_window_module/Cargo.toml",
                &native_context,
            )?,
        ),
        (
            "src/main.rs",
            render_named(
                "families/chattycog_native_window_module/main.rs",
                &native_context,
            )?,
        ),
        (
            "README.md",
            render_named(
                "families/chattycog_native_window_module/README.md",
                &native_context,
            )?,
        ),
        (
            "STATE_TEMPLATE.md",
            render_named(
                "families/chattycog_native_window_module/STATE_TEMPLATE.md",
                &native_context,
            )?,
        ),
        (
            "manifest.json",
            render_named("wrappers/chattycog/manifest.json", &wrapper_context)?,
        ),
        (
            "visual_load.json",
            render_chattycog_visual_load_json(
                module_spec.visual_load.as_ref().expect("native visual load"),
            )?,
        ),
        (
            "HANDSHAKE.md",
            handshake_md,
        ),
        (
            "bridge/status.json",
            format!(
                "{{\n  \"module_id\": \"{}\",\n  \"event_type\": \"suspend_rundown\",\n  \"summary\": \"Native Rust dashboard shell is ready for patch layering.\",\n  \"snapshot\": \"Hosted bridge contract is seeded and waiting for real module state.\",\n  \"tags\": [\"native_window\", \"rust_dashboard\"],\n  \"payload\": {{}},\n  \"updated_at_unix_ms\": 0\n}}\n",
                module_id
            ),
        ),
        ("bridge/log_sources.json", "{\n  \"sources\": []\n}\n".into()),
        (
            "bridge/shared_room_state.json",
            "{\n  \"room_active\": false,\n  \"room_policy\": \"general\"\n}\n".into(),
        ),
        (
            "bridge/shared_room_events.json",
            "{\n  \"events\": []\n}\n".into(),
        ),
        (
            "bridge/outgoing_room_events.json",
            "{\n  \"events\": []\n}\n".into(),
        ),
        (
            "bridge/incoming_assets/module_assets/.keep",
            "module_assets inbox lane\n".into(),
        ),
        (
            "network_capabilities.json",
            render_named(
                "families/chattycog_native_window_module/network_capabilities.json",
                &native_context,
            )?,
        ),
        ("ChattyCogModuleSpec.json", to_string_pretty(&module_spec)?),
        ("ProjectSpec.json", to_string_pretty(&project_spec)?),
        ("AcceptancePlan.json", to_string_pretty(&acceptance_plan)?),
    ];

    write_project_files(&project_dir, files)
}

pub fn build_chattyedu_native_window_module(
    output_root: &Path,
    inputs: &ScaffoldInputs,
) -> Result<BuildArtifacts> {
    if !matches!(
        inputs.family_id,
        Some(FamilyId::ChattyeduNativeWindowModule)
    ) {
        bail!("chattyedu native window builder requires family_id chattyedu_native_window_module");
    }

    let project_dir = output_root.join(&inputs.project_name);
    fs::create_dir_all(project_dir.join("src"))?;
    fs::create_dir_all(project_dir.join("bridge"))?;

    let title = title_or_default(inputs, "Chatty-EDU Native Rust Module");
    let summary = summary_or_default(
        inputs,
        "A deterministic standalone Rust GUI dashboard with removable Chatty-EDU compatibility plug files emitted by ChattyFactory.",
    );
    let display_name = title.to_string();
    let module_id = inputs.project_name.replace('-', "_");
    let self_test_message = format!("{} native_window_self_test_ok", module_id);
    let package_name = inputs.project_name.clone();
    let executable_name = format!("{}.exe", package_name);
    let executable_path = format!("target/debug/{}", executable_name);
    let native_context = serde_json::json!({
        "package_name": package_name,
        "title": title,
        "summary": summary,
        "module_id": module_id,
        "display_name": display_name,
        "title_literal": serde_json::to_string(title)?,
        "summary_literal": serde_json::to_string(summary)?,
        "module_id_literal": serde_json::to_string(&module_id)?,
        "display_name_literal": serde_json::to_string(&display_name)?,
        "self_test_message_literal": serde_json::to_string(&self_test_message)?,
        "self_test_message": self_test_message,
    });

    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        family_id: Some(FamilyId::ChattyeduNativeWindowModule),
        substrate: "rust_native_window_with_chattyedu_wrapper".into(),
        tool_kind: Some("native_window_starter".into()),
        request_summary: Some(inputs.summary.clone()),
        entrypoints: vec!["Cargo.toml".into(), "src/main.rs".into()],
        expected_files: vec![
            "Cargo.toml".into(),
            "src/main.rs".into(),
            "README.md".into(),
            "STATE_TEMPLATE.md".into(),
            "manifest.json".into(),
            "visual_load.json".into(),
            "HANDSHAKE.md".into(),
            "bridge/.gitkeep".into(),
            "network_capabilities.json".into(),
            "ChattyEduModuleSpec.json".into(),
            "ProjectSpec.json".into(),
            "AcceptancePlan.json".into(),
        ],
        features: inputs.feature_tokens.clone(),
        acceptance_commands: vec![
            "cargo_check".into(),
            "cargo_run_native_window_self_test".into(),
        ],
        acceptance_checks: vec![
            AcceptanceCheck {
                check_id: "native-cargo-manifest".into(),
                kind: "exists".into(),
                target: "Cargo.toml".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "native-main-entrypoint".into(),
                kind: "exists".into(),
                target: "src/main.rs".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "native-self-test-marker".into(),
                kind: "contains".into(),
                target: "src/main.rs".into(),
                expected: Some("native_window_self_test_ok".into()),
            },
            AcceptanceCheck {
                check_id: "chattyedu-bridge-env".into(),
                kind: "contains".into(),
                target: "src/main.rs".into(),
                expected: Some("CHATTYEDU_BRIDGE_STATUS".into()),
            },
            AcceptanceCheck {
                check_id: "visual-load-native-window".into(),
                kind: "contains".into(),
                target: "visual_load.json".into(),
                expected: Some("\"kind\": \"native_window\"".into()),
            },
        ],
        wrapper_metadata: vec![
            "manifest.json".into(),
            "visual_load.json".into(),
            "HANDSHAKE.md".into(),
            "bridge/.gitkeep".into(),
            "network_capabilities.json".into(),
            "ChattyEduModuleSpec.json".into(),
        ],
        chattycog_hosting_mode: None,
        chattycog_ui_owner: None,
        chattycog_bridge_capabilities: None,
        helper_services: Vec::new(),
        supported_patch_kinds: vec!["bridge_status_panel".into(), "ready_toggle".into()],
        patch_lanes: Vec::new(),
        acceptance_recipes: Vec::new(),
        operator_bundles: Vec::new(),
        updated_at: None,
    };
    let mut project_spec = project_spec;
    refresh_project_contract_views(&mut project_spec);

    let module_spec = ChattyEduModuleSpec {
        module_spec_id: "chattyfactory.chattyedu_module_spec.v1".into(),
        project_name: inputs.project_name.clone(),
        module_id: module_id.clone(),
        display_name: display_name.clone(),
        description: summary.to_string(),
        visual_kind: "native_window".into(),
        visual_title: display_name.clone(),
        visual_file: executable_path.clone(),
        handshake_path: "HANDSHAKE.md".into(),
        manifest_path: "manifest.json".into(),
        visual_load_path: "visual_load.json".into(),
        network_capabilities_path: "network_capabilities.json".into(),
        bridge_status_env_var: "CHATTYEDU_BRIDGE_STATUS".into(),
        created_at: None,
    };

    let acceptance_plan = AcceptancePlan {
        acceptance_id: format!("acceptance-{}", inputs.project_name),
        request_id: format!("request-{}", inputs.project_name),
        family_id: Some(FamilyId::ChattyeduNativeWindowModule),
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
                check_id: "manifest-exists".into(),
                kind: "exists".into(),
                target: "manifest.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "visual-load-exists".into(),
                kind: "exists".into(),
                target: "visual_load.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "chattyedu-module-contract".into(),
                kind: "chattyedu_module_contract".into(),
                target: "ChattyEduModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "chattyedu-visual-load-contract".into(),
                kind: "chattyedu_visual_load_contract".into(),
                target: "visual_load.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "handshake-exists".into(),
                kind: "exists".into(),
                target: "HANDSHAKE.md".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "cargo-check".into(),
                kind: "cargo_check".into(),
                target: ".".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "native-self-test".into(),
                kind: "cargo_run_output_contains".into(),
                target: "-- --self-test".into(),
                expected: Some(self_test_message.clone()),
            },
        ],
        required_files: project_spec.expected_files.clone(),
        required_markers: vec![
            "native_window_self_test_ok".into(),
            "CHATTYEDU_BRIDGE_STATUS".into(),
            "\"kind\": \"native_window\"".into(),
            "// bridge_status_panel_anchor".into(),
            "// ready_toggle_anchor".into(),
        ],
        commands: vec![
            "cargo_check".into(),
            "cargo_run_native_window_self_test".into(),
        ],
        expected_outputs: vec!["visual_load.json".into(), "src/main.rs".into()],
        helper_checks: Vec::new(),
        schema_checks: Vec::new(),
    };

    let manifest = serde_json::json!({
        "module_id": module_id,
        "display_name": display_name,
        "icon": "window",
        "description": summary,
    });
    let visual_load = serde_json::json!({
        "kind": "native_window",
        "auto_launch": true,
        "window_title_contains": display_name,
        "notes": "This starter keeps native app state inside the module folder and only uses the bridge for Chatty-EDU handoff.",
        "build": {
            "program": "cargo",
            "args": ["build"],
            "cwd": "."
        },
        "launch": {
            "program": executable_path,
            "cwd": "."
        }
    });
    let handshake_md = format!(
        "# {display_name} Handshake\n\n- **module_id**: `{module_id}`\n- **display_name**: `{display_name}`\n\n## What this module is for\n\n{summary}\n\n## Inputs this module expects\n\n- plain-language education or classroom workflow requests\n- follow-up feature and patch requests\n- optional hosted Chatty-EDU bridge state when the module is docked inside Chatty-EDU\n\n## Outputs this module produces\n\n- a standalone Rust GUI dashboard\n- Chatty-EDU discovery and hosting metadata\n- a portable bridge handoff lane through `CHATTYEDU_BRIDGE_STATUS`\n- deterministic project and acceptance contracts\n\n## Suspend rundown template\n\n> **Status:** current state in one sentence\n> **What changed:** short summary of recent progress\n> **Open questions:** anything still unresolved\n> **Next action:** the next useful move\n> **Artifacts:** files or outputs to revisit\n\n## Portable bridge note\n\nThis starter checks `CHATTYEDU_BRIDGE_STATUS` when hosted inside Chatty-EDU and writes a short handoff there. Outside Chatty-EDU it simply ignores the bridge and runs normally.\n"
    );

    let files = vec![
        (
            "Cargo.toml",
            render_named(
                "families/chattyedu_native_window_module/Cargo.toml",
                &native_context,
            )?,
        ),
        (
            "src/main.rs",
            render_named(
                "families/chattyedu_native_window_module/main.rs",
                &native_context,
            )?,
        ),
        (
            "README.md",
            render_named(
                "families/chattyedu_native_window_module/README.md",
                &native_context,
            )?,
        ),
        (
            "STATE_TEMPLATE.md",
            render_named(
                "families/chattyedu_native_window_module/STATE_TEMPLATE.md",
                &native_context,
            )?,
        ),
        ("manifest.json", to_string_pretty(&manifest)?),
        ("visual_load.json", to_string_pretty(&visual_load)?),
        ("HANDSHAKE.md", handshake_md),
        (
            "network_capabilities.json",
            render_named(
                "families/chattyedu_native_window_module/network_capabilities.json",
                &native_context,
            )?,
        ),
        ("bridge/.gitkeep", "".into()),
        ("ChattyEduModuleSpec.json", to_string_pretty(&module_spec)?),
        ("ProjectSpec.json", to_string_pretty(&project_spec)?),
        ("AcceptancePlan.json", to_string_pretty(&acceptance_plan)?),
    ];

    write_project_files(&project_dir, files)
}

pub fn build_chattycog_chattyedu_native_window_module(
    output_root: &Path,
    inputs: &ScaffoldInputs,
) -> Result<BuildArtifacts> {
    if !matches!(
        inputs.family_id,
        Some(FamilyId::ChattycogChattyeduNativeWindowModule)
    ) {
        bail!(
            "chattycog/chattyedu native window builder requires family_id chattycog_chattyedu_native_window_module"
        );
    }

    let project_dir = output_root.join(&inputs.project_name);
    fs::create_dir_all(project_dir.join("src"))?;

    let title = title_or_default(inputs, "Chatty-Cog + Chatty-EDU Native Rust Module");
    let summary = summary_or_default(
        inputs,
        "A deterministic standalone Rust GUI dashboard with both Chatty-Cog and Chatty-EDU compatibility plug files emitted by ChattyFactory.",
    );
    let display_name = title.to_string();
    let module_id = inputs.project_name.replace('-', "_");
    let self_test_message = format!("{} native_window_self_test_ok", module_id);
    let package_name = inputs.project_name.clone();
    let executable_name = format!("{}.exe", package_name);
    let executable_path = format!("target/debug/{}", executable_name);
    let native_context = serde_json::json!({
        "package_name": package_name,
        "title": title,
        "summary": summary,
        "module_id": module_id,
        "display_name": display_name,
        "title_literal": serde_json::to_string(title)?,
        "summary_literal": serde_json::to_string(summary)?,
        "module_id_literal": serde_json::to_string(&module_id)?,
        "display_name_literal": serde_json::to_string(&display_name)?,
        "self_test_message_literal": serde_json::to_string(&self_test_message)?,
        "self_test_message": self_test_message,
    });

    let visual_load = ChattyCogVisualLoadSpec {
        kind: "native_window".into(),
        auto_launch: true,
        title: Some(display_name.clone()),
        notes: Some(
            "ChattyFactory rebuild native Rust dashboard that can be hosted by either Chatty-Cog or Chatty-EDU.".into(),
        ),
        file: None,
        url: None,
        window_title_contains: Some(display_name.clone()),
        build_command: Some(ChattyCogCommandSpec {
            program: "cargo".into(),
            args: vec!["build".into()],
            cwd: Some(".".into()),
        }),
        launch_command: Some(ChattyCogCommandSpec {
            program: executable_path.clone(),
            args: Vec::new(),
            cwd: Some(".".into()),
        }),
        serve_command: None,
        serve_wait_ms: None,
    };
    let chattycog_module_spec = ChattyCogModuleSpec {
        module_spec_id: "chattyfactory.chattycog_module_spec.v1".into(),
        project_name: inputs.project_name.clone(),
        module_id: module_id.clone(),
        display_name: display_name.clone(),
        description: summary.to_string(),
        visual_kind: "native_window".into(),
        visual_title: display_name.clone(),
        visual_file: executable_path.clone(),
        handshake_path: "HANDSHAKE.md".into(),
        manifest_path: "manifest.json".into(),
        visual_load_path: Some("visual_load.json".into()),
        visual_load: Some(visual_load.clone()),
        bridge: ChattyCogBridgeSpec {
            status_path: "bridge/status.json".into(),
            script_path: None,
            log_sources_path: Some("bridge/log_sources.json".into()),
            capabilities: ChattyCogBridgeCapabilities {
                status_enabled: true,
                log_sources_enabled: true,
                shared_room_state_enabled: true,
                shared_room_events_enabled: true,
                outgoing_room_events_enabled: true,
                incoming_asset_lanes: vec!["module_assets".into()],
            },
            recommended_runtime_files: vec![
                "bridge/status.json".into(),
                "bridge/log_sources.json".into(),
                "bridge/shared_room_state.json".into(),
                "bridge/shared_room_events.json".into(),
                "bridge/outgoing_room_events.json".into(),
                "bridge/incoming_assets/module_assets".into(),
            ],
        },
        created_at: None,
    };
    let chattyedu_module_spec = ChattyEduModuleSpec {
        module_spec_id: "chattyfactory.chattyedu_module_spec.v1".into(),
        project_name: inputs.project_name.clone(),
        module_id: module_id.clone(),
        display_name: display_name.clone(),
        description: summary.to_string(),
        visual_kind: "native_window".into(),
        visual_title: display_name.clone(),
        visual_file: executable_path.clone(),
        handshake_path: "HANDSHAKE.md".into(),
        manifest_path: "manifest.json".into(),
        visual_load_path: "visual_load.json".into(),
        network_capabilities_path: "network_capabilities.json".into(),
        bridge_status_env_var: "CHATTYEDU_BRIDGE_STATUS".into(),
        created_at: None,
    };

    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        family_id: Some(FamilyId::ChattycogChattyeduNativeWindowModule),
        substrate: "rust_native_window_with_chattycog_and_chattyedu_wrappers".into(),
        tool_kind: Some("native_window_starter".into()),
        request_summary: Some(inputs.summary.clone()),
        entrypoints: vec!["Cargo.toml".into(), "src/main.rs".into()],
        expected_files: vec![
            "Cargo.toml".into(),
            "src/main.rs".into(),
            "README.md".into(),
            "STATE_TEMPLATE.md".into(),
            "manifest.json".into(),
            "visual_load.json".into(),
            "HANDSHAKE.md".into(),
            "bridge/status.json".into(),
            "bridge/log_sources.json".into(),
            "bridge/shared_room_state.json".into(),
            "bridge/shared_room_events.json".into(),
            "bridge/outgoing_room_events.json".into(),
            "bridge/incoming_assets/module_assets/.keep".into(),
            "network_capabilities.json".into(),
            "ChattyCogModuleSpec.json".into(),
            "ChattyEduModuleSpec.json".into(),
            "ProjectSpec.json".into(),
            "AcceptancePlan.json".into(),
        ],
        features: inputs.feature_tokens.clone(),
        acceptance_commands: vec![
            "cargo_check".into(),
            "cargo_run_native_window_self_test".into(),
        ],
        acceptance_checks: vec![
            AcceptanceCheck {
                check_id: "native-cargo-manifest".into(),
                kind: "exists".into(),
                target: "Cargo.toml".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "native-main-entrypoint".into(),
                kind: "exists".into(),
                target: "src/main.rs".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "native-self-test-marker".into(),
                kind: "contains".into(),
                target: "src/main.rs".into(),
                expected: Some("native_window_self_test_ok".into()),
            },
            AcceptanceCheck {
                check_id: "dual-chattycog-module-contract".into(),
                kind: "chattycog_module_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "dual-chattycog-bridge-contract".into(),
                kind: "chattycog_bridge_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "dual-chattyedu-module-contract".into(),
                kind: "chattyedu_module_contract".into(),
                target: "ChattyEduModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "dual-chattyedu-bridge-env".into(),
                kind: "contains".into(),
                target: "src/main.rs".into(),
                expected: Some("CHATTYEDU_BRIDGE_STATUS".into()),
            },
        ],
        wrapper_metadata: vec![
            "manifest.json".into(),
            "visual_load.json".into(),
            "HANDSHAKE.md".into(),
            "bridge/status.json".into(),
            "bridge/log_sources.json".into(),
            "bridge/shared_room_state.json".into(),
            "bridge/shared_room_events.json".into(),
            "bridge/outgoing_room_events.json".into(),
            "bridge/incoming_assets/module_assets/.keep".into(),
            "network_capabilities.json".into(),
            "ChattyCogModuleSpec.json".into(),
            "ChattyEduModuleSpec.json".into(),
        ],
        chattycog_hosting_mode: Some("hosted_native_window".into()),
        chattycog_ui_owner: Some("module".into()),
        chattycog_bridge_capabilities: Some(chattycog_module_spec.bridge.capabilities.clone()),
        helper_services: Vec::new(),
        supported_patch_kinds: vec!["bridge_status_panel".into(), "ready_toggle".into()],
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
        family_id: Some(FamilyId::ChattycogChattyeduNativeWindowModule),
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
            AcceptanceCheck {
                check_id: "native-self-test".into(),
                kind: "cargo_run_output_contains".into(),
                target: "-- --self-test".into(),
                expected: Some(self_test_message.clone()),
            },
            AcceptanceCheck {
                check_id: "chattycog-module-contract".into(),
                kind: "chattycog_module_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "chattycog-visual-load-contract".into(),
                kind: "chattycog_visual_load_contract".into(),
                target: "visual_load.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "chattycog-bridge-contract".into(),
                kind: "chattycog_bridge_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "chattyedu-module-contract".into(),
                kind: "chattyedu_module_contract".into(),
                target: "ChattyEduModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "chattyedu-visual-load-contract".into(),
                kind: "chattyedu_visual_load_contract".into(),
                target: "visual_load.json".into(),
                expected: None,
            },
        ],
        required_files: project_spec.expected_files.clone(),
        required_markers: vec![
            "native_window_self_test_ok".into(),
            "CHATTYEDU_BRIDGE_STATUS".into(),
            "\"kind\": \"native_window\"".into(),
            "// bridge_status_panel_anchor".into(),
            "// ready_toggle_anchor".into(),
        ],
        commands: vec![
            "cargo_check".into(),
            "cargo_run_native_window_self_test".into(),
        ],
        expected_outputs: vec!["visual_load.json".into(), "src/main.rs".into()],
        helper_checks: Vec::new(),
        schema_checks: Vec::new(),
    };

    let wrapper_context = serde_json::json!({
        "module_id": module_id,
        "display_name": display_name,
        "icon": "window",
        "description": summary,
    });
    let handshake_md = format!(
        "# {display_name}\n\nThis module was built by ChattyFactory as a standalone Rust GUI dashboard that can be dropped into either Chatty-Cog or Chatty-EDU.\n\n## Module identity\n\n- **module_id**: `{module_id}`\n- **display_name**: `{display_name}`\n\n## What this module is for\n\n{summary}\n\n## Host compatibility\n\n- standalone by default\n- prefilled Chatty-Cog native-window plug files\n- prefilled Chatty-EDU bridge contract support through `CHATTYEDU_BRIDGE_STATUS`\n- shared `manifest.json`, `visual_load.json`, and `HANDSHAKE.md`\n\n## Outputs this module produces\n\n- a standalone Rust GUI dashboard ready for deterministic patch layering\n- Chatty-Cog discovery and bridge metadata\n- Chatty-EDU discovery and bridge metadata\n- deterministic project and acceptance contracts\n\n## Suspend rundown template\n\n> **Status:** blank dual-host native dashboard shell is live and ready for feature work\n> **What changed:** standalone GUI substrate and both host compatibility layers were emitted\n> **Open questions:** confirm the next module capability or patch bundle to add\n> **Next action:** apply the next accepted deterministic patch or feature bundle\n> **Artifacts:** `ProjectSpec.json`, `AcceptancePlan.json`, `bridge/status.json`, `network_capabilities.json`\n\n## Notes\n\nRemove whichever host plug files you do not need if you want to keep the dashboard bound to only one ecosystem or fully standalone.\n"
    );

    let manifest_json = render_named("wrappers/chattycog/manifest.json", &wrapper_context)?;
    let visual_load_json = render_chattycog_visual_load_json(
        chattycog_module_spec
            .visual_load
            .as_ref()
            .expect("dual-host native visual load"),
    )?;

    let files = vec![
        (
            "Cargo.toml",
            render_named(
                "families/chattycog_chattyedu_native_window_module/Cargo.toml",
                &native_context,
            )?,
        ),
        (
            "src/main.rs",
            render_named(
                "families/chattycog_chattyedu_native_window_module/main.rs",
                &native_context,
            )?,
        ),
        (
            "README.md",
            render_named(
                "families/chattycog_chattyedu_native_window_module/README.md",
                &native_context,
            )?,
        ),
        (
            "STATE_TEMPLATE.md",
            render_named(
                "families/chattycog_chattyedu_native_window_module/STATE_TEMPLATE.md",
                &native_context,
            )?,
        ),
        ("manifest.json", manifest_json),
        ("visual_load.json", visual_load_json),
        ("HANDSHAKE.md", handshake_md),
        (
            "bridge/status.json",
            format!(
                "{{\n  \"module_id\": \"{}\",\n  \"event_type\": \"suspend_rundown\",\n  \"summary\": \"Dual-host native Rust dashboard shell is ready for patch layering.\",\n  \"snapshot\": \"Shared Chatty-Cog bridge contract is seeded and Chatty-EDU bridge env support is live.\",\n  \"tags\": [\"native_window\", \"rust_dashboard\", \"chattycog\", \"chattyedu\"],\n  \"payload\": {{\n    \"dual_host\": true,\n    \"chattyedu_bridge_env\": \"CHATTYEDU_BRIDGE_STATUS\"\n  }},\n  \"updated_at_unix_ms\": 0\n}}\n",
                module_id
            ),
        ),
        ("bridge/log_sources.json", "{\n  \"sources\": []\n}\n".into()),
        (
            "bridge/shared_room_state.json",
            "{\n  \"room_active\": false,\n  \"room_policy\": \"general\"\n}\n".into(),
        ),
        (
            "bridge/shared_room_events.json",
            "{\n  \"events\": []\n}\n".into(),
        ),
        (
            "bridge/outgoing_room_events.json",
            "{\n  \"events\": []\n}\n".into(),
        ),
        (
            "bridge/incoming_assets/module_assets/.keep",
            "module_assets inbox lane\n".into(),
        ),
        (
            "network_capabilities.json",
            render_named(
                "families/chattycog_chattyedu_native_window_module/network_capabilities.json",
                &native_context,
            )?,
        ),
        (
            "ChattyCogModuleSpec.json",
            to_string_pretty(&chattycog_module_spec)?,
        ),
        (
            "ChattyEduModuleSpec.json",
            to_string_pretty(&chattyedu_module_spec)?,
        ),
        ("ProjectSpec.json", to_string_pretty(&project_spec)?),
        ("AcceptancePlan.json", to_string_pretty(&acceptance_plan)?),
    ];

    write_project_files(&project_dir, files)
}

pub fn build_chattycog_workspace_module(
    output_root: &Path,
    inputs: &ScaffoldInputs,
) -> Result<BuildArtifacts> {
    if !matches!(inputs.family_id, Some(FamilyId::ChattycogWorkspaceModule)) {
        bail!("chattycog workspace builder requires family_id chattycog_workspace_module");
    }

    let project_dir = output_root.join(&inputs.project_name);
    fs::create_dir_all(&project_dir)?;

    let title = title_or_default(inputs, "ChattyCog Workspace Module");
    let summary = summary_or_default(
        inputs,
        "A deterministic ChattyCog workspace-module starter emitted by the ChattyFactory rebuild.",
    );
    let display_name = title.to_string();
    let module_id = inputs.project_name.replace('-', "_");

    let ui_json = serde_json::json!({
        "title": display_name,
        "description": summary,
        "sections": [
            {
                "id": "overview",
                "title": "Overview",
                "description": "ChattyCog-provided workspace surface for a module without its own hosted UI."
            }
        ],
        "fields": [
            {
                "id": "status",
                "label": "Status",
                "type": "text",
                "section": "overview",
                "default": "Workspace module starter ready"
            },
            {
                "id": "notes",
                "label": "Notes",
                "type": "multiline",
                "section": "overview",
                "default": "Use this workspace when the module does not ship its own webview or desktop window."
            }
        ]
    });

    let module_spec = ChattyCogModuleSpec {
        module_spec_id: "chattyfactory.chattycog_module_spec.v1".into(),
        project_name: inputs.project_name.clone(),
        module_id: module_id.clone(),
        display_name: display_name.clone(),
        description: summary.to_string(),
        visual_kind: "workspace".into(),
        visual_title: display_name.clone(),
        visual_file: "ui.json".into(),
        handshake_path: "HANDSHAKE.md".into(),
        manifest_path: "manifest.json".into(),
        visual_load_path: None,
        visual_load: None,
        bridge: ChattyCogBridgeSpec {
            status_path: "bridge/status.json".into(),
            script_path: None,
            log_sources_path: Some("bridge/log_sources.json".into()),
            capabilities: ChattyCogBridgeCapabilities {
                status_enabled: true,
                log_sources_enabled: true,
                shared_room_state_enabled: true,
                shared_room_events_enabled: true,
                outgoing_room_events_enabled: true,
                incoming_asset_lanes: vec!["module_assets".into()],
            },
            recommended_runtime_files: vec![
                "bridge/status.json".into(),
                "bridge/log_sources.json".into(),
                "bridge/shared_room_state.json".into(),
                "bridge/shared_room_events.json".into(),
                "bridge/outgoing_room_events.json".into(),
                "bridge/incoming_assets/module_assets".into(),
            ],
        },
        created_at: None,
    };
    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        family_id: Some(FamilyId::ChattycogWorkspaceModule),
        substrate: "chattycog_workspace_module".into(),
        tool_kind: Some("workspace_module".into()),
        request_summary: Some(inputs.summary.clone()),
        entrypoints: vec!["ui.json".into()],
        expected_files: vec![
            "ui.json".into(),
            "manifest.json".into(),
            "HANDSHAKE.md".into(),
            "bridge/status.json".into(),
            "bridge/log_sources.json".into(),
            "bridge/shared_room_state.json".into(),
            "bridge/shared_room_events.json".into(),
            "bridge/outgoing_room_events.json".into(),
            "bridge/incoming_assets/module_assets/.keep".into(),
            "ChattyCogModuleSpec.json".into(),
            "ProjectSpec.json".into(),
            "AcceptancePlan.json".into(),
        ],
        features: inputs.feature_tokens.clone(),
        acceptance_commands: vec!["workspace_module_contract".into()],
        acceptance_checks: vec![
            AcceptanceCheck {
                check_id: "workspace-module-contract".into(),
                kind: "chattycog_module_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "workspace-bridge-contract".into(),
                kind: "chattycog_bridge_contract".into(),
                target: "ChattyCogModuleSpec.json".into(),
                expected: None,
            },
            AcceptanceCheck {
                check_id: "workspace-ui-exists".into(),
                kind: "exists".into(),
                target: "ui.json".into(),
                expected: None,
            },
        ],
        wrapper_metadata: vec![
            "manifest.json".into(),
            "ui.json".into(),
            "HANDSHAKE.md".into(),
            "bridge/status.json".into(),
            "bridge/log_sources.json".into(),
            "bridge/shared_room_state.json".into(),
            "bridge/shared_room_events.json".into(),
            "bridge/outgoing_room_events.json".into(),
            "bridge/incoming_assets/module_assets/.keep".into(),
            "ChattyCogModuleSpec.json".into(),
        ],
        chattycog_hosting_mode: Some("workspace_surface".into()),
        chattycog_ui_owner: Some("chattycog".into()),
        chattycog_bridge_capabilities: Some(module_spec.bridge.capabilities.clone()),
        helper_services: Vec::new(),
        supported_patch_kinds: vec!["room_state_fields".into(), "session_overview".into()],
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
        family_id: Some(FamilyId::ChattycogWorkspaceModule),
        checks: project_spec.acceptance_checks.clone(),
        required_files: project_spec.expected_files.clone(),
        required_markers: vec!["Workspace module starter ready".into()],
        commands: vec!["workspace_module_contract".into()],
        expected_outputs: vec!["ui.json".into()],
        helper_checks: Vec::new(),
        schema_checks: Vec::new(),
    };

    let wrapper_context = serde_json::json!({
        "module_id": module_id,
        "display_name": display_name,
        "icon": "workspace",
        "description": summary,
        "visual_notes": "ChattyFactory rebuild workspace module",
        "handshake_notes": "This family emits a ChattyCog-hosted workspace module for tools that do not ship their own standalone hosted UI.",
    });

    let files = vec![
        ("ui.json", to_string_pretty(&ui_json)?),
        (
            "manifest.json",
            render_named("wrappers/chattycog/manifest.json", &wrapper_context)?,
        ),
        (
            "HANDSHAKE.md",
            render_named("wrappers/chattycog/HANDSHAKE.md", &wrapper_context)?,
        ),
        (
            "bridge/status.json",
            format!(
                "{{\n  \"module_id\": \"{}\",\n  \"event_type\": \"suspend_rundown\",\n  \"summary\": \"\",\n  \"snapshot\": \"\",\n  \"tags\": [\"workspace\"],\n  \"payload\": {{}},\n  \"updated_at_unix_ms\": 0\n}}\n",
                module_id
            ),
        ),
        ("bridge/log_sources.json", "{\n  \"sources\": []\n}\n".into()),
        (
            "bridge/shared_room_state.json",
            "{\n  \"room_active\": false,\n  \"room_policy\": \"general\"\n}\n".into(),
        ),
        (
            "bridge/shared_room_events.json",
            "{\n  \"events\": []\n}\n".into(),
        ),
        (
            "bridge/outgoing_room_events.json",
            "{\n  \"events\": []\n}\n".into(),
        ),
        (
            "bridge/incoming_assets/module_assets/.keep",
            "module_assets inbox lane\n".into(),
        ),
        ("ChattyCogModuleSpec.json", to_string_pretty(&module_spec)?),
        ("ProjectSpec.json", to_string_pretty(&project_spec)?),
        ("AcceptancePlan.json", to_string_pretty(&acceptance_plan)?),
    ];

    write_project_files(&project_dir, files)
}

pub fn build_python_cli_tool(
    output_root: &Path,
    inputs: &ScaffoldInputs,
) -> Result<BuildArtifacts> {
    if !matches!(inputs.family_id, Some(FamilyId::PythonCliTool)) {
        bail!("python cli builder requires family_id python_cli_tool");
    }
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
        family_id: Some(FamilyId::PythonCliTool),
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
        family_id: Some(FamilyId::PythonCliTool),
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
            render_named("families/python_cli_tool/main.py", &context)?,
        ),
        (
            "README.md",
            render_named("families/python_cli_tool/README.md", &context)?,
        ),
        ("ProjectSpec.json", to_string_pretty(&project_spec)?),
        ("AcceptancePlan.json", to_string_pretty(&acceptance_plan)?),
    ];
    for (path, contents) in fixture_files {
        files.push((path, contents));
    }

    write_project_files(&project_dir, files)
}

pub fn build_rust_cli_tool(output_root: &Path, inputs: &ScaffoldInputs) -> Result<BuildArtifacts> {
    if !matches!(inputs.family_id, Some(FamilyId::RustCliTool)) {
        bail!("rust cli builder requires family_id rust_cli_tool");
    }
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
            FamilyId::RustCliTool,
            "Observe a local inbox lane and emit deterministic helper status/output artifacts for a Rust CLI surface.",
            "Cross-family helper primitive bundle attached to a Rust CLI tool.",
        )?)
    } else {
        None
    };

    let project_spec = ProjectSpec {
        spec_id: "chattyfactory.project_spec.v2".into(),
        project_name: inputs.project_name.clone(),
        family_id: Some(FamilyId::RustCliTool),
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
        family_id: Some(FamilyId::RustCliTool),
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
            render_named("families/rust_cli_tool/Cargo.toml", &context)?,
        ),
        (
            "src/main.rs",
            render_named("families/rust_cli_tool/main.rs", &context)?,
        ),
        (
            "README.md",
            render_named("families/rust_cli_tool/README.md", &context)?,
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
    inputs: &ScaffoldInputs,
    artifacts: &BuildArtifacts,
    starter_override_id: Option<String>,
    starter_override_summary: Option<String>,
    recommended_starter_id: Option<String>,
    recommended_starter_summary: Option<String>,
    starter_recommendation_comparison: Option<String>,
) -> BuildReceipt {
    BuildReceipt {
        receipt_id: format!("build-{}", request_id),
        request_id: request_id.to_string(),
        family_id: inputs.family_id.clone(),
        starter_override_id,
        starter_override_summary,
        recommended_starter_id,
        recommended_starter_summary,
        starter_recommendation_comparison,
        project_name: inputs.project_name.clone(),
        project_dir: artifacts.project_dir.display().to_string(),
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
    let family_id = spec.family_id.as_ref().map(FamilyId::as_str);
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

    if let (Some(family_id), Some(patch_kind)) = (family_id, preferred_patch_kind) {
        if let Some(recipe) =
            patch_recipe_by_kind(family_id, tool_kind, project_features, patch_kind)
        {
            return Ok(Some((recipe.handler)(
                project_dir,
                project_name,
                request_id,
                raw_request,
            )?));
        }
    }

    if let Some(family_id) = family_id {
        if let Some(recipe) =
            patch_recipe_from_request_text(family_id, tool_kind, project_features, &lower_request)
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
    for recipe_id in preferred_patch_recipe_ids {
        let Some(family_id) = spec.family_id.as_ref().map(FamilyId::as_str) else {
            continue;
        };
        let project_features = spec.features.as_slice();
        for recipe in patch_recipe_registry() {
            if let Some(resolved) = patch_recipe_by_kind(
                family_id,
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
        family_id: Some(FamilyId::StaticWebDashboard),
        project_name: project_name.to_string(),
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
        family_id: Some(FamilyId::PythonCliTool),
        project_name: project_name.to_string(),
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
        family_id: Some(FamilyId::PythonCliTool),
        project_name: project_name.to_string(),
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
        family_id: Some(FamilyId::PythonCliTool),
        project_name: project_name.to_string(),
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
        family_id: Some(FamilyId::RustCliTool),
        project_name: project_name.to_string(),
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
        family_id: Some(FamilyId::RustCliTool),
        project_name: project_name.to_string(),
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
        family_id: Some(FamilyId::RustCliTool),
        project_name: project_name.to_string(),
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
        family_id: Some(FamilyId::RustCliTool),
        project_name: project_name.to_string(),
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

pub fn patch_chattycog_webview_bridge_activity_panel(
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

    if !index.contains("bridge-activity-panel") {
        let panel = "\n    <section class=\"module-panel bridge-activity-panel\" data-operator=\"bridge_activity_panel\">\n      <h2>Bridge Activity</h2>\n      <p>Hosted module bridge lanes are ready for room state, room events, outgoing room events, and the module_assets inbox.</p>\n    </section>\n";
        let needle = "    <section class=\"module-panel\">";
        if let Some(pos) = index.find(needle) {
            index.insert_str(pos, panel);
        } else {
            bail!("results panel marker missing in index.html");
        }
    }

    if !styles.contains(".bridge-activity-panel") {
        styles.push_str(
            "\n.bridge-activity-panel {\n  border-color: rgba(46, 118, 149, 0.35);\n  background: linear-gradient(135deg, rgba(46, 118, 149, 0.12), rgba(31, 111, 80, 0.08));\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "bridge-activity-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("bridge-activity-panel".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "bridge-activity-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("bridge-activity-panel".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "bridge-activity-panel");

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
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "bridge_activity_panel".into(),
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
            patch_kind: "bridge_activity_panel".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_metric_strip(
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

    if !index.contains("metric-strip") {
        let strip = "\n    <section class=\"module-grid metric-strip\" data-operator=\"metric_strip\">\n      <article class=\"module-panel metric-tile\"><h2>Room State</h2><p>Synced</p></article>\n      <article class=\"module-panel metric-tile\"><h2>Bridge Inbox</h2><p>module_assets</p></article>\n      <article class=\"module-panel metric-tile\"><h2>Events</h2><p>Live lanes ready</p></article>\n    </section>\n";
        let primary_needle = "    <section class=\"module-panel bridge-activity-panel\"";
        let fallback_needle = "    <section class=\"module-panel\">";
        if let Some(pos) = index.find(primary_needle) {
            index.insert_str(pos, strip);
        } else if let Some(pos) = index.find(fallback_needle) {
            index.insert_str(pos, strip);
        } else {
            bail!("module results marker missing in index.html");
        }
    }

    if !styles.contains(".metric-strip") {
        styles.push_str(
            "\n.metric-strip {\n  margin-bottom: 1.25rem;\n}\n.metric-tile h2 {\n  font-size: 0.9rem;\n  text-transform: uppercase;\n  letter-spacing: 0.08em;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "metric-strip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("metric-strip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "metric-strip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("metric-strip".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "metric-strip");

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
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "metric_strip".into(),
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
            patch_kind: "metric_strip".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_asset_inbox_panel(
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

    if !index.contains("asset-inbox-panel") {
        let inbox_label = spec
            .chattycog_bridge_capabilities
            .as_ref()
            .and_then(|bridge| bridge.incoming_asset_lanes.first().cloned())
            .unwrap_or_else(|| "module_assets".into());
        let panel = format!(
            "\n    <section class=\"module-panel asset-inbox-panel\" data-operator=\"asset_inbox_panel\" data-bridge-lane=\"{inbox_label}\">\n      <h2>Asset Inbox</h2>\n      <p>Incoming assets will arrive through the <strong>{inbox_label}</strong> lane.</p>\n      <div class=\"asset-inbox-chip\">lane: {inbox_label}</div>\n    </section>\n"
        );
        let needle = "    <section class=\"module-panel\">";
        if let Some(pos) = index.find(needle) {
            index.insert_str(pos, &panel);
        } else {
            bail!("results panel marker missing in index.html");
        }
    }

    if !styles.contains(".asset-inbox-panel") {
        styles.push_str(
            "\n.asset-inbox-panel {\n  border-color: rgba(35, 132, 99, 0.35);\n  background: linear-gradient(135deg, rgba(35, 132, 99, 0.12), rgba(12, 48, 32, 0.08));\n}\n\n.asset-inbox-chip {\n  display: inline-flex;\n  margin-top: 0.75rem;\n  padding: 0.3rem 0.75rem;\n  border-radius: 999px;\n  background: rgba(35, 132, 99, 0.14);\n  color: #d7f5e9;\n  font-size: 0.9rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "asset-inbox-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("asset-inbox-panel".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "asset-inbox-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("asset-inbox-panel".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "asset-inbox-panel-lane".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("module_assets".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "asset-inbox-panel");

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
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "asset_inbox_panel".into(),
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
            patch_kind: "asset_inbox_panel".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_panel(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");
    let summary_path = project_dir
        .join("bridge")
        .join("helpers")
        .join("local_inbox")
        .join("summary.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;
    let helper_summary_payload: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&summary_path)?)?;
    let expected_observed_count = helper_summary_payload
        .get("observed_file_count")
        .and_then(|value| value.as_u64())
        .unwrap_or(0)
        .to_string();

    if !index.contains("helper-summary-panel") {
        let panel = "\n    <section class=\"module-panel helper-summary-panel\" data-operator=\"helper_summary_panel\">\n      <h2>Helper Summary</h2>\n      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <ul id=\"helper-summary-files\" class=\"helper-summary-files\"></ul>\n    </section>\n";
        let primary_needle = "    <section class=\"module-panel asset-inbox-panel\"";
        let fallback_needle = "    <section class=\"module-panel\">";
        if let Some(pos) = index.find(primary_needle) {
            index.insert_str(pos, panel);
        } else if let Some(pos) = index.find(fallback_needle) {
            index.insert_str(pos, panel);
        } else {
            bail!("results panel marker missing in index.html");
        }
    }

    if !app_js.contains("helper-summary-status") {
        let helper_block = "\nconst helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryFiles = document.getElementById(\"helper-summary-files\");\n\nasync function loadHelperSummary() {\n  if (!helperSummaryStatus || !helperSummaryFiles) return;\n  try {\n    const response = await fetch(\"bridge/helpers/local_inbox/summary.json\");\n    if (!response.ok) {\n      throw new Error(`summary fetch failed: ${response.status}`);\n    }\n    const summary = await response.json();\n    const count = Number(summary.observed_file_count || 0);\n    helperSummaryStatus.textContent = `Observed ${count} inbox file(s) through ${summary.observed_lane || \"module_assets\"}.`;\n    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    helperSummaryFiles.innerHTML = files.map((name) => `<li>${name}</li>`).join(\"\") || \"<li>No helper outputs yet.</li>\";\n  } catch (_error) {\n    helperSummaryStatus.textContent = \"Helper summary is not available yet.\";\n    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n  }\n}\n";
        if app_js.contains("writeStatus();") {
            app_js = app_js.replacen("writeStatus();", "", 1);
            app_js.push_str(helper_block);
            app_js.push_str("\nwriteStatus();\nloadHelperSummary();\n");
        } else {
            bail!("app.js did not match expected helper summary patch shape");
        }
    }

    if !styles.contains(".helper-summary-panel") {
        styles.push_str(
            "\n.helper-summary-panel {\n  border-color: rgba(82, 132, 204, 0.38);\n  background: linear-gradient(135deg, rgba(82, 132, 204, 0.14), rgba(24, 43, 78, 0.1));\n}\n\n.helper-summary-files {\n  margin: 0.8rem 0 0;\n  padding-left: 1.1rem;\n  color: #dbe8ff;\n}\n\n.helper-summary-files li {\n  margin-bottom: 0.3rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-panel".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-fetch-marker".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("bridge/helpers/local_inbox/summary.json".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-panel".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-fetch-marker".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("bridge/helpers/local_inbox/summary.json".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-output".into(),
            kind: "helper_summary_snapshot".into(),
            target: "bridge/helpers/local_inbox/summary.json".into(),
            expected: Some(expected_observed_count),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "helper-summary-panel");
    ensure_marker(
        &mut acceptance.required_markers,
        "bridge/helpers/local_inbox/summary.json",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_panel".into(),
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
            patch_kind: "helper_summary_panel".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_badges(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-badges") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <ul id=\"helper-summary-files\" class=\"helper-summary-files\"></ul>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <div id=\"helper-summary-badges\" class=\"helper-summary-badges\"></div>\n      <ul id=\"helper-summary-files\" class=\"helper-summary-files\"></ul>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary panel markup missing expected badge insertion point");
        }
    }

    if !app_js.contains("helperSummaryBadges") {
        let old = r#"const helperSummaryStatus = document.getElementById("helper-summary-status");
const helperSummaryFiles = document.getElementById("helper-summary-files");

async function loadHelperSummary() {
  if (!helperSummaryStatus || !helperSummaryFiles) return;
"#;
        let new = r#"const helperSummaryStatus = document.getElementById("helper-summary-status");
const helperSummaryBadges = document.getElementById("helper-summary-badges");
const helperSummaryFiles = document.getElementById("helper-summary-files");

async function loadHelperSummary() {
  if (!helperSummaryStatus || !helperSummaryFiles) return;
"#;
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js did not match expected helper summary badge shape");
        }
    }

    let old_summary = r#"    const count = Number(summary.observed_file_count || 0);
    helperSummaryStatus.textContent = `Observed ${count} inbox file(s) through ${summary.observed_lane || "module_assets"}.`;
    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];
    helperSummaryFiles.innerHTML = files.map((name) => `<li>${name}</li>`).join("") || "<li>No helper outputs yet.</li>";
"#;
    let new_summary = r#"    const count = Number(summary.observed_file_count || 0);
    const filteredCount = Number(summary.filtered_out_file_count || 0);
    const allowedExtensions = Array.isArray(summary.allowed_extensions) ? summary.allowed_extensions : [];
    helperSummaryStatus.textContent = `Observed ${count} inbox file(s) through ${summary.observed_lane || "module_assets"}.`;
    if (helperSummaryBadges) {
      const badges = [];
      badges.push(`<span class="helper-summary-badge">accepted ${count}</span>`);
      badges.push(`<span class="helper-summary-badge">filtered ${filteredCount}</span>`);
      if (allowedExtensions.length) {
        badges.push(`<span class="helper-summary-badge">types ${allowedExtensions.join(", ")}</span>`);
      }
      helperSummaryBadges.innerHTML = badges.join("");
    }
    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];
    helperSummaryFiles.innerHTML = files.map((name) => `<li>${name}</li>`).join("") || "<li>No helper outputs yet.</li>";
"#;
    if app_js.contains(old_summary) {
        app_js = app_js.replacen(old_summary, new_summary, 1);
    }

    let old_error = r#"    helperSummaryStatus.textContent = "Helper summary is not available yet.";
    helperSummaryFiles.innerHTML = "<li>Waiting for bounded helper run.</li>";
"#;
    let new_error = r#"    helperSummaryStatus.textContent = "Helper summary is not available yet.";
    if (helperSummaryBadges) {
      helperSummaryBadges.innerHTML = "";
    }
    helperSummaryFiles.innerHTML = "<li>Waiting for bounded helper run.</li>";
"#;
    if app_js.contains(old_error) {
        app_js = app_js.replacen(old_error, new_error, 1);
    }

    if !styles.contains(".helper-summary-badges") {
        styles.push_str(
            "\n.helper-summary-badges {\n  display: flex;\n  flex-wrap: wrap;\n  gap: 0.5rem;\n  margin: 0.75rem 0 0.25rem;\n}\n\n.helper-summary-badge {\n  display: inline-flex;\n  align-items: center;\n  padding: 0.2rem 0.65rem;\n  border-radius: 999px;\n  background: rgba(90, 159, 255, 0.16);\n  border: 1px solid rgba(90, 159, 255, 0.32);\n  color: #dce8ff;\n  font-size: 0.82rem;\n  letter-spacing: 0.02em;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_badges");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-badges-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-badges".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-badges-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-badges".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-badges-loader".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryBadges".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-badges-styles".into(),
            kind: "contains".into(),
            target: "styles.css".into(),
            expected: Some(".helper-summary-badges".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "helper-summary-badges");
    ensure_marker(&mut acceptance.required_markers, "helperSummaryBadges");

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_badges".into(),
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
            patch_kind: "helper_summary_badges".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_empty_state(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-empty-state") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"helper-summary-empty-state\" class=\"helper-summary-empty-state\" hidden>No helper outputs observed yet.</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected empty-state insertion point");
        }
    }

    if !app_js.contains("helperSummaryEmptyState") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryEmptyState = document.getElementById(\"helper-summary-empty-state\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("helperSummaryEmptyState.hidden = count !== 0;") {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    if (helperSummaryEmptyState) {\n      helperSummaryEmptyState.hidden = count !== 0;\n      helperSummaryEmptyState.textContent = count === 0 ? \"No helper outputs observed yet.\" : \"\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file list insertion point");
        }
    }

    if !app_js
        .contains("helperSummaryEmptyState.textContent = \"Waiting for bounded helper run.\";")
    {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (helperSummaryEmptyState) {\n      helperSummaryEmptyState.hidden = false;\n      helperSummaryEmptyState.textContent = \"Waiting for bounded helper run.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-summary-empty-state") {
        styles.push_str(
            "\n.helper-summary-empty-state {\n  margin: 0.35rem 0 0.6rem;\n  color: #b8cae8;\n  font-size: 0.9rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_empty_state");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-empty-state-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-empty-state".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-empty-state-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryEmptyState".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-empty-state-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-empty-state".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-empty-state-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryEmptyState".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "helper-summary-empty-state",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_empty_state".into(),
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
            patch_kind: "helper_summary_empty_state".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_last_run_stamp(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-last-run-stamp") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"helper-last-run-stamp\" class=\"helper-last-run-stamp\">Last helper update unknown.</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected last-run insertion point");
        }
    }

    if !app_js.contains("helperLastRunStamp") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperLastRunStamp = document.getElementById(\"helper-last-run-stamp\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("helperLastRunStamp.textContent = `Last helper update") {
        let old = "    const count = Number(summary.observed_file_count || 0);\n";
        let new = "    const count = Number(summary.observed_file_count || 0);\n    const updatedAt = String(summary.updated_at || \"unknown\");\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary count binding");
        }

        let old_line = "    helperSummaryStatus.textContent = `Observed ${count} inbox file(s) through ${summary.observed_lane || \"module_assets\"}.`;\n";
        let new_line = "    helperSummaryStatus.textContent = `Observed ${count} inbox file(s) through ${summary.observed_lane || \"module_assets\"}.`;\n    if (helperLastRunStamp) {\n      helperLastRunStamp.textContent = `Last helper update ${updatedAt}.`;\n    }\n";
        if app_js.contains(old_line) {
            app_js = app_js.replacen(old_line, new_line, 1);
        } else {
            bail!("app.js missing expected helper summary status insertion point");
        }
    }

    if !app_js.contains("helperLastRunStamp.textContent = \"Last helper update unavailable.\";") {
        let old =
            "    helperSummaryStatus.textContent = \"Helper summary is not available yet.\";\n";
        let new = "    helperSummaryStatus.textContent = \"Helper summary is not available yet.\";\n    if (helperLastRunStamp) {\n      helperLastRunStamp.textContent = \"Last helper update unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-last-run-stamp") {
        styles.push_str(
            "\n.helper-last-run-stamp {\n  margin: 0.15rem 0 0.7rem;\n  color: #9eb2d3;\n  font-size: 0.84rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_last_run_stamp");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-last-run-stamp-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-last-run-stamp".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-last-run-stamp-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperLastRunStamp".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-last-run-stamp-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-last-run-stamp".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-last-run-stamp-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperLastRunStamp".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "helper-last-run-stamp");

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_last_run_stamp".into(),
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
            patch_kind: "helper_last_run_stamp".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_metadata_row(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-metadata") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <div id=\"helper-summary-metadata\" class=\"helper-summary-metadata\">Loading helper metadata...</div>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected metadata insertion point");
        }
    }

    if !app_js.contains("helperSummaryMetadata") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryMetadata = document.getElementById(\"helper-summary-metadata\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("helperSummaryMetadata.innerHTML = [") {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const allowedExtensions = Array.isArray(summary.allowed_extensions) ? summary.allowed_extensions : [];\n    const discoveredCount = Number(summary.discovered_file_count || files.length || 0);\n    const filteredCount = Number(summary.filtered_out_file_count || 0);\n    if (helperSummaryMetadata) {\n      const laneLabel = String(summary.observed_lane || \"module_assets\");\n      const typesLabel = allowedExtensions.length ? allowedExtensions.join(\", \") : \"all\";\n      helperSummaryMetadata.innerHTML = [\n        `<span class=\"helper-summary-meta-pill\">lane ${laneLabel}</span>`,\n        `<span class=\"helper-summary-meta-pill\">types ${typesLabel}</span>`,\n        `<span class=\"helper-summary-meta-pill\">discovered ${discoveredCount}</span>`,\n        `<span class=\"helper-summary-meta-pill\">filtered ${filteredCount}</span>`\n      ].join(\"\");\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js.contains("helperSummaryMetadata.textContent = \"Helper metadata unavailable.\";") {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (helperSummaryMetadata) {\n      helperSummaryMetadata.textContent = \"Helper metadata unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-summary-metadata") {
        styles.push_str(
            "\n.helper-summary-metadata {\n  display: flex;\n  flex-wrap: wrap;\n  gap: 0.45rem;\n  margin: 0.25rem 0 0.75rem;\n}\n\n.helper-summary-meta-pill {\n  display: inline-flex;\n  align-items: center;\n  padding: 0.18rem 0.6rem;\n  border-radius: 999px;\n  background: rgba(82, 132, 204, 0.16);\n  border: 1px solid rgba(82, 132, 204, 0.28);\n  color: #d8e7ff;\n  font-size: 0.8rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_metadata_row");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-metadata-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-metadata".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-metadata-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryMetadata".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-metadata-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-metadata".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-metadata-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryMetadata".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "helper-summary-metadata");

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_metadata_row".into(),
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
            patch_kind: "helper_summary_metadata_row".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_count_delta(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-count-delta") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"helper-summary-count-delta\" class=\"helper-summary-count-delta\">Checking helper count delta...</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected count delta insertion point");
        }
    }

    if !app_js.contains("helperSummaryCountDelta") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryCountDelta = document.getElementById(\"helper-summary-count-delta\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("helperSummaryCountDelta.textContent = deltaCount > 0") {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const acceptedCount = files.length;\n    const discoveredCount = Number(summary.discovered_file_count || acceptedCount || 0);\n    const deltaCount = Math.max(0, discoveredCount - acceptedCount);\n    if (helperSummaryCountDelta) {\n      helperSummaryCountDelta.textContent = deltaCount > 0\n        ? `Accepted ${acceptedCount} of ${discoveredCount} discovered helper file(s), leaving ${deltaCount} outside the output set.`\n        : `Accepted all ${acceptedCount} discovered helper file(s) into the output set.`;\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js
        .contains("helperSummaryCountDelta.textContent = \"Helper count delta unavailable.\";")
    {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (helperSummaryCountDelta) {\n      helperSummaryCountDelta.textContent = \"Helper count delta unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-summary-count-delta") {
        styles.push_str(
            "\n.helper-summary-count-delta {\n  margin: 0.2rem 0 0.7rem;\n  color: #d4e5ff;\n  font-size: 0.87rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_count_delta");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_count_delta",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_filter_notice");
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_metadata_row");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_discovered_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "secondary_inbox_lane");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-count-delta-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-count-delta".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-count-delta-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryCountDelta".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-count-delta-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-count-delta".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-count-delta-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryCountDelta".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "helper-summary-count-delta",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_count_delta".into(),
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
            patch_kind: "helper_summary_count_delta".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_lane_count_chip(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-lane-count-chip") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"helper-summary-lane-count-chip\" class=\"helper-summary-lane-count-chip\">Checking helper lane count...</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected lane count insertion point");
        }
    }

    if !app_js.contains("helperSummaryLaneCountChip") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryLaneCountChip = document.getElementById(\"helper-summary-lane-count-chip\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("helperSummaryLaneCountChip.textContent = laneCount > 1") {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const observedLanes = Array.isArray(summary.observed_lanes) ? summary.observed_lanes : [String(summary.observed_lane || \"module_assets\")];\n    const laneCount = observedLanes.filter(Boolean).length;\n    if (helperSummaryLaneCountChip) {\n      helperSummaryLaneCountChip.textContent = laneCount > 1\n        ? `Watching ${laneCount} helper inbox lanes.`\n        : `Watching ${laneCount} helper inbox lane.`;\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js
        .contains("helperSummaryLaneCountChip.textContent = \"Helper lane count unavailable.\";")
    {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (helperSummaryLaneCountChip) {\n      helperSummaryLaneCountChip.textContent = \"Helper lane count unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-summary-lane-count-chip") {
        styles.push_str(
            "\n.helper-summary-lane-count-chip {\n  display: inline-flex;\n  align-items: center;\n  padding: 0.18rem 0.6rem;\n  margin: 0.2rem 0 0.7rem;\n  border-radius: 999px;\n  background: rgba(86, 154, 214, 0.18);\n  border: 1px solid rgba(86, 154, 214, 0.3);\n  color: #d8ecff;\n  font-size: 0.82rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_lane_count_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_count_delta",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_lane_count_chip",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_filter_notice");
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_metadata_row");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_discovered_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "secondary_inbox_lane");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-lane-count-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-lane-count-chip".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-lane-count-chip-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryLaneCountChip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-lane-count-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-lane-count-chip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-lane-count-chip-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryLaneCountChip".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "helper-summary-lane-count-chip",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_lane_count_chip".into(),
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
            patch_kind: "helper_summary_lane_count_chip".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_types_chip(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-types-chip") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"helper-summary-types-chip\" class=\"helper-summary-types-chip\">Checking helper type rules...</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected types chip insertion point");
        }
    }

    if !app_js.contains("helperSummaryTypesChip") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryTypesChip = document.getElementById(\"helper-summary-types-chip\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("helperSummaryTypesChip.textContent = typeSegments.length") {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const globalTypes = Array.isArray(summary.allowed_extensions) ? summary.allowed_extensions : [];\n    const laneRules = summary.lane_allowed_extensions && typeof summary.lane_allowed_extensions === \"object\" ? summary.lane_allowed_extensions : {};\n    if (helperSummaryTypesChip) {\n      const laneNames = Object.keys(laneRules);\n      const typeSegments = laneNames.map((laneName) => {\n        const laneTypes = Array.isArray(laneRules[laneName]) ? laneRules[laneName] : [];\n        return `${laneName}: ${laneTypes.length ? laneTypes.join(\", \") : \"all\"}`;\n      });\n      if (!typeSegments.length) {\n        const globalLabel = globalTypes.length ? globalTypes.join(\", \") : \"all file types\";\n        helperSummaryTypesChip.textContent = `Helper types ${globalLabel}`;\n      } else {\n        helperSummaryTypesChip.textContent = `Helper types ${typeSegments.join(\" | \")}`;\n      }\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js.contains("helperSummaryTypesChip.textContent = \"Helper types unavailable.\";") {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (helperSummaryTypesChip) {\n      helperSummaryTypesChip.textContent = \"Helper types unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-summary-types-chip") {
        styles.push_str(
            "\n.helper-summary-types-chip {\n  display: inline-flex;\n  align-items: center;\n  padding: 0.18rem 0.6rem;\n  margin: 0.2rem 0 0.7rem;\n  border-radius: 999px;\n  background: rgba(96, 144, 214, 0.18);\n  border: 1px solid rgba(96, 144, 214, 0.3);\n  color: #d7ebff;\n  font-size: 0.82rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_types_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_count_delta",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_lane_count_chip",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_types_chip");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_filter_notice");
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_metadata_row");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_discovered_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "secondary_inbox_lane");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-types-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-types-chip".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-types-chip-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryTypesChip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-types-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-types-chip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-types-chip-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryTypesChip".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "helper-summary-types-chip",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_types_chip".into(),
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
            patch_kind: "helper_summary_types_chip".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_filter_notice(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-filter-notice") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"helper-summary-filter-notice\" class=\"helper-summary-filter-notice\">Checking helper filter state...</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected filter notice insertion point");
        }
    }

    if !app_js.contains("helperSummaryFilterNotice") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryFilterNotice = document.getElementById(\"helper-summary-filter-notice\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("helperSummaryFilterNotice.textContent = noticeFilteredCount > 0") {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const noticeAllowedExtensions = Array.isArray(summary.allowed_extensions) ? summary.allowed_extensions : [];\n    const noticeFilteredCount = Number(summary.filtered_out_file_count || 0);\n    if (helperSummaryFilterNotice) {\n      const noticeTypesLabel = noticeAllowedExtensions.length ? noticeAllowedExtensions.join(\", \") : \"all file types\";\n      helperSummaryFilterNotice.textContent = noticeFilteredCount > 0\n        ? `Filtered ${noticeFilteredCount} helper file(s) outside ${noticeTypesLabel}.`\n        : `No helper files filtered. Accepting ${noticeTypesLabel}.`;\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js
        .contains("helperSummaryFilterNotice.textContent = \"Helper filter notice unavailable.\";")
    {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (helperSummaryFilterNotice) {\n      helperSummaryFilterNotice.textContent = \"Helper filter notice unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-summary-filter-notice") {
        styles.push_str(
            "\n.helper-summary-filter-notice {\n  margin: 0.2rem 0 0.7rem;\n  color: #c8dbff;\n  font-size: 0.87rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_filter_notice");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-filter-notice-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-filter-notice".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-filter-notice-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryFilterNotice".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-filter-notice-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-filter-notice".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-filter-notice-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryFilterNotice".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "helper-summary-filter-notice",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_filter_notice".into(),
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
            patch_kind: "helper_summary_filter_notice".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_lane_scoped_filter_notice(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");
    let summary_path = project_dir
        .join("bridge")
        .join("helpers")
        .join("local_inbox")
        .join("summary.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;
    let helper_summary_payload: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&summary_path)?)?;
    let expected_observed_count = helper_summary_payload
        .get("observed_file_count")
        .and_then(|value| value.as_u64())
        .unwrap_or(0)
        .to_string();

    if !index.contains("lane-scoped-filter-notice") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"lane-scoped-filter-notice\" class=\"lane-scoped-filter-notice\">Checking lane filter rules...</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected lane-scoped filter insertion point");
        }
    }

    if !app_js.contains("laneScopedFilterNotice") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst laneScopedFilterNotice = document.getElementById(\"lane-scoped-filter-notice\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("laneScopedFilterNotice.textContent = laneFilterLines.length") {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const laneRules = summary.lane_allowed_extensions && typeof summary.lane_allowed_extensions === \"object\" ? summary.lane_allowed_extensions : {};\n    if (laneScopedFilterNotice) {\n      const observedLanes = Array.isArray(summary.observed_lanes) ? summary.observed_lanes : [String(summary.observed_lane || \"module_assets\")];\n      const laneFilterLines = observedLanes.map((laneName) => {\n        const laneExtensions = Array.isArray(laneRules[laneName]) ? laneRules[laneName] : [];\n        const laneRuleLabel = laneExtensions.length ? `${laneExtensions.join(\", \")} only` : \"all file types\";\n        return `${laneName}: ${laneRuleLabel}`;\n      });\n      laneScopedFilterNotice.textContent = laneFilterLines.length\n        ? `Lane rules ${laneFilterLines.join(\" | \")}`\n        : \"Lane rules unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js.contains("laneScopedFilterNotice.textContent = \"Lane filter rules unavailable.\";")
    {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (laneScopedFilterNotice) {\n      laneScopedFilterNotice.textContent = \"Lane filter rules unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".lane-scoped-filter-notice") {
        styles.push_str(
            "\n.lane-scoped-filter-notice {\n  margin: 0.2rem 0 0.7rem;\n  color: #d0e4ff;\n  font-size: 0.86rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "lane_scoped_filter_notice");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_filter_notice");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_discovered_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "secondary_inbox_lane");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "lane-scoped-filter-notice-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("lane-scoped-filter-notice".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "lane-scoped-filter-notice-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("laneScopedFilterNotice".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "lane-scoped-filter-notice-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("lane-scoped-filter-notice".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "lane-scoped-filter-notice-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("laneScopedFilterNotice".into()),
        },
    );
    update_check_expected(
        &mut acceptance.checks,
        "helper-summary-output",
        Some(expected_observed_count),
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "lane-scoped-filter-notice",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "lane_scoped_filter_notice".into(),
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
            patch_kind: "lane_scoped_filter_notice".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_lane_scoped_metadata_row(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("lane-scoped-metadata") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <div id=\"lane-scoped-metadata\" class=\"lane-scoped-metadata\">Loading lane metadata...</div>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected lane metadata insertion point");
        }
    }

    if !app_js.contains("laneScopedMetadata") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst laneScopedMetadata = document.getElementById(\"lane-scoped-metadata\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("laneScopedMetadata.innerHTML = lanePills.join(\"\");") {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const discoveredFiles = Array.isArray(summary.discovered_files) ? summary.discovered_files : [];\n    if (laneScopedMetadata) {\n      const observedLanes = Array.isArray(summary.observed_lanes) ? summary.observed_lanes : [String(summary.observed_lane || \"module_assets\")];\n      const lanePills = observedLanes.map((laneName) => {\n        const discoveredForLane = discoveredFiles.filter((name) => name.startsWith(`${laneName}/`)).length;\n        const acceptedForLane = files.filter((name) => name.startsWith(`${laneName}/`)).length;\n        return `<span class=\"lane-scoped-meta-pill\">${laneName}: ${acceptedForLane}/${discoveredForLane}</span>`;\n      });\n      laneScopedMetadata.innerHTML = lanePills.join(\"\");\n      if (!lanePills.length) {\n        laneScopedMetadata.textContent = \"Lane metadata unavailable.\";\n      }\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js.contains("laneScopedMetadata.textContent = \"Lane metadata unavailable.\";") {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (laneScopedMetadata) {\n      laneScopedMetadata.textContent = \"Lane metadata unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".lane-scoped-metadata") {
        styles.push_str(
            "\n.lane-scoped-metadata {\n  display: flex;\n  flex-wrap: wrap;\n  gap: 0.45rem;\n  margin: 0.25rem 0 0.75rem;\n}\n\n.lane-scoped-meta-pill {\n  display: inline-flex;\n  align-items: center;\n  padding: 0.18rem 0.6rem;\n  border-radius: 999px;\n  background: rgba(99, 146, 214, 0.16);\n  border: 1px solid rgba(99, 146, 214, 0.28);\n  color: #d8e8ff;\n  font-size: 0.8rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "lane_scoped_metadata_row");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_filter_notice");
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_metadata_row");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_discovered_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "secondary_inbox_lane");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "lane-scoped-metadata-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("lane-scoped-metadata".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "lane-scoped-metadata-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("laneScopedMetadata".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "lane-scoped-metadata-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("lane-scoped-metadata".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "lane-scoped-metadata-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("laneScopedMetadata".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "lane-scoped-metadata");

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "lane_scoped_metadata_row".into(),
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
            patch_kind: "lane_scoped_metadata_row".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_discovered_notice(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-discovered-notice") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"helper-summary-discovered-notice\" class=\"helper-summary-discovered-notice\">Checking discovered helper files...</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected discovered notice insertion point");
        }
    }

    if !app_js.contains("helperSummaryDiscoveredNotice") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryDiscoveredNotice = document.getElementById(\"helper-summary-discovered-notice\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js
        .contains("helperSummaryDiscoveredNotice.textContent = discoveredCount > acceptedCount")
    {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const acceptedCount = files.length;\n    const discoveredCount = Number(summary.discovered_file_count || acceptedCount || 0);\n    if (helperSummaryDiscoveredNotice) {\n      helperSummaryDiscoveredNotice.textContent = discoveredCount > acceptedCount\n        ? `Discovered ${discoveredCount} helper file(s), with ${acceptedCount} accepted for output.`\n        : `Discovered ${discoveredCount} helper file(s), all accepted for output.`;\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js.contains(
        "helperSummaryDiscoveredNotice.textContent = \"Helper discovery notice unavailable.\";",
    ) {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (helperSummaryDiscoveredNotice) {\n      helperSummaryDiscoveredNotice.textContent = \"Helper discovery notice unavailable.\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-summary-discovered-notice") {
        styles.push_str(
            "\n.helper-summary-discovered-notice {\n  margin: 0.2rem 0 0.7rem;\n  color: #d2e1ff;\n  font-size: 0.87rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_discovered_notice");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_discovered_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-discovered-notice-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-discovered-notice".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-discovered-notice-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryDiscoveredNotice".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-discovered-notice-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-discovered-notice".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-discovered-notice-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryDiscoveredNotice".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "helper-summary-discovered-notice",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_discovered_notice".into(),
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
            patch_kind: "helper_summary_discovered_notice".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_secondary_inbox_lane(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let helper_spec_path = project_dir
        .join("helpers")
        .join("local_inbox_helper")
        .join("HelperServiceSpec.json");
    let module_spec_path = project_dir.join("ChattyCogModuleSpec.json");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");
    let secondary_keep_path = project_dir
        .join("bridge")
        .join("incoming_assets")
        .join("secondary_assets")
        .join(".keep");
    let secondary_note_path = project_dir
        .join("bridge")
        .join("incoming_assets")
        .join("secondary_assets")
        .join("secondary-inbox-note.txt");

    let mut helper_spec: HelperServiceSpec =
        serde_json::from_str(&fs::read_to_string(&helper_spec_path)?)?;
    let mut module_spec: ChattyCogModuleSpec =
        serde_json::from_str(&fs::read_to_string(&module_spec_path)?)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    let secondary_lane = "bridge/incoming_assets/secondary_assets".to_string();
    let secondary_sample =
        "bridge/incoming_assets/secondary_assets/secondary-inbox-note.txt".to_string();
    let secondary_processed =
        "bridge/helpers/local_inbox/processed/secondary_assets/secondary-inbox-note.txt"
            .to_string();
    let module_processed =
        "bridge/helpers/local_inbox/processed/module_assets/sample-inbox-note.txt".to_string();
    let legacy_processed = "bridge/helpers/local_inbox/processed/sample-inbox-note.txt".to_string();

    ensure_marker(&mut helper_spec.input_paths, &secondary_lane);
    ensure_marker(
        &mut helper_spec.expected_files,
        "bridge/incoming_assets/secondary_assets/.keep",
    );
    ensure_marker(&mut helper_spec.expected_files, &secondary_sample);
    ensure_marker(&mut helper_spec.expected_files, &module_processed);
    ensure_marker(&mut helper_spec.expected_files, &secondary_processed);
    helper_spec
        .expected_files
        .retain(|existing| existing != &legacy_processed);
    if let Some(policy) = helper_spec.launch_policy.as_mut() {
        ensure_marker(
            &mut policy.expected_files,
            "bridge/incoming_assets/secondary_assets",
        );
        ensure_marker(&mut policy.expected_files, &secondary_sample);
    }
    if !helper_spec
        .notes
        .iter()
        .any(|note| note.contains("secondary inbox lane"))
    {
        helper_spec
            .notes
            .push("A secondary inbox lane is attached for multi-lane helper processing.".into());
    }
    upsert_helper_primitive(
        &mut helper_spec,
        HelperPrimitiveSpec {
            primitive_id: "secondary_assets_inbox_lane".into(),
            primitive_kind: "inbox_lane".into(),
            purpose: "Observe the secondary ChattyCog helper inbox lane.".into(),
            input_paths: vec![secondary_lane.clone()],
            output_paths: Vec::new(),
            status_paths: Vec::new(),
            dependency_mode: "standalone".into(),
            requires_primitives: Vec::new(),
            notes: vec!["Adds a second bounded input lane for helper composition.".into()],
            created_at: None,
        },
    );
    if let Some(primitive) = helper_spec
        .primitives
        .iter_mut()
        .find(|primitive| primitive.primitive_id == "local_inbox_processed_output")
    {
        ensure_marker(&mut primitive.input_paths, &secondary_lane);
        ensure_marker(
            &mut primitive.output_paths,
            "bridge/helpers/local_inbox/processed",
        );
        ensure_marker(
            &mut primitive.requires_primitives,
            "secondary_assets_inbox_lane",
        );
        ensure_marker(
            &mut primitive.notes,
            "Includes accepted files from the secondary helper inbox lane.",
        );
    }
    if let Some(primitive) = helper_spec
        .primitives
        .iter_mut()
        .find(|primitive| primitive.primitive_id == "local_inbox_summary_snapshot")
    {
        ensure_marker(&mut primitive.input_paths, &secondary_lane);
        ensure_marker(
            &mut primitive.notes,
            "Summarizes helper activity across the primary and secondary inbox lanes.",
        );
    }

    if !module_spec
        .bridge
        .capabilities
        .incoming_asset_lanes
        .iter()
        .any(|lane| lane == "secondary_assets")
    {
        module_spec
            .bridge
            .capabilities
            .incoming_asset_lanes
            .push("secondary_assets".into());
    }
    ensure_marker(
        &mut module_spec.bridge.recommended_runtime_files,
        "bridge/incoming_assets/secondary_assets",
    );

    ensure_marker(&mut spec.features, "secondary_inbox_lane");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_discovered_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "secondary_inbox_lane");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_marker(
        &mut spec.expected_files,
        "bridge/incoming_assets/secondary_assets/.keep",
    );
    ensure_marker(&mut spec.expected_files, &secondary_sample);
    ensure_marker(&mut spec.expected_files, &module_processed);
    ensure_marker(&mut spec.expected_files, &secondary_processed);
    spec.expected_files
        .retain(|existing| existing != &legacy_processed);
    if let Some(bridge_caps) = spec.chattycog_bridge_capabilities.as_mut() {
        ensure_marker(&mut bridge_caps.incoming_asset_lanes, "secondary_assets");
    }
    if let Some(project_helper) = spec
        .helper_services
        .iter_mut()
        .find(|project_helper| project_helper.helper_id == helper_spec.helper_id)
    {
        *project_helper = helper_spec.clone();
    }

    for checks in [&mut spec.acceptance_checks, &mut acceptance.checks] {
        ensure_check(
            checks,
            AcceptanceCheck {
                check_id: "secondary-inbox-lane-keep".into(),
                kind: "exists".into(),
                target: "bridge/incoming_assets/secondary_assets/.keep".into(),
                expected: None,
            },
        );
        ensure_check(
            checks,
            AcceptanceCheck {
                check_id: "secondary-inbox-lane-sample".into(),
                kind: "exists".into(),
                target: "bridge/incoming_assets/secondary_assets/secondary-inbox-note.txt".into(),
                expected: None,
            },
        );
        ensure_check(
            checks,
            AcceptanceCheck {
                check_id: "secondary-inbox-lane-spec".into(),
                kind: "contains".into(),
                target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
                expected: Some("bridge/incoming_assets/secondary_assets".into()),
            },
        );
        ensure_check(
            checks,
            AcceptanceCheck {
                check_id: "secondary-inbox-lane-summary".into(),
                kind: "contains".into(),
                target: "bridge/helpers/local_inbox/summary.json".into(),
                expected: Some("\"secondary_assets\"".into()),
            },
        );
    }
    update_check_expected(
        &mut spec.acceptance_checks,
        "helper-summary",
        Some("2".into()),
    );
    update_check_expected(
        &mut acceptance.checks,
        "helper-summary-output",
        Some("2".into()),
    );

    ensure_marker(&mut acceptance.required_markers, "\"secondary_assets\"");
    ensure_marker(&mut acceptance.required_markers, "secondary-inbox-note.txt");

    if let Some(parent) = secondary_keep_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&secondary_keep_path, "secondary_assets inbox lane\n")?;
    fs::write(
        &secondary_note_path,
        "secondary helper inbox sample emitted by the rebuild\n",
    )?;

    refresh_project_contract_views(&mut spec);
    fs::write(&helper_spec_path, to_string_pretty(&helper_spec)?)?;
    fs::write(&module_spec_path, to_string_pretty(&module_spec)?)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        helper_spec_path.clone(),
        module_spec_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
        secondary_keep_path.clone(),
        secondary_note_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "secondary_inbox_lane".into(),
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
            patch_kind: "secondary_inbox_lane".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_status_chip(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-status-chip") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"helper-summary-status-chip\" class=\"helper-summary-status-chip\">Checking helper summary status...</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected summary status chip insertion point");
        }
    }

    if !app_js.contains("helperSummaryStatusChip") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryStatusChip = document.getElementById(\"helper-summary-status-chip\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("helperSummaryStatusChip.dataset.summaryStatus") {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const summaryStatus = String(summary.status || \"unknown\");\n    if (helperSummaryStatusChip) {\n      helperSummaryStatusChip.textContent = `Summary status ${summaryStatus}`;\n      helperSummaryStatusChip.dataset.summaryStatus = summaryStatus;\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js.contains("helperSummaryStatusChip.dataset.summaryStatus = \"unavailable\";") {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (helperSummaryStatusChip) {\n      helperSummaryStatusChip.textContent = \"Summary status unavailable\";\n      helperSummaryStatusChip.dataset.summaryStatus = \"unavailable\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-summary-status-chip") {
        styles.push_str(
            "\n.helper-summary-status-chip {\n  display: inline-flex;\n  align-items: center;\n  padding: 0.18rem 0.6rem;\n  margin: 0.2rem 0 0.7rem;\n  border-radius: 999px;\n  background: rgba(82, 110, 156, 0.18);\n  border: 1px solid rgba(82, 110, 156, 0.32);\n  color: #e1ecff;\n  font-size: 0.82rem;\n  text-transform: capitalize;\n}\n\n.helper-summary-status-chip[data-summary-status=\"completed\"] {\n  background: rgba(56, 163, 104, 0.18);\n  border-color: rgba(56, 163, 104, 0.38);\n  color: #d9ffe8;\n}\n\n.helper-summary-status-chip[data-summary-status=\"unavailable\"] {\n  background: rgba(167, 88, 88, 0.18);\n  border-color: rgba(167, 88, 88, 0.34);\n  color: #ffdede;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_count_delta",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_lane_count_chip",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_types_chip");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_status_chip",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_filter_notice");
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_metadata_row");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_discovered_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "secondary_inbox_lane");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-status-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-status-chip".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-status-chip-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryStatusChip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-status-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-status-chip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-status-chip-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryStatusChip".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "helper-summary-status-chip",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_status_chip".into(),
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
            patch_kind: "helper_summary_status_chip".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_summary_updated_at_chip(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-summary-updated-at-chip") {
        let old = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n";
        let new = "      <p id=\"helper-summary-status\">Loading helper summary...</p>\n      <p id=\"helper-summary-updated-at-chip\" class=\"helper-summary-updated-at-chip\">Checking helper update time...</p>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary status missing expected updated-at chip insertion point");
        }
    }

    if !app_js.contains("helperSummaryUpdatedAtChip") {
        if app_js.contains(
            "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
        ) {
            app_js = app_js.replacen(
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\n",
                "const helperSummaryStatus = document.getElementById(\"helper-summary-status\");\nconst helperSummaryUpdatedAtChip = document.getElementById(\"helper-summary-updated-at-chip\");\n",
                1,
            );
        } else {
            bail!("app.js missing expected helper summary status binding");
        }
    }

    if !app_js.contains("helperSummaryUpdatedAtChip.textContent = `Summary updated ${updatedAt}`;")
    {
        let old = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n";
        let new = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    const updatedAt = String(summary.updated_at || \"unknown\");\n    if (helperSummaryUpdatedAtChip) {\n      helperSummaryUpdatedAtChip.textContent = `Summary updated ${updatedAt}`;\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary file insertion point");
        }
    }

    if !app_js
        .contains("helperSummaryUpdatedAtChip.textContent = \"Summary update time unavailable\";")
    {
        let old =
            "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n";
        let new = "    helperSummaryFiles.innerHTML = \"<li>Waiting for bounded helper run.</li>\";\n    if (helperSummaryUpdatedAtChip) {\n      helperSummaryUpdatedAtChip.textContent = \"Summary update time unavailable\";\n    }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        } else {
            bail!("app.js missing expected helper summary error insertion point");
        }
    }

    if !styles.contains(".helper-summary-updated-at-chip") {
        styles.push_str(
            "\n.helper-summary-updated-at-chip {\n  display: inline-flex;\n  align-items: center;\n  padding: 0.18rem 0.6rem;\n  margin: 0.2rem 0 0.7rem;\n  border-radius: 999px;\n  background: rgba(94, 124, 182, 0.18);\n  border: 1px solid rgba(94, 124, 182, 0.32);\n  color: #dfe9ff;\n  font-size: 0.82rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_summary_updated_at_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_empty_state",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_last_run_stamp");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_metadata_row",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_count_delta",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_lane_count_chip",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_types_chip");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_status_chip",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_updated_at_chip",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_filter_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_filter_notice");
    ensure_marker(&mut spec.supported_patch_kinds, "lane_scoped_metadata_row");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "helper_summary_discovered_notice",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "secondary_inbox_lane");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-updated-at-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-updated-at-chip".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-summary-updated-at-chip-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryUpdatedAtChip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-updated-at-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-summary-updated-at-chip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-summary-updated-at-chip-script".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperSummaryUpdatedAtChip".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "helper-summary-updated-at-chip",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_summary_updated_at_chip".into(),
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
            patch_kind: "helper_summary_updated_at_chip".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_helper_status_chip(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("helper-status-chip") {
        let old = "      <h2>Helper Summary</h2>\n";
        let new = "      <h2>Helper Summary</h2>\n      <div id=\"helper-status-chip\" class=\"helper-status-chip\">Loading status...</div>\n";
        if index.contains(old) {
            index = index.replacen(old, new, 1);
        } else {
            bail!("helper summary heading missing expected status chip insertion point");
        }
    }

    if !app_js.contains("helperStatusChip") {
        if app_js.contains(
            r#"const helperSummaryStatus = document.getElementById("helper-summary-status");
const helperSummaryBadges = document.getElementById("helper-summary-badges");
"#,
        ) {
            app_js = app_js.replacen(
                r#"const helperSummaryStatus = document.getElementById("helper-summary-status");
const helperSummaryBadges = document.getElementById("helper-summary-badges");
"#,
                r#"const helperSummaryStatus = document.getElementById("helper-summary-status");
const helperStatusChip = document.getElementById("helper-status-chip");
const helperSummaryBadges = document.getElementById("helper-summary-badges");
"#,
                1,
            );
        } else if app_js.contains(
            r#"const helperSummaryStatus = document.getElementById("helper-summary-status");
const helperSummaryFiles = document.getElementById("helper-summary-files");
"#,
        ) {
            app_js = app_js.replacen(
                r#"const helperSummaryStatus = document.getElementById("helper-summary-status");
const helperSummaryFiles = document.getElementById("helper-summary-files");
"#,
                r#"const helperSummaryStatus = document.getElementById("helper-summary-status");
const helperStatusChip = document.getElementById("helper-status-chip");
const helperSummaryFiles = document.getElementById("helper-summary-files");
"#,
                1,
            );
        } else {
            bail!("app.js did not match expected helper status chip shape");
        }
    }

    if !app_js.contains("helperStatusChip.dataset.helperStatus") {
        let old_summary_line = r#"    helperSummaryStatus.textContent = `Observed ${count} inbox file(s) through ${summary.observed_lane || "module_assets"}.`;
"#;
        let new_summary_line = r#"    helperSummaryStatus.textContent = `Observed ${count} inbox file(s) through ${summary.observed_lane || "module_assets"}.`;
    if (helperStatusChip) {
      const helperStatus = String(summary.status || "unknown");
      helperStatusChip.textContent = helperStatus;
      helperStatusChip.dataset.helperStatus = helperStatus;
    }
"#;
        if app_js.contains(old_summary_line) {
            app_js = app_js.replacen(old_summary_line, new_summary_line, 1);
        } else {
            bail!("app.js summary status line missing expected helper status chip insertion point");
        }
    }

    if !app_js.contains(r#"helperStatusChip.dataset.helperStatus = "unavailable";"#) {
        let old_error_line = r#"    helperSummaryStatus.textContent = "Helper summary is not available yet.";
"#;
        let new_error_line = r#"    helperSummaryStatus.textContent = "Helper summary is not available yet.";
    if (helperStatusChip) {
      helperStatusChip.textContent = "unavailable";
      helperStatusChip.dataset.helperStatus = "unavailable";
    }
"#;
        if app_js.contains(old_error_line) {
            app_js = app_js.replacen(old_error_line, new_error_line, 1);
        } else {
            bail!("app.js error status line missing expected helper status chip insertion point");
        }
    }

    if !styles.contains(".helper-status-chip") {
        styles.push_str(
            "\n.helper-status-chip {\n  display: inline-flex;\n  align-items: center;\n  gap: 0.35rem;\n  margin: 0.35rem 0 0.85rem;\n  padding: 0.2rem 0.7rem;\n  border-radius: 999px;\n  background: rgba(72, 110, 150, 0.18);\n  border: 1px solid rgba(72, 110, 150, 0.34);\n  color: #e3eeff;\n  font-size: 0.82rem;\n  text-transform: capitalize;\n}\n\n.helper-status-chip[data-helper-status=\"completed\"] {\n  background: rgba(56, 163, 104, 0.18);\n  border-color: rgba(56, 163, 104, 0.38);\n  color: #d9ffe8;\n}\n\n.helper-status-chip[data-helper-status=\"unavailable\"] {\n  background: rgba(167, 88, 88, 0.18);\n  border-color: rgba(167, 88, 88, 0.34);\n  color: #ffdede;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_badges");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_status_chip");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-status-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-status-chip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-status-chip-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("helper-status-chip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-status-chip-loader".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("helperStatusChip".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-status-chip-dataset".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("dataset.helperStatus".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-status-chip-styles".into(),
            kind: "contains".into(),
            target: "styles.css".into(),
            expected: Some(".helper-status-chip".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "helper-status-chip");
    ensure_marker(&mut acceptance.required_markers, "helperStatusChip");

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "helper_status_chip".into(),
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
            patch_kind: "helper_status_chip".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_processed_files_panel(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("processed-files-panel") {
        let panel = "\n    <section class=\"module-panel processed-files-panel\" data-operator=\"processed_files_panel\">\n      <h2>Processed Files</h2>\n      <p id=\"processed-files-status\">Loading processed helper files...</p>\n      <ul id=\"processed-files-list\" class=\"processed-files-list\"></ul>\n    </section>\n";
        let primary_needle = "    <section class=\"module-panel helper-summary-panel\"";
        let fallback_needle = "    <section class=\"module-panel\">";
        if let Some(pos) = index.find(primary_needle) {
            index.insert_str(pos, panel);
        } else if let Some(pos) = index.find(fallback_needle) {
            index.insert_str(pos, panel);
        } else {
            bail!("results panel marker missing in index.html");
        }
    }

    if !app_js.contains("processed-files-status") {
        let panel_block = "\nconst processedFilesStatus = document.getElementById(\"processed-files-status\");\nconst processedFilesList = document.getElementById(\"processed-files-list\");\n\nasync function loadProcessedFilesPanel() {\n  if (!processedFilesStatus || !processedFilesList) return;\n  try {\n    const response = await fetch(\"bridge/helpers/local_inbox/summary.json\");\n    if (!response.ok) {\n      throw new Error(`processed files summary fetch failed: ${response.status}`);\n    }\n    const summary = await response.json();\n    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    processedFilesStatus.textContent = files.length\n      ? `Processed ${files.length} helper file(s).`\n      : \"No processed helper files yet.\";\n    processedFilesList.innerHTML = files.map((name) => `<li data-processed-file=\"${name}\">${name}</li>`).join(\"\") || \"<li>No processed helper files yet.</li>\";\n  } catch (_error) {\n    processedFilesStatus.textContent = \"Processed helper files are not available yet.\";\n    processedFilesList.innerHTML = \"<li>Waiting for helper output.</li>\";\n  }\n}\n";
        if app_js.contains("loadHelperSummary();") {
            app_js = app_js.replacen("loadHelperSummary();", "loadHelperSummary();", 1);
            app_js.push_str(panel_block);
            app_js.push_str("\nloadProcessedFilesPanel();\n");
        } else if app_js.contains("writeStatus();") {
            app_js = app_js.replacen("writeStatus();", "", 1);
            app_js.push_str(panel_block);
            app_js.push_str("\nwriteStatus();\nloadProcessedFilesPanel();\n");
        } else {
            bail!("app.js did not match expected processed files patch shape");
        }
    }

    if !styles.contains(".processed-files-panel") {
        styles.push_str(
            "\n.processed-files-panel {\n  border-color: rgba(209, 163, 68, 0.38);\n  background: linear-gradient(135deg, rgba(209, 163, 68, 0.14), rgba(61, 46, 11, 0.1));\n}\n\n.processed-files-list {\n  margin: 0.8rem 0 0;\n  padding-left: 1.1rem;\n  color: #f4e7b8;\n}\n\n.processed-files-list li {\n  margin-bottom: 0.3rem;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "processed_files_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "processed-files-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("processed-files-panel".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "processed-files-fetch-marker".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("loadProcessedFilesPanel".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-files-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("processed-files-panel".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-files-fetch-marker".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("loadProcessedFilesPanel".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-files-helper-output".into(),
            kind: "exists".into(),
            target: "bridge/helpers/local_inbox/processed/sample-inbox-note.txt".into(),
            expected: None,
        },
    );
    ensure_marker(&mut acceptance.required_markers, "processed-files-panel");

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "processed_files_panel".into(),
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
            patch_kind: "processed_files_panel".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_auto_refresh_helper_panels(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let app_js_path = project_dir.join("app.js");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !app_js.contains("scheduleHelperPanelRefresh") {
        let refresh_block = "\nfunction scheduleHelperPanelRefresh() {\n  const refreshIntervalMs = 5000;\n  if (typeof window !== \"undefined\") {\n    window.setInterval(() => {\n      if (typeof loadHelperSummary === \"function\") {\n        loadHelperSummary();\n      }\n      if (typeof loadProcessedFilesPanel === \"function\") {\n        loadProcessedFilesPanel();\n      }\n    }, refreshIntervalMs);\n  }\n}\n";
        app_js.push_str(refresh_block);
        app_js.push_str("\nscheduleHelperPanelRefresh();\n");
    }

    ensure_marker(&mut spec.features, "auto_refresh_helper_panels");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-auto-refresh-marker".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("scheduleHelperPanelRefresh".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-auto-refresh-marker".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("scheduleHelperPanelRefresh".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-auto-refresh-timer".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("window.setInterval".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "scheduleHelperPanelRefresh",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&app_js_path, app_js)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        app_js_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "auto_refresh_helper_panels".into(),
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
            patch_kind: "auto_refresh_helper_panels".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_processed_file_preview_panel(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let index_path = project_dir.join("index.html");
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut index = fs::read_to_string(&index_path)?;
    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    if !index.contains("processed-file-preview-panel") {
        let panel = "\n    <section class=\"module-panel processed-file-preview-panel\" data-operator=\"processed_file_preview_panel\">\n      <h2>Processed File Preview</h2>\n      <p id=\"processed-file-preview-status\">Loading processed helper preview...</p>\n      <pre id=\"processed-file-preview-body\" class=\"processed-file-preview-body\">Waiting for helper output.</pre>\n    </section>\n";
        let primary_needle = "    <section class=\"module-panel processed-files-panel\"";
        let fallback_needle = "    <section class=\"module-panel\">";
        if let Some(pos) = index.find(primary_needle) {
            index.insert_str(pos, panel);
        } else if let Some(pos) = index.find(fallback_needle) {
            index.insert_str(pos, panel);
        } else {
            bail!("results panel marker missing in index.html");
        }
    }

    if !app_js.contains("processed-file-preview-status") {
        let panel_block = "\nconst processedFilePreviewStatus = document.getElementById(\"processed-file-preview-status\");\nconst processedFilePreviewBody = document.getElementById(\"processed-file-preview-body\");\n\nasync function loadProcessedFilePreview() {\n  if (!processedFilePreviewStatus || !processedFilePreviewBody) return;\n  try {\n    const response = await fetch(\"bridge/helpers/local_inbox/summary.json\");\n    if (!response.ok) {\n      throw new Error(`processed preview summary fetch failed: ${response.status}`);\n    }\n    const summary = await response.json();\n    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    if (!files.length) {\n      processedFilePreviewStatus.textContent = \"No processed helper files available for preview.\";\n      processedFilePreviewBody.textContent = \"Waiting for helper output.\";\n      return;\n    }\n    const firstFile = files[0];\n    const previewResponse = await fetch(`bridge/helpers/local_inbox/processed/${firstFile}`);\n    if (!previewResponse.ok) {\n      throw new Error(`processed preview fetch failed: ${previewResponse.status}`);\n    }\n    const previewText = await previewResponse.text();\n    processedFilePreviewStatus.textContent = `Previewing ${firstFile}`;\n    processedFilePreviewBody.textContent = previewText;\n  } catch (_error) {\n    processedFilePreviewStatus.textContent = \"Processed helper preview is not available yet.\";\n    processedFilePreviewBody.textContent = \"Waiting for helper output.\";\n  }\n}\n";
        app_js.push_str(panel_block);
        app_js.push_str("\nloadProcessedFilePreview();\n");
    }

    if app_js.contains("scheduleHelperPanelRefresh")
        && !app_js.contains("typeof loadProcessedFilePreview === \"function\"")
    {
        let old = "      if (typeof loadProcessedFilesPanel === \"function\") {\n        loadProcessedFilesPanel();\n      }\n";
        let new = "      if (typeof loadProcessedFilesPanel === \"function\") {\n        loadProcessedFilesPanel();\n      }\n      if (typeof loadProcessedFilePreview === \"function\") {\n        loadProcessedFilePreview();\n      }\n";
        if app_js.contains(old) {
            app_js = app_js.replacen(old, new, 1);
        }
    }

    if !styles.contains(".processed-file-preview-panel") {
        styles.push_str(
            "\n.processed-file-preview-panel {\n  border-color: rgba(154, 106, 214, 0.38);\n  background: linear-gradient(135deg, rgba(154, 106, 214, 0.14), rgba(48, 26, 84, 0.1));\n}\n\n.processed-file-preview-body {\n  margin-top: 0.8rem;\n  padding: 0.9rem;\n  border-radius: 0.75rem;\n  background: rgba(15, 18, 27, 0.7);\n  color: #efe5ff;\n  white-space: pre-wrap;\n  word-break: break-word;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "processed_file_preview_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "processed-file-preview-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("processed-file-preview-panel".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "processed-file-preview-loader-marker".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("loadProcessedFilePreview".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-file-preview-panel-marker".into(),
            kind: "contains".into(),
            target: "index.html".into(),
            expected: Some("processed-file-preview-panel".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-file-preview-loader-marker".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("loadProcessedFilePreview".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-file-preview-source".into(),
            kind: "exists".into(),
            target: "bridge/helpers/local_inbox/processed/sample-inbox-note.txt".into(),
            expected: None,
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "processed-file-preview-panel",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&index_path, index)?;
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        index_path.clone(),
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "processed_file_preview_panel".into(),
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
            patch_kind: "processed_file_preview_panel".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_processed_file_selection(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let app_js_path = project_dir.join("app.js");
    let styles_path = project_dir.join("styles.css");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut app_js = fs::read_to_string(&app_js_path)?;
    let mut styles = fs::read_to_string(&styles_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    let selection_block = "\nlet selectedProcessedFile = null;\n\nfunction setSelectedProcessedFile(fileName) {\n  selectedProcessedFile = fileName || null;\n  if (typeof loadProcessedFilesPanel === \"function\") {\n    loadProcessedFilesPanel();\n  }\n  if (typeof loadProcessedFilePreview === \"function\") {\n    loadProcessedFilePreview();\n  }\n}\n";
    if app_js.contains("let selectedProcessedFile = null;") {
        app_js = app_js.replacen(selection_block, "\n", 1);
    }
    if app_js.contains("\nconst processedFilesStatus =") {
        app_js = app_js.replacen(
            "\nconst processedFilesStatus =",
            &format!("{selection_block}\nconst processedFilesStatus ="),
            1,
        );
    } else if app_js.contains("\nconst processedFilePreviewStatus =") {
        app_js = app_js.replacen(
            "\nconst processedFilePreviewStatus =",
            &format!("{selection_block}\nconst processedFilePreviewStatus ="),
            1,
        );
    } else {
        app_js.push_str(selection_block);
    }

    let old_files_block = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    processedFilesStatus.textContent = files.length\n      ? `Processed ${files.length} helper file(s).`\n      : \"No processed helper files yet.\";\n    processedFilesList.innerHTML = files.map((name) => `<li data-processed-file=\"${name}\">${name}</li>`).join(\"\") || \"<li>No processed helper files yet.</li>\";\n";
    let new_files_block = "    const files = Array.isArray(summary.observed_files) ? summary.observed_files : [];\n    if (selectedProcessedFile && !files.includes(selectedProcessedFile)) {\n      selectedProcessedFile = files[0] || null;\n    }\n    if (!selectedProcessedFile && files.length) {\n      selectedProcessedFile = files[0];\n    }\n    processedFilesStatus.textContent = files.length\n      ? `Processed ${files.length} helper file(s). Select a file to preview.`\n      : \"No processed helper files yet.\";\n    processedFilesList.innerHTML = files.map((name) => {\n      const selectedClass = selectedProcessedFile === name ? ' class=\"is-selected\"' : '';\n      return `<li${selectedClass} data-processed-file=\"${name}\"><button type=\"button\" data-select-processed-file=\"${name}\">${name}</button></li>`;\n    }).join(\"\") || \"<li>No processed helper files yet.</li>\";\n    const processedFileButtons = processedFilesList.querySelectorAll(\"[data-select-processed-file]\");\n    processedFileButtons.forEach((button) => {\n      button.addEventListener(\"click\", () => {\n        setSelectedProcessedFile(button.getAttribute(\"data-select-processed-file\") || \"\");\n      });\n    });\n";
    if app_js.contains(old_files_block) {
        app_js = app_js.replacen(old_files_block, new_files_block, 1);
    }

    let old_preview_block = "    const firstFile = files[0];\n    const previewResponse = await fetch(`bridge/helpers/local_inbox/processed/${firstFile}`);\n    if (!previewResponse.ok) {\n      throw new Error(`processed preview fetch failed: ${previewResponse.status}`);\n    }\n    const previewText = await previewResponse.text();\n    processedFilePreviewStatus.textContent = `Previewing ${firstFile}`;\n    processedFilePreviewBody.textContent = previewText;\n";
    let new_preview_block = "    if (selectedProcessedFile && !files.includes(selectedProcessedFile)) {\n      selectedProcessedFile = files[0] || null;\n    }\n    const fileToPreview = selectedProcessedFile || files[0];\n    const previewResponse = await fetch(`bridge/helpers/local_inbox/processed/${fileToPreview}`);\n    if (!previewResponse.ok) {\n      throw new Error(`processed preview fetch failed: ${previewResponse.status}`);\n    }\n    const previewText = await previewResponse.text();\n    processedFilePreviewStatus.textContent = `Previewing ${fileToPreview}`;\n    processedFilePreviewBody.textContent = previewText;\n";
    if app_js.contains(old_preview_block) {
        app_js = app_js.replacen(old_preview_block, new_preview_block, 1);
    }

    if !styles.contains(".processed-files-list li.is-selected") {
        styles.push_str(
            "\n.processed-files-list li.is-selected {\n  list-style: disc;\n}\n\n.processed-files-list button {\n  border: 0;\n  background: transparent;\n  color: inherit;\n  padding: 0;\n  cursor: pointer;\n  font: inherit;\n  text-align: left;\n}\n\n.processed-files-list li.is-selected button {\n  color: #fff7d6;\n  font-weight: 700;\n}\n",
        );
    }

    ensure_marker(&mut spec.features, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "processed_file_selection");
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "processed-file-selection-state".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("let selectedProcessedFile = null;".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "processed-file-selection-handler".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("data-select-processed-file".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-file-selection-state".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("let selectedProcessedFile = null;".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-file-selection-handler".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("data-select-processed-file".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-file-selection-preview".into(),
            kind: "contains".into(),
            target: "app.js".into(),
            expected: Some("const fileToPreview = selectedProcessedFile || files[0];".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "processed-file-selection-style".into(),
            kind: "contains".into(),
            target: "styles.css".into(),
            expected: Some(".processed-files-list li.is-selected".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "data-select-processed-file",
    );
    ensure_marker(&mut acceptance.required_markers, "selectedProcessedFile");

    refresh_project_contract_views(&mut spec);
    fs::write(&app_js_path, app_js)?;
    fs::write(&styles_path, styles)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        app_js_path.clone(),
        styles_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "processed_file_selection".into(),
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
            patch_kind: "processed_file_selection".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_webview_file_type_filter(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let helper_spec_path = project_dir
        .join("helpers")
        .join("local_inbox_helper")
        .join("HelperServiceSpec.json");
    let ignored_input_path = project_dir
        .join("bridge")
        .join("incoming_assets")
        .join("module_assets")
        .join("sample-ignore.json");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut helper_spec: HelperServiceSpec =
        serde_json::from_str(&fs::read_to_string(&helper_spec_path)?)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    helper_spec.allowed_extensions.clear();
    helper_spec
        .lane_allowed_extensions
        .insert("module_assets".into(), vec![".txt".into()]);
    if !helper_spec
        .notes
        .iter()
        .any(|note| note.contains("Only .txt inbox files from module_assets should be processed"))
    {
        helper_spec.notes.push(
            "Only .txt inbox files from module_assets should be processed by the bounded helper runner."
                .into(),
        );
    }
    upsert_helper_primitive(
        &mut helper_spec,
        HelperPrimitiveSpec {
            primitive_id: "module_assets_file_type_filter".into(),
            primitive_kind: "file_filter".into(),
            purpose: "Restrict the primary helper inbox lane to .txt files.".into(),
            input_paths: vec!["bridge/incoming_assets/module_assets".into()],
            output_paths: vec!["bridge/helpers/local_inbox/processed".into()],
            status_paths: vec!["bridge/helpers/local_inbox/summary.json".into()],
            dependency_mode: "requires_primitives".into(),
            requires_primitives: vec!["module_assets_inbox_lane".into()],
            notes: vec![
                "Applies a lane-scoped .txt filter to the primary helper inbox lane.".into(),
            ],
            created_at: None,
        },
    );
    if let Some(project_helper) = spec
        .helper_services
        .iter_mut()
        .find(|project_helper| project_helper.helper_id == helper_spec.helper_id)
    {
        project_helper.allowed_extensions = helper_spec.allowed_extensions.clone();
        project_helper.lane_allowed_extensions = helper_spec.lane_allowed_extensions.clone();
        project_helper.primitives = helper_spec.primitives.clone();
        if !project_helper.notes.iter().any(|note| {
            note.contains("Only .txt inbox files from module_assets should be processed")
        }) {
            project_helper.notes.push(
                "Only .txt inbox files from module_assets should be processed by the bounded helper runner.".into(),
            );
        }
    }

    fs::write(
        &ignored_input_path,
        "{\n  \"ignored\": true,\n  \"reason\": \"file_type_filter deterministic patch sample\"\n}\n",
    )?;

    ensure_marker(&mut spec.features, "file_type_filter");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_activity_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "metric_strip");
    ensure_marker(&mut spec.supported_patch_kinds, "asset_inbox_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "helper_summary_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "processed_files_panel");
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "auto_refresh_helper_panels",
    );
    ensure_marker(
        &mut spec.supported_patch_kinds,
        "processed_file_preview_panel",
    );
    ensure_marker(&mut spec.supported_patch_kinds, "file_type_filter");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-spec".into(),
            kind: "contains".into(),
            target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
            expected: Some("\"lane_allowed_extensions\": {".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-lane".into(),
            kind: "contains".into(),
            target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
            expected: Some("\"module_assets\"".into()),
        },
    );
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-extension".into(),
            kind: "contains".into(),
            target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
            expected: Some("\".txt\"".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-spec".into(),
            kind: "contains".into(),
            target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
            expected: Some("\"lane_allowed_extensions\": {".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-lane".into(),
            kind: "contains".into(),
            target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
            expected: Some("\"module_assets\"".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-extension".into(),
            kind: "contains".into(),
            target: "helpers/local_inbox_helper/HelperServiceSpec.json".into(),
            expected: Some("\".txt\"".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-ignored-input".into(),
            kind: "exists".into(),
            target: "bridge/incoming_assets/module_assets/sample-ignore.json".into(),
            expected: None,
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-summary".into(),
            kind: "contains".into(),
            target: "bridge/helpers/local_inbox/summary.json".into(),
            expected: Some("\"filtered_out_file_count\": 1".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-summary-extension".into(),
            kind: "contains".into(),
            target: "bridge/helpers/local_inbox/summary.json".into(),
            expected: Some("\"lane_allowed_extensions\": {".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-summary-lane".into(),
            kind: "contains".into(),
            target: "bridge/helpers/local_inbox/summary.json".into(),
            expected: Some("\"module_assets\"".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "helper-file-type-filter-summary-file".into(),
            kind: "contains".into(),
            target: "bridge/helpers/local_inbox/summary.json".into(),
            expected: Some("sample-ignore.json".into()),
        },
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "\"lane_allowed_extensions\": {",
    );
    ensure_marker(
        &mut acceptance.required_markers,
        "\"filtered_out_file_count\": 1",
    );

    refresh_project_contract_views(&mut spec);
    fs::write(&helper_spec_path, to_string_pretty(&helper_spec)?)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        helper_spec_path.clone(),
        ignored_input_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWebviewModule),
        project_name: project_name.to_string(),
        patch_kind: "file_type_filter".into(),
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
            patch_kind: "file_type_filter".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_native_bridge_status_panel(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let main_path = project_dir.join("src/main.rs");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut main = fs::read_to_string(&main_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;
    let family_id = spec.family_id.clone();

    if !main.contains("Bridge activity panel") {
        let (anchor, insert) = match family_id.as_ref() {
            Some(FamilyId::ChattyeduNativeWindowModule)
            | Some(FamilyId::ChattycogChattyeduNativeWindowModule) => (
                "                // bridge_status_panel_anchor\n",
                "                columns[1].add_space(8.0);\n                columns[1].group(|ui| {\n                    ui.strong(\"Bridge activity panel\");\n                    ui.label(\"Hosted bridge lanes ready: shared room state, room events, outgoing events, and module_assets inbox.\");\n                });\n\n                // bridge_status_panel_anchor\n",
            ),
            _ => (
                "            // bridge_status_panel_anchor\n",
                "            ui.add_space(8.0);\n            ui.group(|ui| {\n                ui.strong(\"Bridge activity panel\");\n                ui.label(\"Hosted bridge lanes ready: shared room state, room events, outgoing events, and module_assets inbox.\");\n            });\n\n            // bridge_status_panel_anchor\n",
            ),
        };
        if main.contains(anchor) {
            main = main.replacen(anchor, insert, 1);
        } else {
            bail!("native window main.rs did not match expected patch shape");
        }
    }

    ensure_marker(&mut spec.features, "bridge_status_panel");
    ensure_marker(&mut spec.supported_patch_kinds, "bridge_status_panel");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "bridge-status-panel-marker".into(),
            kind: "contains".into(),
            target: "src/main.rs".into(),
            expected: Some("Bridge activity panel".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "bridge-status-panel-marker".into(),
            kind: "contains".into(),
            target: "src/main.rs".into(),
            expected: Some("Bridge activity panel".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "Bridge activity panel");

    refresh_project_contract_views(&mut spec);
    fs::write(&main_path, main)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        main_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id,
        project_name: project_name.to_string(),
        patch_kind: "bridge_status_panel".into(),
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
            patch_kind: "bridge_status_panel".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_native_ready_toggle(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let main_path = project_dir.join("src/main.rs");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut main = fs::read_to_string(&main_path)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;
    let family_id = spec.family_id.clone();

    if !main.contains("Ready state toggle") {
        let (anchor, insert) = match family_id.as_ref() {
            Some(FamilyId::ChattyeduNativeWindowModule)
            | Some(FamilyId::ChattycogChattyeduNativeWindowModule) => (
                "                // ready_toggle_anchor\n",
                "                columns[1].add_space(8.0);\n                columns[1].group(|ui| {\n                    ui.strong(\"Ready state toggle\");\n                    if ui.button(\"Ready state toggle\").clicked() {\n                        self.ready_state = !self.ready_state;\n                    }\n                    ui.label(if self.ready_state { \"Ready\" } else { \"Not ready\" });\n                });\n\n                // ready_toggle_anchor\n",
            ),
            _ => (
                "            // ready_toggle_anchor\n",
                "            ui.add_space(8.0);\n            ui.group(|ui| {\n                ui.strong(\"Ready state toggle\");\n                if ui.button(\"Ready state toggle\").clicked() {\n                    self.ready_state = !self.ready_state;\n                }\n                ui.label(if self.ready_state { \"Ready\" } else { \"Not ready\" });\n            });\n\n            // ready_toggle_anchor\n",
            ),
        };
        if main.contains(anchor) {
            main = main.replacen(anchor, insert, 1);
        } else {
            bail!("native window patch anchor missing in main.rs");
        }
    }

    ensure_marker(&mut spec.features, "ready_toggle");
    ensure_marker(&mut spec.supported_patch_kinds, "ready_toggle");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "ready-toggle-marker".into(),
            kind: "contains".into(),
            target: "src/main.rs".into(),
            expected: Some("Ready state toggle".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "ready-toggle-marker".into(),
            kind: "contains".into(),
            target: "src/main.rs".into(),
            expected: Some("Ready state toggle".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "Ready state toggle");

    refresh_project_contract_views(&mut spec);
    fs::write(&main_path, main)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![
        main_path.clone(),
        spec_path.clone(),
        acceptance_path.clone(),
    ];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id,
        project_name: project_name.to_string(),
        patch_kind: "ready_toggle".into(),
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
            patch_kind: "ready_toggle".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_workspace_room_state_fields(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let ui_path = project_dir.join("ui.json");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut ui: serde_json::Value = serde_json::from_str(&fs::read_to_string(&ui_path)?)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    let fields = ui
        .get_mut("fields")
        .and_then(|value| value.as_array_mut())
        .ok_or_else(|| anyhow::anyhow!("ui.json fields array missing"))?;

    if !fields
        .iter()
        .any(|field| field.get("id").and_then(|value| value.as_str()) == Some("room_policy"))
    {
        fields.push(serde_json::json!({
            "id": "room_policy",
            "label": "Room Policy",
            "type": "text",
            "section": "overview",
            "default": "general"
        }));
    }
    if !fields
        .iter()
        .any(|field| field.get("id").and_then(|value| value.as_str()) == Some("session_label"))
    {
        fields.push(serde_json::json!({
            "id": "session_label",
            "label": "Session Label",
            "type": "text",
            "section": "overview",
            "default": "No active host session"
        }));
    }

    ensure_marker(&mut spec.features, "room_state_fields");
    ensure_marker(&mut spec.supported_patch_kinds, "room_state_fields");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "workspace-room-policy-field".into(),
            kind: "contains".into(),
            target: "ui.json".into(),
            expected: Some("\"room_policy\"".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "workspace-room-policy-field".into(),
            kind: "contains".into(),
            target: "ui.json".into(),
            expected: Some("\"room_policy\"".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "\"room_policy\"");

    refresh_project_contract_views(&mut spec);
    fs::write(&ui_path, to_string_pretty(&ui)?)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![ui_path.clone(), spec_path.clone(), acceptance_path.clone()];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWorkspaceModule),
        project_name: project_name.to_string(),
        patch_kind: "room_state_fields".into(),
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
            patch_kind: "room_state_fields".into(),
        },
        patch_receipt,
    ))
}

pub fn patch_chattycog_workspace_session_overview(
    project_dir: &Path,
    project_name: &str,
    request_id: &str,
    request_summary: &str,
) -> Result<(PatchArtifacts, PatchReceipt)> {
    let ui_path = project_dir.join("ui.json");
    let spec_path = project_dir.join("ProjectSpec.json");
    let acceptance_path = project_dir.join("AcceptancePlan.json");

    let mut ui: serde_json::Value = serde_json::from_str(&fs::read_to_string(&ui_path)?)?;
    let mut spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
    let mut acceptance: AcceptancePlan =
        serde_json::from_str(&fs::read_to_string(&acceptance_path)?)?;

    let sections = ui
        .get_mut("sections")
        .and_then(|value| value.as_array_mut())
        .ok_or_else(|| anyhow::anyhow!("ui.json sections array missing"))?;
    if !sections.iter().any(|section| {
        section.get("id").and_then(|value| value.as_str()) == Some("session_overview")
    }) {
        sections.push(serde_json::json!({
            "id": "session_overview",
            "title": "Session Overview",
            "description": "Room-aware session state surfaced from the ChattyCog bridge."
        }));
    }

    let fields = ui
        .get_mut("fields")
        .and_then(|value| value.as_array_mut())
        .ok_or_else(|| anyhow::anyhow!("ui.json fields array missing"))?;
    if !fields
        .iter()
        .any(|field| field.get("id").and_then(|value| value.as_str()) == Some("participants"))
    {
        fields.push(serde_json::json!({
            "id": "participants",
            "label": "Participants",
            "type": "multiline",
            "section": "session_overview",
            "default": "No active participants"
        }));
    }

    ensure_marker(&mut spec.features, "session_overview");
    ensure_marker(&mut spec.supported_patch_kinds, "session_overview");
    ensure_check(
        &mut spec.acceptance_checks,
        AcceptanceCheck {
            check_id: "workspace-session-overview-section".into(),
            kind: "contains".into(),
            target: "ui.json".into(),
            expected: Some("\"session_overview\"".into()),
        },
    );
    ensure_check(
        &mut acceptance.checks,
        AcceptanceCheck {
            check_id: "workspace-session-overview-section".into(),
            kind: "contains".into(),
            target: "ui.json".into(),
            expected: Some("\"session_overview\"".into()),
        },
    );
    ensure_marker(&mut acceptance.required_markers, "\"session_overview\"");

    refresh_project_contract_views(&mut spec);
    fs::write(&ui_path, to_string_pretty(&ui)?)?;
    fs::write(&spec_path, to_string_pretty(&spec)?)?;
    fs::write(&acceptance_path, to_string_pretty(&acceptance)?)?;

    let modified_files = vec![ui_path.clone(), spec_path.clone(), acceptance_path.clone()];
    let patch_receipt = PatchReceipt {
        patch_id: format!("patch-{}", request_id),
        request_id: request_id.to_string(),
        family_id: Some(FamilyId::ChattycogWorkspaceModule),
        project_name: project_name.to_string(),
        patch_kind: "session_overview".into(),
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
            patch_kind: "session_overview".into(),
        },
        patch_receipt,
    ))
}

fn render_static_web_dashboard_bundle(inputs: &ScaffoldInputs) -> Result<WebBuildBundle> {
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
        index_html: render_named("families/static_web_dashboard/index.html", &context)?,
        app_js: render_named("families/static_web_dashboard/app.js", &context)?,
        styles_css: render_named("families/static_web_dashboard/styles.css", &context)?,
        readme_md: render_named("families/static_web_dashboard/README.md", &context)?,
    })
}

fn title_or_default<'a>(inputs: &'a ScaffoldInputs, fallback: &'a str) -> &'a str {
    if inputs.title.trim().is_empty() {
        fallback
    } else {
        &inputs.title
    }
}

fn summary_or_default<'a>(inputs: &'a ScaffoldInputs, fallback: &'a str) -> &'a str {
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
    for operator_id in operator_ids {
        let Some(family_id) = spec.family_id.as_ref().map(FamilyId::as_str) else {
            continue;
        };
        for contribution in operator_contribution_registry() {
            if contribution.family_id == family_id && contribution.operator_id == operator_id {
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
    for recipe_id in recipe_ids {
        let Some(family_id) = spec.family_id.as_ref().map(FamilyId::as_str) else {
            continue;
        };
        for recipe in acceptance_recipe_registry() {
            if recipe.recipe_id != recipe_id || recipe.family_id != family_id {
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
