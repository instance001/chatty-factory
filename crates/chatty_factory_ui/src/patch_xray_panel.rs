use super::*;
use chatty_factory_core::{PatchIntentFreeze, ProjectPatchDiagnosis};

#[derive(Debug, Clone, serde::Deserialize)]
struct PatchDiagnosisPostcheckReceiptView {
    verified_invariants: Vec<String>,
    warnings: Vec<String>,
    #[serde(default)]
    patch_surgical_maturity: Option<String>,
    #[serde(default)]
    contract_confidence_summary: Option<String>,
    #[serde(default)]
    modified_artifact_groups: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    out_of_contract_modified_files: Vec<String>,
    #[serde(default)]
    declared_surface_groups: Vec<String>,
}

#[derive(Debug, Clone)]
struct ProjectPatchXrayView {
    diagnosis_path: String,
    diagnosis: ProjectPatchDiagnosis,
    freeze_path: Option<String>,
    freeze: Option<PatchIntentFreeze>,
    postcheck_path: Option<String>,
}

impl ChattyFactoryUiApp {
    pub(crate) fn render_recent_project_patch_xrays_section(
        &mut self,
        ui: &mut egui::Ui,
        project_name: &str,
    ) {
        let recent_patch_xrays = load_recent_project_patch_xrays(&self.workspace_root, project_name);
        let visible_patch_xrays: Vec<_> = recent_patch_xrays
            .iter()
            .filter(|xray| {
                !self.patch_xray_blocked_only
                    || project_patch_xray_outcome(xray) == "anchor-blocked"
            })
            .collect();
        if recent_patch_xrays.is_empty() {
            return;
        }

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Recent Patch X-rays");
            ui.checkbox(&mut self.patch_xray_blocked_only, "Show blocked only");
        });
        for xray in visible_patch_xrays.into_iter().take(4) {
            let outcome = project_patch_xray_outcome(xray);
            let intended_patch = xray
                .freeze
                .as_ref()
                .and_then(|freeze| freeze.intended_patch_kind.as_deref())
                .unwrap_or("unknown_patch");
            let surgical_maturity = xray
                .freeze
                .as_ref()
                .and_then(|freeze| freeze.patch_surgical_maturity.clone())
                .or_else(|| xray.diagnosis.patch_surgical_maturity.clone())
                .or_else(|| {
                    current_patch_lane_maturity_for_project(
                        self.selected_project_spec.as_ref(),
                        intended_patch,
                    )
                });
            let superseded_by = current_patch_lane_superseded_for_project(
                self.selected_project_spec.as_ref(),
                intended_patch,
            );
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{} | {}",
                    xray.diagnosis.request_id, intended_patch
                ));
                ui.label(project_patch_xray_outcome_badge(outcome));
                if let Some(maturity) = surgical_maturity.as_deref() {
                    ui.label(patch_lane_maturity_badge(&maturity));
                }
            });
            ui.label(format!("Why: {}", project_patch_xray_outcome_reason(xray)));
            if let Some(maturity) = surgical_maturity.as_deref() {
                ui.label(format!(
                    "How: {}",
                    patch_lane_maturity_explanation(maturity)
                ));
            }
            if let Some(superseded_by) = superseded_by.as_deref() {
                ui.label(format!("Use instead: {superseded_by}"));
            }
            if let Some(guard_source) = &xray.diagnosis.structural_guard_source {
                ui.label(format!("Guard: {guard_source}"));
            }
            if !xray.diagnosis.declared_surface_groups.is_empty() {
                ui.label(format!(
                    "Declared groups: {}",
                    xray.diagnosis.declared_surface_groups.join(", ")
                ));
            }
            if !xray.diagnosis.candidate_target_files.is_empty() {
                ui.label(format!(
                    "Targets: {}",
                    xray.diagnosis.candidate_target_files.join(", ")
                ));
            }
            if !xray.diagnosis.declared_ownership_boundaries.is_empty() {
                ui.label("Declared boundaries");
                for note in xray
                    .diagnosis
                    .declared_ownership_boundaries
                    .iter()
                    .take(2)
                {
                    ui.label(format!("- {note}"));
                }
            }
            if !xray.diagnosis.project_structure_notes.is_empty() {
                ui.label("Structure");
                for note in xray.diagnosis.project_structure_notes.iter().take(2) {
                    ui.label(format!("- {note}"));
                }
            }
            if !xray.diagnosis.present_conflicting_anchor_markers.is_empty() {
                ui.label(format!(
                    "Conflicts: {}",
                    xray.diagnosis.present_conflicting_anchor_markers.join(", ")
                ));
            }
            ui.horizontal(|ui| {
                if ui.small_button("Reveal Diagnosis").clicked() {
                    self.reveal_governed_artifact(
                        &xray.diagnosis_path,
                        "Revealed patch diagnosis receipt",
                        "Open failed",
                        "Revealed patch diagnosis receipt",
                        "Open failed",
                        None,
                    );
                }
                if let Some(path) = &xray.freeze_path {
                    if ui.small_button("Reveal Freeze").clicked() {
                        self.reveal_governed_artifact(
                            path,
                            "Revealed patch intent freeze receipt",
                            "Open failed",
                            "Revealed patch intent freeze receipt",
                            "Open failed",
                            None,
                        );
                    }
                }
                if let Some(path) = &xray.postcheck_path {
                    if ui.small_button("Reveal Postcheck").clicked() {
                        self.reveal_governed_artifact(
                            path,
                            "Revealed patch postcheck receipt",
                            "Open failed",
                            "Revealed patch postcheck receipt",
                            "Open failed",
                            None,
                        );
                    }
                }
            });
            ui.separator();
        }
        if self.patch_xray_blocked_only
            && recent_patch_xrays
                .iter()
                .all(|xray| project_patch_xray_outcome(xray) != "anchor-blocked")
        {
            ui.label("No blocked surgeries in recent patch X-rays.");
        }
    }

    pub(crate) fn render_last_result_patch_xray_section(
        &mut self,
        ui: &mut egui::Ui,
        result: &UiExecutionResult,
    ) {
        if result.patch_diagnosis_path.is_none()
            && result.patch_intent_freeze_path.is_none()
            && result.patch_postcheck_path.is_none()
        {
            return;
        }

        let diagnosis = result
            .patch_diagnosis_path
            .as_deref()
            .and_then(load_patch_diagnosis_receipt);
        let freeze = result
            .patch_intent_freeze_path
            .as_deref()
            .and_then(load_patch_intent_freeze_receipt);
        let postcheck = result
            .patch_postcheck_path
            .as_deref()
            .and_then(load_patch_diagnosis_postcheck_receipt);

        ui.separator();
        ui.label(egui::RichText::new("Patch X-ray").strong());
        if let Some(freeze) = &freeze {
            let superseded_by = freeze.intended_patch_kind.as_deref().and_then(|patch_kind| {
                current_patch_lane_superseded_for_execution(result, patch_kind)
            });
            if let Some(patch_kind) = &freeze.intended_patch_kind {
                if let Some(maturity) = freeze
                    .patch_surgical_maturity
                    .clone()
                    .or_else(|| {
                        diagnosis
                            .as_ref()
                            .and_then(|diagnosis| diagnosis.patch_surgical_maturity.clone())
                    })
                    .or_else(|| current_patch_lane_maturity_for_execution(result, patch_kind))
                {
                    ui.horizontal(|ui| {
                        ui.label("Surgical maturity");
                        ui.label(patch_lane_maturity_badge(&maturity));
                    });
                    ui.label(format!(
                        "How: {}",
                        patch_lane_maturity_explanation(&maturity)
                    ));
                }
            }
            if let Some(superseded_by) = superseded_by.as_deref() {
                ui.label(format!("Use instead: {superseded_by}"));
            }
            if let Some(patch_kind) = &freeze.intended_patch_kind {
                ui.label(format!("Intended patch: {patch_kind}"));
            }
            if let Some(guard_source) = &freeze.structural_guard_source {
                ui.label(format!("Structural guard: {guard_source}"));
            }
            if let Some(summary) = &freeze.contract_confidence_summary {
                ui.label(format!("Confidence: {summary}"));
            }
            if let Some(summary) = &freeze.replacement_guidance_summary {
                ui.label(format!("Replacement: {summary}"));
            }
            if !freeze.superseded_by_patch_kinds.is_empty() {
                ui.label(format!(
                    "Use instead: {}",
                    freeze.superseded_by_patch_kinds.join(", ")
                ));
            }
            if !freeze.required_anchor_markers.is_empty() {
                ui.label(format!(
                    "Required anchors: {}",
                    freeze.required_anchor_markers.join(", ")
                ));
            }
            if !freeze.confirmed_anchor_markers.is_empty() {
                ui.label(format!(
                    "Confirmed anchors: {}",
                    freeze.confirmed_anchor_markers.join(", ")
                ));
            }
            if !freeze.present_conflicting_anchor_markers.is_empty() {
                ui.label(format!(
                    "Conflicting anchors: {}",
                    freeze.present_conflicting_anchor_markers.join(", ")
                ));
            }
        } else if let Some(diagnosis) = &diagnosis {
            if let Some(guard_source) = &diagnosis.structural_guard_source {
                ui.label(format!("Structural guard: {guard_source}"));
            }
            if !diagnosis.present_anchor_markers.is_empty() {
                ui.label(format!(
                    "Present anchors: {}",
                    diagnosis.present_anchor_markers.join(", ")
                ));
            }
        }
        if let Some(diagnosis) = &diagnosis {
            if !diagnosis.declared_surface_groups.is_empty() {
                ui.label(format!(
                    "Declared groups: {}",
                    diagnosis.declared_surface_groups.join(", ")
                ));
            }
            if freeze.is_none() {
                if let Some(maturity) = &diagnosis.patch_surgical_maturity {
                    ui.horizontal(|ui| {
                        ui.label("Surgical maturity");
                        ui.label(patch_lane_maturity_badge(maturity));
                    });
                    ui.label(format!(
                        "How: {}",
                        patch_lane_maturity_explanation(maturity)
                    ));
                }
            }
            if !diagnosis.candidate_target_files.is_empty() {
                ui.label(format!(
                    "Target files: {}",
                    diagnosis.candidate_target_files.join(", ")
                ));
            }
            if !diagnosis.declared_ownership_boundaries.is_empty() {
                ui.label("Declared boundaries");
                for note in diagnosis.declared_ownership_boundaries.iter().take(3) {
                    ui.label(format!("- {note}"));
                }
            }
            if !diagnosis.project_structure_notes.is_empty() {
                ui.label("Structure");
                for note in diagnosis.project_structure_notes.iter().take(3) {
                    ui.label(format!("- {note}"));
                }
            }
            if !diagnosis.candidate_insertion_points.is_empty() {
                ui.label(format!(
                    "Insertion points: {}",
                    diagnosis.candidate_insertion_points.join(", ")
                ));
            }
            if !diagnosis.preserve_invariants.is_empty() {
                ui.label("Preserve invariants");
                for invariant in diagnosis.preserve_invariants.iter().take(4) {
                    ui.label(format!("- {invariant}"));
                }
            }
            if !diagnosis.risk_notes.is_empty() {
                ui.label("Risk notes");
                for note in diagnosis.risk_notes.iter().take(3) {
                    ui.label(format!("- {note}"));
                }
            }
        }
        if let Some(postcheck) = &postcheck {
            if freeze.is_none() && diagnosis.is_none() {
                if let Some(maturity) = &postcheck.patch_surgical_maturity {
                    ui.horizontal(|ui| {
                        ui.label("Surgical maturity");
                        ui.label(patch_lane_maturity_badge(maturity));
                    });
                    ui.label(format!(
                        "How: {}",
                        patch_lane_maturity_explanation(maturity)
                    ));
                }
            }
            if let Some(summary) = &postcheck.contract_confidence_summary {
                ui.label(format!("Confidence: {summary}"));
            }
            if !postcheck.declared_surface_groups.is_empty() {
                ui.label(format!(
                    "Postcheck groups: {}",
                    postcheck.declared_surface_groups.join(", ")
                ));
            }
            if !postcheck.modified_artifact_groups.is_empty() {
                ui.label(format!(
                    "Touched groups: {}",
                    postcheck
                        .modified_artifact_groups
                        .keys()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            if !postcheck.verified_invariants.is_empty() {
                ui.label("Postcheck");
                for check in postcheck.verified_invariants.iter().take(4) {
                    ui.label(format!("- {check}"));
                }
            }
            if !postcheck.out_of_contract_modified_files.is_empty() {
                ui.label(format!(
                    "Out-of-contract edits: {}",
                    postcheck.out_of_contract_modified_files.join(", ")
                ));
            }
            if !postcheck.warnings.is_empty() {
                ui.label("Postcheck warnings");
                for warning in postcheck.warnings.iter().take(3) {
                    ui.label(format!("- {warning}"));
                }
            }
        }
        if let Some(path) = &result.patch_diagnosis_path {
            ui.horizontal(|ui| {
                ui.label(format!("Diagnosis: {}", short_path(path)));
                if ui.small_button("Reveal Diagnosis").clicked() {
                    self.reveal_governed_artifact(
                        path,
                        "Revealed patch diagnosis receipt",
                        "Open failed",
                        "Revealed patch diagnosis receipt",
                        "Open failed",
                        None,
                    );
                }
            });
        }
        if let Some(path) = &result.patch_intent_freeze_path {
            ui.horizontal(|ui| {
                ui.label(format!("Intent freeze: {}", short_path(path)));
                if ui.small_button("Reveal Freeze").clicked() {
                    self.reveal_governed_artifact(
                        path,
                        "Revealed patch intent freeze receipt",
                        "Open failed",
                        "Revealed patch intent freeze receipt",
                        "Open failed",
                        None,
                    );
                }
            });
        }
        if let Some(path) = &result.patch_postcheck_path {
            ui.horizontal(|ui| {
                ui.label(format!("Postcheck: {}", short_path(path)));
                if ui.small_button("Reveal Postcheck").clicked() {
                    self.reveal_governed_artifact(
                        path,
                        "Revealed patch postcheck receipt",
                        "Open failed",
                        "Revealed patch postcheck receipt",
                        "Open failed",
                        None,
                    );
                }
            });
        }
    }
}

fn load_patch_diagnosis_receipt(path: &str) -> Option<ProjectPatchDiagnosis> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn load_patch_intent_freeze_receipt(path: &str) -> Option<PatchIntentFreeze> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn load_patch_diagnosis_postcheck_receipt(path: &str) -> Option<PatchDiagnosisPostcheckReceiptView> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn load_recent_project_patch_xrays(
    workspace_root: &Path,
    project_name: &str,
) -> Vec<ProjectPatchXrayView> {
    let diagnoses_dir = workspace_root.join("runtime").join("patch_diagnoses");
    let Ok(entries) = std::fs::read_dir(diagnoses_dir) else {
        return Vec::new();
    };
    let mut xrays = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        let path_string = path.to_string_lossy().to_string();
        if !path_string.ends_with("-diagnosis.json") {
            continue;
        }
        let Some(diagnosis) = load_patch_diagnosis_receipt(&path_string) else {
            continue;
        };
        if diagnosis.project_name != project_name {
            continue;
        }
        let freeze_path = workspace_root
            .join("runtime")
            .join("patch_intent_freezes")
            .join(format!("{}-freeze.json", diagnosis.request_id));
        let freeze_path_string = freeze_path.to_string_lossy().to_string();
        let freeze = load_patch_intent_freeze_receipt(&freeze_path_string);
        let freeze_path = if freeze.is_some() {
            Some(freeze_path_string)
        } else {
            None
        };
        let postcheck_path = workspace_root
            .join("runtime")
            .join("patch_diagnoses")
            .join(format!("{}-postcheck.json", diagnosis.request_id));
        let postcheck_path = if postcheck_path.exists() {
            Some(postcheck_path.to_string_lossy().to_string())
        } else {
            None
        };
        let modified = std::fs::metadata(&path)
            .ok()
            .and_then(|metadata| metadata.modified().ok());
        xrays.push((
            modified,
            ProjectPatchXrayView {
                diagnosis_path: path_string,
                diagnosis,
                freeze_path,
                freeze,
                postcheck_path,
            },
        ));
    }
    xrays.sort_by(|left, right| right.0.cmp(&left.0));
    xrays.into_iter().map(|(_, xray)| xray).collect()
}

fn project_patch_xray_outcome(xray: &ProjectPatchXrayView) -> &'static str {
    if xray.postcheck_path.is_some() {
        return "applied";
    }
    if !xray.diagnosis.present_conflicting_anchor_markers.is_empty() {
        return "anchor-blocked";
    }
    if let Some(freeze) = &xray.freeze {
        if !freeze.required_anchor_markers.is_empty()
            && freeze.confirmed_anchor_markers.len() < freeze.required_anchor_markers.len()
        {
            return "anchor-blocked";
        }
    }
    "duplicate-skip"
}

fn project_patch_xray_outcome_badge(outcome: &str) -> egui::RichText {
    let (label, color) = match outcome {
        "applied" => ("applied", egui::Color32::from_rgb(70, 140, 90)),
        "anchor-blocked" => ("anchor-blocked", egui::Color32::from_rgb(160, 90, 70)),
        "duplicate-skip" => ("duplicate-skip", egui::Color32::from_rgb(140, 120, 70)),
        _ => (outcome, egui::Color32::GRAY),
    };
    egui::RichText::new(format!("[{label}]"))
        .color(color)
        .strong()
}

fn project_patch_xray_outcome_reason(xray: &ProjectPatchXrayView) -> String {
    if xray.postcheck_path.is_some() {
        return "postcheck passed and invariants held".to_string();
    }
    if !xray.diagnosis.present_conflicting_anchor_markers.is_empty() {
        return format!(
            "conflicting evolved anchors already present: {}",
            xray.diagnosis
                .present_conflicting_anchor_markers
                .iter()
                .take(2)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    if let Some(freeze) = &xray.freeze {
        if !freeze.required_anchor_markers.is_empty()
            && freeze.confirmed_anchor_markers.len() < freeze.required_anchor_markers.len()
        {
            let missing = freeze
                .required_anchor_markers
                .iter()
                .filter(|marker| !freeze.confirmed_anchor_markers.contains(*marker))
                .take(2)
                .cloned()
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                return format!("required anchors missing: {}", missing.join(", "));
            }
            return "required insertion anchors were not confirmed".to_string();
        }
    }
    "surface already present, so duplicate insertion was skipped".to_string()
}

fn patch_lane_maturity_badge(maturity: &str) -> egui::RichText {
    let (label, color) = match maturity {
        "narrow_surface_contract" => (
            "narrow-surface",
            egui::Color32::from_rgb(70, 140, 90),
        ),
        "broad_surface_contract" => (
            "broad-surface",
            egui::Color32::from_rgb(90, 120, 150),
        ),
        "legacy_shape_sensitive" => (
            "legacy-shape",
            egui::Color32::from_rgb(170, 110, 70),
        ),
        "anchor_only_contract" => (
            "anchor-only",
            egui::Color32::from_rgb(140, 120, 70),
        ),
        "uncontracted" => ("uncontracted", egui::Color32::from_gray(130)),
        _ => (maturity, egui::Color32::GRAY),
    };
    egui::RichText::new(format!("[{label}]"))
        .color(color)
        .strong()
}

fn patch_lane_maturity_explanation(maturity: &str) -> &'static str {
    match maturity {
        "narrow_surface_contract" => {
            "the lane declares a tight surgical surface with specific artifact groups and structural guard checks"
        }
        "broad_surface_contract" => {
            "the lane declares multiple artifact groups and boundaries, but its allowed surgical surface is still fairly wide"
        }
        "legacy_shape_sensitive" => {
            "the lane depends on an older structural shape and may be blocked when newer evolved surfaces are already present"
        }
        "anchor_only_contract" => {
            "the lane confirms insertion anchors, but does not yet declare a fuller surface-group contract"
        }
        "uncontracted" => {
            "the lane does not yet declare a strong surgical contract, so diagnosis relies more heavily on general heuristics"
        }
        _ => "the lane has a custom maturity classification",
    }
}

fn current_patch_lane_maturity_for_project(
    selected_project_spec: Option<&ProjectSpec>,
    patch_kind: &str,
) -> Option<String> {
    selected_project_spec?
        .patch_lanes
        .iter()
        .find(|lane| lane.patch_kind == patch_kind)
        .map(|lane| lane.surgical_maturity.clone())
        .filter(|maturity| !maturity.trim().is_empty())
}

fn current_patch_lane_superseded_for_project(
    selected_project_spec: Option<&ProjectSpec>,
    patch_kind: &str,
) -> Option<String> {
    selected_project_spec?
        .patch_lanes
        .iter()
        .find(|lane| lane.patch_kind == patch_kind)
        .and_then(|lane| {
            if lane.superseded_by_patch_kinds.is_empty() {
                None
            } else {
                Some(lane.superseded_by_patch_kinds.join(", "))
            }
        })
}

fn current_patch_lane_maturity_for_execution(
    result: &UiExecutionResult,
    patch_kind: &str,
) -> Option<String> {
    result
        .patch_lanes
        .iter()
        .find(|lane| lane.patch_kind == patch_kind)
        .map(|lane| lane.surgical_maturity.clone())
        .filter(|maturity| !maturity.trim().is_empty())
}

fn current_patch_lane_superseded_for_execution(
    result: &UiExecutionResult,
    patch_kind: &str,
) -> Option<String> {
    result
        .patch_lanes
        .iter()
        .find(|lane| lane.patch_kind == patch_kind)
        .and_then(|lane| {
            if lane.superseded_by_patch_kinds.is_empty() {
                None
            } else {
                Some(lane.superseded_by_patch_kinds.join(", "))
            }
        })
}
