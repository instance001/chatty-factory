use super::*;

impl ChattyFactoryUiApp {
    pub(crate) fn render_paired_proof_results_section(
        &mut self,
        ui: &mut egui::Ui,
        paired_receipts: &[(CrossFamilyPairedProofReceiptSummary, PathBuf)],
        filtered_paired_receipts: &[(CrossFamilyPairedProofReceiptSummary, PathBuf)],
        proof_templates: &[PrimitiveProofTemplate],
        comparison_bundles: &[CapabilityComparisonBundle],
    ) {
        if paired_receipts.is_empty() {
            ui.label("No paired proof receipts yet.");
            return;
        }

        if let Some((latest_receipt, latest_path)) =
            filtered_paired_receipts.first().or_else(|| paired_receipts.first())
        {
            self.render_latest_paired_proof_card(
                ui,
                latest_receipt,
                latest_path,
                filtered_paired_receipts,
                paired_receipts,
                comparison_bundles,
            );
        }

        self.render_favorite_paired_proofs(ui, filtered_paired_receipts, comparison_bundles);
        self.render_recent_paired_proofs(ui, filtered_paired_receipts, comparison_bundles);
        self.render_paired_proof_history(
            ui,
            filtered_paired_receipts,
            proof_templates,
            comparison_bundles,
        );
    }

    fn render_latest_paired_proof_card(
        &mut self,
        ui: &mut egui::Ui,
        latest_receipt: &CrossFamilyPairedProofReceiptSummary,
        latest_path: &Path,
        filtered_paired_receipts: &[(CrossFamilyPairedProofReceiptSummary, PathBuf)],
        paired_receipts: &[(CrossFamilyPairedProofReceiptSummary, PathBuf)],
        comparison_bundles: &[CapabilityComparisonBundle],
    ) {
        ui.group(|ui| {
            ui.heading("Latest Proof Status");

            let template_label = proof_receipt_template_label(latest_receipt)
                .unwrap_or_else(|| "Unknown Template".to_string());
            ui.label(format!(
                "{} | {} | artifacts:{} | created:{}",
                if latest_receipt.equivalent_capability_fulfillment {
                    "Equivalent"
                } else {
                    "Diverged"
                },
                template_label,
                paired_proof_artifact_count(latest_receipt, latest_path),
                latest_receipt
                    .created_at
                    .as_deref()
                    .unwrap_or("unknown")
            ));
            ui.label(format!(
                "Projects: {} vs {}",
                latest_receipt.left_project_name, latest_receipt.right_project_name
            ));

            if let Some(last_touched) =
                latest_proof_last_touched(&self.extension_activity, &latest_receipt.receipt_id)
            {
                ui.label(format!("Last touched: {}", last_touched));
            }

            if let Some((previous_receipt, previous_path)) = filtered_paired_receipts
                .iter()
                .nth(1)
                .or_else(|| {
                    paired_receipts
                        .iter()
                        .find(|(receipt, _)| receipt.receipt_id != latest_receipt.receipt_id)
                })
            {
                let diff_lines = build_paired_proof_diff_summary(
                    latest_receipt,
                    latest_path,
                    previous_receipt,
                    previous_path,
                );
                if !diff_lines.is_empty() {
                    ui.separator();
                    ui.label("Latest vs Previous");
                    for line in diff_lines {
                        ui.label(line);
                    }
                }
            }

            ui.label(format!("Shared request: {}", latest_receipt.shared_request));

            if let Some(bundle) =
                proof_receipt_comparison_bundle(latest_receipt, comparison_bundles)
            {
                if let Some(comparison_receipt) =
                    load_capability_comparison_receipt(&latest_receipt.comparison_receipt_path)
                {
                    ui.separator();
                    ui.label("Required bundle");
                    for (capability, present) in
                        required_bundle_status_lines(&bundle, &comparison_receipt)
                    {
                        ui.label(format!(
                            "{} {}",
                            if present { "OK" } else { "Missing" },
                            capability
                        ));
                    }
                }
            }

            self.render_latest_paired_proof_actions(ui, latest_receipt, latest_path);
            self.render_latest_paired_proof_note(ui, latest_receipt);
            self.render_latest_paired_proof_export_history(ui, latest_receipt);
            self.render_latest_paired_proof_timeline(ui, latest_receipt);
        });
    }

    fn render_latest_paired_proof_actions(
        &mut self,
        ui: &mut egui::Ui,
        receipt: &CrossFamilyPairedProofReceiptSummary,
        receipt_path: &Path,
    ) {
        ui.horizontal(|ui| {
            if ui.button("Open Latest Proof").clicked() {
                let path_text = receipt_path.to_string_lossy().to_string();
                match open_path_in_explorer(&path_text, true) {
                    Ok(()) => {
                        self.status_line =
                            format!("Opened paired proof receipt {}", short_path(&path_text));
                        self.push_extension_activity(
                            Some(receipt.receipt_id.clone()),
                            "Open Latest Proof",
                            short_path(&path_text),
                            true,
                        );
                    }
                    Err(error) => {
                        self.status_line =
                            format!("failed to open paired proof receipt: {}", error);
                    }
                }
            }

            if ui.button("Open Latest Compare").clicked() {
                match open_path_in_explorer(&receipt.comparison_receipt_path, true) {
                    Ok(()) => {
                        self.status_line = format!(
                            "Opened comparison receipt {}",
                            short_path(&receipt.comparison_receipt_path)
                        );
                        self.push_extension_activity(
                            Some(receipt.receipt_id.clone()),
                            "Open Latest Compare",
                            short_path(&receipt.comparison_receipt_path),
                            true,
                        );
                    }
                    Err(error) => {
                        self.status_line =
                            format!("failed to open comparison receipt: {}", error);
                    }
                }
            }

            if ui.button("Export Proof Summary").clicked() {
                self.export_paired_proof_summary(receipt, receipt_path);
            }

            if ui.button("Copy Proof Summary").clicked() {
                self.copy_paired_proof_summary(receipt, receipt_path, ui.ctx());
            }

            if ui.button("Open Latest Proof Export").clicked() {
                let path = self.latest_paired_proof_export_path(&receipt.receipt_id);
                if path.exists() {
                    let path_text = path.to_string_lossy().to_string();
                    match open_path_in_explorer(&path_text, true) {
                        Ok(()) => {
                            self.status_line =
                                format!("Opened proof export {}", short_path(&path_text));
                            self.push_extension_activity(
                                Some(receipt.receipt_id.clone()),
                                "Open Latest Proof Export",
                                short_path(&path_text),
                                true,
                            );
                        }
                        Err(error) => {
                            self.status_line = format!("failed to open proof export: {}", error);
                        }
                    }
                } else {
                    self.status_line =
                        "no exported proof summary exists yet for latest proof".to_string();
                }
            }

            let is_favorite = self
                .favorite_paired_proof_receipt_ids
                .contains(&receipt.receipt_id);
            if ui
                .button(if is_favorite { "Unpin Proof" } else { "Pin Proof" })
                .clicked()
            {
                self.toggle_paired_proof_favorite(&receipt.receipt_id);
            }
        });
    }

    fn render_latest_paired_proof_note(
        &mut self,
        ui: &mut egui::Ui,
        receipt: &CrossFamilyPairedProofReceiptSummary,
    ) {
        ui.horizontal(|ui| {
            let current_note = self
                .paired_proof_notes
                .entry(receipt.receipt_id.clone())
                .or_default();
            ui.label("Proof Note");
            ui.text_edit_singleline(current_note);
            if ui.button("Save Proof Note").clicked() {
                self.save_paired_proof_notes();
                self.push_extension_activity(
                    Some(receipt.receipt_id.clone()),
                    "Save Proof Note",
                    receipt.receipt_id.clone(),
                    true,
                );
                self.status_line = format!("Saved proof note for {}", receipt.receipt_id);
            }
            if ui.button("Clear Proof Note").clicked() {
                self.paired_proof_notes.remove(&receipt.receipt_id);
                self.save_paired_proof_notes();
                self.push_extension_activity(
                    Some(receipt.receipt_id.clone()),
                    "Clear Proof Note",
                    receipt.receipt_id.clone(),
                    true,
                );
                self.status_line = format!("Cleared proof note for {}", receipt.receipt_id);
            }
        });

        if let Some(note) = self.paired_proof_notes.get(&receipt.receipt_id) {
            if !note.trim().is_empty() {
                ui.label(format!("Note: {}", note.trim()));
            }
        }
    }

    fn render_latest_paired_proof_export_history(
        &mut self,
        ui: &mut egui::Ui,
        receipt: &CrossFamilyPairedProofReceiptSummary,
    ) {
        let export_history = self.paired_proof_export_history_paths(&receipt.receipt_id);
        if export_history.is_empty() {
            return;
        }

        ui.separator();
        ui.label("Proof Export History");
        for path in export_history.iter().take(5) {
            ui.horizontal(|ui| {
                let path_text = path.to_string_lossy().to_string();
                ui.label(short_path(&path_text));
                if ui.button("Open").clicked() {
                    match open_path_in_explorer(&path_text, true) {
                        Ok(()) => {
                            self.status_line =
                                format!("Opened proof export {}", short_path(&path_text));
                            self.push_extension_activity(
                                Some(receipt.receipt_id.clone()),
                                "Open Proof Export",
                                short_path(&path_text),
                                true,
                            );
                        }
                        Err(error) => {
                            self.status_line = format!("failed to open proof export: {}", error);
                        }
                    }
                }
            });
        }

        if export_history.len() >= 2 {
            let diff_text = build_export_diff(&export_history[0], &export_history[1]);
            ui.separator();
            ui.label("Latest vs Previous Proof Export");
            if ui.button("Copy Diff").clicked() {
                ui.ctx().copy_text(diff_text.clone());
                self.status_line = format!("Copied proof export diff for {}", receipt.receipt_id);
                self.push_extension_activity(
                    Some(receipt.receipt_id.clone()),
                    "Copy Proof Export Diff",
                    receipt.receipt_id.clone(),
                    true,
                );
            }
            egui::CollapsingHeader::new("Proof Export Diff")
                .default_open(false)
                .show(ui, |ui| {
                    ui.code(diff_text);
                });
        }
    }

    fn render_latest_paired_proof_timeline(
        &mut self,
        ui: &mut egui::Ui,
        receipt: &CrossFamilyPairedProofReceiptSummary,
    ) {
        egui::CollapsingHeader::new("Proof Timeline")
            .default_open(false)
            .show(ui, |ui| {
                let activity_lines: Vec<_> = self
                    .extension_activity
                    .iter()
                    .filter(|item| item.entry_id.as_deref() == Some(receipt.receipt_id.as_str()))
                    .map(|item| {
                        format!(
                            "{} | {} | {}",
                            item.timestamp_label, item.title, item.detail
                        )
                    })
                    .collect();
                if activity_lines.is_empty() {
                    ui.label("No proof activity yet.");
                } else {
                    for line in activity_lines.iter().rev().take(20) {
                        ui.label(line);
                    }
                }
                if ui.button("Clear Proof").clicked() {
                    self.clear_proof_activity(&receipt.receipt_id);
                }
            });
    }

    fn render_favorite_paired_proofs(
        &mut self,
        ui: &mut egui::Ui,
        filtered_paired_receipts: &[(CrossFamilyPairedProofReceiptSummary, PathBuf)],
        comparison_bundles: &[CapabilityComparisonBundle],
    ) {
        let favorite_receipts: Vec<_> = filtered_paired_receipts
            .iter()
            .filter(|(receipt, _)| {
                self.favorite_paired_proof_receipt_ids
                    .contains(&receipt.receipt_id)
            })
            .collect();

        if favorite_receipts.is_empty() {
            return;
        }

        ui.separator();
        ui.heading("Favorite Proofs");
        for (receipt, path) in favorite_receipts {
            self.render_compact_paired_proof_row(ui, receipt, path, comparison_bundles, true);
        }
    }

    fn render_recent_paired_proofs(
        &mut self,
        ui: &mut egui::Ui,
        filtered_paired_receipts: &[(CrossFamilyPairedProofReceiptSummary, PathBuf)],
        comparison_bundles: &[CapabilityComparisonBundle],
    ) {
        ui.separator();
        ui.heading("Recent Paired Proofs");
        for (receipt, path) in filtered_paired_receipts.iter().take(8) {
            self.render_compact_paired_proof_row(ui, receipt, path, comparison_bundles, false);
        }
    }

    fn render_paired_proof_history(
        &mut self,
        ui: &mut egui::Ui,
        filtered_paired_receipts: &[(CrossFamilyPairedProofReceiptSummary, PathBuf)],
        proof_templates: &[PrimitiveProofTemplate],
        comparison_bundles: &[CapabilityComparisonBundle],
    ) {
        ui.separator();
        egui::CollapsingHeader::new("Proof History")
            .default_open(false)
            .show(ui, |ui| {
                if filtered_paired_receipts.is_empty() {
                    ui.label(format!(
                        "No proof receipts match the {} filter.",
                        proof_history_filter_label(
                            &self.proof_history_template_filter,
                            proof_templates
                        )
                    ));
                    return;
                }

                for (receipt, path) in filtered_paired_receipts.iter().take(30) {
                    self.render_compact_paired_proof_row(
                        ui,
                        receipt,
                        path,
                        comparison_bundles,
                        false,
                    );
                }
            });
    }

    fn render_compact_paired_proof_row(
        &mut self,
        ui: &mut egui::Ui,
        receipt: &CrossFamilyPairedProofReceiptSummary,
        path: &Path,
        comparison_bundles: &[CapabilityComparisonBundle],
        show_unpin: bool,
    ) {
        ui.group(|ui| {
            let template_label = proof_receipt_template_label(receipt)
                .unwrap_or_else(|| "Unknown Template".to_string());
            ui.label(format!(
                "{} | {} | {}",
                template_label,
                receipt.receipt_id,
                if receipt.equivalent_capability_fulfillment {
                    "Equivalent"
                } else {
                    "Diverged"
                }
            ));

            let compact_summary = paired_proof_required_bundle_summary(receipt, comparison_bundles);
            ui.label(format!(
                "{} | created:{} | artifacts:{}",
                compact_summary,
                receipt.created_at.as_deref().unwrap_or("unknown"),
                paired_proof_artifact_count(receipt, path)
            ));

            if let Some(note) = self.paired_proof_notes.get(&receipt.receipt_id) {
                if !note.trim().is_empty() {
                    ui.label(format!("Note: {}", note.trim()));
                }
            }

            ui.horizontal(|ui| {
                let path_text = path.to_string_lossy().to_string();
                if ui.button("Open").clicked() {
                    match open_path_in_explorer(&path_text, true) {
                        Ok(()) => {
                            self.status_line =
                                format!("Opened paired proof receipt {}", short_path(&path_text));
                        }
                        Err(error) => {
                            self.status_line =
                                format!("failed to open paired proof receipt: {}", error);
                        }
                    }
                }
                if ui.button("Copy").clicked() {
                    self.copy_paired_proof_summary(receipt, path, ui.ctx());
                }
                if ui.button("Export").clicked() {
                    self.export_paired_proof_summary(receipt, path);
                }
                if show_unpin && ui.button("Unpin").clicked() {
                    self.toggle_paired_proof_favorite(&receipt.receipt_id);
                }
            });
        });
    }
}

fn latest_proof_last_touched(
    activity: &[ExtensionActivityItem],
    receipt_id: &str,
) -> Option<String> {
    activity
        .iter()
        .rev()
        .find(|item| item.entry_id.as_deref() == Some(receipt_id))
        .map(|item| item.timestamp_label.clone())
}

fn paired_proof_required_bundle_summary(
    receipt: &CrossFamilyPairedProofReceiptSummary,
    comparison_bundles: &[CapabilityComparisonBundle],
) -> String {
    let Some(bundle) = proof_receipt_comparison_bundle(receipt, comparison_bundles) else {
        return "No comparison bundle".to_string();
    };
    let Some(comparison_receipt) =
        load_capability_comparison_receipt(&receipt.comparison_receipt_path)
    else {
        return "Comparison receipt unavailable".to_string();
    };
    compact_required_bundle_summary(&bundle, &comparison_receipt)
}
