use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use chatty_factory_control::ControlPlane;
use chatty_factory_core::{
    build_starter_choices, default_request_text, is_known_build_starter_id,
};
use chatty_factory_host::{HostActionResult, HostBridge, HostPlannerOptions};
use chatty_factory_families::built_in_families;
use serde::Serialize;

fn main() -> Result<()> {
    let families = built_in_families();
    let control = ControlPlane::milestone_one();

    let output_root = PathBuf::from("output");
    let runtime_root = PathBuf::from("runtime");
    let host_bridge = HostBridge::new(std::env::current_dir()?);
    fs::create_dir_all(&output_root)?;
    fs::create_dir_all(&runtime_root)?;

    let raw_args = std::env::args().skip(1).collect::<Vec<_>>();
    let (planner_response_path, args) = parse_cli_args(raw_args);
    if matches!(args.first().map(String::as_str), Some("planner-run")) {
        return run_planner_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("runtime-models")) {
        return run_runtime_models_mode(&host_bridge, &runtime_root, &args);
    }
    if matches!(args.first().map(String::as_str), Some("runtime-smoke")) {
        return run_runtime_smoke_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("run-helper")) {
        return run_helper_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("helper-status")) {
        return run_helper_status_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("compare-helper-monitoring")) {
        return run_compare_helper_monitoring_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("run-proof-template")) {
        return run_proof_template_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("run-retry-search-model-proof")
    ) {
        return run_retry_search_model_proof_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("run-cross-family-helper-monitoring-proof")
    ) {
        return run_cross_family_helper_monitoring_proof_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("stop-helper")) {
        return run_stop_helper_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("scaffold-extension")) {
        return run_scaffold_extension_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("register-proof-harness-bundle")
    ) {
        return run_register_proof_harness_bundle_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("pending-extensions")) {
        return run_pending_extensions_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("refresh-proof-harness-registry")
    ) {
        return run_refresh_proof_harness_registry_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("refresh-composition-governance")
    ) {
        return run_refresh_composition_governance_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("refresh-patch-governance")
    ) {
        return run_refresh_patch_governance_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("refresh-project-patch-readiness")
    ) {
        return run_refresh_project_patch_readiness_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("refresh-helper-governance")
    ) {
        return run_refresh_helper_governance_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("refresh-family-governance")
    ) {
        return run_refresh_family_governance_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("refresh-template-governance")
    ) {
        return run_refresh_template_governance_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("refresh-bridge-governance")
    ) {
        return run_refresh_bridge_governance_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("approve-proposed-constraint")
    ) {
        return run_approve_proposed_constraint_mode(&host_bridge, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("deactivate-approved-constraint")
    ) {
        return run_set_approved_constraint_active_mode(&host_bridge, &args, false);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("activate-approved-constraint")
    ) {
        return run_set_approved_constraint_active_mode(&host_bridge, &args, true);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("archive-unmatched-inactive-constraints")
    ) {
        return run_archive_unmatched_inactive_constraints_mode(&host_bridge);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("deactivate-low-value-active-constraints")
    ) {
        return run_deactivate_low_value_active_constraints_mode(&host_bridge);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("restore-approved-constraint")
    ) {
        return run_restore_approved_constraint_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("implement-extension")) {
        return run_implement_extension_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("archive-extension")) {
        return run_archive_extension_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("validate-extension")) {
        return run_validate_extension_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("prepare-extension-promotion")) {
        return run_prepare_extension_promotion_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("prepare-extension-apply-patch")) {
        return run_prepare_extension_apply_patch_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("consume-extension-apply-patch")) {
        return run_consume_extension_apply_patch_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("validate-live-extension")) {
        return run_validate_live_extension_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("selected-project")) {
        return run_selected_project_status_mode(&host_bridge, &runtime_root, &args);
    }
    if matches!(args.first().map(String::as_str), Some("project-browser")) {
        return run_project_browser_mode(&host_bridge, &runtime_root, &args);
    }
    if matches!(args.first().map(String::as_str), Some("select-project")) {
        return run_select_project_mode(&host_bridge, &runtime_root, &args);
    }
    if matches!(
        args.first().map(String::as_str),
        Some("clear-selected-project")
    ) {
        return run_clear_selected_project_mode(&host_bridge, &runtime_root, &args);
    }
    if matches!(args.first().map(String::as_str), Some("reverify-build")) {
        return run_reverify_build_mode(&host_bridge, &args);
    }
    if matches!(args.first().map(String::as_str), Some("patch")) {
        return run_patch_mode(
            &host_bridge,
            &args,
        );
    }
    if matches!(args.first().map(String::as_str), Some("build")) {
        return run_build_mode(&host_bridge, &args);
    }

    let clean_args = strip_runtime_control_args(&args);
    let raw_request = if clean_args.is_empty() {
        default_request_text().to_string()
    } else {
        clean_args.join(" ")
    };
    validate_build_starter_override_arg(build_starter_override_from_args(&args))?;

    let planner_response = planner_response_path.as_deref().map(PathBuf::from);
    let result = if let Some(starter_override_id) = build_starter_override_from_args(&args) {
        host_bridge.build_request_with_starter_override(
            &raw_request,
            Some(starter_override_id),
            &planner_options_from_args(&args),
        )?
    } else {
        host_bridge.smart_request(
            &raw_request,
            planner_response.as_deref(),
            &planner_options_from_args(&args),
        )?
    };
    println!("ChattyFactory rebuild");
    println!("Built-in families: {}", families.len());
    println!("Milestone-one control nodes: {}", control.node_count());
    println!("Milestone-one control edges: {}", control.edge_count());
    print_host_action_result(&result, has_json_flag(&args))?;

    Ok(())
}

fn run_patch_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    if args.len() < 3 {
        anyhow::bail!("patch mode requires: patch <project_name> <request>");
    }
    let project_name = &args[1];
    let clean_args = strip_runtime_control_args(args);
    let raw_request = clean_args[2..].join(" ");
    execute_patch_request(
        host_bridge,
        project_name,
        &raw_request,
        args,
    )
}

fn run_build_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let clean_args = strip_runtime_control_args(args);
    if clean_args.len() < 2 {
        anyhow::bail!("build requires: build [--starter <family_id|auto>] <request>");
    }
    let raw_request = clean_args[1..].join(" ");
    validate_build_starter_override_arg(build_starter_override_from_args(args))?;
    let result = host_bridge.build_request_with_starter_override(
        &raw_request,
        build_starter_override_from_args(args),
        &planner_options_from_args(args),
    )?;
    println!("ChattyFactory deterministic build");
    println!("Request: {}", raw_request);
    if let Some(starter_override_id) = build_starter_override_from_args(args) {
        println!("starter={}", starter_override_id);
    } else {
        println!("starter=auto");
    }
    print_host_action_result(&result, has_json_flag(args))
}

fn run_reverify_build_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("reverify-build requires: reverify-build <project_name>");
    }
    let result = host_bridge.reverify_build_project(&args[1])?;
    print_host_action_result(&result, has_json_flag(args))
}

fn run_approve_proposed_constraint_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!(
            "approve-proposed-constraint requires: approve-proposed-constraint <request_id_or_receipt_path>"
        );
    }
    let result = host_bridge.approve_proposed_constraint(&args[1])?;
    print_host_action_result(&result, has_json_flag(args))
}

fn run_set_approved_constraint_active_mode(
    host_bridge: &HostBridge,
    args: &[String],
    active: bool,
) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!(
            "{} requires: {} <constraint_id>",
            if active {
                "activate-approved-constraint"
            } else {
                "deactivate-approved-constraint"
            },
            if active {
                "activate-approved-constraint"
            } else {
                "deactivate-approved-constraint"
            }
        );
    }
    let result = host_bridge.set_approved_constraint_active(&args[1], active)?;
    print_host_action_result(&result, has_json_flag(args))
}

fn run_select_project_mode(
    host_bridge: &HostBridge,
    runtime_root: &PathBuf,
    args: &[String],
) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("select-project requires: select-project <project_name>");
    }
    let project_name = &args[1];
    let result = host_bridge.select_project(project_name)?;
    if has_json_flag(args) {
        if let Some(browser_state) = result.browser_state {
            return print_json(&browser_state);
        }
    }
    println!("ChattyFactory active project");
    println!("Selected project: {}", project_name);
    println!(
        "Session receipt: {}",
        runtime_root.join("selected_project_session.json").display()
    );
    Ok(())
}

fn run_project_browser_mode(
    host_bridge: &HostBridge,
    runtime_root: &PathBuf,
    args: &[String],
) -> Result<()> {
    let result = host_bridge.refresh_project_browser()?;
    let state = result
        .browser_state
        .ok_or_else(|| anyhow::anyhow!("missing browser state from host bridge"))?;
    if has_json_flag(args) {
        return print_json(&state);
    }
    println!("ChattyFactory project browser");
    println!(
        "Browser state receipt: {}",
        runtime_root.join("project_browser_state.json").display()
    );
    println!("Projects discovered: {}", state.projects.len());
    if let Some(session) = state.selected_project_session {
        println!("Selected project: {}", session.project_name);
    } else {
        println!("Selected project: none");
    }
    if let Some(session) = state.active_project_session {
        println!("Last touched project: {}", session.project_name);
    } else {
        println!("Last touched project: none");
    }
    Ok(())
}

fn run_selected_project_status_mode(
    host_bridge: &HostBridge,
    runtime_root: &PathBuf,
    args: &[String],
) -> Result<()> {
    let result = host_bridge.refresh_project_browser()?;
    let state = result
        .browser_state
        .ok_or_else(|| anyhow::anyhow!("missing browser state from host bridge"))?;
    if has_json_flag(args) {
        return print_json(&state);
    }
    let selected = state.selected_project_session;
    let active = state.active_project_session;

    println!("ChattyFactory project session");
    match selected {
        Some(session) => {
            println!("Selected project: {}", session.project_name);
            println!("Source: {}", session.source_kind);
            println!(
                "Session receipt: {}",
                runtime_root.join("selected_project_session.json").display()
            );
        }
        None => {
            println!("Selected project: none");
        }
    }

    if let Some(session) = active {
        println!("Last touched project: {}", session.project_name);
        println!(
            "Active session receipt: {}",
            runtime_root.join("active_project_session.json").display()
        );
    } else {
        println!("Last touched project: none");
    }

    Ok(())
}

fn run_scaffold_extension_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let integrate = args.iter().any(|arg| arg == "--integrate");
    let promote = args.iter().any(|arg| arg == "--promote");
    let positional = args
        .iter()
        .skip(1)
        .filter(|arg| arg.as_str() != "--integrate" && arg.as_str() != "--promote")
        .cloned()
        .collect::<Vec<_>>();
    if positional.is_empty() {
        anyhow::bail!(
            "scaffold-extension requires: scaffold-extension [--integrate] [--promote] <stub_dir_or_extension_spec_json>"
        );
    }
    let result = host_bridge.scaffold_extension_from_stub(
        &PathBuf::from(&positional[0]),
        integrate,
        promote,
    )?;
    println!("ChattyFactory extension scaffold");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_register_proof_harness_bundle_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    if args.len() < 3 {
        anyhow::bail!(
            "register-proof-harness-bundle requires: register-proof-harness-bundle <template_manifest_json> <comparison_bundle_manifest_json>"
        );
    }
    let result = host_bridge.register_proof_harness_bundle(
        &PathBuf::from(&args[1]),
        &PathBuf::from(&args[2]),
    )?;
    println!("ChattyFactory proof harness bundle registration");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_helper_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("run-helper requires: run-helper <project_name> [helper_id]");
    }
    let helper_id = args.get(2).map(String::as_str);
    let result = host_bridge.run_project_helpers(&args[1], helper_id)?;
    println!("ChattyFactory helper runner");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_helper_status_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("helper-status requires: helper-status <project_name> [helper_id]");
    }
    let helper_id = args.get(2).map(String::as_str);
    let result = host_bridge.helper_status(&args[1], helper_id)?;
    println!("ChattyFactory helper status");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_compare_helper_monitoring_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    if args.len() < 3 {
        anyhow::bail!(
            "compare-helper-monitoring requires: compare-helper-monitoring <left_project_name> <right_project_name>"
        );
    }
    let result = host_bridge.compare_helper_monitoring_projects(&args[1], &args[2])?;
    println!("ChattyFactory helper monitoring comparison");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_cross_family_helper_monitoring_proof_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    let clean_args = strip_runtime_control_args(args);
    let shared_request = if clean_args.len() > 1 {
        clean_args[1..].join(" ")
    } else {
        "build me a helper-backed monitoring surface that watches two inbox lanes, filters module assets to txt, and surfaces processed file status and preview".to_string()
    };
    let result = host_bridge.run_cross_family_helper_monitoring_proof(
        &shared_request,
        &planner_options_from_args(args),
    )?;
    println!("ChattyFactory cross-family paired proof");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_retry_search_model_proof_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    let result =
        host_bridge.run_retry_search_model_escalation_proof(&planner_options_from_args(args))?;
    println!("ChattyFactory retry-search model proof");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_proof_template_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let clean_args = strip_runtime_control_args(args);
    if clean_args.len() < 2 {
        anyhow::bail!("run-proof-template requires: run-proof-template <template_id> [shared request]");
    }
    let template_id = &clean_args[1];
    let shared_request = clean_args
        .get(2..)
        .filter(|parts| !parts.is_empty())
        .map(|parts| parts.join(" "));
    let result = host_bridge.run_proof_template(
        template_id,
        shared_request.as_deref(),
        &planner_options_from_args(args),
    )?;
    println!("ChattyFactory proof template runner");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_stop_helper_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("stop-helper requires: stop-helper <project_name> [helper_id]");
    }
    let helper_id = args.get(2).map(String::as_str);
    let result = host_bridge.stop_project_helpers(&args[1], helper_id)?;
    println!("ChattyFactory helper stop");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_pending_extensions_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let result = host_bridge.pending_extensions()?;
    println!("ChattyFactory pending extensions");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_refresh_proof_harness_registry_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    let result = host_bridge.refresh_proof_harness_registry()?;
    println!("ChattyFactory proof harness refresh");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_refresh_composition_governance_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    let result = host_bridge.refresh_composition_governance_registry()?;
    println!("ChattyFactory composition governance refresh");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_refresh_patch_governance_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let result = host_bridge.refresh_patch_governance_registry()?;
    println!("ChattyFactory patch governance refresh");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_refresh_project_patch_readiness_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    let result = host_bridge.refresh_project_patch_readiness_registry()?;
    println!("ChattyFactory project patch readiness refresh");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_refresh_helper_governance_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let result = host_bridge.refresh_helper_governance_registry()?;
    println!("ChattyFactory helper governance refresh");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_refresh_family_governance_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let result = host_bridge.refresh_family_governance_registry()?;
    println!("ChattyFactory family governance refresh");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_refresh_template_governance_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let result = host_bridge.refresh_template_governance_registry()?;
    println!("ChattyFactory template governance refresh");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_refresh_bridge_governance_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let result = host_bridge.refresh_bridge_governance_registry()?;
    println!("ChattyFactory bridge governance refresh");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_implement_extension_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("implement-extension requires: implement-extension <entry_id>");
    }
    let result = host_bridge.mark_pending_extension_implemented(&args[1])?;
    println!("ChattyFactory pending extension");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_archive_extension_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("archive-extension requires: archive-extension <entry_id> [reason]");
    }
    let reason = if args.len() > 2 {
        Some(args[2..].join(" "))
    } else {
        None
    };
    let result = host_bridge.archive_extension(&args[1], reason.as_deref())?;
    println!("ChattyFactory extension archive");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_validate_extension_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("validate-extension requires: validate-extension <entry_id>");
    }
    let result = host_bridge.validate_extension(&args[1])?;
    println!("ChattyFactory extension validation");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_prepare_extension_promotion_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!(
            "prepare-extension-promotion requires: prepare-extension-promotion <entry_id>"
        );
    }
    let result = host_bridge.prepare_extension_promotion(&args[1])?;
    println!("ChattyFactory extension promotion");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_prepare_extension_apply_patch_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!(
            "prepare-extension-apply-patch requires: prepare-extension-apply-patch <entry_id>"
        );
    }
    let result = host_bridge.prepare_extension_apply_patch(&args[1])?;
    println!("ChattyFactory extension apply_patch");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_consume_extension_apply_patch_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!(
            "consume-extension-apply-patch requires: consume-extension-apply-patch <entry_id>"
        );
    }
    let result = host_bridge.consume_extension_apply_patch(&args[1])?;
    println!("ChattyFactory extension host wiring");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_validate_live_extension_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!("validate-live-extension requires: validate-live-extension <entry_id>");
    }
    let result = host_bridge.validate_live_extension(&args[1])?;
    println!("ChattyFactory extension live validation");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_clear_selected_project_mode(
    host_bridge: &HostBridge,
    runtime_root: &PathBuf,
    args: &[String],
) -> Result<()> {
    let browser_state = host_bridge
        .clear_selected_project()?
        .browser_state
        .ok_or_else(|| anyhow::anyhow!("missing browser state from host bridge"))?;
    if has_json_flag(args) {
        return print_json(&browser_state);
    }

    println!("ChattyFactory project session");
    println!("Selected project cleared");
    println!(
        "Session receipt: {}",
        runtime_root.join("selected_project_session.json").display()
    );
    Ok(())
}

fn execute_patch_request(
    host_bridge: &HostBridge,
    project_name: &str,
    raw_request: &str,
    args: &[String],
) -> Result<()> {
    let result = host_bridge.patch_request(
        project_name,
        raw_request,
        &planner_options_from_args(args),
    )?;
    println!("ChattyFactory deterministic patch");
    println!("Project: {}", project_name);
    println!("Request: {}", raw_request);
    print_host_action_result(&result, has_json_flag(args))?;
    Ok(())
}

fn run_archive_unmatched_inactive_constraints_mode(host_bridge: &HostBridge) -> Result<()> {
    let result = host_bridge.archive_unmatched_inactive_constraints()?;
    println!("ChattyFactory constraint shelf archive");
    print_host_action_result(&result, false)?;
    Ok(())
}

fn run_deactivate_low_value_active_constraints_mode(host_bridge: &HostBridge) -> Result<()> {
    let result = host_bridge.deactivate_low_value_active_constraints()?;
    println!("ChattyFactory constraint shelf deactivate");
    print_host_action_result(&result, false)?;
    Ok(())
}

fn run_restore_approved_constraint_mode(
    host_bridge: &HostBridge,
    args: &[String],
) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!(
            "restore-approved-constraint requires: restore-approved-constraint <constraint_id>"
        );
    }
    let result = host_bridge.restore_constraint_from_history(&args[1])?;
    println!("ChattyFactory constraint shelf restore");
    print_host_action_result(&result, has_json_flag(args))?;
    Ok(())
}

fn parse_cli_args(args: Vec<String>) -> (Option<String>, Vec<String>) {
    let mut planner_response_path = None;
    let mut remaining = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == "--planner-response" && i + 1 < args.len() {
            planner_response_path = Some(args[i + 1].clone());
            i += 2;
        } else if args[i] == "--starter" && i + 1 < args.len() {
            remaining.push(args[i].clone());
            remaining.push(args[i + 1].clone());
            i += 2;
        } else {
            remaining.push(args[i].clone());
            i += 1;
        }
    }
    (planner_response_path, remaining)
}

fn strip_runtime_control_args(args: &[String]) -> Vec<String> {
    let mut remaining = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--auto-planner" | "--skip-launch" | "--json" => {
                i += 1;
            }
            "--model" | "--port" | "--starter" => {
                i += if i + 1 < args.len() { 2 } else { 1 };
            }
            _ => {
                remaining.push(args[i].clone());
                i += 1;
            }
        }
    }
    remaining
}

fn planner_options_from_args(args: &[String]) -> HostPlannerOptions {
    let (requested_model, requested_port, _) = parse_runtime_args(args);
    HostPlannerOptions {
        auto_planner: args.iter().any(|arg| arg == "--auto-planner"),
        requested_model,
        requested_port,
    }
}

fn has_json_flag(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--json")
}

fn build_starter_override_from_args(args: &[String]) -> Option<&str> {
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == "--starter" {
            return args.get(i + 1).map(String::as_str).filter(|value| !value.is_empty());
        }
        i += 1;
    }
    None
}

fn validate_build_starter_override_arg(starter_override_id: Option<&str>) -> Result<()> {
    if let Some(starter_override_id) = starter_override_id {
        if !is_known_build_starter_id(starter_override_id) {
            let known = build_starter_choices()
                .iter()
                .map(|choice| choice.id)
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::bail!(
                "unknown build starter '{}'; known starters: {}",
                starter_override_id,
                known
            );
        }
    }
    Ok(())
}

fn print_json<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn print_host_action_result(result: &HostActionResult, as_json: bool) -> Result<()> {
    if as_json {
        return print_json(result);
    }

    println!("{}", result.summary);
    for detail in &result.details {
        println!("{}", detail);
    }
    if let Some(execution) = &result.execution_result {
        println!("kind={}", execution.kind);
        println!("request_id={}", execution.request_id);
        println!("project={}", execution.project_name);
        if let Some(starter_override_id) = &execution.starter_override_id {
            println!("starter_override_id={starter_override_id}");
        }
        if let Some(starter_override_summary) = &execution.starter_override_summary {
            println!("starter_override_summary={starter_override_summary}");
        }
        if let Some(family_id) = &execution.family_id {
            println!("family={family_id}");
        }
        if let Some(tool_kind) = &execution.tool_kind {
            println!("tool={tool_kind}");
        }
        if let Some(patch_kind) = &execution.patch_kind {
            println!("patch_kind={patch_kind}");
        }
        if let Some(route_class) = &execution.composition_route_class {
            println!("composition_route_class={route_class}");
        }
        if let Some(path) = &execution.composable_route_plan_path {
            println!("composable_route_plan={path}");
        }
        if let Some(path) = &execution.composition_review_receipt_path {
            println!("composition_review_receipt={path}");
        }
        if let Some(request_mode) = &execution.followup_request_mode {
            println!("followup_request_mode={request_mode}");
        }
        println!(
            "plan_confidence={} ({})",
            execution.plan_confidence_score, execution.plan_confidence_band
        );
        println!("llm_review={}", execution.needs_llm_review);
        if let Some(status) = &execution.acceptance_status {
            println!("acceptance={status}");
        }
        for note in &execution.route_notes {
            println!("route_note={note}");
        }
        for note in &execution.followup_rationale {
            println!("followup_note={note}");
        }
        for lane in &execution.patch_lanes {
            println!(
                "patch_lane={} status={} mode={} maturity={} readiness={} reason={}",
                lane.patch_kind,
                lane.availability_status,
                lane.dependency_mode,
                lane.surgical_maturity,
                lane.effective_preflight_readiness,
                lane.preflight_readiness_reason
            );
        }
        for recipe in &execution.acceptance_recipes {
            println!(
                "acceptance_recipe={} status={} command={}",
                recipe.recipe_id, recipe.availability_status, recipe.command_id
            );
        }
        for bundle in &execution.operator_bundles {
            println!(
                "operator_bundle={} status={} operators={}",
                bundle.bundle_id,
                bundle.availability_status,
                bundle.operator_ids.join(",")
            );
        }
        if let Some(mode) = &execution.chattycog_hosting_mode {
            println!("chattycog_hosting_mode={mode}");
        }
        if let Some(owner) = &execution.chattycog_ui_owner {
            println!("chattycog_ui_owner={owner}");
        }
        if let Some(bridge) = &execution.chattycog_bridge_capabilities {
            println!(
                "chattycog_bridge=status:{} log_sources:{} shared_room_state:{} shared_room_events:{} outgoing_room_events:{} inbox_lanes={}",
                bridge.status_enabled,
                bridge.log_sources_enabled,
                bridge.shared_room_state_enabled,
                bridge.shared_room_events_enabled,
                bridge.outgoing_room_events_enabled,
                bridge.incoming_asset_lanes.join(",")
            );
        }
        for helper in &execution.helper_services {
            println!(
                "helper_service={} kind={} entrypoint={}",
                helper.helper_id, helper.helper_kind, helper.entrypoint
            );
        }
        for receipt in &execution.helper_runtime_receipts {
            println!(
                "helper_runtime={} status={} outputs={}",
                receipt.helper_id,
                receipt.launch_status,
                receipt.observed_output_files.join(",")
            );
        }
        println!("files={}", execution.file_paths.len());
    } else if let Some(fallback) = &result.fallback_result {
        println!("fallback_request_id={}", fallback.request_id);
        if let Some(mode) = &fallback.mode {
            println!("fallback_mode={mode}");
        }
        if let Some(route_class) = &fallback.composition_route_class {
            println!("fallback_composition_route_class={route_class}");
        }
        if let Some(path) = &fallback.composable_route_plan_path {
            println!("fallback_composable_route_plan={path}");
        }
        if let Some(path) = &fallback.composition_review_receipt_path {
            println!("fallback_composition_review_receipt={path}");
        }
        println!("fallback_goal={}", fallback.interpreted_goal);
        println!("fallback_question={}", fallback.question);
        println!("fallback_next_step={}", fallback.recommended_next_step);
        if let Some(class) = &fallback.build_failure_class {
            println!("fallback_build_failure_class={class}");
        }
        if let Some(mode) = &fallback.build_failure_mode {
            println!("fallback_build_failure_mode={mode}");
        }
        for constraint_id in &fallback.matched_approved_constraint_ids {
            println!("fallback_approved_constraint_id={constraint_id}");
        }
        for summary in &fallback.matched_approved_constraint_summaries {
            println!("fallback_approved_constraint_summary={summary}");
        }
        for reason in &fallback.reasons {
            println!("fallback_reason={reason}");
        }
        for family_id in &fallback.candidate_family_ids {
            println!("fallback_candidate_family={family_id}");
        }
        for capability in &fallback.requested_capabilities {
            println!("fallback_capability={capability}");
        }
        println!(
            "fallback_extension_kind={}",
            fallback.suggested_extension_kind
        );
        if let Some(family_id) = &fallback.suggested_family_id {
            println!("fallback_suggested_family={family_id}");
        }
        if let Some(tool_kind) = &fallback.suggested_tool_kind {
            println!("fallback_suggested_tool_kind={tool_kind}");
        }
        if let Some(patch_kind) = &fallback.suggested_patch_kind {
            println!("fallback_suggested_patch_kind={patch_kind}");
        }
        if let Some(mode) = &fallback.suggested_hosting_mode {
            println!("fallback_suggested_hosting_mode={mode}");
        }
        for capability in &fallback.suggested_bridge_capabilities {
            println!("fallback_suggested_bridge_capability={capability}");
        }
        for artifact in &fallback.suggested_artifacts {
            println!("fallback_suggested_artifact={artifact}");
        }
        for class in &fallback.missing_family_build_primitive_classes {
            println!("fallback_missing_family_build_class={class}");
        }
        for class in &fallback.missing_patch_primitive_classes {
            println!("fallback_missing_patch_class={class}");
        }
        for kind in &fallback.missing_helper_primitive_kinds {
            println!("fallback_missing_helper_kind={kind}");
        }
        for target in &fallback.acceptance_targets {
            println!("fallback_acceptance_target={target}");
        }
        for note in &fallback.implementation_notes {
            println!("fallback_implementation_note={note}");
        }
        for entry_id in &fallback.pending_extension_ids {
            println!("fallback_pending_extension_id={entry_id}");
        }
        for root in &fallback.pending_extension_scaffold_roots {
            println!("fallback_pending_extension_scaffold={root}");
        }
        if let Some(mode) = &fallback.chattycog_requested_hosting_mode {
            println!("fallback_chattycog_requested_hosting_mode={mode}");
        }
        for mode in &fallback.chattycog_valid_hosting_modes {
            println!("fallback_chattycog_valid_hosting_mode={mode}");
        }
        for capability in &fallback.chattycog_requested_bridge_capabilities {
            println!("fallback_chattycog_requested_bridge_capability={capability}");
        }
        for capability in &fallback.chattycog_supported_bridge_capabilities {
            println!("fallback_chattycog_supported_bridge_capability={capability}");
        }
        if let Some(summary) = &fallback.proposed_constraint_summary {
            println!("fallback_proposed_constraint_summary={summary}");
        }
        if let Some(guidance) = &fallback.proposed_constraint_replacement_guidance {
            println!("fallback_proposed_constraint_guidance={guidance}");
        }
        if let Some(path) = &fallback.build_verification_path {
            println!("fallback_build_verification={path}");
        }
        if let Some(path) = &fallback.proposed_constraint_path {
            println!("fallback_proposed_constraint={path}");
        }
        if let Some(path) = &fallback.stub_bundle_path {
            println!("fallback_stub_bundle={path}");
        }
    } else if let Some(runtime_refresh) = &result.runtime_refresh {
        if let Some(config) = &runtime_refresh.config {
            println!("host={}:{}", config.host, config.port);
            println!("gpu_layers={}", config.gpu_layers);
        }
        if let Some(catalog) = &runtime_refresh.catalog {
            println!("models={}", catalog.models.len());
        }
    } else if let Some(browser_state) = &result.browser_state {
        println!("projects={}", browser_state.projects.len());
        if let Some(selected) = &browser_state.selected_project_session {
            println!("selected={}", selected.project_name);
        }
        if let Some(active) = &browser_state.active_project_session {
            println!("active={}", active.project_name);
        }
    }
    Ok(())
}

fn run_runtime_smoke_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    let (_, _, skip_launch) = parse_runtime_args(args);
    let result = host_bridge.runtime_smoke(&planner_options_from_args(args), skip_launch)?;
    println!("ChattyFactory runtime");
    print_host_action_result(&result, has_json_flag(args))
}

fn run_runtime_models_mode(
    host_bridge: &HostBridge,
    _runtime_root: &PathBuf,
    args: &[String],
) -> Result<()> {
    let result = host_bridge.refresh_runtime(&planner_options_from_args(args))?;
    if has_json_flag(args) {
        return print_json(&result);
    }
    println!("ChattyFactory runtime model catalog");
    print_host_action_result(&result, false)
}

fn run_planner_mode(host_bridge: &HostBridge, args: &[String]) -> Result<()> {
    if args.len() < 2 {
        anyhow::bail!(
            "planner-run requires: planner-run <handoff_json> [--model <path>] [--port <port>]"
        );
    }
    let handoff_path = PathBuf::from(&args[1]);
    let result = host_bridge.planner_run(&handoff_path, &planner_options_from_args(args))?;
    println!("ChattyFactory planner");
    print_host_action_result(&result, has_json_flag(args))
}

fn parse_runtime_args(args: &[String]) -> (Option<String>, Option<u16>, bool) {
    let mut model = None;
    let mut port = None;
    let mut skip_launch = false;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--model" if i + 1 < args.len() => {
                model = Some(args[i + 1].clone());
                i += 2;
            }
            "--port" if i + 1 < args.len() => {
                port = args[i + 1].parse::<u16>().ok();
                i += 2;
            }
            "--skip-launch" => {
                skip_launch = true;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    (model, port, skip_launch)
}
