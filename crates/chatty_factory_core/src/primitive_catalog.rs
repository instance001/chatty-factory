pub const HELPER_PRIMITIVE_KINDS: &[&str] = &[
    "inbox_lane",
    "processed_output",
    "summary_emitter",
    "status_reporter",
    "file_filter",
];

pub const PATCH_PRIMITIVE_CLASSES: &[&str] = &[
    "summary_surface",
    "status_chip",
    "info_chip",
    "metadata_row",
    "notice_surface",
    "filter_rule",
    "inbox_lane_extension",
    "selection_control",
    "refresh_behavior",
    "timestamp_row",
    "badge_strip",
    "export_output",
    "patch_extension",
];

pub fn patch_primitive_class(patch_kind: &str) -> &'static str {
    match patch_kind {
        "secondary_inbox_lane" => "inbox_lane_extension",
        "file_type_filter" | "severity_filter" | "column_filter" => "filter_rule",
        "helper_summary_panel"
        | "processed_files_panel"
        | "processed_file_preview_panel"
        | "asset_inbox_panel"
        | "bridge_activity_panel"
        | "bridge_status_panel"
        | "progress_banner"
        | "session_overview"
        | "room_state_fields" => "summary_surface",
        "helper_summary_status_chip" | "helper_status_chip" => "status_chip",
        "helper_summary_lane_count_chip"
        | "helper_summary_types_chip"
        | "helper_summary_updated_at_chip" => "info_chip",
        "helper_summary_count_delta"
        | "helper_summary_metadata_row"
        | "lane_scoped_metadata_row" => "metadata_row",
        "helper_summary_filter_notice"
        | "lane_scoped_filter_notice"
        | "helper_summary_empty_state"
        | "helper_summary_discovered_notice" => "notice_surface",
        "processed_file_selection" | "ready_toggle" => "selection_control",
        "auto_refresh_helper_panels" => "refresh_behavior",
        "helper_last_run_stamp" => "timestamp_row",
        "helper_summary_badges" | "metric_strip" => "badge_strip",
        "json_export" | "json_output" | "file_output" | "email_sender" => "export_output",
        _ => "patch_extension",
    }
}

pub fn patch_primitive_classes_for_kinds(patch_kinds: &[String]) -> Vec<String> {
    let mut classes = Vec::new();
    for patch_kind in patch_kinds {
        let class = patch_primitive_class(patch_kind).to_string();
        if !classes.contains(&class) {
            classes.push(class);
        }
    }
    classes
}

pub fn helper_primitive_kind_catalog() -> Vec<String> {
    HELPER_PRIMITIVE_KINDS
        .iter()
        .map(|kind| (*kind).to_string())
        .collect()
}
