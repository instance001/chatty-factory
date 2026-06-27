use chatty_factory_core::{
    infer_route_hints, CapabilityTransition, ExoskeletonTarget, RequestPlan, RequestRecord,
    RouteDecision, SubstrateKind,
};
use petgraph::stable_graph::{NodeIndex, StableDiGraph};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlNodeKind {
    NormalizeRequest,
    ClassifyMode,
    DetectWrapperIntent,
    ChooseSubstrate,
    ResolveBuildSeedInputs,
    RenderBuildSeed,
    RenderWrapper,
    EmitProjectSpec,
    BuildAcceptancePlan,
    RunAcceptance,
    ClassifyFailure,
    StopSuccess,
    StopFail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlNode {
    pub kind: ControlNodeKind,
    pub label: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlEdge {
    pub reason: &'static str,
    pub substrate: Option<SubstrateKind>,
}

#[derive(Debug, Default)]
pub struct ControlPlane {
    graph: StableDiGraph<ControlNode, ControlEdge>,
}

impl ControlPlane {
    pub fn milestone_one() -> Self {
        let mut graph = StableDiGraph::new();

        let normalize = add(
            &mut graph,
            ControlNodeKind::NormalizeRequest,
            "normalize_request",
        );
        let classify = add(&mut graph, ControlNodeKind::ClassifyMode, "classify_mode");
        let wrapper = add(
            &mut graph,
            ControlNodeKind::DetectWrapperIntent,
            "detect_wrapper_intent",
        );
        let choose = add(
            &mut graph,
            ControlNodeKind::ChooseSubstrate,
            "choose_substrate",
        );
        let build_seed = add(
            &mut graph,
            ControlNodeKind::ResolveBuildSeedInputs,
            "resolve_build_seed_inputs",
        );
        let render_build_seed = add(
            &mut graph,
            ControlNodeKind::RenderBuildSeed,
            "render_build_seed",
        );
        let render_wrapper = add(&mut graph, ControlNodeKind::RenderWrapper, "render_wrapper");
        let project_spec = add(
            &mut graph,
            ControlNodeKind::EmitProjectSpec,
            "emit_project_spec",
        );
        let acceptance_plan = add(
            &mut graph,
            ControlNodeKind::BuildAcceptancePlan,
            "build_acceptance_plan",
        );
        let run_acceptance = add(&mut graph, ControlNodeKind::RunAcceptance, "run_acceptance");
        let classify_failure = add(
            &mut graph,
            ControlNodeKind::ClassifyFailure,
            "classify_failure",
        );
        let stop_success = add(&mut graph, ControlNodeKind::StopSuccess, "stop_success");
        let stop_fail = add(&mut graph, ControlNodeKind::StopFail, "stop_fail");

        connect(&mut graph, normalize, classify, "mode_is_new_build", None);
        connect(&mut graph, classify, wrapper, "dashboard_fit", None);
        connect(&mut graph, wrapper, choose, "wrapper_requested", None);
        connect(
            &mut graph,
            choose,
            build_seed,
            "substrate_supported",
            Some(SubstrateKind::StaticWeb),
        );
        connect(
            &mut graph,
            choose,
            build_seed,
            "substrate_supported",
            Some(SubstrateKind::Webview),
        );
        connect(
            &mut graph,
            choose,
            build_seed,
            "substrate_supported",
            Some(SubstrateKind::NativeWindow),
        );
        connect(
            &mut graph,
            choose,
            build_seed,
            "substrate_supported",
            Some(SubstrateKind::Workspace),
        );
        connect(&mut graph, build_seed, render_build_seed, "inputs_ready", None);
        connect(
            &mut graph,
            render_build_seed,
            render_wrapper,
            "wrapper_optional",
            None,
        );
        connect(
            &mut graph,
            render_wrapper,
            project_spec,
            "files_emitted",
            None,
        );
        connect(
            &mut graph,
            project_spec,
            acceptance_plan,
            "contract_emitted",
            None,
        );
        connect(
            &mut graph,
            acceptance_plan,
            run_acceptance,
            "acceptance_built",
            None,
        );
        connect(
            &mut graph,
            run_acceptance,
            stop_success,
            "acceptance_passed",
            None,
        );
        connect(
            &mut graph,
            run_acceptance,
            classify_failure,
            "acceptance_failed",
            None,
        );
        connect(
            &mut graph,
            classify_failure,
            stop_fail,
            "failure_classified",
            None,
        );

        Self { graph }
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn choose_milestone_one_route(
        &self,
        request: &RequestRecord,
        plan: &RequestPlan,
    ) -> RouteDecision {
        let wants_wrapper = matches!(
            request.exoskeleton_target,
            Some(ExoskeletonTarget::ChattyCog)
        );
        let wants_cli = plan
            .inferred_substrate_candidates
            .iter()
            .any(|substrate| matches!(substrate, SubstrateKind::Cli))
            && matches!(
                request.desired_surface,
                Some(chatty_factory_core::DesiredSurface::Cli)
            );
        let selected_substrate_kind = if wants_wrapper {
            plan.inferred_substrate_candidates
                .first()
                .cloned()
                .or(Some(SubstrateKind::Webview))
        } else if let Some(first) = plan.inferred_substrate_candidates.first() {
            Some(first.clone())
        } else if request
            .requested_capabilities
            .iter()
            .any(|cap| cap == "rust")
        {
            Some(SubstrateKind::Cli)
        } else if wants_cli {
            Some(SubstrateKind::Cli)
        } else {
            Some(SubstrateKind::StaticWeb)
        };
        let capability_transition = if wants_wrapper {
            Some(CapabilityTransition::WrapperEmission)
        } else {
            Some(CapabilityTransition::None)
        };

        let (mut selected_operator_ids, selected_wrapper_ids) = infer_route_hints(request);
        for operator_id in &plan.planner_operator_ids {
            if !selected_operator_ids
                .iter()
                .any(|existing| existing.0 == *operator_id)
            {
                selected_operator_ids.push(chatty_factory_core::OperatorId(operator_id.clone()));
            }
        }
        let mut decision_reasons = Vec::new();
        decision_reasons.push("substrate_route_selected_from_request_plan".into());
        if wants_wrapper {
            decision_reasons.push("wrapper_intent_detected".into());
        } else if matches!(selected_substrate_kind, Some(SubstrateKind::NativeWindow)) {
            decision_reasons.push("native_window_substrate_fit_detected".into());
        } else if matches!(selected_substrate_kind, Some(SubstrateKind::Webview)) {
            decision_reasons.push("webview_substrate_fit_detected".into());
        } else if matches!(selected_substrate_kind, Some(SubstrateKind::Workspace)) {
            decision_reasons.push("workspace_substrate_fit_detected".into());
        } else if matches!(selected_substrate_kind, Some(SubstrateKind::Cli)) {
            decision_reasons.push("cli_substrate_fit_detected".into());
            if request.requested_capabilities.iter().any(|cap| cap == "rust") {
                decision_reasons.push("rust_cli_adapter_selected".into());
            } else if wants_cli {
                decision_reasons.push("python_cli_adapter_selected".into());
            }
        } else if matches!(selected_substrate_kind, Some(SubstrateKind::StaticWeb)) {
            decision_reasons.push("static_web_substrate_default".into());
        } else if wants_cli {
            decision_reasons.push("cli_substrate_fit_detected".into());
        } else {
            decision_reasons.push("substrate_default_resolution".into());
        }
        if let Some(tool_kind) = &plan.inferred_tool_kind {
            decision_reasons.push(format!("tool_kind={tool_kind}"));
        }
        if plan.rationale.iter().any(|reason| {
            reason.starts_with("no exact deterministic substrate matched from the request record")
        }) {
            decision_reasons.push("gauntlet_substrate_retry".into());
        }
        if !plan.planner_operator_ids.is_empty() {
            decision_reasons.push(format!(
                "planner_operator_bundle={}",
                plan.planner_operator_ids.join(",")
            ));
        }
        if !plan.planner_operator_bundle_ids.is_empty() {
            decision_reasons.push(format!(
                "planner_operator_bundles={}",
                plan.planner_operator_bundle_ids.join(",")
            ));
        }
        if !plan.planner_acceptance_recipe_ids.is_empty() {
            decision_reasons.push(format!(
                "planner_acceptance_recipes={}",
                plan.planner_acceptance_recipe_ids.join(",")
            ));
        }
        decision_reasons.extend(
            plan.rationale
                .iter()
                .filter(|reason| {
                    reason.starts_with("adapter-aware primitive routing preferred family")
                        || reason.starts_with("adapter-aware-family-reason:")
                })
                .cloned(),
        );

        RouteDecision {
            route_id: chatty_factory_core::timestamp_id("route"),
            request_id: request.request_id.clone(),
            selected_substrate_kind,
            selected_operator_ids,
            selected_wrapper_ids,
            selected_behavior_kind: None,
            capability_transition,
            decision_reasons,
            next_attempt_level: Some("next_attempt_substrate".into()),
            needs_llm_review: plan.needs_llm_review,
            created_at: Some(chatty_factory_core::timestamp_id("created")),
        }
    }
}

fn add(
    graph: &mut StableDiGraph<ControlNode, ControlEdge>,
    kind: ControlNodeKind,
    label: &'static str,
) -> NodeIndex {
    graph.add_node(ControlNode { kind, label })
}

fn connect(
    graph: &mut StableDiGraph<ControlNode, ControlEdge>,
    a: NodeIndex,
    b: NodeIndex,
    reason: &'static str,
    substrate: Option<SubstrateKind>,
) {
    graph.add_edge(a, b, ControlEdge { reason, substrate });
}
