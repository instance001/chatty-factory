use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Result};
use chatty_factory_core::{
    AcceptancePlan, ChattyCogModuleSpec, ChattyCogVisualLoadSpec, ChattyEduModuleSpec,
    FailureClass,
    HelperServiceSpec, HelperStatusSnapshot,
};

pub fn classify_failure(summary: &str) -> FailureClass {
    let lower = summary.to_ascii_lowercase();
    if lower.contains("schema") || lower.contains("metadata") {
        FailureClass::InvalidMetadata
    } else if lower.contains("helper summary surface contract failed")
        || lower.contains("helper processed")
        || lower.contains("helper service spec")
        || lower.contains("helper status snapshot")
        || lower.contains("helper summary snapshot")
    {
        FailureClass::HelperWiringFailure
    } else if lower.contains("chattycog module contract failed")
        || lower.contains("chattycog visual load contract failed")
        || lower.contains("chattycog bridge contract failed")
        || lower.contains("chattyedu module contract failed")
        || lower.contains("chattyedu visual load contract failed")
    {
        FailureClass::StructuredCodeGenerationMismatch
    } else if lower.contains("policy")
        || lower.contains("outside output root")
        || lower.contains("safe relative path")
    {
        FailureClass::PolicyViolation
    } else if (lower.contains("missing") && lower.contains("file"))
        || (lower.contains("acceptance check") && lower.contains("missing"))
        || lower.contains("no such file")
    {
        FailureClass::MissingExpectedFiles
    } else if lower.contains("syntax") {
        FailureClass::SyntaxFailure
    } else if lower.contains("build") || lower.contains("compile") {
        FailureClass::BuildFailure
    } else {
        FailureClass::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::classify_failure;
    use chatty_factory_core::FailureClass;

    #[test]
    fn helper_contract_failures_classify_as_helper_wiring_failure() {
        let summary =
            "helper summary surface contract failed: 'helperSummaryStatusChip' missing from app.js";
        assert_eq!(
            classify_failure(summary),
            FailureClass::HelperWiringFailure
        );
    }

    #[test]
    fn chattycog_contract_failures_classify_as_structured_generation_mismatch() {
        let summary = "chattycog bridge contract failed: missing bridge/status.json";
        assert_eq!(
            classify_failure(summary),
            FailureClass::StructuredCodeGenerationMismatch
        );
    }

    #[test]
    fn chattyedu_contract_failures_classify_as_structured_generation_mismatch() {
        let summary =
            "chattyedu module contract failed: main.rs is missing the bridge env var contract";
        assert_eq!(
            classify_failure(summary),
            FailureClass::StructuredCodeGenerationMismatch
        );
    }
}

pub fn verify_acceptance_plan(project_dir: &Path, plan: &AcceptancePlan) -> Result<()> {
    for check in &plan.checks {
        let target = project_dir.join(&check.target);
        match check.kind.as_str() {
            "exists" => {
                if !target.exists() {
                    bail!(
                        "acceptance check '{}' failed: missing {}",
                        check.check_id,
                        check.target
                    );
                }
            }
            "contains" => {
                let expected = check.expected.as_deref().ok_or_else(|| {
                    anyhow::anyhow!("check '{}' missing expected value", check.check_id)
                })?;
                let contents = fs::read_to_string(&target)?;
                if !contents.contains(expected) {
                    bail!(
                        "acceptance check '{}' failed: '{}' not found in {}",
                        check.check_id,
                        expected,
                        check.target
                    );
                }
            }
            "python_output_contains" => {
                let expected = check.expected.as_deref().ok_or_else(|| {
                    anyhow::anyhow!("check '{}' missing expected value", check.check_id)
                })?;
                let out = run_python_command(project_dir, &check.target)?;
                if !out.contains(expected) {
                    bail!(
                        "acceptance check '{}' failed: '{}' not found in python output",
                        check.check_id,
                        expected
                    );
                }
            }
            "cargo_check" => {
                run_cargo_check(project_dir)?;
            }
            "cargo_run_output_contains" => {
                let expected = check.expected.as_deref().ok_or_else(|| {
                    anyhow::anyhow!("check '{}' missing expected value", check.check_id)
                })?;
                let out = run_cargo_run(project_dir, &check.target)?;
                if !out.contains(expected) {
                    bail!(
                        "acceptance check '{}' failed: '{}' not found in cargo output",
                        check.check_id,
                        expected
                    );
                }
            }
            "python_run_success" => {
                run_python_command(project_dir, &check.target)?;
            }
            "cargo_run_success" => {
                run_cargo_run(project_dir, &check.target)?;
            }
            "chattycog_module_contract" => {
                verify_chattycog_module_contract(project_dir, &check.target)?;
            }
            "chattycog_visual_load_contract" => {
                verify_chattycog_visual_load_contract(project_dir, &check.target)?;
            }
            "chattycog_bridge_contract" => {
                verify_chattycog_bridge_contract(project_dir, &check.target)?;
            }
            "chattyedu_module_contract" => {
                verify_chattyedu_module_contract(project_dir, &check.target)?;
            }
            "chattyedu_visual_load_contract" => {
                verify_chattyedu_visual_load_contract(project_dir, &check.target)?;
            }
            "helper_service_spec" => {
                verify_helper_service_spec(project_dir, &check.target, check.expected.as_deref())?;
            }
            "helper_status_snapshot" => {
                verify_helper_status_snapshot(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "helper_summary_snapshot" => {
                verify_helper_summary_snapshot(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "helper_summary_discovered_count" => {
                verify_helper_summary_numeric_field(
                    project_dir,
                    &check.target,
                    "discovered_file_count",
                    check.expected.as_deref(),
                )?;
            }
            "helper_summary_filtered_count" => {
                verify_helper_summary_numeric_field(
                    project_dir,
                    &check.target,
                    "filtered_out_file_count",
                    check.expected.as_deref(),
                )?;
            }
            "helper_summary_observed_lanes" => {
                verify_helper_summary_observed_lanes(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "helper_summary_lane_rules" => {
                verify_helper_summary_lane_rules(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "helper_summary_filtered_files" => {
                verify_helper_summary_filtered_files(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "helper_summary_surface_contract" => {
                verify_helper_summary_surface_contract(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "helper_processed_files_surface_contract" => {
                verify_helper_processed_files_surface_contract(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "helper_processed_preview_surface_contract" => {
                verify_helper_processed_preview_surface_contract(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "helper_processed_selection_surface_contract" => {
                verify_helper_processed_selection_surface_contract(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "helper_processed_output_contract" => {
                verify_helper_processed_output_contract(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            "static_dashboard_helper_monitoring_surface_contract" => {
                verify_static_dashboard_helper_monitoring_surface_contract(
                    project_dir,
                    &check.target,
                    check.expected.as_deref(),
                )?;
            }
            other => bail!("unsupported acceptance check kind '{}'", other),
        }
    }
    Ok(())
}

fn verify_chattycog_module_contract(project_dir: &Path, target: &str) -> Result<()> {
    let spec_path = project_dir.join(target);
    let spec: ChattyCogModuleSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;

    let manifest_path = project_dir.join(&spec.manifest_path);
    let handshake_path = project_dir.join(&spec.handshake_path);
    let bridge_status_path = project_dir.join(&spec.bridge.status_path);

    for path in [&manifest_path, &handshake_path, &bridge_status_path] {
        if !path.exists() {
            bail!("chattycog module contract failed: missing {}", path.display());
        }
    }
    if let Some(visual_load_path) = &spec.visual_load_path {
        let path = project_dir.join(visual_load_path);
        if !path.exists() {
            bail!("chattycog module contract failed: missing {}", path.display());
        }
    }
    if let Some(script_path) = &spec.bridge.script_path {
        let path = project_dir.join(script_path);
        if !path.exists() {
            bail!("chattycog module contract failed: missing {}", path.display());
        }
    }

    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(&manifest_path)?)?;
    let handshake = fs::read_to_string(&handshake_path)?;
    let bridge_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&bridge_status_path)?)?;

    if manifest
        .get("module_id")
        .and_then(|value| value.as_str())
        != Some(spec.module_id.as_str())
    {
        bail!("chattycog module contract failed: manifest module_id did not match module spec");
    }
    if manifest
        .get("display_name")
        .and_then(|value| value.as_str())
        != Some(spec.display_name.as_str())
    {
        bail!("chattycog module contract failed: manifest display_name did not match module spec");
    }
    match (&spec.visual_load_path, &spec.visual_load) {
        (Some(visual_load_path), Some(visual_load_spec)) => {
            let visual_load_path = project_dir.join(visual_load_path);
            let visual_load: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&visual_load_path)?)?;
            if visual_load
                .get("kind")
                .and_then(|value| value.as_str())
                != Some(spec.visual_kind.as_str())
            {
                bail!("chattycog module contract failed: visual_load kind did not match module spec");
            }
            verify_visual_load_value(&visual_load, visual_load_spec)?;
        }
        (None, None) => {
            if spec.visual_kind != "workspace" {
                bail!("chattycog module contract failed: non-workspace module missing visual_load spec");
            }
        }
        _ => {
            bail!("chattycog module contract failed: visual_load path/spec mismatch");
        }
    }
    if !handshake.contains(&spec.module_id) || !handshake.contains(&spec.display_name) {
        bail!("chattycog module contract failed: HANDSHAKE.md is missing module identity fields");
    }
    if bridge_status.get("event_type").and_then(|value| value.as_str())
        != Some("suspend_rundown")
    {
        bail!("chattycog module contract failed: bridge status stub must use suspend_rundown");
    }
    if bridge_status.get("module_id").and_then(|value| value.as_str())
        != Some(spec.module_id.as_str())
    {
        bail!("chattycog module contract failed: bridge status module_id did not match module spec");
    }
    Ok(())
}

fn verify_chattycog_visual_load_contract(project_dir: &Path, target: &str) -> Result<()> {
    let visual_load_path = project_dir.join(target);
    let value: serde_json::Value = serde_json::from_str(&fs::read_to_string(&visual_load_path)?)?;
    let spec = parse_visual_load_spec(&value)?;
    verify_visual_load_value(&value, &spec)?;

    match spec.kind.as_str() {
        "webview" => {
            if spec.file.is_none() && spec.url.is_none() {
                bail!("chattycog visual load contract failed: webview requires file or url");
            }
        }
        "native_window" => {
            if spec.launch_command.is_none() {
                bail!(
                    "chattycog visual load contract failed: native_window requires launch command"
                );
            }
        }
        other => bail!("chattycog visual load contract failed: unsupported kind '{}'", other),
    }
    Ok(())
}

fn verify_chattycog_bridge_contract(project_dir: &Path, target: &str) -> Result<()> {
    let spec_path = project_dir.join(target);
    let spec: ChattyCogModuleSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;

    let bridge_status_path = project_dir.join(&spec.bridge.status_path);
    if !bridge_status_path.exists() {
        bail!("chattycog bridge contract failed: missing {}", bridge_status_path.display());
    }
    if spec.bridge.capabilities.log_sources_enabled {
        let Some(log_sources_path) = spec.bridge.log_sources_path.as_ref() else {
            bail!("chattycog bridge contract failed: log_sources enabled but path missing");
        };
        let full_path = project_dir.join(log_sources_path);
        if !full_path.exists() {
            bail!("chattycog bridge contract failed: missing {}", full_path.display());
        }
        let log_sources: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&full_path)?)?;
        if log_sources.get("sources").and_then(|value| value.as_array()).is_none() {
            bail!("chattycog bridge contract failed: log_sources.json missing sources array");
        }
    }
    if spec.bridge.capabilities.shared_room_state_enabled {
        let shared_room_state_path = project_dir.join("bridge/shared_room_state.json");
        if !shared_room_state_path.exists() {
            bail!(
                "chattycog bridge contract failed: missing {}",
                shared_room_state_path.display()
            );
        }
        let shared_room_state: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&shared_room_state_path)?)?;
        if !shared_room_state.is_object() {
            bail!(
                "chattycog bridge contract failed: shared_room_state.json must be a json object"
            );
        }
    }
    if spec.bridge.capabilities.shared_room_events_enabled {
        let shared_room_events_path = project_dir.join("bridge/shared_room_events.json");
        if !shared_room_events_path.exists() {
            bail!(
                "chattycog bridge contract failed: missing {}",
                shared_room_events_path.display()
            );
        }
        let shared_room_events: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&shared_room_events_path)?)?;
        if shared_room_events
            .get("events")
            .and_then(|value| value.as_array())
            .is_none()
        {
            bail!(
                "chattycog bridge contract failed: shared_room_events.json missing events array"
            );
        }
    }
    if spec.bridge.capabilities.outgoing_room_events_enabled {
        let outgoing_room_events_path = project_dir.join("bridge/outgoing_room_events.json");
        if !outgoing_room_events_path.exists() {
            bail!(
                "chattycog bridge contract failed: missing {}",
                outgoing_room_events_path.display()
            );
        }
        let outgoing_room_events: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&outgoing_room_events_path)?)?;
        if outgoing_room_events
            .get("events")
            .and_then(|value| value.as_array())
            .is_none()
        {
            bail!(
                "chattycog bridge contract failed: outgoing_room_events.json missing events array"
            );
        }
    }
    for lane_id in &spec.bridge.capabilities.incoming_asset_lanes {
        let lane_path = project_dir.join("bridge").join("incoming_assets").join(lane_id);
        if !lane_path.exists() || !lane_path.is_dir() {
            bail!(
                "chattycog bridge contract failed: missing incoming asset lane directory {}",
                lane_path.display()
            );
        }
    }
    Ok(())
}

fn verify_chattyedu_module_contract(project_dir: &Path, target: &str) -> Result<()> {
    let spec_path = project_dir.join(target);
    let spec: ChattyEduModuleSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;

    let manifest_path = project_dir.join(&spec.manifest_path);
    let handshake_path = project_dir.join(&spec.handshake_path);
    let visual_load_path = project_dir.join(&spec.visual_load_path);
    let network_capabilities_path = project_dir.join(&spec.network_capabilities_path);
    let main_rs_path = project_dir.join("src/main.rs");

    for path in [
        &manifest_path,
        &handshake_path,
        &visual_load_path,
        &network_capabilities_path,
        &main_rs_path,
    ] {
        if !path.exists() {
            bail!("chattyedu module contract failed: missing {}", path.display());
        }
    }

    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(&manifest_path)?)?;
    let visual_load: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&visual_load_path)?)?;
    let network_capabilities: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&network_capabilities_path)?)?;
    let handshake = fs::read_to_string(&handshake_path)?;
    let main_rs = fs::read_to_string(&main_rs_path)?;

    if manifest
        .get("module_id")
        .and_then(|value| value.as_str())
        != Some(spec.module_id.as_str())
    {
        bail!("chattyedu module contract failed: manifest module_id did not match module spec");
    }
    if manifest
        .get("display_name")
        .and_then(|value| value.as_str())
        != Some(spec.display_name.as_str())
    {
        bail!("chattyedu module contract failed: manifest display_name did not match module spec");
    }
    if visual_load
        .get("kind")
        .and_then(|value| value.as_str())
        != Some(spec.visual_kind.as_str())
    {
        bail!("chattyedu module contract failed: visual_load kind did not match module spec");
    }
    if !handshake.contains(&spec.module_id) || !handshake.contains(&spec.display_name) {
        bail!("chattyedu module contract failed: HANDSHAKE.md is missing module identity fields");
    }
    if !main_rs.contains(&spec.bridge_status_env_var) {
        bail!("chattyedu module contract failed: main.rs is missing the bridge env var contract");
    }
    if network_capabilities
        .get("features")
        .and_then(|value| value.as_array())
        .is_none()
    {
        bail!("chattyedu module contract failed: network_capabilities.json missing features array");
    }
    Ok(())
}

fn verify_chattyedu_visual_load_contract(project_dir: &Path, target: &str) -> Result<()> {
    let visual_load_path = project_dir.join(target);
    let value: serde_json::Value = serde_json::from_str(&fs::read_to_string(&visual_load_path)?)?;
    let spec = parse_visual_load_spec(&value)?;
    verify_visual_load_value(&value, &spec)?;

    match spec.kind.as_str() {
        "native_window" => {
            if spec.launch_command.is_none() {
                bail!(
                    "chattyedu visual load contract failed: native_window requires launch command"
                );
            }
        }
        other => bail!(
            "chattyedu visual load contract failed: unsupported kind '{}'",
            other
        ),
    }
    Ok(())
}

fn verify_helper_service_spec(
    project_dir: &Path,
    target: &str,
    expected_helper_id: Option<&str>,
) -> Result<()> {
    let helper_path = project_dir.join(target);
    let helper_spec: HelperServiceSpec =
        serde_json::from_str(&fs::read_to_string(&helper_path)?)?;
    if let Some(expected) = expected_helper_id {
        if helper_spec.helper_id != expected {
            bail!(
                "helper service spec at '{}' did not match expected helper id '{}'",
                target,
                expected
            );
        }
    }
    if helper_spec.primitives.is_empty() {
        bail!(
            "helper service spec at '{}' must declare at least one helper primitive",
            target
        );
    }
    Ok(())
}

fn verify_helper_status_snapshot(
    project_dir: &Path,
    target: &str,
    expected_status: Option<&str>,
) -> Result<()> {
    let status_path = project_dir.join(target);
    let snapshot: HelperStatusSnapshot =
        serde_json::from_str(&fs::read_to_string(&status_path)?)?;
    if let Some(expected) = expected_status {
        if snapshot.status != expected {
            bail!(
                "helper status snapshot at '{}' did not match expected status '{}'",
                target,
                expected
            );
        }
    }
    Ok(())
}

fn verify_helper_summary_snapshot(
    project_dir: &Path,
    target: &str,
    expected_count: Option<&str>,
) -> Result<()> {
    verify_helper_summary_numeric_field(project_dir, target, "observed_file_count", expected_count)
}

fn verify_helper_summary_numeric_field(
    project_dir: &Path,
    target: &str,
    field_name: &str,
    expected_count: Option<&str>,
) -> Result<()> {
    let summary_path = project_dir.join(target);
    let summary: serde_json::Value = serde_json::from_str(&fs::read_to_string(&summary_path)?)?;
    if let Some(expected) = expected_count {
        let observed = summary
            .get(field_name)
            .and_then(|value| value.as_u64())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "helper summary snapshot at '{}' missing {}",
                    target,
                    field_name
                )
            })?;
        let expected_value = expected.parse::<u64>()?;
        if observed != expected_value {
            bail!(
                "helper summary snapshot at '{}' expected {}={} but found {}",
                target,
                field_name,
                expected_value,
                observed
            );
        }
    }
    Ok(())
}

fn verify_helper_summary_observed_lanes(
    project_dir: &Path,
    target: &str,
    expected_lanes: Option<&str>,
) -> Result<()> {
    let summary_path = project_dir.join(target);
    let summary: serde_json::Value = serde_json::from_str(&fs::read_to_string(&summary_path)?)?;
    if let Some(expected) = expected_lanes {
        let mut observed_lanes = summary
            .get("observed_lanes")
            .and_then(|value| value.as_array())
            .map(|lanes| {
                lanes
                    .iter()
                    .filter_map(|lane| lane.as_str().map(str::to_string))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        observed_lanes.sort();
        observed_lanes.dedup();
        let observed = observed_lanes.join(",");
        if observed != expected {
            bail!(
                "helper summary snapshot at '{}' expected observed_lanes='{}' but found '{}'",
                target,
                expected,
                observed
            );
        }
    }
    Ok(())
}

fn verify_helper_summary_lane_rules(
    project_dir: &Path,
    target: &str,
    expected_rules: Option<&str>,
) -> Result<()> {
    let summary_path = project_dir.join(target);
    let summary: serde_json::Value = serde_json::from_str(&fs::read_to_string(&summary_path)?)?;
    if let Some(expected) = expected_rules {
        let lane_rules = summary
            .get("lane_allowed_extensions")
            .and_then(|value| value.as_object())
            .map(|rules| {
                let mut rendered = rules
                    .iter()
                    .map(|(lane, extensions)| {
                        let extension_text = extensions
                            .as_array()
                            .map(|values| {
                                values
                                    .iter()
                                    .filter_map(|value| value.as_str().map(str::to_string))
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                            .join("|");
                        format!("{lane}={extension_text}")
                    })
                    .collect::<Vec<_>>();
                rendered.sort();
                rendered.join(",")
            })
            .unwrap_or_default();
        if lane_rules != expected {
            bail!(
                "helper summary snapshot at '{}' expected lane rules '{}' but found '{}'",
                target,
                expected,
                lane_rules
            );
        }
    }
    Ok(())
}

fn verify_helper_summary_filtered_files(
    project_dir: &Path,
    target: &str,
    expected_files: Option<&str>,
) -> Result<()> {
    let summary_path = project_dir.join(target);
    let summary: serde_json::Value = serde_json::from_str(&fs::read_to_string(&summary_path)?)?;
    if let Some(expected) = expected_files {
        let mut filtered_files = summary
            .get("filtered_out_files")
            .and_then(|value| value.as_array())
            .map(|files| {
                files
                    .iter()
                    .filter_map(|file| file.as_str().map(str::to_string))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        filtered_files.sort();
        filtered_files.dedup();
        let observed = filtered_files.join(",");
        if observed != expected {
            bail!(
                "helper summary snapshot at '{}' expected filtered files '{}' but found '{}'",
                target,
                expected,
                observed
            );
        }
    }
    Ok(())
}

fn verify_helper_summary_surface_contract(
    project_dir: &Path,
    target: &str,
    expected_components: Option<&str>,
) -> Result<()> {
    let index_path = project_dir.join(target);
    let app_js_path = project_dir.join("app.js");
    let index = fs::read_to_string(&index_path)?;
    let app_js = fs::read_to_string(&app_js_path)?;
    let components = expected_components
        .unwrap_or("panel")
        .split(',')
        .map(str::trim)
        .filter(|component| !component.is_empty())
        .collect::<Vec<_>>();

    if components.iter().any(|component| *component == "panel") {
        for needle in [
            "helper-summary-panel",
            "helper-summary-status",
            "helper-summary-files",
        ] {
            if !index.contains(needle) {
                bail!(
                    "helper summary surface contract failed: '{}' missing from {}",
                    needle,
                    target
                );
            }
        }
        for needle in [
            "loadHelperSummary",
            "bridge/helpers/local_inbox/summary.json",
            "helperSummaryFiles",
        ] {
            if !app_js.contains(needle) {
                bail!(
                    "helper summary surface contract failed: '{}' missing from app.js",
                    needle
                );
            }
        }
    }

    let optional_checks = [
        ("status_chip", "helper-summary-status-chip", "helperSummaryStatusChip"),
        (
            "lane_count_chip",
            "helper-summary-lane-count-chip",
            "helperSummaryLaneCountChip",
        ),
        ("types_chip", "helper-summary-types-chip", "helperSummaryTypesChip"),
        ("count_delta", "helper-summary-count-delta", "helperSummaryCountDelta"),
    ];
    for (component, index_needle, script_needle) in optional_checks {
        if components.iter().any(|item| *item == component) {
            if !index.contains(index_needle) {
                bail!(
                    "helper summary surface contract failed: '{}' missing from {}",
                    index_needle,
                    target
                );
            }
            if !app_js.contains(script_needle) {
                bail!(
                    "helper summary surface contract failed: '{}' missing from app.js",
                    script_needle
                );
            }
        }
    }

    Ok(())
}

fn verify_helper_processed_files_surface_contract(
    project_dir: &Path,
    target: &str,
    expected_components: Option<&str>,
) -> Result<()> {
    let index_path = project_dir.join(target);
    let app_js_path = project_dir.join("app.js");
    let index = fs::read_to_string(&index_path)?;
    let app_js = fs::read_to_string(&app_js_path)?;
    let components = expected_components
        .unwrap_or("panel")
        .split(',')
        .map(str::trim)
        .filter(|component| !component.is_empty())
        .collect::<Vec<_>>();

    if components.iter().any(|component| *component == "panel") {
        for needle in [
            "processed-files-panel",
            "processed-files-status",
            "processed-files-list",
        ] {
            if !index.contains(needle) {
                bail!(
                    "helper processed files surface contract failed: '{}' missing from {}",
                    needle,
                    target
                );
            }
        }
        for needle in [
            "loadProcessedFilesPanel",
            "bridge/helpers/local_inbox/summary.json",
            "processedFilesList",
        ] {
            if !app_js.contains(needle) {
                bail!(
                    "helper processed files surface contract failed: '{}' missing from app.js",
                    needle
                );
            }
        }
    }

    if components.iter().any(|component| *component == "selection") {
        for needle in ["data-select-processed-file", "selectedProcessedFile"] {
            if !app_js.contains(needle) {
                bail!(
                    "helper processed files surface contract failed: '{}' missing from app.js",
                    needle
                );
            }
        }
    }

    Ok(())
}

fn verify_helper_processed_preview_surface_contract(
    project_dir: &Path,
    target: &str,
    expected_components: Option<&str>,
) -> Result<()> {
    let index_path = project_dir.join(target);
    let app_js_path = project_dir.join("app.js");
    let index = fs::read_to_string(&index_path)?;
    let app_js = fs::read_to_string(&app_js_path)?;
    let components = expected_components
        .unwrap_or("panel")
        .split(',')
        .map(str::trim)
        .filter(|component| !component.is_empty())
        .collect::<Vec<_>>();

    if components.iter().any(|component| *component == "panel") {
        for needle in [
            "processed-file-preview-panel",
            "processed-file-preview-status",
            "processed-file-preview-body",
        ] {
            if !index.contains(needle) {
                bail!(
                    "helper processed preview surface contract failed: '{}' missing from {}",
                    needle,
                    target
                );
            }
        }
        for needle in [
            "loadProcessedFilePreview",
            "bridge/helpers/local_inbox/summary.json",
            "processedFilePreviewBody",
        ] {
            if !app_js.contains(needle) {
                bail!(
                    "helper processed preview surface contract failed: '{}' missing from app.js",
                    needle
                );
            }
        }
    }

    if components.iter().any(|component| *component == "selection")
        && !app_js.contains("selectedProcessedFile")
    {
        bail!(
            "helper processed preview surface contract failed: 'selectedProcessedFile' missing from app.js"
        );
    }

    Ok(())
}

fn verify_helper_processed_selection_surface_contract(
    project_dir: &Path,
    target: &str,
    _expected: Option<&str>,
) -> Result<()> {
    let app_js_path = project_dir.join(target);
    let styles_path = project_dir.join("styles.css");
    let app_js = fs::read_to_string(&app_js_path)?;
    let styles = fs::read_to_string(&styles_path)?;

    for needle in [
        "let selectedProcessedFile = null;",
        "data-select-processed-file",
        "setSelectedProcessedFile",
        "const fileToPreview = selectedProcessedFile || files[0];",
    ] {
        if !app_js.contains(needle) {
            bail!(
                "helper processed selection surface contract failed: '{}' missing from {}",
                needle,
                target
            );
        }
    }

    if !styles.contains(".processed-files-list li.is-selected") {
        bail!(
            "helper processed selection surface contract failed: '.processed-files-list li.is-selected' missing from styles.css"
        );
    }

    Ok(())
}

fn verify_helper_processed_output_contract(
    project_dir: &Path,
    target: &str,
    _expected: Option<&str>,
) -> Result<()> {
    let summary_path = project_dir.join(target);
    let summary: serde_json::Value = serde_json::from_str(&fs::read_to_string(&summary_path)?)?;
    let observed_files = summary
        .get("observed_files")
        .and_then(|value| value.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if observed_files.is_empty() {
        bail!("helper processed output contract failed: observed_files was empty");
    }

    let observed_count = summary
        .get("observed_file_count")
        .and_then(|value| value.as_u64())
        .unwrap_or_default() as usize;
    if observed_count != observed_files.len() {
        bail!(
            "helper processed output contract failed: observed_file_count={} but observed_files.len()={}",
            observed_count,
            observed_files.len()
        );
    }

    let processed_root = summary_path.parent().map(|parent| parent.join("processed"));
    let Some(processed_root) = processed_root else {
        bail!("helper processed output contract failed: summary path had no parent");
    };
    for relative_path in observed_files {
        let output_path = processed_root.join(&relative_path);
        if !output_path.exists() {
            bail!(
                "helper processed output contract failed: '{}' missing from {}",
                relative_path,
                processed_root.display()
            );
        }
    }

    Ok(())
}

fn verify_static_dashboard_helper_monitoring_surface_contract(
    project_dir: &Path,
    target: &str,
    expected_components: Option<&str>,
) -> Result<()> {
    let index_path = project_dir.join(target);
    let app_js_path = project_dir.join("app.js");
    let index = fs::read_to_string(&index_path)?;
    let app_js = fs::read_to_string(&app_js_path)?;
    let components = expected_components
        .unwrap_or("summary")
        .split(',')
        .map(str::trim)
        .filter(|component| !component.is_empty())
        .collect::<Vec<_>>();

    if components.iter().any(|component| *component == "summary") {
        for needle in [
            "helper-monitor-panel",
            "helper-summary-status",
            "helper-summary-count",
            "helper-summary-lanes",
        ] {
            if !index.contains(needle) {
                bail!(
                    "static dashboard helper monitoring surface contract failed: '{}' missing from {}",
                    needle,
                    target
                );
            }
        }
        for needle in [
            "loadStaticDashboardHelperMonitoring",
            "bridge/helpers/local_inbox/summary.json",
            "helperSummaryStatus",
            "helperSummaryCount",
            "helperSummaryLanes",
        ] {
            if !app_js.contains(needle) {
                bail!(
                    "static dashboard helper monitoring surface contract failed: '{}' missing from app.js",
                    needle
                );
            }
        }
    }

    if components.iter().any(|component| *component == "preview") {
        for needle in [
            "helper-preview-panel",
            "helper-preview-status",
            "helper-preview-body",
        ] {
            if !index.contains(needle) {
                bail!(
                    "static dashboard helper monitoring surface contract failed: '{}' missing from {}",
                    needle,
                    target
                );
            }
        }
        for needle in ["helperPreviewStatus", "helperPreviewBody", "processed/${previewFile}"] {
            if !app_js.contains(needle) {
                bail!(
                    "static dashboard helper monitoring surface contract failed: '{}' missing from app.js",
                    needle
                );
            }
        }
    }

    Ok(())
}

fn parse_visual_load_spec(value: &serde_json::Value) -> Result<ChattyCogVisualLoadSpec> {
    Ok(serde_json::from_value(value.clone())?)
}

fn verify_visual_load_value(
    value: &serde_json::Value,
    spec: &ChattyCogVisualLoadSpec,
) -> Result<()> {
    if value.get("kind").and_then(|item| item.as_str()) != Some(spec.kind.as_str()) {
        bail!("chattycog visual load contract failed: kind mismatch");
    }
    if value
        .get("auto_launch")
        .and_then(|item| item.as_bool())
        != Some(spec.auto_launch)
    {
        bail!("chattycog visual load contract failed: auto_launch mismatch");
    }
    if let Some(title) = &spec.title {
        if value.get("title").and_then(|item| item.as_str()) != Some(title.as_str()) {
            bail!("chattycog visual load contract failed: title mismatch");
        }
    }
    if let Some(file) = &spec.file {
        if value.get("file").and_then(|item| item.as_str()) != Some(file.as_str()) {
            bail!("chattycog visual load contract failed: file mismatch");
        }
    }
    if let Some(url) = &spec.url {
        if value.get("url").and_then(|item| item.as_str()) != Some(url.as_str()) {
            bail!("chattycog visual load contract failed: url mismatch");
        }
    }
    Ok(())
}

fn run_python_command(project_dir: &Path, target: &str) -> Result<String> {
    let mut args = target.split_whitespace().collect::<Vec<_>>();
    if args.is_empty() {
        bail!("python command target was empty");
    }
    let script = args.remove(0);
    let candidates = [
        vec!["py", "-3", script],
        vec!["py", script],
        vec!["python", script],
        vec!["python3", script],
    ];

    for candidate in candidates {
        let mut cmd = Command::new(candidate[0]);
        cmd.args(&candidate[1..])
            .args(&args)
            .current_dir(project_dir);
        match cmd.output() {
            Ok(output) => {
                if !output.status.success() {
                    continue;
                }
                return Ok(String::from_utf8_lossy(&output.stdout).to_string());
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
            Err(err) => return Err(err.into()),
        }
    }

    bail!("unable to execute python command '{}'", target)
}

fn run_cargo_check(project_dir: &Path) -> Result<()> {
    let output = Command::new("cargo")
        .arg("check")
        .current_dir(project_dir)
        .output()?;
    if !output.status.success() {
        bail!(
            "cargo check failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

fn run_cargo_run(project_dir: &Path, target: &str) -> Result<String> {
    let args = if target.trim().is_empty() {
        Vec::new()
    } else {
        target.split_whitespace().collect::<Vec<_>>()
    };
    let output = Command::new("cargo")
        .arg("run")
        .args(args)
        .current_dir(project_dir)
        .output()?;
    if !output.status.success() {
        bail!(
            "cargo run failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
