#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildStarterChoice {
    pub id: &'static str,
    pub label: &'static str,
    pub lifecycle: &'static str,
    pub best_for: &'static str,
}

const BUILD_STARTER_CHOICES: [BuildStarterChoice; 9] = [
    BuildStarterChoice {
        id: "auto",
        label: "Auto",
        lifecycle: "adaptive routing",
        best_for: "letting normal request routing choose the family when you do not want to pin a starter",
    },
    BuildStarterChoice {
        id: "chattycog_native_window_module",
        label: "Chatty-Cog Rust Native Dashboard",
        lifecycle: "active starter",
        best_for: "standalone Rust GUI dashboards that should also load inside Chatty-Cog via native-window plug files",
    },
    BuildStarterChoice {
        id: "chattyedu_native_window_module",
        label: "Chatty-EDU Rust Native Dashboard",
        lifecycle: "active starter",
        best_for: "standalone Rust GUI dashboards that should also load inside Chatty-EDU via its native-window plug files",
    },
    BuildStarterChoice {
        id: "chattycog_chattyedu_native_window_module",
        label: "Chatty-Cog + Chatty-EDU Rust Native Dashboard",
        lifecycle: "active starter",
        best_for: "standalone Rust GUI dashboards that should be droppable into either Chatty-Cog or Chatty-EDU with both compatibility layers prefilled",
    },
    BuildStarterChoice {
        id: "chattycog_webview_module",
        label: "Chatty-Cog Webview Module",
        lifecycle: "frozen legacy starter",
        best_for: "legacy hosted webview modules when you explicitly need the older module shape",
    },
    BuildStarterChoice {
        id: "chattycog_workspace_module",
        label: "Chatty-Cog Workspace Module",
        lifecycle: "frozen legacy starter",
        best_for: "legacy workspace-style Chatty-Cog module layouts when you explicitly need that older starter",
    },
    BuildStarterChoice {
        id: "static_web_dashboard",
        label: "Static Web Dashboard",
        lifecycle: "transitional starter",
        best_for: "simple HTML, CSS, and JavaScript dashboards without native-window hosting",
    },
    BuildStarterChoice {
        id: "rust_cli_tool",
        label: "Rust CLI Tool",
        lifecycle: "transitional starter",
        best_for: "standalone Rust command-line tools and file-processing utilities",
    },
    BuildStarterChoice {
        id: "python_cli_tool",
        label: "Python CLI Tool",
        lifecycle: "transitional starter",
        best_for: "standalone Python command-line tools and quick local automation scripts",
    },
];

pub fn build_starter_choices() -> &'static [BuildStarterChoice] {
    &BUILD_STARTER_CHOICES
}

pub fn build_starter_label(id: &str) -> &'static str {
    build_starter_choices()
        .iter()
        .find(|choice| choice.id == id)
        .map(|choice| choice.label)
        .unwrap_or("Auto")
}

pub fn build_starter_picker_label(id: &str) -> String {
    build_starter_choices()
        .iter()
        .find(|choice| choice.id == id)
        .map(|choice| format!("{} [{}]", choice.label, choice.lifecycle))
        .unwrap_or_else(|| "Auto [adaptive routing]".to_string())
}

pub fn build_starter_lifecycle(id: &str) -> &'static str {
    build_starter_choices()
        .iter()
        .find(|choice| choice.id == id)
        .map(|choice| choice.lifecycle)
        .unwrap_or("adaptive routing")
}

pub fn build_starter_best_for(id: &str) -> &'static str {
    build_starter_choices()
        .iter()
        .find(|choice| choice.id == id)
        .map(|choice| choice.best_for)
        .unwrap_or("letting normal request routing choose the family when you do not want to pin a starter")
}

pub fn is_known_build_starter_id(id: &str) -> bool {
    build_starter_choices().iter().any(|choice| choice.id == id)
}
