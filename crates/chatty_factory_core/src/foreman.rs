use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use serde::Serialize;

use crate::{
    chattycog_valid_hosting_modes, contains_any, expand_operator_bundle_ids,
    infer_capabilities_from_text, infer_chattycog_bridge_capabilities_from_text,
    infer_chattycog_hosting_mode_from_text, infer_chattycog_hosting_modes_from_text,
    infer_cli_tool_kind_from_text, infer_explicit_stack_from_text, infer_patch_kind_from_text,
    infer_request_tool_kind_from_text, is_supported_explicit_stack,
    request_has_cli_shape, request_has_dashboard_shape, request_has_vague_improvement,
    request_has_web_shape, request_mentions_chattycog, request_mentions_chattyedu,
    request_mentions_python, request_mentions_rust, supported_chattycog_bridge_capabilities,
    DesiredSurface, ExoskeletonTarget, OperatorId, PlannerHandoff, PlannerResponse,
    BuildSeedInputs, ProjectSpec, RequestMode, RequestPlan, RequestRecord, RouteDecision,
    SubstrateKind, WrapperId,
};

pub fn default_request_text() -> &'static str {
    "build me a simple local dashboard"
}

pub fn normalize_patch_request(project_name: &str, raw_request: &str) -> RequestRecord {
    let mut record = normalize_request(raw_request);
    record.mode = Some(RequestMode::Patch);
    record.active_project = Some(project_name.to_string());
    record
}

pub fn timestamp_id(prefix: &str) -> String {
    static ID_COUNTER: AtomicU64 = AtomicU64::new(0);
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let counter = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}-{millis}-{counter}")
}

pub fn normalize_request(raw_request: &str) -> RequestRecord {
    let raw = raw_request.trim();
    let lower = raw.to_ascii_lowercase();
    let explicit_stack = infer_explicit_stack_from_text(&lower);

    let wants_chattyedu = request_mentions_chattyedu(&lower);
    let wants_chattycog = request_mentions_chattycog(&lower);
    let wants_dual_host = wants_chattycog
        && wants_chattyedu
        && contains_any(
            &lower,
            &[
                "both",
                "either",
                "dual host",
                "dual-host",
                "both hosts",
                "either host",
                "both exoskeletons",
                "either exoskeleton",
                "chatty-cog and chatty-edu",
                "chattycog and chattyedu",
            ],
        );
    let wants_rust = request_mentions_rust(&lower);
    let wants_python = request_mentions_python(&lower);
    let requested_chattycog_modes = infer_chattycog_hosting_modes_from_text(&lower);

    let requested_capabilities = infer_capabilities_from_text(&lower);
    let unsupported_stack = explicit_stack
        .as_deref()
        .map(|stack| !is_supported_explicit_stack(stack))
        .unwrap_or(false);
    let helper_or_service_heavy = requested_capabilities.iter().any(|cap| {
        matches!(
            cap.as_str(),
            "backend_service"
                | "websocket"
                | "api_server"
                | "daemon"
                | "background_job"
                | "email_delivery"
        )
    });

    let desired_surface = if wants_python || wants_rust {
        Some(DesiredSurface::Cli)
    } else if request_has_web_shape(&lower) {
        Some(DesiredSurface::Web)
    } else {
        Some(DesiredSurface::Unknown)
    };

    let exoskeleton_target = if wants_chattycog && !wants_dual_host {
        Some(ExoskeletonTarget::ChattyCog)
    } else {
        Some(ExoskeletonTarget::None)
    };

    let mut ambiguity_flags = Vec::new();
    if matches!(desired_surface, Some(DesiredSurface::Unknown)) {
        ambiguity_flags.push("surface_unclear".into());
    }
    if let Some(stack) = explicit_stack.as_deref() {
        if !is_supported_explicit_stack(stack) {
            ambiguity_flags.push(format!("unsupported_explicit_stack:{stack}"));
        }
    }
    if helper_or_service_heavy {
        ambiguity_flags.push("request_requires_helper_or_service_lane".into());
    }
    if wants_chattycog && requested_chattycog_modes.len() > 1 {
        ambiguity_flags.push(format!(
            "chattycog_hosting_mode_conflict:{}",
            requested_chattycog_modes.join(",")
        ));
    }
    let inferred_tool_kind_hint = infer_request_tool_kind_from_text(
        raw,
        wants_chattycog,
        wants_chattyedu,
        matches!(desired_surface, Some(DesiredSurface::Cli)),
    )
    .map(str::to_string);
    let candidate_substrate_kinds = infer_request_substrate_kinds(
        raw,
        desired_surface.as_ref(),
        inferred_tool_kind_hint.as_deref(),
        wants_chattycog,
        wants_chattyedu,
        wants_dual_host,
        helper_or_service_heavy,
        unsupported_stack,
    );
    RequestRecord {
        request_id: timestamp_id("request"),
        raw_request: raw.to_string(),
        mode: Some(RequestMode::NewBuild),
        active_project: None,
        explicit_stack,
        desired_surface,
        requested_capabilities,
        exoskeleton_target,
        candidate_substrate_kinds,
        ambiguity_flags,
        created_at: Some(timestamp_id("created")),
    }
}

pub fn derive_build_seed_inputs(
    request: &RequestRecord,
    plan: &RequestPlan,
    route: &RouteDecision,
) -> BuildSeedInputs {
    let substrate_label = route
        .selected_substrate_kind
        .as_ref()
        .map(SubstrateKind::as_str)
        .unwrap_or("build");
    let tool_kind = plan.inferred_tool_kind.as_deref();

    let project_name = sanitize_name(
        match (route.selected_substrate_kind.as_ref(), tool_kind) {
            (Some(SubstrateKind::Cli), Some("python" | "csv_report")) => "python_cli_tool",
            (Some(SubstrateKind::Cli), Some("rust" | "log_summary")) => "rust_cli_tool",
            (Some(SubstrateKind::Cli), _) => "cli_tool",
            (Some(SubstrateKind::StaticWeb), _) => "static_web",
            (Some(SubstrateKind::Webview), _) => "webview",
            (Some(SubstrateKind::NativeWindow), _) => "native_window",
            (Some(SubstrateKind::Workspace), _) => "workspace",
            (None, _) => "build",
        },
        &request.raw_request,
    );

    let title = request_title(&request.raw_request, substrate_label);
    let summary = request_summary(&request.raw_request, substrate_label);

    BuildSeedInputs {
        project_name,
        title,
        summary,
        copy_bundle: route
            .selected_operator_ids
            .iter()
            .map(|op| op.0.clone())
            .collect(),
        feature_tokens: infer_feature_tokens(&request.raw_request),
        style_preset: Some("factory_default".into()),
        wrapper_target: request.exoskeleton_target.clone(),
        entrypoint_config: infer_entrypoint_config(request, plan.inferred_tool_kind.as_deref()),
        fixture_config: infer_fixture_config(request, plan.inferred_tool_kind.as_deref()),
    }
}

pub fn apply_planner_response(plan: &RequestPlan, response: &PlannerResponse) -> RequestPlan {
    let mut next = plan.clone();
    if let Some(substrate_kind) = &response.recommended_substrate_kind {
        next.inferred_substrate_candidates = vec![substrate_kind.clone()];
    }
    if let Some(tool_kind) = &response.recommended_tool_kind {
        next.inferred_tool_kind = Some(tool_kind.clone());
    }
    if let Some(patch_kind) = &response.recommended_patch_kind {
        next.intended_patch_kind = Some(patch_kind.clone());
    }
    next.planner_patch_recipe_ids
        .extend(response.recommended_patch_recipe_ids.clone());
    next.planner_suggested_patch_kinds
        .extend(response.recommended_composition_patch_kinds.clone());
    next.planner_operator_bundle_ids
        .extend(response.recommended_operator_bundle_ids.clone());
    next.planner_operator_ids
        .extend(response.recommended_operator_ids.clone());
    next.planner_operator_ids.extend(expand_operator_bundle_ids(
        &response.recommended_operator_bundle_ids,
    ));
    next.planner_acceptance_recipe_ids
        .extend(response.recommended_acceptance_recipe_ids.clone());
    next.rationale.extend(response.rationale.clone());
    next.execution_steps = response.execution_steps.clone();
    next.planner_acceptance_checks
        .extend(response.acceptance_checks_to_add.clone());
    next.planner_required_markers
        .extend(response.required_markers_to_add.clone());
    next.planner_acceptance_commands
        .extend(response.acceptance_commands_to_add.clone());
    next.planner_expected_outputs
        .extend(response.expected_outputs_to_add.clone());
    next.planner_suggested_patch_kinds
        .extend(response.suggested_patch_kinds.clone());
    next.planner_suggested_features
        .extend(response.suggested_features.clone());
    next.planner_patch_recipe_ids.sort();
    next.planner_patch_recipe_ids.dedup();
    next.planner_operator_bundle_ids.sort();
    next.planner_operator_bundle_ids.dedup();
    next.planner_operator_ids.sort();
    next.planner_operator_ids.dedup();
    next.planner_acceptance_recipe_ids.sort();
    next.planner_acceptance_recipe_ids.dedup();
    next.planner_acceptance_checks
        .sort_by(|left, right| left.check_id.cmp(&right.check_id));
    next.planner_acceptance_checks
        .dedup_by(|left, right| left.check_id == right.check_id);
    next.planner_required_markers.sort();
    next.planner_required_markers.dedup();
    next.planner_acceptance_commands.sort();
    next.planner_acceptance_commands.dedup();
    next.planner_expected_outputs.sort();
    next.planner_expected_outputs.dedup();
    next.planner_suggested_patch_kinds.sort();
    next.planner_suggested_patch_kinds.dedup();
    next.planner_suggested_features.sort();
    next.planner_suggested_features.dedup();
    next.confidence_score = 95;
    next.confidence_band = "planner_override".into();
    next.escalation_reasons.clear();
    next.needs_llm_review = false;
    next
}

pub fn derive_request_plan(
    request: &RequestRecord,
    active_spec: Option<&ProjectSpec>,
) -> RequestPlan {
    let inferred_tool_kind = if matches!(request.mode, Some(RequestMode::Patch)) {
        active_spec
            .and_then(|spec| spec.tool_kind.clone())
            .or_else(|| infer_tool_kind(request))
    } else {
        infer_tool_kind(request)
    };
    let intended_patch_kind =
        active_spec.and_then(|spec| infer_patch_kind(&request.raw_request, spec));
    let lower = request.raw_request.to_ascii_lowercase();
    let wants_chattycog = matches!(
        request.exoskeleton_target,
        Some(ExoskeletonTarget::ChattyCog)
    ) || request_mentions_chattycog(&lower);
    let wants_chattyedu = request_mentions_chattyedu(&lower);
    let wants_dual_host = wants_chattycog && wants_chattyedu;
    let helper_or_service_heavy = request.requested_capabilities.iter().any(|cap| {
        matches!(
            cap.as_str(),
            "backend_service"
                | "websocket"
                | "api_server"
                | "daemon"
                | "background_job"
                | "email_delivery"
        )
    });
    let unsupported_stack = request
        .explicit_stack
        .as_deref()
        .map(|stack| !is_supported_explicit_stack(stack))
        .unwrap_or(false);
    let exact_substrate_candidates = if matches!(request.mode, Some(RequestMode::Patch)) {
        active_spec
            .and_then(project_spec_substrate_kind_for_planning)
            .map(|substrate_kind| vec![substrate_kind])
            .unwrap_or_else(|| request.candidate_substrate_kinds.clone())
    } else {
        request.candidate_substrate_kinds.clone()
    };
    let used_evidence_driven_substrate_recovery =
        !matches!(request.mode, Some(RequestMode::Patch)) && exact_substrate_candidates.is_empty();
    let inferred_substrate_candidates = if used_evidence_driven_substrate_recovery {
        infer_request_substrate_kinds(
            &request.raw_request,
            request.desired_surface.as_ref(),
            inferred_tool_kind.as_deref(),
            wants_chattycog,
            wants_chattyedu,
            wants_dual_host,
            helper_or_service_heavy,
            unsupported_stack,
        )
    } else {
        exact_substrate_candidates
    };
    let available_patch_kinds = active_spec
        .map(|spec| spec.supported_patch_kinds.clone())
        .unwrap_or_default();

    let mut execution_steps = Vec::new();
    let mut constraints = Vec::new();
    let mut rationale = Vec::new();

    if matches!(request.mode, Some(RequestMode::Patch)) {
        execution_steps.push("inspect active project contract and accepted patch lanes".into());
        execution_steps.push("select deterministic patch lane when a strong match exists".into());
        execution_steps.push("apply host-owned patch and rerun acceptance".into());
        if !available_patch_kinds.is_empty() {
            rationale.push(format!(
                "active project advertises patch lanes: {}",
                available_patch_kinds.join(", ")
            ));
        }
    } else {
        execution_steps.push("normalize request into native build intent".into());
        execution_steps.push("infer the smallest bounded substrate that can be executed safely".into());
        execution_steps.push("render deterministic build seed, fixtures, and contracts".into());
        execution_steps.push("run substrate acceptance before greenlighting the build".into());
    }

    if let Some(tool_kind) = inferred_tool_kind.as_deref() {
        rationale.push(format!("inferred tool kind: {tool_kind}"));
    }
    if !inferred_substrate_candidates.is_empty() {
        rationale.push(format!(
            "bounded substrate candidates: {}",
            inferred_substrate_candidates
                .iter()
                .map(SubstrateKind::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if used_evidence_driven_substrate_recovery {
        rationale.push(format!(
            "no exact deterministic substrate matched from the request record; using next-attempt substrate candidates derived from evidence: {}",
            inferred_substrate_candidates
                .iter()
                .map(SubstrateKind::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    let requested_chattycog_modes = infer_chattycog_hosting_modes_from_text(&lower);
    let requested_bridge_capabilities = infer_chattycog_bridge_capabilities_from_text(&lower);
    if let Some(mode) = infer_chattycog_hosting_mode_from_text(&lower) {
        rationale.push(format!("requested ChattyCog hosting mode: {mode}"));
    }
    if !requested_bridge_capabilities.is_empty() {
        rationale.push(format!(
            "requested ChattyCog bridge capabilities: {}",
            requested_bridge_capabilities.join(", ")
        ));
    }
    if let Some(patch_kind) = intended_patch_kind.as_deref() {
        rationale.push(format!("matched patch intent: {patch_kind}"));
    }
    if matches!(request.desired_surface, Some(DesiredSurface::Unknown))
        && !matches!(request.mode, Some(RequestMode::Patch))
    {
        constraints.push("surface is not explicit in the request".into());
    }
    if let Some(stack) = request.explicit_stack.as_deref() {
        if !is_supported_explicit_stack(stack) {
            constraints.push(format!(
                "explicit stack `{stack}` is not supported by a bounded deterministic substrate yet"
            ));
        }
    }
    if request.requested_capabilities.iter().any(|cap| {
        matches!(
            cap.as_str(),
            "backend_service" | "websocket" | "api_server" | "daemon"
        )
        }) {
        constraints.push(
            "request asks for service/backend capabilities that do not have a bounded deterministic substrate yet"
                .into(),
        );
    }
    if matches!(
        request.exoskeleton_target,
        Some(ExoskeletonTarget::ChattyCog)
    ) {
        if requested_chattycog_modes.len() > 1 {
            constraints.push(format!(
                "request mixes multiple ChattyCog hosting modes: {}",
                requested_chattycog_modes.join(", ")
            ));
            constraints.push(format!(
                "choose one ChattyCog hosting mode: {}",
                chattycog_valid_hosting_modes().join(", ")
            ));
        }
        if request.requested_capabilities.iter().any(|cap| {
            matches!(
                cap.as_str(),
                "backend_service"
                    | "websocket"
                    | "api_server"
                    | "daemon"
                    | "background_job"
                    | "email_delivery"
            )
        }) {
            constraints.push(
                "requested ChattyCog module shape exceeds the current deterministic hosting contract"
                    .into(),
            );
        }
        let supported_bridge_capabilities = supported_chattycog_bridge_capabilities();
        let unsupported_bridge_capabilities = requested_bridge_capabilities
            .iter()
            .filter(|cap| {
                !supported_bridge_capabilities
                    .iter()
                    .any(|item| item == *cap)
            })
            .cloned()
            .collect::<Vec<_>>();
        if !unsupported_bridge_capabilities.is_empty() {
            constraints.push(format!(
                "requested ChattyCog bridge capabilities are not implemented by the current bounded substrate adapters: {}",
                unsupported_bridge_capabilities.join(", ")
            ));
            constraints.push(format!(
                "currently supported ChattyCog bridge capabilities: {}",
                supported_bridge_capabilities.join(", ")
            ));
        }
    }
    if inferred_substrate_candidates.is_empty() {
        constraints.push("no bounded substrate candidates were inferred".into());
    }
    if !request.ambiguity_flags.is_empty() && !matches!(request.mode, Some(RequestMode::Patch)) {
        constraints.extend(request.ambiguity_flags.clone());
    }
    let (mut confidence_score, mut confidence_band, escalation_reasons) = compute_plan_confidence(
        request,
        &inferred_substrate_candidates,
        inferred_tool_kind.as_deref(),
        intended_patch_kind.as_deref(),
        &available_patch_kinds,
        &constraints,
    );
    if used_evidence_driven_substrate_recovery
        && inferred_tool_kind.is_some()
        && !inferred_substrate_candidates.is_empty()
    {
        confidence_score = confidence_score.max(60);
        if confidence_score >= 60 && confidence_band == "low" {
            confidence_band = "medium".into();
        }
        rationale.push(
            "next-attempt substrate pass remained admissible despite the lack of an exact deterministic substrate"
                .into(),
        );
    }
    let needs_llm_review = confidence_score < 60;

    RequestPlan {
        plan_id: timestamp_id("plan"),
        request_id: request.request_id.clone(),
        mode: request.mode.clone(),
        interpreted_goal: interpreted_goal(
            request,
            inferred_tool_kind.as_deref(),
            intended_patch_kind.as_deref(),
        ),
        inferred_substrate_candidates,
        inferred_tool_kind,
        intended_patch_kind: intended_patch_kind.clone(),
        available_patch_kinds,
        planner_patch_recipe_ids: Vec::new(),
        planner_operator_bundle_ids: Vec::new(),
        planner_operator_ids: Vec::new(),
        planner_acceptance_recipe_ids: Vec::new(),
        execution_steps,
        constraints,
        rationale,
        planner_acceptance_checks: Vec::new(),
        planner_required_markers: Vec::new(),
        planner_acceptance_commands: Vec::new(),
        planner_expected_outputs: Vec::new(),
        planner_suggested_patch_kinds: Vec::new(),
        planner_suggested_features: Vec::new(),
        confidence_score,
        confidence_band,
        escalation_reasons,
        needs_llm_review,
        created_at: Some(timestamp_id("created")),
    }
}

pub fn derive_planner_handoff(
    request: &RequestRecord,
    plan: &RequestPlan,
    active_spec: Option<&ProjectSpec>,
) -> PlannerHandoff {
    let mut requested_output = vec![
        "structured substrate recommendation".into(),
        "tool kind or patch kind recommendation".into(),
        "optional patch recipe ids for host-owned bolt-on lanes".into(),
        "optional operator bundle ids for host-owned composition".into(),
        "execution steps the host can follow".into(),
        "acceptance expectations or repair hints".into(),
        "optional additive acceptance checks, markers, and commands".into(),
        "optional acceptance recipe ids for host-owned verification".into(),
        "optional future patch kinds or feature suggestions".into(),
    ];
    if matches!(request.mode, Some(RequestMode::Patch)) {
        requested_output.push("patch strategy for the active project".into());
    }
    if let Some(spec) = active_spec {
        if !spec.supported_patch_kinds.is_empty() {
            requested_output.push("whether to reuse or extend known patch lanes".into());
        }
    }

    PlannerHandoff {
        handoff_id: timestamp_id("planner-handoff"),
        request_id: request.request_id.clone(),
        source_plan_id: plan.plan_id.clone(),
        mode: request.mode.clone(),
        active_project: request.active_project.clone(),
        interpreted_goal: plan.interpreted_goal.clone(),
        inferred_substrate_candidates: plan.inferred_substrate_candidates.clone(),
        inferred_tool_kind: plan.inferred_tool_kind.clone(),
        available_patch_kinds: plan.available_patch_kinds.clone(),
        candidate_request_modes: Vec::new(),
        candidate_active_projects: Vec::new(),
        candidate_active_project_summaries: Vec::new(),
        candidate_patch_recipe_ids: Vec::new(),
        candidate_composition_patch_kinds: Vec::new(),
        candidate_composition_patch_primitive_classes: Vec::new(),
        candidate_composition_base_build_primitive_classes: Vec::new(),
        candidate_composition_layers: Vec::new(),
        candidate_composition_helper_primitive_ids: Vec::new(),
        candidate_composition_helper_primitive_kinds: Vec::new(),
        candidate_composition_adapter_semantics: Vec::new(),
        candidate_operator_bundle_ids: Vec::new(),
        candidate_acceptance_recipe_ids: Vec::new(),
        rationale: plan.rationale.clone(),
        escalation_reasons: plan.escalation_reasons.clone(),
        requested_output,
        created_at: Some(timestamp_id("created")),
    }
}

fn infer_request_substrate_kinds(
    raw_request: &str,
    desired_surface: Option<&DesiredSurface>,
    inferred_tool_kind: Option<&str>,
    wants_chattycog: bool,
    wants_chattyedu: bool,
    wants_dual_host: bool,
    helper_or_service_heavy: bool,
    unsupported_stack: bool,
) -> Vec<SubstrateKind> {
    let lower = raw_request.to_ascii_lowercase();
    let mut substrates = Vec::new();
    if unsupported_stack || helper_or_service_heavy {
        return substrates;
    }

    if wants_dual_host {
        substrates.push(SubstrateKind::NativeWindow);
    } else if wants_chattycog && contains_any(&lower, &["webview", "browser tab", "hosted webview"]) {
        substrates.push(SubstrateKind::Webview);
    } else if wants_chattycog || wants_chattyedu {
        substrates.push(SubstrateKind::NativeWindow);
    } else if matches!(desired_surface, Some(DesiredSurface::Cli)) || request_has_cli_shape(&lower)
    {
        substrates.push(SubstrateKind::Cli);
    } else if contains_any(
        &lower,
        &["native window", "desktop", "rust gui", "eframe", "egui", "tkinter"],
    ) {
        substrates.push(SubstrateKind::NativeWindow);
    } else if request_has_web_shape(&lower) || request_has_dashboard_shape(&lower) {
        substrates.push(SubstrateKind::StaticWeb);
    } else if inferred_tool_kind.is_some() {
        substrates.push(SubstrateKind::StaticWeb);
    }

    substrates.dedup();
    substrates
}

pub fn persist_json_pretty<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = serde_json::to_string_pretty(value)?;
    fs::write(path, text)?;
    Ok(())
}

fn infer_feature_tokens(raw_request: &str) -> Vec<String> {
    let lower = raw_request.to_ascii_lowercase();
    let mut features = vec!["status_panel".into(), "results_panel".into()];
    if contains_any(&lower, &["metric", "dashboard"]) {
        features.push("metric_card".into());
    }
    if contains_any(&lower, &["action", "button", "control"]) {
        features.push("action_toolbar".into());
    }
    features.sort();
    features.dedup();
    features
}

pub fn infer_route_hints(request: &RequestRecord) -> (Vec<OperatorId>, Vec<WrapperId>) {
    let lower = request.raw_request.to_ascii_lowercase();
    let mut operators = Vec::new();
    let mut wrappers = Vec::new();

    if contains_any(&lower, &["dashboard", "metric"]) {
        operators.push(OperatorId("metric_card".into()));
    }
    if contains_any(&lower, &["status", "dashboard"]) {
        operators.push(OperatorId("status_panel".into()));
    }
    if contains_any(&lower, &["result", "table"]) {
        operators.push(OperatorId("results_panel".into()));
    }
    if contains_any(&lower, &["action", "button", "control"]) {
        operators.push(OperatorId("action_toolbar".into()));
    }

    if matches!(
        request.exoskeleton_target,
        Some(ExoskeletonTarget::ChattyCog)
    ) {
        wrappers.push(WrapperId("chattycog_wrapper".into()));
    }

    (operators, wrappers)
}

fn infer_entrypoint_config(
    request: &RequestRecord,
    tool_kind_override: Option<&str>,
) -> Vec<String> {
    let lower = request.raw_request.to_ascii_lowercase();
    if tool_kind_override.is_none()
        && !matches!(request.desired_surface, Some(DesiredSurface::Cli))
        && !request_has_cli_shape(&lower)
    {
        return Vec::new();
    }
    let mut out = Vec::new();
    let tool_kind = tool_kind_override
        .or_else(|| infer_cli_tool_kind_from_text(&lower))
        .unwrap_or("directory_audit");
    out.push(format!("tool_kind={tool_kind}"));
    out
}

fn infer_tool_kind(request: &RequestRecord) -> Option<String> {
    let lower = request.raw_request.to_ascii_lowercase();
    let wants_chattycog = matches!(
        request.exoskeleton_target,
        Some(ExoskeletonTarget::ChattyCog)
    ) || request_mentions_chattycog(&lower);
    let wants_chattyedu = request_mentions_chattyedu(&lower);
    let desired_surface_cli = matches!(request.desired_surface, Some(DesiredSurface::Cli));
    infer_request_tool_kind_from_text(
        &lower,
        wants_chattycog,
        wants_chattyedu,
        desired_surface_cli,
    )
    .map(str::to_string)
}

fn infer_patch_kind(raw_request: &str, spec: &ProjectSpec) -> Option<String> {
    let lower = raw_request.to_ascii_lowercase();
    infer_patch_kind_from_text(&lower, &spec.supported_patch_kinds)
}

fn infer_fixture_config(request: &RequestRecord, tool_kind_override: Option<&str>) -> Vec<String> {
    let tool_kind = infer_entrypoint_config(request, tool_kind_override)
        .into_iter()
        .find(|item| item.starts_with("tool_kind="))
        .unwrap_or_default();
    if tool_kind == "tool_kind=file_sorter" {
        vec!["fixtures=input_output".into()]
    } else if tool_kind == "tool_kind=csv_report" {
        vec!["fixtures=csv_input".into()]
    } else if tool_kind == "tool_kind=log_summary" {
        vec!["fixtures=log_input".into()]
    } else if tool_kind == "tool_kind=text_stats" {
        vec!["fixtures=text_input".into()]
    } else if tool_kind == "tool_kind=directory_audit" {
        vec!["fixtures=directory_input".into()]
    } else {
        Vec::new()
    }
}

fn sanitize_name(fallback: &str, raw_request: &str) -> String {
    let candidate = raw_request
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    let collapsed = candidate
        .split('_')
        .filter(|s| !s.is_empty())
        .take(6)
        .collect::<Vec<_>>()
        .join("_");
    if collapsed.is_empty() {
        fallback.into()
    } else {
        collapsed
    }
}

fn project_spec_substrate_kind_for_planning(spec: &ProjectSpec) -> Option<SubstrateKind> {
    match spec.substrate.trim() {
        "static_web" => Some(SubstrateKind::StaticWeb),
        "python_cli" | "rust_cli" | "cli" => Some(SubstrateKind::Cli),
        "webview" => Some(SubstrateKind::Webview),
        "native_window" => Some(SubstrateKind::NativeWindow),
        "workspace" => Some(SubstrateKind::Workspace),
        _ => None,
    }
}

fn request_title(raw_request: &str, substrate_label: &str) -> String {
    let trimmed = raw_request.trim();
    if trimmed.is_empty() {
        format!("{} build", substrate_label.replace('_', " "))
    } else {
        let mut chars = trimmed.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => format!("{} build", substrate_label.replace('_', " ")),
        }
    }
}

fn request_summary(raw_request: &str, substrate_label: &str) -> String {
    if raw_request.trim().is_empty() {
        format!(
            "A deterministic {} build from the ChattyFactory rebuild.",
            substrate_label
        )
    } else {
        format!(
            "Deterministic {} build for request: {}",
            substrate_label,
            raw_request.trim()
        )
    }
}

fn interpreted_goal(
    request: &RequestRecord,
    inferred_tool_kind: Option<&str>,
    intended_patch_kind: Option<&str>,
) -> String {
    match request.mode {
        Some(RequestMode::Patch) => {
            let project_name = request
                .active_project
                .as_deref()
                .unwrap_or("active project");
            if let Some(patch_kind) = intended_patch_kind {
                format!("Patch {project_name} by applying the {patch_kind} capability")
            } else {
                format!("Patch {project_name} according to the follow-up request")
            }
        }
        _ => {
            if let Some(tool_kind) = inferred_tool_kind {
                format!("Build a {} project that satisfies the request", tool_kind)
            } else {
                "Build a project that satisfies the request".into()
            }
        }
    }
}

fn compute_plan_confidence(
    request: &RequestRecord,
    inferred_substrate_candidates: &[SubstrateKind],
    inferred_tool_kind: Option<&str>,
    intended_patch_kind: Option<&str>,
    available_patch_kinds: &[String],
    constraints: &[String],
) -> (u8, String, Vec<String>) {
    let mut score: i32 = if matches!(request.mode, Some(RequestMode::Patch)) {
        55
    } else {
        50
    };
    let mut escalation_reasons = Vec::new();

    if !inferred_substrate_candidates.is_empty() {
        score += 15;
    } else {
        score -= 25;
        escalation_reasons.push("no bounded substrate candidates were inferred".into());
    }

    if inferred_tool_kind.is_some() {
        score += 15;
    } else if !matches!(request.mode, Some(RequestMode::Patch)) {
        score -= 15;
        escalation_reasons.push("tool kind could not be inferred confidently".into());
    }

    if matches!(request.mode, Some(RequestMode::Patch)) {
        if intended_patch_kind.is_some() {
            score += 20;
        } else {
            score -= 25;
            escalation_reasons.push("follow-up request did not match a known patch lane".into());
        }
        if !available_patch_kinds.is_empty() {
            score += 10;
        } else {
            escalation_reasons.push("active project does not advertise known patch lanes".into());
        }
    }

    if !constraints.is_empty() {
        score -= (constraints.len() as i32) * 10;
        escalation_reasons.extend(constraints.iter().cloned());
    }
    if constraints.iter().any(|item| {
        item.contains("requested ChattyCog bridge capabilities are not implemented")
            || item.contains("requested ChattyCog module shape exceeds the current deterministic hosting contract")
    }) {
        score -= 20;
    }

    if request.raw_request.trim().len() < 12 {
        score -= 10;
        escalation_reasons.push("request is very short and may underspecify intent".into());
    }

    let lower = request.raw_request.to_ascii_lowercase();
    let vague_improvement = request_has_vague_improvement(&lower);
    if vague_improvement && !matches!(request.mode, Some(RequestMode::Patch)) {
        score -= 25;
        escalation_reasons.push(
            "request asks for improvement without naming a concrete project shape or capability"
                .into(),
        );
    }
    if vague_improvement
        && matches!(request.mode, Some(RequestMode::Patch))
        && intended_patch_kind.is_none()
    {
        score -= 20;
        escalation_reasons.push(
            "follow-up request asks for improvement without matching a concrete patch lane".into(),
        );
    }

    let score = score.clamp(0, 100) as u8;
    let band = if score >= 80 {
        "high"
    } else if score >= 60 {
        "medium"
    } else {
        "low"
    };

    (score, band.into(), escalation_reasons)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn novel_dashboard_requests_keep_a_next_attempt_substrate_candidate() {
        let request = normalize_request("build me a golang dashboard for project tracking");
        let plan = derive_request_plan(&request, None);

        assert_eq!(
            plan.inferred_substrate_candidates.first(),
            Some(&SubstrateKind::StaticWeb)
        );
        assert_eq!(plan.inferred_tool_kind.as_deref(), Some("dashboard"));
        assert!(
            plan.rationale
                .iter()
                .any(|reason| reason.contains("next-attempt substrate candidates derived from evidence")),
            "expected next-attempt substrate rationale, got {:?}",
            plan.rationale
        );
        assert!(
            !plan.needs_llm_review,
            "expected evidence-based next-attempt substrate pass to stay executable, got {:?}",
            plan
        );
    }

    #[test]
    fn kanban_requests_are_not_forced_into_llm_review() {
        let request =
            normalize_request("build me an electron desktop kanban app with drag and drop cards");
        let plan = derive_request_plan(&request, None);

        assert_eq!(plan.inferred_tool_kind.as_deref(), Some("dashboard"));
        assert!(
            !plan.needs_llm_review,
            "expected kanban dashboard request to stay above review threshold, got {:?}",
            plan
        );
    }
}
