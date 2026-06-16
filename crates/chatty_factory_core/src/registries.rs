#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorBundleSpec {
    pub bundle_id: &'static str,
    pub family_ids: &'static [&'static str],
    pub operator_ids: &'static [&'static str],
}

const OPERATOR_BUNDLES: &[OperatorBundleSpec] = &[
    OperatorBundleSpec {
        bundle_id: "dashboard_standard_surface",
        family_ids: &["static_web_dashboard", "chattycog_webview_module"],
        operator_ids: &[
            "metric_card",
            "status_panel",
            "results_panel",
            "action_toolbar",
        ],
    },
    OperatorBundleSpec {
        bundle_id: "dashboard_status_focus",
        family_ids: &["static_web_dashboard", "chattycog_webview_module"],
        operator_ids: &["status_panel", "results_panel"],
    },
];

pub fn operator_bundle_registry() -> &'static [OperatorBundleSpec] {
    OPERATOR_BUNDLES
}

pub fn candidate_operator_bundle_ids_for(family_id: &str) -> Vec<String> {
    OPERATOR_BUNDLES
        .iter()
        .filter(|spec| {
            spec.family_ids
                .iter()
                .any(|candidate| candidate == &family_id)
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
