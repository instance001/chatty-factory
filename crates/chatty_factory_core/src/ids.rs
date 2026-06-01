use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyId {
    StaticWebDashboard,
    #[serde(alias = "chattycog_basic_dashboard")]
    ChattycogWebviewModule,
    ChattycogNativeWindowModule,
    ChattycogWorkspaceModule,
    PythonCliTool,
    RustCliTool,
}

impl FamilyId {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StaticWebDashboard => "static_web_dashboard",
            Self::ChattycogWebviewModule => "chattycog_webview_module",
            Self::ChattycogNativeWindowModule => "chattycog_native_window_module",
            Self::ChattycogWorkspaceModule => "chattycog_workspace_module",
            Self::PythonCliTool => "python_cli_tool",
            Self::RustCliTool => "rust_cli_tool",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrapperId(pub String);
