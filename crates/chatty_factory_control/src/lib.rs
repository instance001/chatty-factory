use chatty_factory_core::{
    infer_route_hints, CapabilityTransition, ExoskeletonTarget, FamilyId, RequestPlan,
    RequestRecord, RouteDecision,
};
use petgraph::stable_graph::{NodeIndex, StableDiGraph};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlNodeKind {
    NormalizeRequest,
    ClassifyMode,
    DetectWrapperIntent,
    ChooseFamily,
    ResolveScaffoldInputs,
    RenderScaffold,
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
    pub family: Option<FamilyId>,
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
        let choose = add(&mut graph, ControlNodeKind::ChooseFamily, "choose_family");
        let scaffold = add(
            &mut graph,
            ControlNodeKind::ResolveScaffoldInputs,
            "resolve_scaffold_inputs",
        );
        let render_scaffold = add(
            &mut graph,
            ControlNodeKind::RenderScaffold,
            "render_scaffold",
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
            scaffold,
            "family_supported",
            Some(FamilyId::StaticWebDashboard),
        );
        connect(
            &mut graph,
            choose,
            scaffold,
            "family_supported",
            Some(FamilyId::ChattycogWebviewModule),
        );
        connect(
            &mut graph,
            choose,
            scaffold,
            "family_supported",
            Some(FamilyId::ChattycogNativeWindowModule),
        );
        connect(
            &mut graph,
            choose,
            scaffold,
            "family_supported",
            Some(FamilyId::ChattycogWorkspaceModule),
        );
        connect(&mut graph, scaffold, render_scaffold, "inputs_ready", None);
        connect(
            &mut graph,
            render_scaffold,
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
            .inferred_family_candidates
            .iter()
            .any(|family| matches!(family, FamilyId::PythonCliTool | FamilyId::RustCliTool))
            && matches!(
                request.desired_surface,
                Some(chatty_factory_core::DesiredSurface::Cli)
            );
        let selected_family_id = if wants_wrapper {
            plan.inferred_family_candidates
                .first()
                .cloned()
                .or(Some(FamilyId::ChattycogWebviewModule))
        } else if let Some(first) = plan.inferred_family_candidates.first() {
            Some(first.clone())
        } else if request
            .requested_capabilities
            .iter()
            .any(|cap| cap == "rust")
        {
            Some(FamilyId::RustCliTool)
        } else if wants_cli {
            Some(FamilyId::PythonCliTool)
        } else {
            Some(FamilyId::StaticWebDashboard)
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
        decision_reasons.push("route_selected_from_request_plan".into());
        if wants_wrapper {
            decision_reasons.push("wrapper_intent_detected".into());
        } else if matches!(
            selected_family_id,
            Some(FamilyId::ChattycogChattyeduNativeWindowModule)
        ) {
            decision_reasons.push("dual_native_window_fit_detected".into());
        } else if matches!(
            selected_family_id,
            Some(FamilyId::ChattycogNativeWindowModule)
        ) {
            decision_reasons.push("chattycog_native_window_fit_detected".into());
        } else if matches!(
            selected_family_id,
            Some(FamilyId::ChattyeduNativeWindowModule)
        ) {
            decision_reasons.push("chattyedu_native_window_fit_detected".into());
        } else if matches!(selected_family_id, Some(FamilyId::RustCliTool)) {
            decision_reasons.push("rust_cli_fit_detected".into());
        } else if wants_cli {
            decision_reasons.push("python_cli_fit_detected".into());
        } else {
            decision_reasons.push("standalone_dashboard_default".into());
        }
        if let Some(tool_kind) = &plan.inferred_tool_kind {
            decision_reasons.push(format!("tool_kind={tool_kind}"));
        }
        if plan.rationale.iter().any(|reason| {
            reason.starts_with("no exact deterministic family matched; using scaffold substrate")
        }) {
            decision_reasons.push("scaffold_substrate_fallback".into());
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
            selected_family_id,
            selected_operator_ids,
            selected_wrapper_ids,
            selected_behavior_kind: None,
            capability_transition,
            decision_reasons,
            fallback_level: Some("family".into()),
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
    family: Option<FamilyId>,
) {
    graph.add_edge(a, b, ControlEdge { reason, family });
}
