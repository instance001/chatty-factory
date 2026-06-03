use super::*;

impl ChattyFactoryUiApp {
    pub(crate) fn render_runtime_registry_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("Runtime Status");
        if let Some(config) = &self.runtime_status.config {
            ui.label(format!("Host: {}:{}", config.host, config.port));
            ui.label(format!("Context: {}", config.context_size));
            ui.label(format!("GPU layers: {}", config.gpu_layers));
            ui.label(format!(
                "Default model: {}",
                short_path(config.default_model_path.as_deref().unwrap_or("none"))
            ));
        } else {
            ui.label("No runtime config found yet.");
        }
        ui.separator();
        if let Some(catalog) = &self.runtime_status.catalog {
            ui.label(format!("Discovered models: {}", catalog.models.len()));
            ui.label(format!(
                "Fast: {}",
                short_path(catalog.preferred_fast_model_path.as_deref().unwrap_or("none"))
            ));
            ui.label(format!(
                "Balanced: {}",
                short_path(
                    catalog
                        .preferred_balanced_model_path
                        .as_deref()
                        .unwrap_or("none")
                )
            ));
            ui.label(format!(
                "Heavy: {}",
                short_path(catalog.preferred_heavy_model_path.as_deref().unwrap_or("none"))
            ));
        } else {
            ui.label("No runtime model catalog found yet.");
        }
        ui.separator();
        ui.heading("Family Usage");
        if let Some(summary) = load_family_usage_summary(&self.workspace_root) {
            let canonical_shell_entries = summary
                .families
                .iter()
                .filter(|entry| is_canonical_ecosystem_shell_family(&entry.family_id))
                .collect::<Vec<_>>();
            let canonical_shell_project_count = canonical_shell_entries
                .iter()
                .map(|entry| entry.project_count)
                .sum::<usize>();
            ui.label(format!("Built projects tracked: {}", summary.total_projects));
            ui.label(format!(
                "Canonical ecosystem shell projects: {} across {} family surface(s)",
                canonical_shell_project_count,
                canonical_shell_entries.len()
            ));
            if summary.ecosystem_project_counts.is_empty() {
                ui.label("No ecosystem-native family usage recorded yet.");
            } else {
                for (ecosystem, count) in &summary.ecosystem_project_counts {
                    ui.label(format!("{ecosystem}: {count} project(s)"));
                }
            }
            if !canonical_shell_entries.is_empty() {
                ui.label("Canonical ecosystem shell set");
                for entry in canonical_shell_entries.iter().take(3) {
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
            if !summary.families.is_empty() {
                ui.label("Top family usage");
                for entry in summary.families.iter().take(5) {
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
            ui.horizontal_wrapped(|ui| {
                if ui.small_button("Refresh family governance now").clicked() {
                    self.spawn_task(UiTask::RefreshFamilyGovernance);
                }
                ui.separator();
                ui.label(format!("Usage updated: {}", summary.updated_at));
            });
        } else {
            ui.label("No family usage summary found yet.");
            if ui.small_button("Refresh family governance now").clicked() {
                self.spawn_task(UiTask::RefreshFamilyGovernance);
            }
        }
        ui.separator();
        ui.heading("Starter Usage");
        if let Some(summary) = load_starter_usage_summary(&self.workspace_root) {
            let recent_builds = load_recent_build_receipts(&self.workspace_root);
            let recent_overrides = recent_builds
                .iter()
                .filter(|receipt| {
                    receipt.starter_recommendation_comparison.as_deref()
                        == Some("overrode_normal_routing")
                })
                .collect::<Vec<_>>();
            ui.label(format!("Build receipts tracked: {}", summary.total_build_receipts));
            ui.label(format!(
                "Explicit mechanical starter builds: {}",
                summary.explicit_override_builds
            ));
            ui.label(format!("Auto-routed builds: {}", summary.auto_routed_builds));
            ui.label(format!(
                "Matched normal recommendation: {}",
                summary.matched_recommendation_builds
            ));
            ui.label(format!(
                "Overrode normal recommendation: {}",
                summary.overridden_recommendation_builds
            ));
            ui.label(format!(
                "Recent override events in view: {}",
                recent_overrides.len()
            ));
            if !summary.starters.is_empty() {
                ui.label("Top starter usage");
                for entry in summary.starters.iter().take(5) {
                    ui.label(format!(
                        "- {} [{} | {}] -> {} build(s)",
                        entry.starter_label,
                        entry.starter_id,
                        entry.starter_lifecycle,
                        entry.build_count
                    ));
                }
            }
            egui::CollapsingHeader::new("Starter Usage Deep View")
                .default_open(false)
                .show(ui, |ui| {
                    if !recent_builds.is_empty() {
                        ui.label("Recent starter decisions");
                        for receipt in recent_builds.iter().take(5) {
                            let chosen_starter_id =
                                receipt.starter_override_id.as_deref().unwrap_or("auto");
                            let chosen_label = build_starter_label(chosen_starter_id);
                            let comparison = receipt
                                .starter_recommendation_comparison
                                .as_deref()
                                .unwrap_or("unknown");
                            let recommended_label = receipt
                                .recommended_starter_id
                                .as_deref()
                                .map(build_starter_label)
                                .unwrap_or("none");
                            ui.label(format!(
                                "- {} -> chosen: {} [{}] | recommended: {} | comparison: {}",
                                receipt.project_name,
                                chosen_label,
                                chosen_starter_id,
                                recommended_label,
                                comparison
                            ));
                            if let Some(summary) = receipt.starter_override_summary.as_deref() {
                                ui.label(format!("  override: {summary}"));
                            }
                            if let Some(summary) = receipt.recommended_starter_summary.as_deref() {
                                ui.label(format!("  recommendation: {summary}"));
                            }
                        }
                    }
                    if !recent_overrides.is_empty() {
                        ui.separator();
                        ui.label("Recent overrides only");
                        for receipt in recent_overrides.into_iter().take(5) {
                            let chosen_starter_id =
                                receipt.starter_override_id.as_deref().unwrap_or("auto");
                            let chosen_label = build_starter_label(chosen_starter_id);
                            let recommended_label = receipt
                                .recommended_starter_id
                                .as_deref()
                                .map(build_starter_label)
                                .unwrap_or("none");
                            ui.label(format!(
                                "- {} -> override: {} [{}] instead of {}",
                                receipt.project_name,
                                chosen_label,
                                chosen_starter_id,
                                recommended_label
                            ));
                            if let Some(summary) = receipt.starter_override_summary.as_deref() {
                                ui.label(format!("  override: {summary}"));
                            }
                        }
                    }
                });
            ui.horizontal_wrapped(|ui| {
                if ui.small_button("Refresh family governance now").clicked() {
                    self.spawn_task(UiTask::RefreshFamilyGovernance);
                }
                ui.separator();
                ui.label(format!("Starter usage id: {}", summary.summary_id));
                ui.separator();
                ui.label(format!("Starter usage updated: {}", summary.updated_at));
            });
        } else {
            ui.label("No starter usage summary found yet.");
            if ui.small_button("Refresh family governance now").clicked() {
                self.spawn_task(UiTask::RefreshFamilyGovernance);
            }
        }
        ui.separator();
        ui.heading("Extension Registry");
        if let Some(registry) = self.extension_registry.clone() {
            ui.label(format!(
                "Shipped: {} | Active: {} | Archived: {}",
                registry.fully_live_count,
                registry.active_entries.len(),
                registry.archived_count
            ));
            let regressed_proofs =
                count_proof_baseline_status(&registry, "regressed_since_last_pass");
            let changed_proofs =
                count_proof_baseline_status(&registry, "changed_since_last_pass");
            let no_baseline_proofs =
                count_proof_baseline_status(&registry, "no_passing_baseline");
            let regressed_compositions =
                count_composition_baseline_status(&registry, "regressed_since_last_live");
            let changed_compositions =
                count_composition_baseline_status(&registry, "changed_since_last_live");
            let no_baseline_compositions =
                count_composition_baseline_status(&registry, "no_live_baseline");
            let regressed_patches =
                count_patch_baseline_status(&registry, "regressed_since_last_live");
            let changed_patches =
                count_patch_baseline_status(&registry, "changed_since_last_live");
            let no_baseline_patches =
                count_patch_baseline_status(&registry, "no_live_baseline");
            let regressed_helpers =
                count_helper_baseline_status(&registry, "regressed_since_last_live");
            let changed_helpers =
                count_helper_baseline_status(&registry, "changed_since_last_live");
            let no_baseline_helpers =
                count_helper_baseline_status(&registry, "no_live_baseline");
            let regressed_bridges =
                count_bridge_baseline_status(&registry, "regressed_since_last_live");
            let changed_bridges =
                count_bridge_baseline_status(&registry, "changed_since_last_live");
            let no_baseline_bridges =
                count_bridge_baseline_status(&registry, "no_live_baseline");
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("Regressed proofs: {regressed_proofs}"));
                ui.separator();
                ui.label(format!("Changed since last pass: {changed_proofs}"));
                ui.separator();
                ui.label(format!("No baseline yet: {no_baseline_proofs}"));
                ui.separator();
                ui.label(format!(
                    "Regressed composition bundles: {regressed_compositions}"
                ));
                ui.separator();
                ui.label(format!(
                    "Changed composition bundles: {changed_compositions}"
                ));
                ui.separator();
                ui.label(format!(
                    "Composition bundles with no baseline: {no_baseline_compositions}"
                ));
                ui.separator();
                ui.label(format!("Regressed patch recipes: {regressed_patches}"));
                ui.separator();
                ui.label(format!("Changed patch recipes: {changed_patches}"));
                ui.separator();
                ui.label(format!(
                    "Patch recipes with no baseline: {no_baseline_patches}"
                ));
                ui.separator();
                ui.label(format!("Regressed helper lanes: {regressed_helpers}"));
                ui.separator();
                ui.label(format!("Changed helper lanes: {changed_helpers}"));
                ui.separator();
                ui.label(format!(
                    "Helper lanes with no baseline: {no_baseline_helpers}"
                ));
                ui.separator();
                ui.label(format!("Regressed bridge lanes: {regressed_bridges}"));
                ui.separator();
                ui.label(format!("Changed bridge lanes: {changed_bridges}"));
                ui.separator();
                ui.label(format!(
                    "Bridge lanes with no baseline: {no_baseline_bridges}"
                ));
                ui.separator();
                if ui.small_button("Refresh proof governance now").clicked() {
                    self.spawn_task(UiTask::RefreshProofHarnessRegistry);
                }
                ui.separator();
                if ui.small_button("Refresh composition governance now").clicked() {
                    self.spawn_task(UiTask::RefreshCompositionGovernance);
                }
                ui.separator();
                if ui.small_button("Refresh patch governance now").clicked() {
                    self.spawn_task(UiTask::RefreshPatchGovernance);
                }
                ui.separator();
                if ui.small_button("Refresh helper governance now").clicked() {
                    self.spawn_task(UiTask::RefreshHelperGovernance);
                }
                ui.separator();
                if ui.small_button("Refresh bridge governance now").clicked() {
                    self.spawn_task(UiTask::RefreshBridgeGovernance);
                }
            });
            egui::CollapsingHeader::new("Extension Registry Controls and Filters")
                .default_open(false)
                .show(ui, |ui| {
            if ui
                .checkbox(
                    &mut self.auto_refresh_stale_proof_governance,
                    "Auto refresh stale proof governance when newer proof receipts exist",
                )
                .changed()
            {
                self.save_paired_proof_ui_preferences();
            }
            if ui
                .checkbox(
                    &mut self.auto_refresh_stale_composition_governance,
                    "Auto refresh stale composition governance on launch",
                )
                .changed()
            {
                self.save_paired_proof_ui_preferences();
            }
            if ui
                .checkbox(
                    &mut self.auto_refresh_stale_patch_governance,
                    "Auto refresh stale patch governance on launch",
                )
                .changed()
            {
                self.save_paired_proof_ui_preferences();
            }
            if ui
                .checkbox(
                    &mut self.auto_refresh_stale_helper_governance,
                    "Auto refresh stale helper governance on launch",
                )
                .changed()
            {
                self.save_paired_proof_ui_preferences();
            }
            if ui
                .checkbox(
                    &mut self.auto_refresh_stale_bridge_governance,
                    "Auto refresh stale bridge governance on launch",
                )
                .changed()
            {
                self.save_paired_proof_ui_preferences();
            }
            if ui
                .checkbox(
                    &mut self.auto_refresh_stale_family_governance,
                    "Auto refresh stale family governance on launch",
                )
                .changed()
            {
                self.save_paired_proof_ui_preferences();
            }
            if ui
                .checkbox(
                    &mut self.auto_refresh_stale_template_governance,
                    "Auto refresh stale template governance on launch",
                )
                .changed()
            {
                self.save_paired_proof_ui_preferences();
            }
            if let Some(status) = &self.proof_governance_refresh_status {
                let warning = if proof_governance_refresh_is_stale(status) {
                    let warning = if status.newer_proof_receipts_exist {
                        format!(
                            "Proof governance view is stale ({} minutes old) and newer proof receipts exist{} Refresh strongly recommended.",
                            status.age_minutes.unwrap_or(0),
                            status
                                .latest_proof_receipt_label
                                .as_deref()
                                .map(|label| format!(" (latest receipt file: {label})."))
                                .unwrap_or_else(|| ".".to_string())
                        )
                    } else {
                        governance_stale_warning(
                            "Proof governance",
                            status.age_minutes.unwrap_or(0),
                        )
                    };
                    Some((egui::Color32::from_rgb(214, 143, 58), warning))
                } else if proof_governance_auto_refresh_in_cooldown(
                    self.last_auto_proof_governance_refresh_unix_secs,
                ) && status.newer_proof_receipts_exist
                {
                    Some((
                        egui::Color32::from_rgb(184, 160, 84),
                        format!(
                            "Newer proof receipts exist, but {}",
                            governance_cooldown_warning(
                                "proof governance",
                                "Refresh proof governance now",
                            )
                            .to_lowercase()
                        ),
                    ))
                } else {
                    None
                };
                render_governance_refresh_state(
                    ui,
                    governance_refresh_status_summary(
                        "proof governance",
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
                    governance_never_refreshed_summary("proof governance"),
                    Some((
                        egui::Color32::from_rgb(214, 143, 58),
                        governance_never_refreshed_warning("Proof governance"),
                    )),
                );
            }
            if let Some(status) = &self.composition_governance_refresh_status {
                let warning = if composition_governance_refresh_is_stale(status) {
                    Some((
                        egui::Color32::from_rgb(214, 143, 58),
                        governance_stale_warning(
                            "Composition governance",
                            status.age_minutes.unwrap_or(0),
                        ),
                    ))
                } else if composition_governance_auto_refresh_in_cooldown(
                    self.last_auto_composition_governance_refresh_unix_secs,
                ) {
                    Some((
                        egui::Color32::from_rgb(184, 160, 84),
                        governance_cooldown_warning(
                            "composition governance",
                            "Refresh composition governance now",
                        ),
                    ))
                } else {
                    None
                };
                render_governance_refresh_state(
                    ui,
                    governance_refresh_status_summary(
                        "composition governance",
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
                    governance_never_refreshed_summary("composition governance"),
                    Some((
                        egui::Color32::from_rgb(214, 143, 58),
                        governance_never_refreshed_warning("Composition governance"),
                    )),
                );
            }
            if let Some(status) = &self.patch_governance_refresh_status {
                let warning = if patch_governance_refresh_is_stale(status) {
                    Some((
                        egui::Color32::from_rgb(214, 143, 58),
                        governance_stale_warning("Patch governance", status.age_minutes.unwrap_or(0)),
                    ))
                } else if patch_governance_auto_refresh_in_cooldown(
                    self.last_auto_patch_governance_refresh_unix_secs,
                ) {
                    Some((
                        egui::Color32::from_rgb(184, 160, 84),
                        governance_cooldown_warning(
                            "patch governance",
                            "Refresh patch governance now",
                        ),
                    ))
                } else {
                    None
                };
                render_governance_refresh_state(
                    ui,
                    governance_refresh_status_summary(
                        "patch governance",
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
                    governance_never_refreshed_summary("patch governance"),
                    Some((
                        egui::Color32::from_rgb(214, 143, 58),
                        governance_never_refreshed_warning("Patch governance"),
                    )),
                );
            }
            if let Some(status) = &self.helper_governance_refresh_status {
                let warning = if helper_governance_refresh_is_stale(status) {
                    Some((
                        egui::Color32::from_rgb(214, 143, 58),
                        governance_stale_warning("Helper governance", status.age_minutes.unwrap_or(0)),
                    ))
                } else if helper_governance_auto_refresh_in_cooldown(
                    self.last_auto_helper_governance_refresh_unix_secs,
                ) {
                    Some((
                        egui::Color32::from_rgb(184, 160, 84),
                        governance_cooldown_warning(
                            "helper governance",
                            "Refresh helper governance now",
                        ),
                    ))
                } else {
                    None
                };
                render_governance_refresh_state(
                    ui,
                    governance_refresh_status_summary(
                        "helper governance",
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
                    governance_never_refreshed_summary("helper governance"),
                    Some((
                        egui::Color32::from_rgb(214, 143, 58),
                        governance_never_refreshed_warning("Helper governance"),
                    )),
                );
            }
            if let Some(status) = &self.bridge_governance_refresh_status {
                let warning = if bridge_governance_refresh_is_stale(status) {
                    Some((
                        egui::Color32::from_rgb(214, 143, 58),
                        governance_stale_warning("Bridge governance", status.age_minutes.unwrap_or(0)),
                    ))
                } else if bridge_governance_auto_refresh_in_cooldown(
                    self.last_auto_bridge_governance_refresh_unix_secs,
                ) {
                    Some((
                        egui::Color32::from_rgb(184, 160, 84),
                        governance_cooldown_warning(
                            "bridge governance",
                            "Refresh bridge governance now",
                        ),
                    ))
                } else {
                    None
                };
                render_governance_refresh_state(
                    ui,
                    governance_refresh_status_summary(
                        "bridge governance",
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
                    governance_never_refreshed_summary("bridge governance"),
                    Some((
                        egui::Color32::from_rgb(214, 143, 58),
                        governance_never_refreshed_warning("Bridge governance"),
                    )),
                );
            }
            egui::CollapsingHeader::new("Family Governance Deep View")
                .default_open(false)
                .show(ui, |ui| {
                    self.render_family_governance_panel(ui);
                });
            egui::CollapsingHeader::new("Template Governance Deep View")
                .default_open(false)
                .show(ui, |ui| {
                    self.render_template_governance_panel(ui);
                });
            ui.separator();
            egui::CollapsingHeader::new("Extension Registry Deep View")
                .default_open(false)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Filter");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.extension_registry_query)
                                .hint_text("family, tool, patch, status..."),
                        );
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Sort");
                        ui.selectable_value(
                            &mut self.extension_registry_sort,
                            ExtensionRegistrySort::RecentFirst,
                            "Recent",
                        );
                        ui.selectable_value(
                            &mut self.extension_registry_sort,
                            ExtensionRegistrySort::StatusFirst,
                            "Status",
                        );
                        ui.selectable_value(
                            &mut self.extension_registry_sort,
                            ExtensionRegistrySort::FamilyToolPatch,
                            "Family/Tool",
                        );
                        ui.selectable_value(
                            &mut self.extension_registry_sort,
                            ExtensionRegistrySort::ProofRiskFirst,
                            "Proof risk",
                        );
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.selectable_value(
                            &mut self.extension_registry_scope,
                            ExtensionRegistryScope::All,
                            "All",
                        );
                        ui.selectable_value(
                            &mut self.extension_registry_scope,
                            ExtensionRegistryScope::Shipped,
                            "Shipped",
                        );
                        ui.selectable_value(
                            &mut self.extension_registry_scope,
                            ExtensionRegistryScope::Active,
                            "Active",
                        );
                        ui.selectable_value(
                            &mut self.extension_registry_scope,
                            ExtensionRegistryScope::Archived,
                            "Archived",
                        );
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Proof quality");
                        ui.selectable_value(
                            &mut self.extension_proof_quality_filter,
                            ProofQualityFilter::All,
                            "All",
                        );
                        ui.selectable_value(
                            &mut self.extension_proof_quality_filter,
                            ProofQualityFilter::Passing,
                            "Passing",
                        );
                        ui.selectable_value(
                            &mut self.extension_proof_quality_filter,
                            ProofQualityFilter::RunnableDiverged,
                            "Diverged",
                        );
                        ui.selectable_value(
                            &mut self.extension_proof_quality_filter,
                            ProofQualityFilter::CatalogResolved,
                            "Catalog",
                        );
                        ui.selectable_value(
                            &mut self.extension_proof_quality_filter,
                            ProofQualityFilter::NeedsContractFix,
                            "Needs fix",
                        );
                        ui.selectable_value(
                            &mut self.extension_proof_quality_filter,
                            ProofQualityFilter::Unknown,
                            "Unknown",
                        );
                    });
                });
            ui.horizontal_wrapped(|ui| {
                ui.label("Proof baseline");
                ui.selectable_value(
                    &mut self.extension_proof_baseline_filter,
                    ProofBaselineFilter::All,
                    "All",
                );
                ui.selectable_value(
                    &mut self.extension_proof_baseline_filter,
                    ProofBaselineFilter::Stable,
                    "Stable",
                );
                ui.selectable_value(
                    &mut self.extension_proof_baseline_filter,
                    ProofBaselineFilter::Changed,
                    "Changed",
                );
                ui.selectable_value(
                    &mut self.extension_proof_baseline_filter,
                    ProofBaselineFilter::Regressed,
                    "Regressed",
                );
                ui.selectable_value(
                    &mut self.extension_proof_baseline_filter,
                    ProofBaselineFilter::NoBaseline,
                    "No baseline",
                );
                ui.selectable_value(
                    &mut self.extension_proof_baseline_filter,
                    ProofBaselineFilter::Unknown,
                    "Unknown",
                );
            });
            ui.horizontal_wrapped(|ui| {
                ui.label("Composition baseline");
                ui.selectable_value(
                    &mut self.extension_composition_baseline_filter,
                    CompositionBaselineFilter::All,
                    "All",
                );
                ui.selectable_value(
                    &mut self.extension_composition_baseline_filter,
                    CompositionBaselineFilter::Stable,
                    "Stable",
                );
                ui.selectable_value(
                    &mut self.extension_composition_baseline_filter,
                    CompositionBaselineFilter::Changed,
                    "Changed",
                );
                ui.selectable_value(
                    &mut self.extension_composition_baseline_filter,
                    CompositionBaselineFilter::Regressed,
                    "Regressed",
                );
                ui.selectable_value(
                    &mut self.extension_composition_baseline_filter,
                    CompositionBaselineFilter::NoBaseline,
                    "No baseline",
                );
                ui.selectable_value(
                    &mut self.extension_composition_baseline_filter,
                    CompositionBaselineFilter::Unknown,
                    "Unknown",
                );
            });
            ui.horizontal_wrapped(|ui| {
                ui.label("Patch baseline");
                ui.selectable_value(
                    &mut self.extension_patch_baseline_filter,
                    PatchBaselineFilter::All,
                    "All",
                );
                ui.selectable_value(
                    &mut self.extension_patch_baseline_filter,
                    PatchBaselineFilter::Stable,
                    "Stable",
                );
                ui.selectable_value(
                    &mut self.extension_patch_baseline_filter,
                    PatchBaselineFilter::Changed,
                    "Changed",
                );
                ui.selectable_value(
                    &mut self.extension_patch_baseline_filter,
                    PatchBaselineFilter::Regressed,
                    "Regressed",
                );
                ui.selectable_value(
                    &mut self.extension_patch_baseline_filter,
                    PatchBaselineFilter::NoBaseline,
                    "No baseline",
                );
                ui.selectable_value(
                    &mut self.extension_patch_baseline_filter,
                    PatchBaselineFilter::Unknown,
                    "Unknown",
                );
            });
            ui.horizontal_wrapped(|ui| {
                ui.label("Helper baseline");
                ui.selectable_value(
                    &mut self.extension_helper_baseline_filter,
                    HelperBaselineFilter::All,
                    "All",
                );
                ui.selectable_value(
                    &mut self.extension_helper_baseline_filter,
                    HelperBaselineFilter::Stable,
                    "Stable",
                );
                ui.selectable_value(
                    &mut self.extension_helper_baseline_filter,
                    HelperBaselineFilter::Changed,
                    "Changed",
                );
                ui.selectable_value(
                    &mut self.extension_helper_baseline_filter,
                    HelperBaselineFilter::Regressed,
                    "Regressed",
                );
                ui.selectable_value(
                    &mut self.extension_helper_baseline_filter,
                    HelperBaselineFilter::NoBaseline,
                    "No baseline",
                );
                ui.selectable_value(
                    &mut self.extension_helper_baseline_filter,
                    HelperBaselineFilter::Unknown,
                    "Unknown",
                );
            });
            ui.horizontal_wrapped(|ui| {
                ui.label("Bridge baseline");
                ui.selectable_value(
                    &mut self.extension_bridge_baseline_filter,
                    BridgeBaselineFilter::All,
                    "All",
                );
                ui.selectable_value(
                    &mut self.extension_bridge_baseline_filter,
                    BridgeBaselineFilter::Stable,
                    "Stable",
                );
                ui.selectable_value(
                    &mut self.extension_bridge_baseline_filter,
                    BridgeBaselineFilter::Changed,
                    "Changed",
                );
                ui.selectable_value(
                    &mut self.extension_bridge_baseline_filter,
                    BridgeBaselineFilter::Regressed,
                    "Regressed",
                );
                ui.selectable_value(
                    &mut self.extension_bridge_baseline_filter,
                    BridgeBaselineFilter::NoBaseline,
                    "No baseline",
                );
                ui.selectable_value(
                    &mut self.extension_bridge_baseline_filter,
                    BridgeBaselineFilter::Unknown,
                    "Unknown",
                );
            });
                });
            let mut filtered_shipped = registry
                .fully_live_entries
                .iter()
                .filter(|entry| {
                    extension_entry_matches_filters(
                        entry,
                        &self.extension_registry_query,
                        self.extension_proof_quality_filter,
                        self.extension_proof_baseline_filter,
                        self.extension_composition_baseline_filter,
                        self.extension_patch_baseline_filter,
                        self.extension_helper_baseline_filter,
                        self.extension_bridge_baseline_filter,
                    )
                })
                .collect::<Vec<_>>();
            sort_extension_entries(&mut filtered_shipped, self.extension_registry_sort);
            let mut filtered_active = registry
                .active_entries
                .iter()
                .filter(|entry| {
                    extension_entry_matches_filters(
                        entry,
                        &self.extension_registry_query,
                        self.extension_proof_quality_filter,
                        self.extension_proof_baseline_filter,
                        self.extension_composition_baseline_filter,
                        self.extension_patch_baseline_filter,
                        self.extension_helper_baseline_filter,
                        self.extension_bridge_baseline_filter,
                    )
                })
                .collect::<Vec<_>>();
            sort_extension_entries(&mut filtered_active, self.extension_registry_sort);
            let mut filtered_archived = registry
                .archived_entries
                .iter()
                .filter(|entry| {
                    extension_entry_matches_filters(
                        entry,
                        &self.extension_registry_query,
                        self.extension_proof_quality_filter,
                        self.extension_proof_baseline_filter,
                        self.extension_composition_baseline_filter,
                        self.extension_patch_baseline_filter,
                        self.extension_helper_baseline_filter,
                        self.extension_bridge_baseline_filter,
                    )
                })
                .collect::<Vec<_>>();
            sort_extension_entries(&mut filtered_archived, self.extension_registry_sort);
            let showing_any = !filtered_shipped.is_empty()
                || !filtered_active.is_empty()
                || !filtered_archived.is_empty();
            let mut filtered_favorites = registry
                .active_entries
                .iter()
                .chain(registry.fully_live_entries.iter())
                .chain(registry.archived_entries.iter())
                .filter(|entry| {
                    self.favorite_extension_entry_ids.contains(&entry.entry_id)
                        && extension_entry_matches_filters(
                            entry,
                            &self.extension_registry_query,
                            self.extension_proof_quality_filter,
                            self.extension_proof_baseline_filter,
                            self.extension_composition_baseline_filter,
                            self.extension_patch_baseline_filter,
                            self.extension_helper_baseline_filter,
                            self.extension_bridge_baseline_filter,
                        )
                })
                .collect::<Vec<_>>();
            sort_extension_entries(&mut filtered_favorites, self.extension_registry_sort);
            let recent_lookup = self
                .recent_extension_entry_ids
                .iter()
                .filter_map(|id| {
                    registry
                        .active_entries
                        .iter()
                        .chain(registry.fully_live_entries.iter())
                        .chain(registry.archived_entries.iter())
                        .find(|entry| entry.entry_id == *id)
                })
                .filter(|entry| {
                    extension_entry_matches_filters(
                        entry,
                        &self.extension_registry_query,
                        self.extension_proof_quality_filter,
                        self.extension_proof_baseline_filter,
                        self.extension_composition_baseline_filter,
                        self.extension_patch_baseline_filter,
                        self.extension_helper_baseline_filter,
                        self.extension_bridge_baseline_filter,
                    )
                })
                .collect::<Vec<_>>();
            if !showing_any {
                ui.label("No lanes match the current filter.");
            }
            if !recent_lookup.is_empty() {
                ui.label("Recent");
                for entry in recent_lookup.into_iter().take(6) {
                    ui.horizontal(|ui| {
                        let mut label = format!(
                            "{} | {} | {} | {}",
                            entry.family_id.as_deref().unwrap_or("unknown_family"),
                            entry.tool_kind.as_deref().unwrap_or("none"),
                            entry.patch_kind.as_deref().unwrap_or("none"),
                            extension_layers_summary(entry)
                        );
                        if let Some(badge) = proof_quality_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = proof_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = composition_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = composition_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = patch_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = patch_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = helper_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = helper_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = bridge_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = bridge_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        let is_selected = self
                            .selected_extension_entry_id
                            .as_deref()
                            .map(|id| id == entry.entry_id)
                            .unwrap_or(false);
                        if ui.selectable_label(is_selected, label).clicked() {
                            self.selected_extension_entry_id = Some(entry.entry_id.clone());
                            self.mark_extension_recent(&entry.entry_id);
                        }
                    });
                }
                ui.separator();
            }
            if !filtered_favorites.is_empty() {
                ui.label("Favorites");
                for entry in filtered_favorites.into_iter().take(8) {
                    ui.horizontal(|ui| {
                        let mut label = format!(
                            "{} | {} | {} | {}",
                            entry.family_id.as_deref().unwrap_or("unknown_family"),
                            entry.tool_kind.as_deref().unwrap_or("none"),
                            entry.patch_kind.as_deref().unwrap_or("none"),
                            extension_layers_summary(entry)
                        );
                        if let Some(badge) = proof_quality_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = proof_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = composition_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = composition_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = patch_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = patch_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = helper_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = helper_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = bridge_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = bridge_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        let is_selected = self
                            .selected_extension_entry_id
                            .as_deref()
                            .map(|id| id == entry.entry_id)
                            .unwrap_or(false);
                        if ui.selectable_label(is_selected, label).clicked() {
                            self.selected_extension_entry_id = Some(entry.entry_id.clone());
                            self.mark_extension_recent(&entry.entry_id);
                        }
                        let pin_label = if self.favorite_extension_entry_ids.contains(&entry.entry_id)
                        {
                            "Unpin"
                        } else {
                            "Pin"
                        };
                        if ui.small_button(pin_label).clicked() {
                            self.toggle_extension_favorite(&entry.entry_id);
                        }
                    });
                }
                ui.separator();
            }
            if (self.extension_registry_scope == ExtensionRegistryScope::All
                || self.extension_registry_scope == ExtensionRegistryScope::Shipped)
                && !filtered_shipped.is_empty()
            {
                ui.label("Shipped lanes");
                for entry in filtered_shipped.into_iter().take(8) {
                    let mut label = format!(
                        "{} | {} | {}",
                        entry.family_id.as_deref().unwrap_or("unknown_family"),
                        entry.tool_kind.as_deref().unwrap_or("none"),
                        entry.patch_kind.as_deref().unwrap_or("none")
                    );
                    if let Some(badge) = proof_quality_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = proof_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = composition_drift_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = composition_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = patch_drift_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = patch_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = helper_drift_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = helper_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = bridge_drift_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = bridge_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    let is_selected = self
                        .selected_extension_entry_id
                        .as_deref()
                        .map(|id| id == entry.entry_id)
                        .unwrap_or(false);
                    if ui.selectable_label(is_selected, label).clicked() {
                        self.selected_extension_entry_id = Some(entry.entry_id.clone());
                        self.mark_extension_recent(&entry.entry_id);
                    }
                }
            }
            if (self.extension_registry_scope == ExtensionRegistryScope::All
                || self.extension_registry_scope == ExtensionRegistryScope::Active)
                && !filtered_active.is_empty()
            {
                ui.separator();
                ui.label("Active queue");
                for entry in filtered_active.into_iter().take(8) {
                    ui.horizontal(|ui| {
                        let mut label = format!(
                            "{} [{}] {}",
                            entry.patch_kind.as_deref().unwrap_or("none"),
                            entry.status,
                            entry.family_id.as_deref().unwrap_or("unknown_family")
                        );
                        if let Some(badge) = proof_quality_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = proof_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = composition_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = composition_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = patch_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = patch_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = helper_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = helper_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = bridge_drift_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        if let Some(badge) = bridge_baseline_badge(entry) {
                            label.push_str(&format!(" | {badge}"));
                        }
                        let is_selected = self
                            .selected_extension_entry_id
                            .as_deref()
                            .map(|id| id == entry.entry_id)
                            .unwrap_or(false);
                        if ui.selectable_label(is_selected, label).clicked() {
                            self.selected_extension_entry_id = Some(entry.entry_id.clone());
                            self.mark_extension_recent(&entry.entry_id);
                        }
                        if ui.small_button("Archive").clicked() {
                            self.spawn_task(UiTask::ArchiveExtension {
                                entry_id: entry.entry_id.clone(),
                                reason: "archived from UI".to_string(),
                            });
                        }
                    });
                }
            }
            if (self.extension_registry_scope == ExtensionRegistryScope::All
                || self.extension_registry_scope == ExtensionRegistryScope::Archived)
                && !filtered_archived.is_empty()
            {
                ui.separator();
                ui.label("Archived lanes");
                for entry in filtered_archived.into_iter().take(8) {
                    let mut label = format!(
                        "{} | {}",
                        entry.patch_kind.as_deref().unwrap_or("none"),
                        entry.archived_reason.as_deref().unwrap_or("retired")
                    );
                    if let Some(badge) = proof_quality_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = proof_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = composition_drift_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = composition_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = patch_drift_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = patch_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = helper_drift_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = helper_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = bridge_drift_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    if let Some(badge) = bridge_baseline_badge(entry) {
                        label.push_str(&format!(" | {badge}"));
                    }
                    let is_selected = self
                        .selected_extension_entry_id
                        .as_deref()
                        .map(|id| id == entry.entry_id)
                        .unwrap_or(false);
                    if ui.selectable_label(is_selected, label).clicked() {
                        self.selected_extension_entry_id = Some(entry.entry_id.clone());
                        self.mark_extension_recent(&entry.entry_id);
                    }
                }
            }
            if let Some(entry) = self.selected_extension_entry() {
                let export_history_paths = self.extension_export_history_paths(&entry.entry_id);
                let mut note_value = self
                    .extension_notes
                    .get(&entry.entry_id)
                    .cloned()
                    .unwrap_or_default();
                let scaffold_root = PathBuf::from(&entry.scaffold_root);
                let implementation_notes_path = scaffold_root.join("IMPLEMENTATION_NOTES.md");
                let acceptance_targets_path = scaffold_root.join("acceptance_targets.json");
                let source_stub_path = PathBuf::from(&entry.source_stub_path);
                let latest_proof_receipt = entry.patch_kind.as_deref().and_then(|template_id| {
                    latest_proof_receipt_for_template(&self.workspace_root, template_id)
                });
                let blockers = extension_status_blockers(&entry);
                let promotion_hint_count = if let (Some(promotion_path), Some(first_integrated_path)) =
                    (entry.promotion_artifacts.first(), entry.integrated_paths.first())
                {
                    let left = load_text_preview(Path::new(promotion_path), 1400);
                    let right = load_text_preview(Path::new(first_integrated_path), 1400);
                    match (left.as_deref(), right.as_deref()) {
                        (Some(left), Some(right)) => compare_mismatch_hints(left, right).len(),
                        _ => 0,
                    }
                } else {
                    0
                };
                let apply_patch_hint_count = if let (Some(apply_patch_path), Some(first_integrated_path)) =
                    (entry.apply_patch_artifacts.first(), entry.integrated_paths.first())
                {
                    let left = load_text_preview(Path::new(apply_patch_path), 1400);
                    let right = load_text_preview(Path::new(first_integrated_path), 1400);
                    match (left.as_deref(), right.as_deref()) {
                        (Some(left), Some(right)) => compare_mismatch_hints(left, right).len(),
                        _ => 0,
                    }
                } else {
                    0
                };
                let mismatch_hint_count = promotion_hint_count + apply_patch_hint_count;
                let (readiness_label, readiness_color) =
                    lane_readiness_tone(&entry.status, blockers.len(), mismatch_hint_count);
                ui.separator();
                ui.label("Lane Details");
                ui.label(format!("Entry: {}", entry.entry_id));
                ui.label(format!("Status: {}", entry.status));
                ui.label(format!(
                    "Family: {}",
                    entry.family_id.as_deref().unwrap_or("unknown_family")
                ));
                ui.label(format!(
                    "Tool: {}",
                    entry.tool_kind.as_deref().unwrap_or("none")
                ));
                ui.label(format!(
                    "Patch: {}",
                    entry.patch_kind.as_deref().unwrap_or("none")
                ));
                ui.label(format!("Kind: {}", entry.extension_kind));
                self.render_extension_governance_section(
                    ui,
                    &entry,
                    latest_proof_receipt.as_ref(),
                );
                let is_favorite = self.favorite_extension_entry_ids.contains(&entry.entry_id);
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        egui::RichText::new(format!("{} ", readiness_label))
                            .color(readiness_color)
                            .strong(),
                    );
                    ui.label(format!("status={}", entry.status));
                    ui.separator();
                    ui.label(format!("blockers={}", blockers.len()));
                    ui.separator();
                    ui.label(format!("compare_hints={mismatch_hint_count}"));
                });
                ui.horizontal(|ui| {
                    if ui
                        .small_button(if is_favorite {
                            "Unpin Favorite"
                        } else {
                            "Pin Favorite"
                        })
                        .clicked()
                    {
                        self.toggle_extension_favorite(&entry.entry_id);
                    }
                    if ui.small_button("Export Summary").clicked() {
                        self.export_extension_summary(&entry, &blockers, mismatch_hint_count);
                    }
                    if ui.small_button("Copy Summary").clicked() {
                        let note = self
                            .extension_notes
                            .get(&entry.entry_id)
                            .map(|value| value.trim().to_string())
                            .filter(|value| !value.is_empty());
                        let summary = build_extension_summary_markdown(
                            &entry,
                            &blockers,
                            mismatch_hint_count,
                            note,
                        );
                        ui.ctx().copy_text(summary);
                        self.status_line = "Copied lane summary to clipboard".to_string();
                        self.push_extension_activity(
                            Some(entry.entry_id.clone()),
                            "Copy Lane Summary",
                            entry.entry_id.clone(),
                            true,
                        );
                        self.push_toast("Copied lane summary to clipboard", ToastKind::Success);
                    }
                    if ui.small_button("Open Latest Export").clicked() {
                        let export_path = self.latest_extension_export_path(&entry.entry_id);
                        match open_path_in_explorer(export_path.to_string_lossy().as_ref(), true) {
                            Ok(()) => {
                                self.status_line = "Revealed latest lane export".to_string();
                                self.push_extension_activity(
                                    Some(entry.entry_id.clone()),
                                    "Open Latest Export",
                                    short_path(export_path.to_string_lossy().as_ref()),
                                    true,
                                );
                                self.push_toast(
                                    "Revealed latest lane export",
                                    ToastKind::Success,
                                );
                            }
                            Err(error) => {
                                self.status_line = format!("Open failed: {error}");
                                self.push_extension_activity(
                                    Some(entry.entry_id.clone()),
                                    "Open Latest Export",
                                    error.to_string(),
                                    false,
                                );
                                self.push_toast(
                                    format!("Open failed: {error}"),
                                    ToastKind::Error,
                                );
                            }
                        }
                    }
                });
                if !export_history_paths.is_empty() {
                    ui.label("Export History");
                    for export_path in export_history_paths.iter().take(5) {
                        ui.horizontal(|ui| {
                            ui.label(short_path(export_path.to_string_lossy().as_ref()));
                            if ui.small_button("Open").clicked() {
                                match open_path_in_explorer(
                                    export_path.to_string_lossy().as_ref(),
                                    true,
                                ) {
                                    Ok(()) => {
                                        self.status_line = "Revealed lane export".to_string();
                                        self.push_extension_activity(
                                            Some(entry.entry_id.clone()),
                                            "Open Export History",
                                            short_path(export_path.to_string_lossy().as_ref()),
                                            true,
                                        );
                                        self.push_toast(
                                            "Revealed lane export",
                                            ToastKind::Success,
                                        );
                                    }
                                    Err(error) => {
                                        self.status_line = format!("Open failed: {error}");
                                        self.push_extension_activity(
                                            Some(entry.entry_id.clone()),
                                            "Open Export History",
                                            error.to_string(),
                                            false,
                                        );
                                        self.push_toast(
                                            format!("Open failed: {error}"),
                                            ToastKind::Error,
                                        );
                                    }
                                }
                            }
                        });
                    }
                    if export_history_paths.len() >= 2 {
                        let latest_path = &export_history_paths[0];
                        let previous_path = &export_history_paths[1];
                        let export_diff = build_export_diff(latest_path, previous_path);
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label("Latest vs Previous");
                            if ui.small_button("Copy Diff").clicked() {
                                ui.ctx().copy_text(export_diff.clone());
                                self.status_line = "Copied export diff to clipboard".to_string();
                                self.push_extension_activity(
                                    Some(entry.entry_id.clone()),
                                    "Copy Export Diff",
                                    entry.entry_id.clone(),
                                    true,
                                );
                                self.push_toast(
                                    "Copied export diff to clipboard",
                                    ToastKind::Success,
                                );
                            }
                        });
                        egui::CollapsingHeader::new("Export Diff")
                            .default_open(false)
                            .show(ui, |ui| {
                                let mut preview = export_diff.clone();
                                ui.add(
                                    egui::TextEdit::multiline(&mut preview)
                                        .font(egui::TextStyle::Monospace)
                                        .desired_rows(10)
                                        .interactive(false),
                                );
                            });
                    }
                }
                if let Some(reason) = entry.archived_reason.as_deref() {
                    ui.label(format!("Archive reason: {reason}"));
                }
                let lane_timeline = self
                    .extension_activity
                    .iter()
                    .rev()
                    .filter(|item| item.entry_id.as_deref() == Some(&entry.entry_id))
                    .take(8)
                    .cloned()
                    .collect::<Vec<_>>();
                if !lane_timeline.is_empty() {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Lane Timeline");
                        if ui.small_button("Clear Lane").clicked() {
                            self.clear_lane_activity(&entry.entry_id);
                        }
                    });
                    for item in lane_timeline {
                        let color = if item.success {
                            egui::Color32::from_rgb(84, 168, 108)
                        } else {
                            egui::Color32::from_rgb(204, 96, 96)
                        };
                        ui.label(egui::RichText::new(&item.title).color(color).strong());
                        ui.label(format!("{} | {}", item.detail, item.timestamp_label));
                        ui.add_space(4.0);
                    }
                }
                ui.separator();
                ui.label("Lane Note");
                ui.add(
                    egui::TextEdit::multiline(&mut note_value)
                        .desired_rows(3)
                        .hint_text("Short reminder, blocker, or review note..."),
                );
                self.extension_notes
                    .insert(entry.entry_id.clone(), note_value.clone());
                ui.horizontal(|ui| {
                    if ui.small_button("Save Note").clicked() {
                        self.save_extension_notes();
                        self.status_line = "Saved lane note".to_string();
                        self.push_extension_activity(
                            Some(entry.entry_id.clone()),
                            "Save Note",
                            entry.entry_id.clone(),
                            true,
                        );
                        self.push_toast("Saved lane note", ToastKind::Success);
                    }
                    if ui.small_button("Clear Note").clicked() {
                        self.extension_notes
                            .insert(entry.entry_id.clone(), String::new());
                        self.save_extension_notes();
                        self.status_line = "Cleared lane note".to_string();
                        self.push_extension_activity(
                            Some(entry.entry_id.clone()),
                            "Clear Note",
                            entry.entry_id.clone(),
                            true,
                        );
                        self.push_toast("Cleared lane note", ToastKind::Success);
                    }
                });
                ui.horizontal(|ui| match entry.status.as_str() {
                    "pending_implementation" => {
                        if ui.small_button("Implement").clicked() {
                            self.spawn_task(UiTask::ImplementExtension {
                                entry_id: entry.entry_id.clone(),
                            });
                        }
                    }
                    "implemented" => {
                        if ui.small_button("Validate").clicked() {
                            self.spawn_task(UiTask::ValidateExtension {
                                entry_id: entry.entry_id.clone(),
                            });
                        }
                    }
                    "validated_ready" => {
                        if ui.small_button("Prepare Promotion").clicked() {
                            self.spawn_task(UiTask::PrepareExtensionPromotion {
                                entry_id: entry.entry_id.clone(),
                            });
                        }
                    }
                    "promotion_prepared" => {
                        if ui.small_button("Prepare Patch").clicked() {
                            self.spawn_task(UiTask::PrepareExtensionApplyPatch {
                                entry_id: entry.entry_id.clone(),
                            });
                        }
                    }
                    "apply_patch_ready" => {
                        if ui.small_button("Host Wire").clicked() {
                            self.spawn_task(UiTask::ConsumeExtensionApplyPatch {
                                entry_id: entry.entry_id.clone(),
                            });
                        }
                    }
                    "host_wired" => {
                        if ui.small_button("Mark Fully Live").clicked() {
                            self.spawn_task(UiTask::ValidateLiveExtension {
                                entry_id: entry.entry_id.clone(),
                            });
                        }
                    }
                    _ => {}
                });
                self.render_extension_workbench_section(
                    ui,
                    &entry,
                    &blockers,
                    &source_stub_path,
                    &implementation_notes_path,
                    &acceptance_targets_path,
                );
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Extension Activity");
                if !self.extension_activity.is_empty() && ui.small_button("Clear All").clicked() {
                    self.clear_all_activity();
                }
            });
            if self.extension_activity.is_empty() {
                ui.label("No extension actions yet.");
            } else {
                for item in self.extension_activity.iter().rev().take(6) {
                    let color = if item.success {
                        egui::Color32::from_rgb(84, 168, 108)
                    } else {
                        egui::Color32::from_rgb(204, 96, 96)
                    };
                    ui.label(egui::RichText::new(&item.title).color(color).strong());
                    ui.label(format!("{} | {}", item.detail, item.timestamp_label));
                    ui.add_space(4.0);
                }
            }
        } else {
            ui.label("No extension registry loaded yet.");
        }
    }
}
