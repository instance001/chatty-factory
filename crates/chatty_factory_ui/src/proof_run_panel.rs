use super::*;

impl ChattyFactoryUiApp {
    pub(crate) fn render_proof_run_controls_section(
        &mut self,
        ui: &mut egui::Ui,
        proof_templates: &[PrimitiveProofTemplate],
        selected_proof_template: &Option<PrimitiveProofTemplate>,
        selected_comparison_bundle: &Option<CapabilityComparisonBundle>,
        previous_selected_profile_name: &str,
        previous_selected_template_id: &str,
        previous_follow_filter: bool,
    ) {
        ui.label("Run a generalized proof template across its declared family pair.");
        self.render_proof_profile_controls(ui, previous_selected_profile_name);
        self.render_proof_template_controls(
            ui,
            proof_templates,
            selected_proof_template,
            selected_comparison_bundle,
            previous_selected_template_id,
            previous_follow_filter,
        );
        self.render_proof_request_controls(ui);
        self.render_proof_history_filter_controls(ui, proof_templates);
    }

    fn render_proof_profile_controls(
        &mut self,
        ui: &mut egui::Ui,
        previous_selected_profile_name: &str,
    ) {
        ui.horizontal(|ui| {
            ui.label("Proof Profile");
            egui::ComboBox::from_id_source("proof-run-profile")
                .selected_text(proof_profile_label(&self.selected_proof_profile_name))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.selected_proof_profile_name,
                        "custom".to_string(),
                        "Custom",
                    );
                    for profile in &self.proof_run_profiles {
                        ui.selectable_value(
                            &mut self.selected_proof_profile_name,
                            profile.profile_name.clone(),
                            &profile.profile_name,
                        );
                    }
                });
            ui.label("Save as");
            ui.add(
                egui::TextEdit::singleline(&mut self.proof_profile_name_input)
                    .desired_width(140.0)
                    .hint_text("profile name"),
            );
            if ui.small_button("Save Profile").clicked() {
                let profile_name = self.proof_profile_name_input.clone();
                self.save_current_proof_run_profile(&profile_name);
            }
            if ui.small_button("Duplicate Profile").clicked() {
                let profile_name = self.proof_profile_name_input.clone();
                self.duplicate_selected_proof_run_profile(&profile_name);
            }
            if ui
                .add_enabled(
                    self.selected_proof_profile_name != "custom",
                    egui::Button::new("Delete Profile"),
                )
                .clicked()
            {
                self.delete_selected_proof_run_profile();
            }
            if ui.small_button("Reset Defaults").clicked() {
                self.reset_proof_run_defaults();
            }
        });
        if self.selected_proof_profile_name != previous_selected_profile_name {
            if let Some(profile) = self
                .proof_run_profiles
                .iter()
                .find(|profile| profile.profile_name == self.selected_proof_profile_name)
                .cloned()
            {
                self.apply_proof_run_profile(&profile);
            }
            self.save_paired_proof_ui_preferences();
        }
    }

    fn render_proof_template_controls(
        &mut self,
        ui: &mut egui::Ui,
        proof_templates: &[PrimitiveProofTemplate],
        selected_proof_template: &Option<PrimitiveProofTemplate>,
        selected_comparison_bundle: &Option<CapabilityComparisonBundle>,
        previous_selected_template_id: &str,
        previous_follow_filter: bool,
    ) {
        egui::ComboBox::from_label("Proof Template")
            .selected_text(
                selected_proof_template
                    .as_ref()
                    .map(proof_template_label)
                    .unwrap_or_else(|| "Unknown template".to_string()),
            )
            .show_ui(ui, |ui| {
                for template in proof_templates {
                    ui.selectable_value(
                        &mut self.selected_proof_template_id,
                        template.template_id.clone(),
                        proof_template_label(template),
                    );
                }
            });
        if self.pin_history_filter_to_selected_template
            && self.proof_history_template_filter != self.selected_proof_template_id
        {
            self.proof_history_template_filter = self.selected_proof_template_id.clone();
        }
        ui.checkbox(
            &mut self.pin_history_filter_to_selected_template,
            "Follow selected template in history",
        );
        if self.selected_proof_template_id != previous_selected_template_id
            || self.pin_history_filter_to_selected_template != previous_follow_filter
        {
            self.save_paired_proof_ui_preferences();
        }

        if let Some(template) = selected_proof_template {
            ui.label(format!(
                "Families: {}",
                template
                    .target_family_ids
                    .iter()
                    .map(|family| family.as_str())
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ));
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.label(
                    egui::RichText::new("Template Scope")
                        .strong()
                        .color(egui::Color32::from_rgb(88, 132, 201)),
                );
                ui.horizontal(|ui| {
                    ui.label(format!("Template id: {}", template.template_id));
                    ui.separator();
                    ui.label(format!("Kind: {}", template.template_kind));
                    if ui.small_button("Open Template Contract").clicked() {
                        let template_path = proof_template_manifest_path(
                            &self.workspace_root,
                            &template.template_id,
                        )
                        .unwrap_or_else(|| {
                            self.workspace_root
                                .join("crates")
                                .join("chatty_factory_core")
                                .join("src")
                                .join("proof_harness.rs")
                        });
                        let path_text = template_path.to_string_lossy().to_string();
                        match open_path_in_explorer(&path_text, true) {
                            Ok(()) => {
                                self.status_line = "Opened proof template contract".to_string();
                                self.push_extension_activity(
                                    Some(template.template_id.clone()),
                                    "Open Proof Template Contract",
                                    short_path(&path_text),
                                    true,
                                );
                                self.push_toast(
                                    "Opened proof template contract",
                                    ToastKind::Success,
                                );
                            }
                            Err(error) => {
                                self.status_line = format!("Open failed: {error}");
                                self.push_extension_activity(
                                    Some(template.template_id.clone()),
                                    "Open Proof Template Contract",
                                    error.to_string(),
                                    false,
                                );
                                self.push_toast(format!("Open failed: {error}"), ToastKind::Error);
                            }
                        }
                    }
                    if ui.small_button("Open Comparison Bundle Contract").clicked() {
                        let bundle_path = capability_comparison_bundle_manifest_path(
                            &self.workspace_root,
                            &template.execution_recipe.comparison_bundle_id,
                        )
                        .unwrap_or_else(|| {
                            self.workspace_root
                                .join("crates")
                                .join("chatty_factory_core")
                                .join("src")
                                .join("proof_harness.rs")
                        });
                        let path_text = bundle_path.to_string_lossy().to_string();
                        match open_path_in_explorer(&path_text, true) {
                            Ok(()) => {
                                self.status_line = "Opened comparison bundle contract".to_string();
                                self.push_extension_activity(
                                    Some(template.template_id.clone()),
                                    "Open Comparison Bundle Contract",
                                    short_path(&path_text),
                                    true,
                                );
                                self.push_toast(
                                    "Opened comparison bundle contract",
                                    ToastKind::Success,
                                );
                            }
                            Err(error) => {
                                self.status_line = format!("Open failed: {error}");
                                self.push_extension_activity(
                                    Some(template.template_id.clone()),
                                    "Open Comparison Bundle Contract",
                                    error.to_string(),
                                    false,
                                );
                                self.push_toast(format!("Open failed: {error}"), ToastKind::Error);
                            }
                        }
                    }
                });
                ui.label(proof_template_description(template));
                if !template.required_composition_layers.is_empty() {
                    ui.label(format!(
                        "Layers: {}",
                        template.required_composition_layers.join(", ")
                    ));
                }
                if !template.required_capability_classes.is_empty() {
                    ui.label("Required capabilities");
                    for capability in template.required_capability_classes.iter().take(8) {
                        ui.label(format!("- {capability}"));
                    }
                }
                if !template.optional_capability_classes.is_empty() {
                    ui.label(format!(
                        "Optional capabilities: {}",
                        template.optional_capability_classes.join(", ")
                    ));
                }
                if !template.optional_enrichment_steps.is_empty() {
                    ui.label(format!(
                        "Possible enrichments: {}",
                        template.optional_enrichment_steps.join(", ")
                    ));
                }
                if let Some(bundle) = selected_comparison_bundle {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new("Comparison Bundle")
                            .strong()
                            .color(egui::Color32::from_rgb(201, 132, 88)),
                    );
                    ui.label(format!("Bundle id: {}", bundle.bundle_id));
                    ui.label(format!("Mode: {}", bundle.equivalence_mode));
                    ui.label(format!(
                        "Minimum shared capability count: {}",
                        bundle.minimum_shared_capability_count
                    ));
                    if !bundle.required_shared_capability_classes.is_empty() {
                        ui.label("Required shared capabilities");
                        for capability in bundle.required_shared_capability_classes.iter().take(8) {
                            ui.label(format!("- {capability}"));
                        }
                    }
                    if !bundle.optional_shared_capability_classes.is_empty() {
                        ui.label(format!(
                            "Optional shared capabilities: {}",
                            bundle.optional_shared_capability_classes.join(", ")
                        ));
                    }
                    if !bundle.tolerated_left_only_capability_classes.is_empty() {
                        ui.label(format!(
                            "Tolerated left-only capabilities: {}",
                            bundle.tolerated_left_only_capability_classes.join(", ")
                        ));
                    }
                    if !bundle.tolerated_right_only_capability_classes.is_empty() {
                        ui.label(format!(
                            "Tolerated right-only capabilities: {}",
                            bundle.tolerated_right_only_capability_classes.join(", ")
                        ));
                    }
                }
            });
        }
    }

    fn render_proof_request_controls(&mut self, ui: &mut egui::Ui) {
        let latest_retry_search_proof =
            load_latest_retry_search_proof_receipt(&self.workspace_root);
        let estimated_retry_search_outer_timeout_secs = latest_retry_search_proof
            .as_ref()
            .map(|(receipt, _)| receipt.expected_outer_timeout_secs)
            .or_else(|| {
                estimate_retry_search_outer_timeout_secs(
                    &self.runtime_status,
                    self.planner_model.trim(),
                )
            });
        let shell_timeout_buffer_secs = latest_retry_search_proof
            .as_ref()
            .and_then(|(receipt, _)| {
                self.runtime_status
                    .config
                    .as_ref()
                    .map(|config| config.shell_timeout_buffer_secs)
                    .or_else(|| Some(receipt.cleanup_overhead_secs.max(60)))
            })
            .or_else(|| {
                self.runtime_status
                    .config
                    .as_ref()
                    .map(|config| config.shell_timeout_buffer_secs)
            });
        let recommended_shell_timeout_secs = estimated_retry_search_outer_timeout_secs
            .zip(shell_timeout_buffer_secs)
            .map(|(secs, buffer)| secs.saturating_add(buffer));

        ui.add(
            egui::TextEdit::multiline(&mut self.paired_proof_request_input)
                .hint_text(
                    "Optional shared request. Leave blank for the curated proof template seed.",
                )
                .desired_rows(3),
        );
        ui.add_space(6.0);
        ui.group(|ui| {
            ui.label(
                egui::RichText::new("Proof Runtime Posture")
                    .strong()
                    .color(egui::Color32::from_rgb(112, 148, 96)),
            );
            if let Some(config) = &self.runtime_status.config {
                ui.label(format!(
                    "Current runtime request budgets: launch {}s, planner {}s, model-task {}s",
                    config.launch_timeout_secs,
                    config.planner_request_timeout_secs,
                    config.model_task_request_timeout_secs
                ));
            } else {
                ui.label("No runtime config loaded yet; proof budgets will use the current runtime defaults on disk.");
            }
            ui.label(
                "Shell timeout is not proof failure by itself. Treat the factory receipt as the source of truth for long-running proof work.",
            );
            if let Some((receipt, receipt_path)) = latest_retry_search_proof.as_ref() {
                ui.label(format!(
                    "Latest retry-search ladder proof: {} [{}]",
                    receipt
                        .final_outcome
                        .as_deref()
                        .unwrap_or(receipt.status.as_str()),
                    receipt.status
                ));
                ui.label(format!(
                    "Latest ladder ceiling: {}s across {} model candidate(s) and {} retry posture(s)",
                    receipt.expected_outer_timeout_secs,
                    receipt.model_candidate_count,
                    receipt.retry_posture_count
                ));
                ui.label(format!(
                    "Latest ladder receipt: {}",
                    short_path(receipt_path.to_string_lossy().as_ref())
                ));
                if receipt.internal_timeout_observed {
                    ui.label(
                        "Latest ladder proof observed an internal timeout inside the factory runtime."
                    );
                } else if receipt.method_space_exhausted {
                    ui.label(
                        "Latest ladder proof exhausted the full available candidate ladder."
                    );
                } else if receipt.status == "running" {
                    ui.label(
                        "Latest ladder proof is still marked running, so an external cutoff should be treated as inconclusive."
                    );
                }
            } else {
                ui.label(
                    "No retry-search ladder proof receipt found yet. Once one exists, this panel will show its budget and final outcome here."
                );
            }
            ui.add_space(6.0);
            if ui
                .add_enabled(
                    !self.task_running,
                    egui::Button::new("Run Retry-Search Ladder Proof"),
                )
                .clicked()
            {
                self.spawn_task(UiTask::RunRetrySearchLadderProof {
                    auto_planner: self.auto_planner,
                    port: self.planner_port.trim().to_string(),
                    model: self.planner_model.trim().to_string(),
                });
            }
            if let Some(outer_timeout_secs) = estimated_retry_search_outer_timeout_secs {
                ui.label(format!(
                    "Factory-owned ladder ceiling: {}s",
                    outer_timeout_secs
                ));
            }
            if let Some(shell_timeout_secs) = recommended_shell_timeout_secs {
                ui.label(format!(
                    "Recommended shell timeout: {}s",
                    shell_timeout_secs
                ));
            }
            if let Some(buffer_secs) = shell_timeout_buffer_secs {
                ui.label(format!(
                    "Shell timeout buffer: {}s",
                    buffer_secs
                ));
            }
        });
        if ui
            .add_enabled(!self.task_running, egui::Button::new("Run Proof Template"))
            .clicked()
        {
            self.spawn_task(UiTask::RunProofTemplate {
                template_id: self.selected_proof_template_id.clone(),
                request: self.paired_proof_request_input.trim().to_string(),
                auto_planner: self.auto_planner,
                port: self.planner_port.trim().to_string(),
                model: self.planner_model.trim().to_string(),
            });
        }
    }

    fn render_proof_history_filter_controls(
        &mut self,
        ui: &mut egui::Ui,
        proof_templates: &[PrimitiveProofTemplate],
    ) {
        let previous_history_filter = self.proof_history_template_filter.clone();
        ui.horizontal(|ui| {
            ui.label("History Filter");
            egui::ComboBox::from_id_source("proof-history-template-filter")
                .selected_text(proof_history_filter_label(
                    &self.proof_history_template_filter,
                    proof_templates,
                ))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.proof_history_template_filter,
                        "all".to_string(),
                        "All Templates",
                    );
                    for template in proof_templates {
                        ui.selectable_value(
                            &mut self.proof_history_template_filter,
                            template.template_id.clone(),
                            proof_template_label(template),
                        );
                    }
                });
        });
        if self.proof_history_template_filter != previous_history_filter {
            self.save_paired_proof_ui_preferences();
        }
    }
}

fn estimate_retry_search_outer_timeout_secs(
    runtime_status: &RuntimeStatusView,
    requested_model: &str,
) -> Option<u64> {
    let config = runtime_status.config.as_ref()?;
    let retry_posture_count = 2u64;
    let cleanup_overhead_secs = 15u64;
    let model_candidate_count = if !requested_model.trim().is_empty() {
        1u64
    } else if runtime_status.catalog.is_some() {
        3u64
    } else {
        1u64
    };
    let per_attempt_budget = config
        .launch_timeout_secs
        .saturating_add(config.model_task_request_timeout_secs)
        .saturating_add(cleanup_overhead_secs);
    Some(
        retry_posture_count
            .saturating_mul(model_candidate_count)
            .saturating_mul(per_attempt_budget),
    )
}
