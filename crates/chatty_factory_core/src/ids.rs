use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubstrateKind {
    StaticWeb,
    Webview,
    NativeWindow,
    Workspace,
    Cli,
}

impl SubstrateKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StaticWeb => "static_web",
            Self::Webview => "webview",
            Self::NativeWindow => "native_window",
            Self::Workspace => "workspace",
            Self::Cli => "cli",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrapperId(pub String);
