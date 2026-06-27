#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorBundleSpec {
    pub bundle_id: &'static str,
    pub substrate_kinds: &'static [&'static str],
    pub tool_kinds: &'static [&'static str],
    pub operator_ids: &'static [&'static str],
}

const OPERATOR_BUNDLES: &[OperatorBundleSpec] = &[
    OperatorBundleSpec {
        bundle_id: "dashboard_standard_surface",
        substrate_kinds: &["static_web", "webview", "native_window"],
        tool_kinds: &["dashboard"],
        operator_ids: &[
            "metric_card",
            "status_panel",
            "results_panel",
            "action_toolbar",
        ],
    },
    OperatorBundleSpec {
        bundle_id: "dashboard_status_focus",
        substrate_kinds: &["static_web", "webview", "native_window"],
        tool_kinds: &["dashboard"],
        operator_ids: &["status_panel", "results_panel"],
    },
];

pub fn operator_bundle_registry() -> &'static [OperatorBundleSpec] {
    OPERATOR_BUNDLES
}

pub fn candidate_operator_bundle_ids_for_context(
    substrate_kind: Option<&str>,
    tool_kind: Option<&str>,
) -> Vec<String> {
    OPERATOR_BUNDLES
        .iter()
        .filter(|spec| {
            let substrate_match = substrate_kind.map_or(true, |candidate| {
                spec.substrate_kinds
                    .iter()
                    .any(|supported| supported == &candidate)
            });
            let tool_match = tool_kind.map_or(true, |candidate| {
                spec.tool_kinds
                    .iter()
                    .any(|supported| supported == &candidate)
            });
            substrate_match && tool_match
        })
        .map(|spec| spec.bundle_id.to_string())
        .collect()
}

pub fn expand_operator_bundle_ids(bundle_ids: &[String]) -> Vec<String> {
    let mut operator_ids = Vec::new();
    for bundle_id in bundle_ids {
        if let Some(spec) = OPERATOR_BUNDLES
            .iter()
            .find(|spec| spec.bundle_id == bundle_id)
        {
            for operator_id in spec.operator_ids {
                if !operator_ids.iter().any(|existing| existing == operator_id) {
                    operator_ids.push((*operator_id).to_string());
                }
            }
        }
    }
    operator_ids
}
