use super::*;

impl ChattyFactoryUiApp {
    pub(crate) fn render_extension_workbench_section(
        &mut self,
        ui: &mut egui::Ui,
        entry: &PendingExtensionEntry,
        blockers: &[String],
        source_stub_path: &Path,
        implementation_notes_path: &Path,
        acceptance_targets_path: &Path,
    ) {
        ui.label(format!(
            "Attempt bundle root: {}",
            short_path(&entry.attempt_bundle_root)
        ));
        ui.label(format!(
            "Source spec: {}",
            short_path(&entry.source_stub_path)
        ));
        ui.horizontal(|ui| {
            if ui.small_button("Open Bundle").clicked() {
                self.reveal_governed_artifact(
                    &entry.attempt_bundle_root,
                    "Opened attempt bundle folder",
                    "Open failed",
                    "Opened attempt bundle folder",
                    "Open failed",
                    Some((Some(entry.entry_id.clone()), "Open Bundle")),
                );
            }
            if ui.small_button("Reveal Spec").clicked() {
                self.reveal_governed_artifact(
                    &entry.source_stub_path,
                    "Revealed source spec",
                    "Open failed",
                    "Revealed source spec",
                    "Open failed",
                    Some((Some(entry.entry_id.clone()), "Reveal Spec")),
                );
            }
            if let Some(first_path) = entry.integrated_paths.first() {
                if ui.small_button("Reveal First File").clicked() {
                    self.reveal_governed_artifact(
                        first_path,
                        "Revealed first integrated file",
                        "Open failed",
                        "Revealed first integrated file",
                        "Open failed",
                        Some((Some(entry.entry_id.clone()), "Reveal First File")),
                    );
                }
            }
            if let Some(first_path) = entry.promotion_artifacts.first() {
                if ui.small_button("Reveal Promotion").clicked() {
                    self.reveal_governed_artifact(
                        first_path,
                        "Revealed promotion artifact",
                        "Open failed",
                        "Revealed promotion artifact",
                        "Open failed",
                        Some((Some(entry.entry_id.clone()), "Reveal Promotion")),
                    );
                }
            }
            if let Some(first_path) = entry.apply_patch_artifacts.first() {
                if ui.small_button("Reveal Patch Artifact").clicked() {
                    self.reveal_governed_artifact(
                        first_path,
                        "Revealed apply-patch artifact",
                        "Open failed",
                        "Revealed apply-patch artifact",
                        "Open failed",
                        Some((Some(entry.entry_id.clone()), "Reveal Patch Artifact")),
                    );
                }
            }
        });
        if !entry.integrated_paths.is_empty() {
            ui.label("Integrated files");
            for path in entry.integrated_paths.iter().take(5) {
                ui.label(format!("- {}", short_path(path)));
            }
        }
        if !entry.promotion_artifacts.is_empty() {
            ui.label("Promotion artifacts");
            for path in entry.promotion_artifacts.iter().take(5) {
                ui.label(format!("- {}", short_path(path)));
            }
        }
        if !entry.apply_patch_artifacts.is_empty() {
            ui.label("Apply-patch artifacts");
            for path in entry.apply_patch_artifacts.iter().take(4) {
                ui.label(format!("- {}", short_path(path)));
            }
        }
        ui.separator();
        ui.label("Attempt Workbench");
        if !blockers.is_empty() {
            ui.label("Current blockers");
            for blocker in blockers {
                ui.label(format!("- {blocker}"));
            }
        }
        ui.horizontal(|ui| {
            if implementation_notes_path.exists() && ui.small_button("Reveal Notes").clicked() {
                self.reveal_governed_artifact(
                    implementation_notes_path.to_string_lossy().as_ref(),
                    "Revealed implementation notes",
                    "Open failed",
                    "Revealed implementation notes",
                    "Open failed",
                    Some((Some(entry.entry_id.clone()), "Reveal Notes")),
                );
            }
            if acceptance_targets_path.exists() && ui.small_button("Reveal Targets").clicked() {
                self.reveal_governed_artifact(
                    acceptance_targets_path.to_string_lossy().as_ref(),
                    "Revealed acceptance targets",
                    "Open failed",
                    "Revealed acceptance targets",
                    "Open failed",
                    Some((Some(entry.entry_id.clone()), "Reveal Targets")),
                );
            }
        });
        egui::CollapsingHeader::new("Source Spec Preview")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(mut preview) = load_text_preview(source_stub_path, 1600) {
                    ui.add(
                        egui::TextEdit::multiline(&mut preview)
                            .font(egui::TextStyle::Monospace)
                            .desired_rows(10)
                            .interactive(false),
                    );
                } else {
                    ui.label("No source spec preview available.");
                }
            });
        egui::CollapsingHeader::new("Implementation Notes")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(mut preview) = load_text_preview(implementation_notes_path, 1600) {
                    ui.add(
                        egui::TextEdit::multiline(&mut preview)
                            .font(egui::TextStyle::Monospace)
                            .desired_rows(10)
                            .interactive(false),
                    );
                } else {
                    ui.label("No implementation notes found.");
                }
            });
        egui::CollapsingHeader::new("Acceptance Targets")
            .default_open(false)
            .show(ui, |ui| {
                if let Some(mut preview) = load_text_preview(acceptance_targets_path, 1600) {
                    ui.add(
                        egui::TextEdit::multiline(&mut preview)
                            .font(egui::TextStyle::Monospace)
                            .desired_rows(8)
                            .interactive(false),
                    );
                } else {
                    ui.label("No acceptance target file found.");
                }
            });
        if let (Some(promotion_path), Some(first_integrated_path)) = (
            entry.promotion_artifacts.first(),
            entry.integrated_paths.first(),
        ) {
            egui::CollapsingHeader::new("Promotion Compare")
                .default_open(false)
                .show(ui, |ui| {
                    let left_preview = load_text_preview(Path::new(promotion_path), 1400);
                    let right_preview = load_text_preview(Path::new(first_integrated_path), 1400);
                    if let (Some(left), Some(right)) =
                        (left_preview.as_deref(), right_preview.as_deref())
                    {
                        let hints = compare_mismatch_hints(left, right);
                        if !hints.is_empty() {
                            ui.label("Mismatch hints");
                            for hint in hints.iter().take(5) {
                                ui.label(format!("- {hint}"));
                            }
                            ui.add_space(6.0);
                        }
                    }
                    ui.columns(2, |columns| {
                        columns[0].label(format!(
                            "Promotion artifact: {}",
                            short_path(promotion_path)
                        ));
                        if let Some(mut preview) = left_preview {
                            columns[0].add(
                                egui::TextEdit::multiline(&mut preview)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_rows(12)
                                    .interactive(false),
                            );
                        } else {
                            columns[0].label("No preview available for the promotion artifact.");
                        }

                        columns[1].label(format!(
                            "Live integrated file: {}",
                            short_path(first_integrated_path)
                        ));
                        if let Some(mut preview) = right_preview {
                            columns[1].add(
                                egui::TextEdit::multiline(&mut preview)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_rows(12)
                                    .interactive(false),
                            );
                        } else {
                            columns[1].label("No preview available for the live integrated file.");
                        }
                    });
                });
        }
        if let (Some(apply_patch_path), Some(first_integrated_path)) = (
            entry.apply_patch_artifacts.first(),
            entry.integrated_paths.first(),
        ) {
            egui::CollapsingHeader::new("Apply-Patch Compare")
                .default_open(false)
                .show(ui, |ui| {
                    let left_preview = load_text_preview(Path::new(apply_patch_path), 1400);
                    let right_preview = load_text_preview(Path::new(first_integrated_path), 1400);
                    if let (Some(left), Some(right)) =
                        (left_preview.as_deref(), right_preview.as_deref())
                    {
                        let hints = compare_mismatch_hints(left, right);
                        if !hints.is_empty() {
                            ui.label("Mismatch hints");
                            for hint in hints.iter().take(5) {
                                ui.label(format!("- {hint}"));
                            }
                            ui.add_space(6.0);
                        }
                    }
                    ui.columns(2, |columns| {
                        columns[0].label(format!(
                            "Apply-patch artifact: {}",
                            short_path(apply_patch_path)
                        ));
                        if let Some(mut preview) = left_preview {
                            columns[0].add(
                                egui::TextEdit::multiline(&mut preview)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_rows(12)
                                    .interactive(false),
                            );
                        } else {
                            columns[0].label("No preview available for the apply-patch artifact.");
                        }

                        columns[1].label(format!(
                            "Live integrated file: {}",
                            short_path(first_integrated_path)
                        ));
                        if let Some(mut preview) = right_preview {
                            columns[1].add(
                                egui::TextEdit::multiline(&mut preview)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_rows(12)
                                    .interactive(false),
                            );
                        } else {
                            columns[1].label("No preview available for the live integrated file.");
                        }
                    });
                });
        }
        if let Some(first_path) = entry.integrated_paths.first() {
            egui::CollapsingHeader::new("First Integrated File Preview")
                .default_open(false)
                .show(ui, |ui| {
                    if let Some(mut preview) = load_text_preview(Path::new(first_path), 1600) {
                        ui.add(
                            egui::TextEdit::multiline(&mut preview)
                                .font(egui::TextStyle::Monospace)
                                .desired_rows(8)
                                .interactive(false),
                        );
                    } else {
                        ui.label("No preview available for the integrated file.");
                    }
                });
        }
    }
}
