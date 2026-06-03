use super::*;
use chatty_factory_core::{
    build_starter_best_for, build_starter_choices, build_starter_label,
    build_starter_lifecycle, build_starter_picker_label, derive_request_plan,
    normalize_request,
};

impl ChattyFactoryUiApp {
    pub(crate) fn render_request_action_panel(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("Requests");
        ui.label(egui::RichText::new("Build a new project, or patch the selected one.").weak());
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label(egui::RichText::new("Request").strong());
            ui.add(
                egui::TextEdit::multiline(&mut self.request_input)
                    .hint_text("Describe the build or follow-up change you want")
                    .desired_rows(5),
            );
        });

        self.render_request_planner_controls(ui);
        self.render_request_action_buttons(ui);

        ui.label(
            egui::RichText::new(format!(
                "Current target project: {}",
                self.selected_project().unwrap_or("none")
            ))
            .small()
            .weak(),
        );
    }

    fn render_request_planner_controls(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        let previous_auto_planner = self.auto_planner;
        let previous_planner_port = self.planner_port.clone();
        let previous_planner_model = self.planner_model.clone();
        let previous_build_starter_override = self.build_starter_override_id.clone();
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label(egui::RichText::new("Planner and Starter").strong());
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.auto_planner, "Auto planner");
                ui.label("Planner port");
                ui.add(egui::TextEdit::singleline(&mut self.planner_port).desired_width(70.0));
                ui.label("Model");
                ui.add(egui::TextEdit::singleline(&mut self.planner_model).desired_width(120.0));
            });
            ui.horizontal(|ui| {
                ui.label("Build starter");
                egui::ComboBox::from_id_source("build-starter-override-picker")
                    .selected_text(build_starter_picker_label(&self.build_starter_override_id))
                    .show_ui(ui, |ui| {
                        for choice in build_starter_choices() {
                            ui.selectable_value(
                                &mut self.build_starter_override_id,
                                choice.id.to_string(),
                                format!("{} [{}]", choice.label, choice.lifecycle),
                            );
                        }
                    });
            });
            ui.label(
                egui::RichText::new(
                    "Build starter selection is mechanical. `Auto` leaves family choice to normal routing.",
                )
                .small()
                .weak(),
            );
            ui.label(format!(
                "Selected starter lifecycle: {}",
                build_starter_lifecycle(&self.build_starter_override_id)
            ));
            ui.label(format!(
                "Best for: {}",
                build_starter_best_for(&self.build_starter_override_id)
            ));
            self.render_build_starter_recommendation_hint(ui);
        });
        if self.auto_planner != previous_auto_planner
            || self.planner_port != previous_planner_port
            || self.planner_model != previous_planner_model
            || self.build_starter_override_id != previous_build_starter_override
        {
            self.save_paired_proof_ui_preferences();
        }
    }

    fn render_request_action_buttons(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label(egui::RichText::new("Actions").strong());
            ui.horizontal(|ui| {
                let build_enabled = !self.request_input.trim().is_empty() && !self.task_running;
                if ui
                    .add_enabled(build_enabled, egui::Button::new("Build Request"))
                    .clicked()
                {
                    self.spawn_task(UiTask::BuildRequest {
                        request: self.request_input.trim().to_string(),
                        starter_override_id: build_starter_override_id_for_task(
                            &self.build_starter_override_id,
                        ),
                        auto_planner: self.auto_planner,
                        port: self.planner_port.trim().to_string(),
                        model: self.planner_model.trim().to_string(),
                    });
                }

                let patch_enabled = build_enabled && self.selected_project().is_some();
                if ui
                    .add_enabled(patch_enabled, egui::Button::new("Patch Selected Project"))
                    .clicked()
                {
                    if let Some(project_name) = self.selected_project() {
                        self.spawn_task(UiTask::PatchRequest {
                            project_name: project_name.to_string(),
                            request: self.request_input.trim().to_string(),
                            auto_planner: self.auto_planner,
                            port: self.planner_port.trim().to_string(),
                            model: self.planner_model.trim().to_string(),
                        });
                    }
                }
            });
        });
    }

    fn render_build_starter_recommendation_hint(&self, ui: &mut egui::Ui) {
        let request = self.request_input.trim();
        if request.is_empty() {
            return;
        }

        let plan = derive_request_plan(&normalize_request(request), None);
        let Some(recommended_family) = plan.inferred_family_candidates.first() else {
            return;
        };

        let recommended_id = recommended_family.as_str();
        let recommended_label = build_starter_label(recommended_id);
        let selected_lifecycle = build_starter_lifecycle(&self.build_starter_override_id);
        let recommended_lifecycle = build_starter_lifecycle(recommended_id);
        let comparison = if self.build_starter_override_id == "auto" {
            "Auto will follow this recommendation."
        } else if self.build_starter_override_id == recommended_id {
            "Your mechanical selection matches normal routing."
        } else {
            "Your mechanical selection overrides normal routing."
        };

        let mut hint = format!(
            "Normal routing would likely choose: {} [{}]",
            recommended_label, recommended_id
        );
        if let Some(tool_kind) = &plan.inferred_tool_kind {
            hint.push_str(&format!(" via {}", tool_kind));
        }
        hint.push_str(&format!(
            " | confidence {} ({})",
            plan.confidence_score, plan.confidence_band
        ));
        if plan.needs_llm_review {
            hint.push_str(" | planner review likely");
        }
        ui.label(hint);
        ui.label(comparison);
        if self.build_starter_override_id != "auto"
            && self.build_starter_override_id != recommended_id
            && selected_lifecycle.contains("frozen legacy")
            && recommended_lifecycle.contains("active")
        {
            ui.colored_label(
                egui::Color32::from_rgb(210, 120, 60),
                format!(
                    "Warning: you selected a frozen legacy starter while normal routing recommends the active starter `{}`.",
                    recommended_id
                ),
            );
        }
    }
}

fn build_starter_override_id_for_task(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "auto" {
        None
    } else {
        Some(trimmed.to_string())
    }
}
