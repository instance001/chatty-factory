use super::*;

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
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label(egui::RichText::new("Planner").strong());
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.auto_planner, "Auto planner");
                ui.label("Planner port");
                ui.add(egui::TextEdit::singleline(&mut self.planner_port).desired_width(70.0));
                ui.label("Model");
                ui.add(egui::TextEdit::singleline(&mut self.planner_model).desired_width(120.0));
            });
        });
        if self.auto_planner != previous_auto_planner
            || self.planner_port != previous_planner_port
            || self.planner_model != previous_planner_model
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
}
