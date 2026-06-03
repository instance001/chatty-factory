use crate::{contracts::{FamilyCapabilityManifest, FamilyPrimitiveAdapter}, FamilyId};

pub fn built_in_family_manifests() -> Vec<FamilyCapabilityManifest> {
    vec![
        manifest(include_str!(
            "../../../families/manifests/static_web_dashboard.json"
        )),
        manifest(include_str!(
            "../../../families/manifests/chattycog_webview_module.json"
        )),
        manifest(include_str!(
            "../../../families/manifests/chattycog_native_window_module.json"
        )),
        manifest(include_str!(
            "../../../families/manifests/chattyedu_native_window_module.json"
        )),
        manifest(include_str!(
            "../../../families/manifests/chattycog_chattyedu_native_window_module.json"
        )),
        manifest(include_str!(
            "../../../families/manifests/chattycog_workspace_module.json"
        )),
        manifest(include_str!(
            "../../../families/manifests/python_cli_tool.json"
        )),
        manifest(include_str!(
            "../../../families/manifests/rust_cli_tool.json"
        )),
    ]
}

fn manifest(json: &str) -> FamilyCapabilityManifest {
    serde_json::from_str(json).expect("family capability manifest must be valid JSON")
}

pub fn manifest_for_family(family_id: &FamilyId) -> Option<FamilyCapabilityManifest> {
    built_in_family_manifests()
        .into_iter()
        .find(|manifest| &manifest.family_id == family_id)
}

pub fn build_primitive_classes_for_family(family_id: &FamilyId) -> Vec<String> {
    manifest_for_family(family_id)
        .map(|manifest| manifest.provided_build_primitive_classes)
        .unwrap_or_default()
}

pub fn primitive_adapters_for_family(family_id: &FamilyId) -> Vec<FamilyPrimitiveAdapter> {
    manifest_for_family(family_id)
        .map(|manifest| manifest.primitive_adapters)
        .unwrap_or_default()
}

pub fn primitive_adapter_names_for_family_layer(
    family_id: &FamilyId,
    composition_layer: &str,
) -> Vec<String> {
    primitive_adapters_for_family(family_id)
        .into_iter()
        .filter(|adapter| adapter.composition_layer == composition_layer)
        .map(|adapter| adapter.primitive_name)
        .collect()
}

pub fn is_supported_explicit_stack(stack: &str) -> bool {
    built_in_family_manifests().into_iter().any(|manifest| {
        manifest
            .supported_stack_ids
            .iter()
            .any(|supported| supported == stack)
    })
}

pub fn rank_family_candidates(
    lower: &str,
    requested_capabilities: &[String],
) -> Vec<FamilyId> {
    let mut scored = built_in_family_manifests()
        .into_iter()
        .map(|manifest| {
            let mut score = 0usize;
            let forbidden = manifest
                .forbids_capabilities
                .iter()
                .any(|cap| requested_capabilities.iter().any(|requested| requested == cap));
            let helper_required = manifest
                .requires_helper_for
                .iter()
                .any(|cap| requested_capabilities.iter().any(|requested| requested == cap));
            if forbidden || helper_required {
                return (manifest.family_id, 0usize);
            }
            for token in manifest
                .explicit_stack_keywords
                .iter()
                .chain(manifest.route_keywords.iter())
            {
                if lower.contains(token) {
                    score += 1;
                }
            }
            (manifest.family_id, score)
        })
        .collect::<Vec<_>>();

    scored.sort_by(|a, b| b.1.cmp(&a.1));
    let mut out = scored
        .into_iter()
        .filter(|(_, score)| *score > 0)
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    if out.is_empty() {
        out = built_in_family_manifests()
            .into_iter()
            .map(|manifest| manifest.family_id)
            .collect();
    }
    out
}
