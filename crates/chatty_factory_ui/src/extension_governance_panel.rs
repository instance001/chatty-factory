use super::*;

impl ChattyFactoryUiApp {
    pub(crate) fn render_extension_governance_section(
        &mut self,
        ui: &mut egui::Ui,
        entry: &PendingExtensionEntry,
        latest_proof_receipt: Option<&(CrossFamilyPairedProofReceiptSummary, PathBuf)>,
    ) {
        if entry.extension_kind == "proof_harness_bundle" {
            ui.label(format!(
                "Proof quality: {}",
                entry.proof_quality_status.as_deref().unwrap_or("unknown")
            ));
            if let Some(lineage_status) = proof_lineage_status(entry) {
                ui.label(format!("Proof lineage: {lineage_status}"));
            }
            if let Some(seed) = entry.proof_seed_template_id.as_deref() {
                ui.label(format!("Seed template: {seed}"));
            }
            if let Some(seed) = entry.proof_seed_bundle_id.as_deref() {
                ui.label(format!("Seed bundle: {seed}"));
            }
            if let Some(drift_status) = entry.proof_drift_status.as_deref() {
                ui.label(format!("Proof drift: {drift_status}"));
            }
            if let Some(change_status) = entry.proof_change_since_last_pass_status.as_deref() {
                ui.label(format!("Change since last pass: {change_status}"));
            }
            ui.label(format!(
                "Scaffolded from: {}",
                if entry.source_stub_path.trim().is_empty() {
                    "none".to_string()
                } else {
                    entry.source_stub_path.clone()
                }
            ));
            if let Some(path) = entry.proof_lineage_receipt_path.as_deref() {
                ui.label(format!("Lineage receipt: {path}"));
                if ui.small_button("Open Lineage Receipt").clicked() {
                    self.reveal_governed_artifact(
                        path,
                        "Revealed proof lineage receipt",
                        "Failed to reveal proof lineage receipt",
                        "Revealed proof lineage receipt",
                        "Failed to reveal proof lineage receipt",
                        Some((Some(entry.entry_id.clone()), "Open Lineage Receipt")),
                    );
                }
            }
            for note in &entry.proof_drift_notes {
                ui.label(format!("Drift note: {note}"));
            }
            for note in &entry.proof_change_since_last_pass_notes {
                ui.label(format!("Baseline note: {note}"));
            }
            if let Some((receipt, receipt_path)) = latest_proof_receipt {
                ui.label(format!("Latest proof receipt: {}", receipt.receipt_id));
                ui.label(format!(
                    "Latest proof outcome: {}",
                    if receipt.equivalent_capability_fulfillment {
                        "passing"
                    } else {
                        "diverged"
                    }
                ));
                ui.label(format!("Latest proof artifact: {}", receipt_path.display()));
                ui.horizontal(|ui| {
                    if ui.small_button("Open Latest Proof").clicked() {
                        self.reveal_governed_artifact(
                            &receipt_path.display().to_string(),
                            "Revealed latest proof receipt",
                            "Open failed",
                            "Revealed latest proof receipt",
                            "Open failed",
                            Some((Some(entry.entry_id.clone()), "Open Latest Proof")),
                        );
                    }
                    if ui.small_button("Open Latest Compare").clicked() {
                        self.reveal_governed_artifact(
                            &receipt.comparison_receipt_path,
                            "Revealed latest comparison receipt",
                            "Open failed",
                            "Revealed latest comparison receipt",
                            "Open failed",
                            Some((Some(entry.entry_id.clone()), "Open Latest Compare")),
                        );
                    }
                });
            } else {
                ui.label("Latest proof receipt: none");
            }
            if !entry.proof_quality_notes.is_empty() {
                ui.label("Proof quality notes");
                for note in &entry.proof_quality_notes {
                    ui.label(format!("- {note}"));
                }
            }
        }
        if entry.extension_kind == "composition_bundle" {
            let composition_acceptance_paths = entry
                .integrated_paths
                .iter()
                .filter(|path| path.contains("operator_registry") && path.contains("acceptance_recipes"))
                .cloned()
                .collect::<Vec<_>>();
            let artifact_summary = extension_governed_artifact_set_summary(entry);
            render_governance_detail_block(
                ui,
                "Composition",
                artifact_summary.as_deref(),
                entry.composition_drift_status.as_deref(),
                entry.composition_change_since_last_live_status.as_deref(),
                &entry.composition_drift_notes,
                &entry.composition_change_since_last_live_notes,
            );
            if let Some(path) = entry.composition_lineage_receipt_path.as_deref() {
                ui.label(format!("Composition receipt: {path}"));
                if ui.small_button("Open Composition Receipt").clicked() {
                    self.reveal_governed_artifact(
                        path,
                        "Revealed composition governance receipt",
                        "Failed to reveal composition governance receipt",
                        "Revealed composition governance receipt",
                        "Failed to reveal composition governance receipt",
                        Some((Some(entry.entry_id.clone()), "Open Composition Receipt")),
                    );
                }
            }
            if !composition_acceptance_paths.is_empty() {
                ui.label(format!(
                    "Patch-side acceptance contracts: {}",
                    composition_acceptance_paths.len()
                ));
                for acceptance_path in &composition_acceptance_paths {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(format!("Acceptance artifact: {}", short_path(acceptance_path)));
                        if ui.small_button("Open Acceptance").clicked() {
                            self.reveal_governed_artifact(
                                acceptance_path,
                                "Revealed composition acceptance artifact",
                                "Failed to reveal composition acceptance artifact",
                                "Revealed composition acceptance artifact",
                                "Failed to reveal composition acceptance artifact",
                                Some((Some(entry.entry_id.clone()), "Open Composition Acceptance")),
                            );
                        }
                    });
                }
            }
        }
        if entry.extension_kind == "patch_recipe" {
            let patch_acceptance_path = entry
                .integrated_paths
                .iter()
                .find(|path| path.contains("operator_registry") && path.contains("acceptance_recipes"))
                .cloned();
            let artifact_summary = extension_governed_artifact_set_summary(entry);
            render_governance_detail_block(
                ui,
                "Patch",
                artifact_summary.as_deref(),
                entry.patch_drift_status.as_deref(),
                entry.patch_change_since_last_live_status.as_deref(),
                &entry.patch_drift_notes,
                &entry.patch_change_since_last_live_notes,
            );
            if let Some(path) = entry.patch_lineage_receipt_path.as_deref() {
                ui.label(format!("Patch receipt: {path}"));
                if ui.small_button("Open Patch Receipt").clicked() {
                    self.reveal_governed_artifact(
                        path,
                        "Revealed patch governance receipt",
                        "Failed to reveal patch governance receipt",
                        "Revealed patch governance receipt",
                        "Failed to reveal patch governance receipt",
                        Some((Some(entry.entry_id.clone()), "Open Patch Receipt")),
                    );
                }
            }
            if let Some(acceptance_path) = patch_acceptance_path.as_deref() {
                ui.horizontal_wrapped(|ui| {
                    ui.label(format!(
                        "Paired acceptance contract: {}",
                        short_path(acceptance_path)
                    ));
                    if ui.small_button("Open Acceptance").clicked() {
                        self.reveal_governed_artifact(
                            acceptance_path,
                            "Revealed paired acceptance contract",
                            "Failed to reveal paired acceptance contract",
                            "Revealed paired acceptance contract",
                            "Failed to reveal paired acceptance contract",
                            Some((Some(entry.entry_id.clone()), "Open Patch Acceptance")),
                        );
                    }
                });
            }
        }
        if entry.extension_kind == "helper_lane" {
            let artifact_summary = extension_governed_artifact_set_summary(entry);
            render_governance_detail_block(
                ui,
                "Helper",
                artifact_summary.as_deref(),
                entry.helper_drift_status.as_deref(),
                entry.helper_change_since_last_live_status.as_deref(),
                &entry.helper_drift_notes,
                &entry.helper_change_since_last_live_notes,
            );
            if let Some(path) = entry.helper_lineage_receipt_path.as_deref() {
                ui.label(format!("Helper receipt: {path}"));
                if ui.small_button("Open Helper Receipt").clicked() {
                    self.reveal_governed_artifact(
                        path,
                        "Revealed helper governance receipt",
                        "Failed to reveal helper governance receipt",
                        "Revealed helper governance receipt",
                        "Failed to reveal helper governance receipt",
                        Some((Some(entry.entry_id.clone()), "Open Helper Receipt")),
                    );
                }
            }
        }
        if entry.extension_kind == "chattycog_bridge_lane" {
            let artifact_summary = extension_governed_artifact_set_summary(entry);
            render_governance_detail_block(
                ui,
                "Bridge",
                artifact_summary.as_deref(),
                entry.bridge_drift_status.as_deref(),
                entry.bridge_change_since_last_live_status.as_deref(),
                &entry.bridge_drift_notes,
                &entry.bridge_change_since_last_live_notes,
            );
            if let Some(path) = entry.bridge_lineage_receipt_path.as_deref() {
                ui.label(format!("Bridge receipt: {path}"));
                if ui.small_button("Open Bridge Receipt").clicked() {
                    self.reveal_governed_artifact(
                        path,
                        "Revealed bridge governance receipt",
                        "Failed to reveal bridge governance receipt",
                        "Revealed bridge governance receipt",
                        "Failed to reveal bridge governance receipt",
                        Some((Some(entry.entry_id.clone()), "Open Bridge Receipt")),
                    );
                }
            }
        }
        ui.label(format!(
            "Unresolved layers: {}",
            if entry.unresolved_layers.is_empty() {
                "none".to_string()
            } else {
                entry.unresolved_layers.join(", ")
            }
        ));
        if !entry.missing_family_build_primitive_classes.is_empty() {
            ui.label("Missing family build classes");
            for class in &entry.missing_family_build_primitive_classes {
                ui.label(format!("- {class}"));
            }
        }
        if !entry.missing_patch_primitive_classes.is_empty() {
            ui.label("Missing patch classes");
            for class in &entry.missing_patch_primitive_classes {
                ui.label(format!("- {class}"));
            }
        }
        if !entry.missing_helper_primitive_kinds.is_empty() {
            ui.label("Missing helper kinds");
            for kind in &entry.missing_helper_primitive_kinds {
                ui.label(format!("- {kind}"));
            }
        }
    }
}
