use super::*;

impl ChattyFactoryUiApp {
    pub(crate) fn render_family_governance_panel(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("Family Governance");
        let family_governance_receipts = load_family_governance_receipts(&self.workspace_root);
        let stable_families = family_governance_receipts
            .iter()
            .filter(|receipt| {
                receipt.change_since_last_live_status == "stable_since_last_live"
                    || receipt.change_since_last_live_status == "baseline_recorded"
            })
            .count();
        let changed_families = family_governance_receipts
            .iter()
            .filter(|receipt| receipt.change_since_last_live_status == "changed_since_last_live")
            .count();
        let regressed_families = family_governance_receipts
            .iter()
            .filter(|receipt| receipt.change_since_last_live_status == "regressed_since_last_live")
            .count();
        let ecosystem_native_families = family_governance_receipts
            .iter()
            .filter(|receipt| is_canonical_ecosystem_shell_family(&receipt.family_id))
            .count();
        let family_usage_summary = load_family_usage_summary(&self.workspace_root);
        if render_governance_metric_strip(
            ui,
            &[
                format!("Governed families: {}", family_governance_receipts.len()),
                format!("Canonical ecosystem shell families: {ecosystem_native_families}"),
                format!(
                    "Built family-backed projects: {}",
                    family_usage_summary
                        .as_ref()
                        .map(|summary| summary.total_projects)
                        .unwrap_or(0)
                ),
                format!("Stable families: {stable_families}"),
                format!("Changed families: {changed_families}"),
                format!("Regressed families: {regressed_families}"),
            ],
            "Refresh family governance now",
        ) {
            self.spawn_task(UiTask::RefreshFamilyGovernance);
        }
        if let Some(status) = &self.family_governance_refresh_status {
            let warning = if family_governance_refresh_is_stale(status) {
                Some((
                    egui::Color32::from_rgb(214, 143, 58),
                    governance_stale_warning("Family governance", status.age_minutes.unwrap_or(0)),
                ))
            } else if family_governance_auto_refresh_in_cooldown(
                self.last_auto_family_governance_refresh_unix_secs,
            ) {
                Some((
                    egui::Color32::from_rgb(184, 160, 84),
                    governance_cooldown_warning(
                        "family governance",
                        "Refresh family governance now",
                    ),
                ))
            } else {
                None
            };
            render_governance_refresh_state(
                ui,
                governance_refresh_status_summary(
                    "family governance",
                    &status.updated_at,
                    status.refreshed_at_label.as_deref(),
                    &status.status_id,
                    status.refreshed_entries,
                    status.skipped_entries,
                ),
                warning,
            );
        } else {
            render_governance_refresh_state(
                ui,
                governance_never_refreshed_summary("family governance"),
                Some((
                    egui::Color32::from_rgb(214, 143, 58),
                    governance_never_refreshed_warning("Family governance"),
                )),
            );
        }
        if !family_governance_receipts.is_empty() {
            if self.selected_family_governance_id.is_none()
                || !family_governance_receipts.iter().any(|receipt| {
                    Some(receipt.family_id.as_str())
                        == self.selected_family_governance_id.as_deref()
                })
            {
                self.selected_family_governance_id = family_governance_receipts
                    .first()
                    .map(|receipt| receipt.family_id.clone());
            }
            egui::ComboBox::from_id_source("family-governance-picker")
                .selected_text(
                    self.selected_family_governance_id
                        .as_deref()
                        .unwrap_or("select family"),
                )
                .show_ui(ui, |ui| {
                    for receipt in &family_governance_receipts {
                        ui.selectable_value(
                            &mut self.selected_family_governance_id,
                            Some(receipt.family_id.clone()),
                            family_governance_picker_label(receipt),
                        );
                    }
                });
            if let Some(receipt) = family_governance_receipts.iter().find(|receipt| {
                Some(receipt.family_id.as_str()) == self.selected_family_governance_id.as_deref()
            }) {
                ui.label(format!(
                    "Family: {}",
                    family_summary_label_from_receipt(receipt)
                ));
                if is_canonical_ecosystem_shell_family(&receipt.family_id) {
                    ui.label("Shell role: canonical ecosystem-native starter");
                }
                if let Some(ecosystem) = receipt.family_ecosystem.as_deref() {
                    ui.label(format!("Ecosystem: {ecosystem}"));
                }
                if let Some(summary) = family_usage_summary.as_ref() {
                    if let Some(entry) = summary
                        .families
                        .iter()
                        .find(|entry| entry.family_id == receipt.family_id)
                    {
                        ui.label(format!(
                            "Built projects using this family: {}",
                            entry.project_count
                        ));
                    } else {
                        ui.label("Built projects using this family: 0");
                    }
                    if let Some(ecosystem) = receipt.family_ecosystem.as_deref() {
                        if let Some(count) = summary.ecosystem_project_counts.get(ecosystem) {
                            ui.label(format!(
                                "Built projects across the {ecosystem} ecosystem: {count}"
                            ));
                        }
                    }
                }
                ui.label(format!("Primary substrate: {}", receipt.primary_substrate));
                ui.label(format!("Lifecycle: {}", receipt.lifecycle_status));
                if !receipt.lifecycle_notes.is_empty() {
                    for note in receipt.lifecycle_notes.iter().take(3) {
                        ui.label(format!("- {note}"));
                    }
                }
                ui.label(format!(
                    "Supported tool kinds: {}",
                    if receipt.supported_tool_kinds.is_empty() {
                        "none".to_string()
                    } else {
                        receipt.supported_tool_kinds.join(", ")
                    }
                ));
                ui.label(format!(
                    "Build primitive classes: {}",
                    if receipt.provided_build_primitive_classes.is_empty() {
                        "none".to_string()
                    } else {
                        receipt.provided_build_primitive_classes.join(", ")
                    }
                ));
                ui.label(format!(
                    "Primitive adapters: {}",
                    if receipt.primitive_adapter_ids.is_empty() {
                        "none".to_string()
                    } else {
                        receipt.primitive_adapter_ids.join(", ")
                    }
                ));
                render_governance_detail_block(
                    ui,
                    "Family",
                    Some(family_governed_artifact_set_summary(receipt).as_str()),
                    Some(&receipt.drift_status),
                    Some(&receipt.change_since_last_live_status),
                    &receipt.drift_notes,
                    &receipt.change_since_last_live_notes,
                );
                ui.label(format!("Manifest: {}", short_path(&receipt.manifest_path)));
                ui.horizontal_wrapped(|ui| {
                    let receipt_path = self
                        .workspace_root
                        .join("runtime")
                        .join("family_governance_receipts")
                        .join(format!("{}.json", receipt.family_id));
                    if ui.small_button("Open Family Receipt").clicked() {
                        self.reveal_governed_artifact(
                            receipt_path.to_string_lossy().as_ref(),
                            "Revealed family governance receipt",
                            "Failed to reveal family governance receipt",
                            "Revealed family governance receipt",
                            "Failed to reveal family governance receipt",
                            None,
                        );
                    }
                    if ui.small_button("Open Family Manifest").clicked() {
                        self.reveal_governed_artifact(
                            &receipt.manifest_path,
                            "Revealed family manifest",
                            "Failed to reveal family manifest",
                            "Revealed family manifest",
                            "Failed to reveal family manifest",
                            None,
                        );
                    }
                });
            }
            if let Some(summary) = family_usage_summary.as_ref() {
                ui.separator();
                ui.label(format!(
                    "Family usage summary: {} built project(s) across {} governed family surface(s)",
                    summary.total_projects,
                    summary.families.len()
                ));
                ui.label(format!("Usage summary id: {}", summary.summary_id));
                ui.label(format!("Usage updated: {}", summary.updated_at));
                for entry in summary.families.iter().take(6) {
                    let label = match entry.family_ecosystem.as_deref() {
                        Some(ecosystem) => format!(
                            "- {} [ecosystem: {}] -> {} project(s)",
                            entry.family_display_name, ecosystem, entry.project_count
                        ),
                        None => format!(
                            "- {} -> {} project(s)",
                            entry.family_display_name, entry.project_count
                        ),
                    };
                    ui.label(label);
                }
            }
        } else {
            ui.label("No governed family receipts found yet.");
        }
    }

    pub(crate) fn render_template_governance_panel(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("Template Governance");
        let template_governance_receipts = load_template_governance_receipts(&self.workspace_root);
        let stable_templates = template_governance_receipts
            .iter()
            .filter(|receipt| {
                receipt.change_since_last_live_status == "stable_since_last_live"
                    || receipt.change_since_last_live_status == "baseline_recorded"
            })
            .count();
        let changed_templates = template_governance_receipts
            .iter()
            .filter(|receipt| receipt.change_since_last_live_status == "changed_since_last_live")
            .count();
        let regressed_templates = template_governance_receipts
            .iter()
            .filter(|receipt| receipt.change_since_last_live_status == "regressed_since_last_live")
            .count();
        if render_governance_metric_strip(
            ui,
            &[
                format!(
                    "Governed template bundles: {}",
                    template_governance_receipts.len()
                ),
                format!("Stable template bundles: {stable_templates}"),
                format!("Changed template bundles: {changed_templates}"),
                format!("Regressed template bundles: {regressed_templates}"),
            ],
            "Refresh template governance now",
        ) {
            self.spawn_task(UiTask::RefreshTemplateGovernance);
        }
        if let Some(status) = &self.template_governance_refresh_status {
            let warning = if template_governance_refresh_is_stale(status) {
                Some((
                    egui::Color32::from_rgb(214, 143, 58),
                    governance_stale_warning(
                        "Template governance",
                        status.age_minutes.unwrap_or(0),
                    ),
                ))
            } else if template_governance_auto_refresh_in_cooldown(
                self.last_auto_template_governance_refresh_unix_secs,
            ) {
                Some((
                    egui::Color32::from_rgb(184, 160, 84),
                    governance_cooldown_warning(
                        "template governance",
                        "Refresh template governance now",
                    ),
                ))
            } else {
                None
            };
            render_governance_refresh_state(
                ui,
                governance_refresh_status_summary(
                    "template governance",
                    &status.updated_at,
                    status.refreshed_at_label.as_deref(),
                    &status.status_id,
                    status.refreshed_entries,
                    status.skipped_entries,
                ),
                warning,
            );
        } else {
            render_governance_refresh_state(
                ui,
                governance_never_refreshed_summary("template governance"),
                Some((
                    egui::Color32::from_rgb(214, 143, 58),
                    governance_never_refreshed_warning("Template governance"),
                )),
            );
        }
        if !template_governance_receipts.is_empty() {
            if self.selected_template_governance_id.is_none()
                || !template_governance_receipts.iter().any(|receipt| {
                    Some(receipt.template_bundle_id.as_str())
                        == self.selected_template_governance_id.as_deref()
                })
            {
                self.selected_template_governance_id = template_governance_receipts
                    .first()
                    .map(|receipt| receipt.template_bundle_id.clone());
            }
            egui::ComboBox::from_id_source("template-governance-picker")
                .selected_text(
                    self.selected_template_governance_id
                        .as_deref()
                        .unwrap_or("select template bundle"),
                )
                .show_ui(ui, |ui| {
                    for receipt in &template_governance_receipts {
                        ui.selectable_value(
                            &mut self.selected_template_governance_id,
                            Some(receipt.template_bundle_id.clone()),
                            &receipt.template_bundle_id,
                        );
                    }
                });
            if let Some(receipt) = template_governance_receipts.iter().find(|receipt| {
                Some(receipt.template_bundle_id.as_str())
                    == self.selected_template_governance_id.as_deref()
            }) {
                ui.label(format!("Template category: {}", receipt.template_category));
                ui.label(format!(
                    "Template root: {}",
                    short_path(&receipt.template_root)
                ));
                ui.label(format!("Artifact count: {}", receipt.artifact_paths.len()));
                render_governance_detail_block(
                    ui,
                    "Template",
                    Some(template_governed_artifact_set_summary(receipt).as_str()),
                    Some(&receipt.drift_status),
                    Some(&receipt.change_since_last_live_status),
                    &receipt.drift_notes,
                    &receipt.change_since_last_live_notes,
                );
                for path in &receipt.artifact_paths {
                    ui.label(format!("- {}", short_path(path)));
                }
                ui.horizontal_wrapped(|ui| {
                    let receipt_path = self
                        .workspace_root
                        .join("runtime")
                        .join("template_governance_receipts")
                        .join(format!("{}.json", receipt.template_bundle_id));
                    if ui.small_button("Open Template Receipt").clicked() {
                        self.reveal_governed_artifact(
                            receipt_path.to_string_lossy().as_ref(),
                            "Revealed template governance receipt",
                            "Failed to reveal template governance receipt",
                            "Revealed template governance receipt",
                            "Failed to reveal template governance receipt",
                            None,
                        );
                    }
                    if ui.small_button("Open Template Root").clicked() {
                        self.reveal_governed_artifact(
                            &receipt.template_root,
                            "Revealed template bundle root",
                            "Failed to reveal template bundle root",
                            "Revealed template bundle root",
                            "Failed to reveal template bundle root",
                            None,
                        );
                    }
                });
            }
        } else {
            ui.label("No governed template receipts found yet.");
        }
    }
}
