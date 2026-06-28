const CHATTYCOG_TERMS: &[&str] = &["chattycog", "chatty-cog", "module"];
const CHATTYEDU_TERMS: &[&str] = &["chattyedu", "chatty-edu"];
const RUST_TERMS: &[&str] = &["rust", "cargo", "compiled tool", "compiled utility"];
const PYTHON_TERMS: &[&str] = &[
    "python",
    "script",
    "cli",
    "command line",
    "file sorter",
    "sort files",
    "utility",
];
const WEB_SHAPE_TERMS: &[&str] = &["browser", "web", "webview"];
const DASHBOARD_TERMS: &[&str] = &["dashboard", "panel", "metric"];
const CLI_SHAPE_TERMS: &[&str] = &[
    "sorter",
    "sort files",
    "csv",
    "spreadsheet",
    "tabular",
    "log",
    "logs",
    "trace",
    "stats",
    "word count",
    "line count",
    "count text",
    "audit",
    "inventory",
    "inspect",
    "directory",
    "folder",
    "cli",
    "script",
    "command line",
    "python",
    "rust",
    "utility",
];
const VAGUE_IMPROVEMENT_TERMS: &[&str] = &["better", "improve", "fix it", "upgrade", "upgrade it"];
const FOLLOWUP_ACTION_TERMS: &[&str] = &["add ", "export", "filter", "banner"];
const EXPLICIT_BUILD_SHAPE_TERMS: &[&str] = &[
    "dashboard",
    "python",
    "rust",
    "chattycog",
    "chatty-cog",
    "module",
    "web",
    "browser",
];

const PATCH_KIND_RULES: &[(&str, &[&str], &[&str])] = &[
    ("progress_banner", &["progress banner", "banner"], &[]),
    (
        "bridge_activity_panel",
        &["bridge activity panel", "bridge panel", "activity panel"],
        &["bridge"],
    ),
    (
        "bridge_status_panel",
        &["bridge status panel", "bridge status"],
        &["bridge", "status"],
    ),
    (
        "asset_inbox_panel",
        &["asset inbox panel", "inbox panel", "asset panel"],
        &["inbox", "panel"],
    ),
    (
        "helper_summary_panel",
        &[
            "helper summary panel",
            "inbox summary panel",
            "helper status panel",
        ],
        &["helper", "summary", "panel"],
    ),
    (
        "helper_summary_badges",
        &[
            "helper summary badges",
            "summary badges",
            "helper badges",
            "filter badges",
        ],
        &["summary", "badges"],
    ),
    (
        "helper_summary_empty_state",
        &[
            "helper summary empty state",
            "helper empty state",
            "summary empty state",
            "empty helper state",
        ],
        &["empty", "state"],
    ),
    (
        "helper_last_run_stamp",
        &[
            "helper last run stamp",
            "last run stamp",
            "helper last updated",
            "helper update stamp",
            "last helper update",
        ],
        &["last", "run"],
    ),
    (
        "helper_summary_metadata_row",
        &[
            "helper summary metadata row",
            "helper metadata row",
            "summary metadata row",
            "helper metadata",
        ],
        &["summary", "metadata"],
    ),
    (
        "helper_summary_count_delta",
        &[
            "helper summary count delta",
            "helper count delta",
            "summary count delta",
            "helper count difference",
        ],
        &["count", "delta"],
    ),
    (
        "helper_summary_lane_count_chip",
        &[
            "helper summary lane count chip",
            "helper lane count chip",
            "lane count chip",
            "summary lane count chip",
        ],
        &["lane", "count", "chip"],
    ),
    (
        "helper_summary_types_chip",
        &[
            "helper summary types chip",
            "helper types chip",
            "summary types chip",
            "types chip",
        ],
        &["types", "chip"],
    ),
    (
        "helper_summary_updated_at_chip",
        &[
            "helper summary updated at chip",
            "summary updated at chip",
            "helper updated at chip",
            "updated at chip",
        ],
        &["updated", "chip"],
    ),
    (
        "helper_summary_filter_notice",
        &[
            "helper summary filter notice",
            "helper filter notice",
            "summary filter notice",
        ],
        &["summary", "filter", "notice"],
    ),
    (
        "lane_scoped_filter_notice",
        &[
            "lane scoped filter notice",
            "lane filter notice",
            "per lane filter notice",
            "lane scoped filter",
        ],
        &["lane", "filter"],
    ),
    (
        "lane_scoped_metadata_row",
        &[
            "lane scoped metadata row",
            "lane metadata row",
            "per lane metadata row",
            "lane scoped metadata",
        ],
        &["lane", "metadata"],
    ),
    (
        "helper_summary_discovered_notice",
        &[
            "helper summary discovered notice",
            "helper discovered notice",
            "discovered notice",
            "discovered files notice",
        ],
        &["discovered", "notice"],
    ),
    (
        "secondary_inbox_lane",
        &[
            "secondary inbox lane",
            "second inbox lane",
            "secondary helper inbox",
            "extra inbox lane",
        ],
        &["secondary", "inbox"],
    ),
    (
        "helper_summary_status_chip",
        &[
            "helper summary status chip",
            "summary status chip",
            "helper summary status badge",
        ],
        &["summary", "status", "chip"],
    ),
    (
        "helper_status_chip",
        &["helper status chip", "helper status badge"],
        &["helper", "status"],
    ),
    (
        "processed_files_panel",
        &[
            "processed files panel",
            "processed inbox panel",
            "helper files panel",
        ],
        &["processed", "files"],
    ),
    (
        "auto_refresh_helper_panels",
        &[
            "auto refresh helper panels",
            "auto refresh helper panel",
            "refresh helper panels",
        ],
        &["refresh", "helper"],
    ),
    (
        "processed_file_preview_panel",
        &[
            "processed file preview panel",
            "file preview panel",
            "helper preview panel",
        ],
        &["preview", "file"],
    ),
    (
        "processed_file_selection",
        &[
            "processed file selection",
            "select processed file",
            "preview selected file",
            "processed file picker",
        ],
        &["select", "processed", "file"],
    ),
    (
        "file_type_filter",
        &[
            "file type filter",
            "helper file filter",
            "txt only filter",
            "text file filter",
        ],
        &["file", "type", "filter"],
    ),
    (
        "metric_strip",
        &["metric strip", "metric cards", "status cards"],
        &["metric"],
    ),
    (
        "json_output",
        &[
            "json summary",
            "log json output",
            "json output for the summary",
        ],
        &["json", "summary"],
    ),
    ("json_export", &["json export", "json output"], &["json"]),
    (
        "column_filter",
        &["column filter", "select column", "only column"],
        &["column"],
    ),
    (
        "email_sender",
        &["email sender", "email report", "email export", "send email"],
        &["email"],
    ),
    (
        "new_patch_lane",
        &[
            "markdown export",
            "markdown summary",
            "markdown report",
            "summary markdown",
        ],
        &["markdown"],
    ),
    (
        "severity_filter",
        &["severity filter", "severity only", "select severity"],
        &["severity"],
    ),
    (
        "file_output",
        &["file output", "write file", "save summary"],
        &["output"],
    ),
    (
        "ready_toggle",
        &["ready toggle", "ready button", "ready state toggle"],
        &["ready"],
    ),
    (
        "room_state_fields",
        &["room state fields", "room state", "session fields"],
        &["room", "state"],
    ),
    (
        "session_overview",
        &["session overview", "session section", "room overview"],
        &["session", "overview"],
    ),
];

const CAPABILITY_RULES: &[(&str, &[&str])] = &[
    ("dashboard", &["dashboard"]),
    ("module_wrapper", &["module"]),
    ("search", &["search"]),
    ("data_table", &["table"]),
    ("cli", &["cli", "script", "command line"]),
    ("rust", &["rust", "cargo"]),
    ("backend_service", &["service", "server", "backend"]),
    ("websocket", &["websocket", "socket"]),
    ("api_server", &["api", "endpoint"]),
    ("daemon", &["daemon", "background worker"]),
    ("local_filesystem", &["filesystem", "local files"]),
    ("background_job", &["background job", "scheduler"]),
    ("email_delivery", &["email", "smtp"]),
];

const EXPLICIT_STACK_RULES: &[(&str, &[&str])] = &[
    ("golang", &["golang", "go service", "go backend"]),
    ("typescript", &["typescript"]),
    ("node", &["javascript", "nodejs", "node "]),
    ("python", &["python"]),
    ("rust", &["rust", "cargo"]),
    ("web", &["web", "browser"]),
];

pub fn contains_any(lower: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| lower.contains(term))
}

pub fn contains_all(lower: &str, terms: &[&str]) -> bool {
    !terms.is_empty() && terms.iter().all(|term| lower.contains(term))
}

pub fn request_mentions_chattycog(lower: &str) -> bool {
    contains_any(lower, CHATTYCOG_TERMS)
}

pub fn request_mentions_chattyedu(lower: &str) -> bool {
    contains_any(lower, CHATTYEDU_TERMS)
}

pub fn request_mentions_rust(lower: &str) -> bool {
    contains_any(lower, RUST_TERMS)
}

pub fn request_mentions_python(lower: &str) -> bool {
    contains_any(lower, PYTHON_TERMS)
}

pub fn request_has_web_shape(lower: &str) -> bool {
    contains_any(lower, WEB_SHAPE_TERMS)
}

pub fn request_has_dashboard_shape(lower: &str) -> bool {
    contains_any(lower, DASHBOARD_TERMS)
        || contains_any(
            lower,
            &[
                "kanban",
                "task tracker",
                "project tracker",
                "tracking board",
                "drag and drop cards",
            ],
        )
}

pub fn request_has_cli_shape(lower: &str) -> bool {
    contains_any(lower, CLI_SHAPE_TERMS)
}

pub fn request_has_vague_improvement(lower: &str) -> bool {
    contains_any(lower, VAGUE_IMPROVEMENT_TERMS)
}

pub fn request_looks_like_followup_action(lower: &str) -> bool {
    contains_any(lower, FOLLOWUP_ACTION_TERMS)
}

pub fn request_has_explicit_build_shape(lower: &str) -> bool {
    contains_any(lower, EXPLICIT_BUILD_SHAPE_TERMS)
}

pub fn infer_capabilities_from_text(lower: &str) -> Vec<String> {
    let mut caps = Vec::new();
    for (capability, terms) in CAPABILITY_RULES {
        if contains_any(lower, terms) {
            caps.push((*capability).to_string());
        }
    }
    caps.sort();
    caps.dedup();
    caps
}

pub fn infer_explicit_stack_from_text(lower: &str) -> Option<String> {
    EXPLICIT_STACK_RULES
        .iter()
        .find(|(_, terms)| contains_any(lower, terms))
        .map(|(stack, _)| (*stack).to_string())
}

pub fn infer_cli_tool_kind_from_text(lower: &str) -> Option<&'static str> {
    if contains_any(lower, &["sorter", "sort files"]) {
        Some("file_sorter")
    } else if contains_any(lower, &["csv", "spreadsheet", "tabular"]) {
        Some("csv_report")
    } else if contains_any(lower, &["log", "logs", "trace"]) {
        Some("log_summary")
    } else if contains_any(
        lower,
        &[
            "stats",
            "word count",
            "line count",
            "character count",
            "text counter",
            "count text",
            "--text",
            "--file",
            "characters=",
            "words=",
            "lines=",
        ],
    ) {
        Some("text_stats")
    } else if contains_any(
        lower,
        &[
            "audit",
            "inventory",
            "report",
            "inspect",
            "directory",
            "folder",
        ],
    ) {
        Some("directory_audit")
    } else {
        None
    }
}

pub fn infer_request_tool_kind_from_text(
    lower: &str,
    wants_chattycog: bool,
    wants_chattyedu: bool,
    desired_surface_cli: bool,
) -> Option<&'static str> {
    if (wants_chattycog || wants_chattyedu)
        && contains_any(lower, &["native window", "desktop", "tkinter"])
    {
        Some("dashboard")
    } else if wants_chattycog
        && contains_any(lower, &["workspace", "ui.json", "headless", "notes module"])
    {
        Some("dashboard")
    } else if wants_chattycog
        && contains_any(
            lower,
            &[
                "webview",
                "embedded webview",
                "hosted webview",
                "browser tab",
            ],
        )
    {
        Some("dashboard")
    } else if wants_chattycog {
        Some("dashboard")
    } else if wants_chattyedu {
        Some("dashboard")
    } else if request_has_dashboard_shape(lower) {
        Some("dashboard")
    } else if desired_surface_cli || request_has_cli_shape(lower) {
        infer_cli_tool_kind_from_text(lower).or(Some("directory_audit"))
    } else {
        None
    }
}

pub fn infer_chattycog_hosting_modes_from_text(lower: &str) -> Vec<&'static str> {
    let mut modes = Vec::new();

    if contains_any(lower, &["webview", "embedded webview", "hosted webview"]) {
        modes.push("hosted_webview");
    }
    if contains_any(
        lower,
        &["native window", "desktop", "tkinter", "separate window"],
    ) {
        modes.push("hosted_native_window");
    }
    modes.sort();
    modes.dedup();
    modes
}

pub fn infer_chattycog_hosting_mode_from_text(lower: &str) -> Option<&'static str> {
    let modes = infer_chattycog_hosting_modes_from_text(lower);
    if modes.len() == 1 {
        Some(modes[0])
    } else {
        None
    }
}

pub fn chattycog_valid_hosting_modes() -> Vec<String> {
    vec!["hosted_webview".into(), "hosted_native_window".into()]
}

pub fn infer_chattycog_bridge_capabilities_from_text(lower: &str) -> Vec<String> {
    let mut capabilities = Vec::new();

    if contains_any(lower, &["status", "status.json", "handoff status"]) {
        capabilities.push("status".into());
    }
    if contains_any(
        lower,
        &["log source", "log sources", "module logs", "tail logs"],
    ) {
        capabilities.push("log_sources".into());
    }
    if contains_any(lower, &["shared room state", "room state", "room policy"]) {
        capabilities.push("shared_room_state".into());
    }
    if contains_any(
        lower,
        &["shared room events", "room events", "incoming room events"],
    ) {
        capabilities.push("shared_room_events".into());
    }
    if contains_any(
        lower,
        &["outgoing room events", "emit room event", "send room event"],
    ) {
        capabilities.push("outgoing_room_events".into());
    }
    if contains_any(
        lower,
        &[
            "incoming assets",
            "incoming asset lane",
            "asset lane",
            "bridge inbox",
            "inbox lane",
        ],
    ) {
        capabilities.push("incoming_asset_lanes".into());
    }

    capabilities.sort();
    capabilities.dedup();
    capabilities
}

pub fn supported_chattycog_bridge_capabilities() -> Vec<String> {
    vec![
        "status".into(),
        "log_sources".into(),
        "shared_room_state".into(),
        "shared_room_events".into(),
        "outgoing_room_events".into(),
        "incoming_asset_lanes".into(),
    ]
}

pub fn infer_patch_kind_from_text(lower: &str, supported_patch_kinds: &[String]) -> Option<String> {
    for patch_kind in supported_patch_kinds {
        if lower.contains(&patch_kind.replace('_', " ")) || lower.contains(patch_kind) {
            return Some(patch_kind.clone());
        }
    }

    for (patch_kind, any_terms, fallback_terms) in PATCH_KIND_RULES {
        if contains_any(lower, any_terms) || contains_all(lower, fallback_terms) {
            return Some((*patch_kind).to_string());
        }
    }

    None
}

pub fn should_route_followup_via_planner_text(lower: &str) -> bool {
    let short = lower.trim().len() < 48;
    let vague_followup =
        request_has_vague_improvement(lower) || request_looks_like_followup_action(lower);
    let explicit_build_shape = request_has_explicit_build_shape(lower);
    (short && vague_followup) || (!explicit_build_shape && lower.starts_with("make "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_chattycog_requests_prefer_dashboard_tool_kind() {
        let tool_kind = infer_request_tool_kind_from_text(
            "build me a chattycog dashboard module",
            true,
            false,
            false,
        );
        assert_eq!(tool_kind, Some("dashboard"));
    }

    #[test]
    fn explicit_chattycog_webview_requests_stay_dashboard_shaped() {
        let tool_kind = infer_request_tool_kind_from_text(
            "build me a chattycog webview module",
            true,
            false,
            false,
        );
        assert_eq!(tool_kind, Some("dashboard"));
    }

    #[test]
    fn generic_chattyedu_requests_prefer_dashboard_tool_kind() {
        let tool_kind = infer_request_tool_kind_from_text(
            "build me a chatty-edu lesson dashboard module",
            false,
            true,
            false,
        );
        assert_eq!(tool_kind, Some("dashboard"));
    }

    #[test]
    fn dashboard_word_alone_is_not_treated_as_web_shape() {
        assert!(!request_has_web_shape("build me a dashboard"));
        assert!(request_has_web_shape("build me a browser dashboard"));
    }

    #[test]
    fn kanban_requests_are_treated_as_dashboard_shape() {
        assert!(request_has_dashboard_shape(
            "build me a desktop kanban app with drag and drop cards"
        ));
        assert_eq!(
            infer_request_tool_kind_from_text(
                "build me a desktop kanban app with drag and drop cards",
                false,
                false,
                false,
            ),
            Some("dashboard")
        );
    }
}
