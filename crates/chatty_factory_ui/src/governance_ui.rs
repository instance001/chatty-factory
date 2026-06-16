use eframe::egui;

pub fn governance_drift_summary(label: &str, status: &str) -> String {
    format!("{label} drift: {status}")
}

pub fn governance_baseline_summary(status: &str) -> String {
    format!("Change since last live: {status}")
}

pub fn governance_drift_note_summary(label: &str, note: &str) -> String {
    format!("{label} drift note: {note}")
}

pub fn governance_baseline_note_summary(label: &str, note: &str) -> String {
    format!("{label} baseline note: {note}")
}

pub fn governance_refresh_status_summary(
    label: &str,
    updated_at: &str,
    refreshed_at_label: Option<&str>,
    status_id: &str,
    refreshed_entries: usize,
    skipped_entries: usize,
) -> String {
    format!(
        "Last {label} refresh: {updated_at} | file {} | id {status_id} | refreshed {refreshed_entries} | skipped {skipped_entries}",
        refreshed_at_label.unwrap_or("unknown")
    )
}

pub fn governance_never_refreshed_summary(label: &str) -> String {
    format!("Last {label} refresh: none recorded yet")
}

pub fn governance_stale_warning(label: &str, age_minutes: u64) -> String {
    format!("{label} view is stale ({age_minutes} minutes old). Refresh recommended.")
}

pub fn governance_never_refreshed_warning(label: &str) -> String {
    format!("{label} has never been refreshed. Refresh recommended.")
}

pub fn governance_cooldown_warning(label: &str, refresh_action_label: &str) -> String {
    format!(
        "Launch auto-refresh for {label} is in cooldown. Use \"{refresh_action_label}\" to bypass cooldown."
    )
}

pub fn render_governance_metric_strip(
    ui: &mut egui::Ui,
    metrics: &[String],
    refresh_button_label: &str,
) -> bool {
    let mut clicked = false;
    ui.horizontal_wrapped(|ui| {
        for metric in metrics.iter() {
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(224, 228, 232))
                .stroke(egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgb(176, 184, 194),
                ))
                .rounding(6.0)
                .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(metric)
                                .strong()
                                .small()
                                .color(egui::Color32::from_rgb(32, 36, 42)),
                        )
                        .wrap(false),
                    );
                });
        }
        if !metrics.is_empty() {
            ui.separator();
        }
        if ui.small_button(refresh_button_label).clicked() {
            clicked = true;
        }
    });
    clicked
}

pub fn render_governance_refresh_state(
    ui: &mut egui::Ui,
    summary_line: String,
    warning: Option<(egui::Color32, String)>,
) {
    ui.label(egui::RichText::new(summary_line).small().weak());
    if let Some((color, message)) = warning {
        ui.colored_label(color, message);
    }
}

pub fn render_governance_detail_block(
    ui: &mut egui::Ui,
    label: &str,
    artifact_summary: Option<&str>,
    drift_status: Option<&str>,
    baseline_status: Option<&str>,
    drift_notes: &[String],
    baseline_notes: &[String],
) {
    if let Some(summary) = artifact_summary {
        ui.label(summary);
    }
    if let Some(status) = drift_status {
        ui.label(governance_drift_summary(label, status));
    }
    if let Some(status) = baseline_status {
        ui.label(governance_baseline_summary(status));
    }
    for note in drift_notes {
        ui.label(governance_drift_note_summary(label, note));
    }
    for note in baseline_notes {
        ui.label(governance_baseline_note_summary(label, note));
    }
}
