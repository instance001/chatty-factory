use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail, Result};

use crate::{
    DiscoveredModel, PlannerExecutionReceipt, PlannerHandoff, PlannerResponse,
    RuntimeCapabilityRecord, RuntimeConfig, RuntimeDiscoveryReceipt, RuntimeModelAssessment,
    RuntimeModelCatalogReceipt, RuntimeSmokeReceipt,
};

struct PlannerParseOutcome {
    response: PlannerResponse,
    parse_mode: String,
    finish_reason: Option<String>,
    degraded_recovery_used: bool,
}

pub fn default_runtime_config(runtime_root: &Path, models_root: &Path) -> Result<RuntimeConfig> {
    let runtime_root = runtime_root
        .canonicalize()
        .unwrap_or_else(|_| runtime_root.to_path_buf());
    let models_root = models_root
        .canonicalize()
        .unwrap_or_else(|_| models_root.to_path_buf());
    let discovered_models = discover_models(&models_root)?;
    let default_model_path = choose_preferred_model(&discovered_models);

    Ok(RuntimeConfig {
        config_id: crate::timestamp_id("runtime-config"),
        runtime_root: runtime_root.display().to_string(),
        models_root: models_root.display().to_string(),
        server_executable: runtime_root.join("llama-server.exe").display().to_string(),
        cli_executable: runtime_root.join("llama-cli.exe").display().to_string(),
        default_model_path,
        host: "127.0.0.1".into(),
        port: 8080,
        context_size: 2048,
        gpu_layers: 99,
        launch_timeout_secs: 90,
        created_at: Some(crate::timestamp_id("created")),
    })
}

pub fn discover_runtime(
    runtime_root: &Path,
    models_root: &Path,
) -> Result<RuntimeDiscoveryReceipt> {
    let runtime_root = runtime_root
        .canonicalize()
        .unwrap_or_else(|_| runtime_root.to_path_buf());
    let models_root = models_root
        .canonicalize()
        .unwrap_or_else(|_| models_root.to_path_buf());
    let server_executable = runtime_root.join("llama-server.exe");
    let cli_executable = runtime_root.join("llama-cli.exe");
    if !server_executable.exists() {
        bail!(
            "missing runtime server executable at {}",
            server_executable.display()
        );
    }
    if !cli_executable.exists() {
        bail!(
            "missing runtime cli executable at {}",
            cli_executable.display()
        );
    }

    let server_version_output = run_version_probe(&server_executable, &runtime_root)?;
    let cli_version_output = run_version_probe(&cli_executable, &runtime_root)?;
    let discovered_models = discover_models(&models_root)?;
    let preferred_model_path = choose_preferred_model(&discovered_models);
    let vulkan_backend_present = runtime_root.join("ggml-vulkan.dll").exists()
        || server_version_output
            .to_ascii_lowercase()
            .contains("vulkan backend")
        || cli_version_output
            .to_ascii_lowercase()
            .contains("vulkan backend");

    Ok(RuntimeDiscoveryReceipt {
        discovery_id: crate::timestamp_id("runtime-discovery"),
        runtime_root: runtime_root.display().to_string(),
        models_root: models_root.display().to_string(),
        server_executable: server_executable.display().to_string(),
        cli_executable: cli_executable.display().to_string(),
        vulkan_backend_present,
        server_version_output,
        cli_version_output,
        discovered_models: discovered_models.clone(),
        preferred_model_path: preferred_model_path.clone(),
        planner_runtime_capability: RuntimeCapabilityRecord {
            backend_kind: if vulkan_backend_present {
                "vulkan".into()
            } else {
                "cpu_only".into()
            },
            vulkan_available: vulkan_backend_present,
            server_available: true,
            cli_available: true,
            discovered_model_count: discovered_models.len(),
            preferred_model_path,
        },
        created_at: Some(crate::timestamp_id("created")),
    })
}

pub fn build_runtime_model_catalog(models_root: &Path) -> Result<RuntimeModelCatalogReceipt> {
    let models_root = models_root
        .canonicalize()
        .unwrap_or_else(|_| models_root.to_path_buf());
    let discovered_models = discover_models(&models_root)?;
    let assessments = discovered_models
        .iter()
        .map(assess_model_for_planner)
        .collect::<Vec<_>>();

    Ok(RuntimeModelCatalogReceipt {
        catalog_id: crate::timestamp_id("runtime-model-catalog"),
        models_root: models_root.display().to_string(),
        preferred_fast_model_path: choose_model_for_profile(&discovered_models, "fast"),
        preferred_balanced_model_path: choose_model_for_profile(&discovered_models, "balanced"),
        preferred_heavy_model_path: choose_model_for_profile(&discovered_models, "heavy"),
        models: assessments,
        created_at: Some(crate::timestamp_id("created")),
    })
}

pub fn run_runtime_smoke(
    config: &RuntimeConfig,
    launch_server: bool,
) -> Result<RuntimeSmokeReceipt> {
    let runtime_root = PathBuf::from(&config.runtime_root);
    let server_executable = PathBuf::from(&config.server_executable);
    let version_output = run_version_probe(&server_executable, &runtime_root)?;
    let version_probe_ok = version_output.to_ascii_lowercase().contains("version:");

    let mut receipt = RuntimeSmokeReceipt {
        smoke_id: crate::timestamp_id("runtime-smoke"),
        config_id: config.config_id.clone(),
        model_path: config.default_model_path.clone(),
        launch_args: Vec::new(),
        version_probe_ok,
        server_launch_attempted: launch_server,
        server_started: false,
        http_probe_ok: false,
        process_killed: false,
        stdout_log_path: None,
        stderr_log_path: None,
        notes: Vec::new(),
        created_at: Some(crate::timestamp_id("created")),
    };

    if !launch_server {
        receipt
            .notes
            .push("server launch skipped; version probe only".into());
        return Ok(receipt);
    }

    let model_path = config
        .default_model_path
        .as_ref()
        .ok_or_else(|| anyhow!("no GGUF model available for runtime smoke"))?;
    if !Path::new(model_path).exists() {
        bail!("configured smoke model does not exist at {}", model_path);
    }

    let logs_dir = runtime_root.join("logs");
    fs::create_dir_all(&logs_dir)?;
    let stdout_log_path = logs_dir.join(format!("{}-stdout.log", receipt.smoke_id));
    let stderr_log_path = logs_dir.join(format!("{}-stderr.log", receipt.smoke_id));
    let stdout_log = File::create(&stdout_log_path)?;
    let stderr_log = File::create(&stderr_log_path)?;

    let launch_args = vec![
        "-m".into(),
        model_path.clone(),
        "-ngl".into(),
        config.gpu_layers.to_string(),
        "-c".into(),
        config.context_size.to_string(),
        "--host".into(),
        config.host.clone(),
        "--port".into(),
        config.port.to_string(),
    ];
    receipt.launch_args = launch_args.clone();
    receipt.stdout_log_path = Some(stdout_log_path.display().to_string());
    receipt.stderr_log_path = Some(stderr_log_path.display().to_string());

    let mut child = Command::new(&server_executable)
        .args(&launch_args)
        .current_dir(&runtime_root)
        .stdout(Stdio::from(stdout_log))
        .stderr(Stdio::from(stderr_log))
        .spawn()?;

    let deadline = Instant::now() + Duration::from_secs(config.launch_timeout_secs);
    let mut server_started = false;
    let mut http_probe_ok = false;
    while Instant::now() < deadline {
        if let Some(status) = child.try_wait()? {
            receipt
                .notes
                .push(format!("server exited early with status {status}"));
            break;
        }
        if http_probe(config.host.as_str(), config.port)? {
            server_started = true;
            http_probe_ok = true;
            break;
        }
        thread::sleep(Duration::from_millis(750));
    }

    receipt.server_started = server_started;
    receipt.http_probe_ok = http_probe_ok;

    child.kill()?;
    let _ = child.wait();
    receipt.process_killed = true;
    if !server_started {
        receipt
            .notes
            .push("server launch did not reach a responsive HTTP state before timeout".into());
    }

    Ok(receipt)
}

pub fn run_local_planner(
    config: &RuntimeConfig,
    handoff: &PlannerHandoff,
    planner_response_dir: &Path,
    raw_response_dir: &Path,
    planner_model_override: Option<&str>,
) -> Result<(PlannerResponse, PlannerExecutionReceipt)> {
    let runtime_root = PathBuf::from(&config.runtime_root);
    let server_executable = PathBuf::from(&config.server_executable);
    let model_path = planner_model_override
        .map(str::to_string)
        .or_else(|| config.default_model_path.clone())
        .ok_or_else(|| anyhow!("no model configured for local planner"))?;
    if !Path::new(&model_path).exists() {
        bail!("planner model does not exist at {}", model_path);
    }

    fs::create_dir_all(planner_response_dir)?;
    fs::create_dir_all(raw_response_dir)?;

    let stdout_log_path =
        raw_response_dir.join(format!("planner-server-{}-stdout.log", handoff.handoff_id));
    let stderr_log_path =
        raw_response_dir.join(format!("planner-server-{}-stderr.log", handoff.handoff_id));
    let stdout_log = File::create(&stdout_log_path)?;
    let stderr_log = File::create(&stderr_log_path)?;

    let launch_args = vec![
        "-m".into(),
        model_path.clone(),
        "-ngl".into(),
        config.gpu_layers.to_string(),
        "-c".into(),
        config.context_size.to_string(),
        "--host".into(),
        config.host.clone(),
        "--port".into(),
        config.port.to_string(),
    ];

    let mut child = Command::new(&server_executable)
        .args(&launch_args)
        .current_dir(&runtime_root)
        .stdout(Stdio::from(stdout_log))
        .stderr(Stdio::from(stderr_log))
        .spawn()?;

    let mut receipt = PlannerExecutionReceipt {
        execution_id: crate::timestamp_id("planner-exec"),
        config_id: config.config_id.clone(),
        source_handoff_id: handoff.handoff_id.clone(),
        model_path: model_path.clone(),
        launch_args: launch_args.clone(),
        response_path: None,
        raw_response_path: None,
        server_started: false,
        http_request_ok: false,
        process_killed: false,
        parse_mode: None,
        finish_reason: None,
        degraded_recovery_used: false,
        should_escalate: false,
        notes: vec![
            format!("stdout_log={}", stdout_log_path.display()),
            format!("stderr_log={}", stderr_log_path.display()),
        ],
        created_at: Some(crate::timestamp_id("created")),
    };

    let result = (|| -> Result<PlannerResponse> {
        let deadline = Instant::now() + Duration::from_secs(config.launch_timeout_secs);
        while Instant::now() < deadline {
            if let Some(status) = child.try_wait()? {
                receipt
                    .notes
                    .push(format!("planner server exited early with status {status}"));
                break;
            }
            if http_probe(config.host.as_str(), config.port)? {
                receipt.server_started = true;
                break;
            }
            thread::sleep(Duration::from_millis(750));
        }

        if !receipt.server_started {
            bail!("planner server did not become ready before timeout");
        }

        let request_body = build_planner_chat_request(handoff)?;
        let raw_response = post_json_request(
            config.host.as_str(),
            config.port,
            "/v1/chat/completions",
            &request_body,
        )?;
        receipt.http_request_ok = true;

        let raw_response_path =
            raw_response_dir.join(format!("planner-response-{}-raw.json", handoff.handoff_id));
        fs::write(&raw_response_path, &raw_response)?;
        receipt.raw_response_path = Some(raw_response_path.display().to_string());

        let parse_outcome = parse_planner_response(&raw_response, handoff)?;
        receipt.parse_mode = Some(parse_outcome.parse_mode.clone());
        receipt.finish_reason = parse_outcome.finish_reason.clone();
        receipt.degraded_recovery_used = parse_outcome.degraded_recovery_used;
        receipt.should_escalate = parse_outcome.degraded_recovery_used;
        if receipt.degraded_recovery_used {
            receipt
                .notes
                .push("planner response required degraded host-side recovery".into());
        }

        let planner_response = parse_outcome.response;
        let response_path =
            planner_response_dir.join(format!("{}.json", planner_response.response_id));
        fs::write(
            &response_path,
            serde_json::to_string_pretty(&planner_response)?,
        )?;
        receipt.response_path = Some(response_path.display().to_string());

        Ok(planner_response)
    })();

    let _ = child.kill();
    let _ = child.wait();
    receipt.process_killed = true;

    match result {
        Ok(planner_response) => Ok((planner_response, receipt)),
        Err(err) => Err(err),
    }
}

pub fn resolve_model_choice(
    requested_model: Option<&str>,
    catalog: &RuntimeModelCatalogReceipt,
    fallback: Option<&str>,
) -> Option<String> {
    let requested_model = requested_model
        .map(str::trim)
        .filter(|value| !value.is_empty());
    if let Some(requested_model) = requested_model {
        if Path::new(requested_model).exists() {
            return Some(requested_model.to_string());
        }

        match requested_model.to_ascii_lowercase().as_str() {
            "fast" | "small" | "test" => {
                if catalog.preferred_fast_model_path.is_some() {
                    return catalog.preferred_fast_model_path.clone();
                }
            }
            "balanced" | "mid" | "medium" => {
                if catalog.preferred_balanced_model_path.is_some() {
                    return catalog.preferred_balanced_model_path.clone();
                }
            }
            "heavy" | "large" | "rescue" => {
                if catalog.preferred_heavy_model_path.is_some() {
                    return catalog.preferred_heavy_model_path.clone();
                }
            }
            _ => {}
        }

        if let Some(exact_match) = catalog
            .models
            .iter()
            .find(|model| model.file_name.eq_ignore_ascii_case(requested_model))
        {
            return Some(exact_match.path.clone());
        }

        if let Some(partial_match) = catalog.models.iter().find(|model| {
            model
                .file_name
                .to_ascii_lowercase()
                .contains(&requested_model.to_ascii_lowercase())
        }) {
            return Some(partial_match.path.clone());
        }
    }

    fallback.map(str::to_string)
}

fn discover_models(models_root: &Path) -> Result<Vec<DiscoveredModel>> {
    if !models_root.exists() {
        return Ok(Vec::new());
    }
    let mut models = Vec::new();
    for entry in fs::read_dir(models_root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
            continue;
        };
        if !extension.eq_ignore_ascii_case("gguf") {
            continue;
        }
        let metadata = entry.metadata()?;
        let resolved_path = path.canonicalize().unwrap_or(path.clone());
        models.push(DiscoveredModel {
            file_name: entry.file_name().to_string_lossy().to_string(),
            path: resolved_path.display().to_string(),
            size_bytes: metadata.len(),
        });
    }
    models.sort_by(|left, right| left.file_name.cmp(&right.file_name));
    Ok(models)
}

fn choose_preferred_model(models: &[DiscoveredModel]) -> Option<String> {
    choose_model_for_profile(models, "fast")
}

fn planner_model_sort_key(model: &DiscoveredModel) -> (u8, u64, String) {
    let name = model.file_name.to_ascii_lowercase();
    let tier = if name.contains("qwen3-8b") {
        0
    } else if name.contains("8b") && (name.contains("qwen") || name.contains("instruct")) {
        1
    } else if name.contains("hermes") || name.contains("gemma") || name.contains("devstral") {
        2
    } else if model.size_bytes <= 12 * 1024 * 1024 * 1024 {
        3
    } else if model.size_bytes <= 16 * 1024 * 1024 * 1024 {
        4
    } else {
        5
    };
    (tier, model.size_bytes, name)
}

fn choose_model_for_profile(models: &[DiscoveredModel], profile: &str) -> Option<String> {
    let profile = profile.to_ascii_lowercase();
    match profile.as_str() {
        "balanced" => models
            .iter()
            .min_by_key(|model| balanced_model_sort_key(model))
            .map(|model| model.path.clone()),
        "heavy" => models
            .iter()
            .min_by_key(|model| heavy_model_sort_key(model))
            .map(|model| model.path.clone()),
        _ => models
            .iter()
            .min_by_key(|model| planner_model_sort_key(model))
            .map(|model| model.path.clone()),
    }
}

fn balanced_model_sort_key(model: &DiscoveredModel) -> (u8, u64, String) {
    let name = model.file_name.to_ascii_lowercase();
    let tier = if name.contains("13b") || name.contains("12b") {
        0
    } else if name.contains("20b") || name.contains("24b") {
        1
    } else if name.contains("8b") {
        2
    } else {
        3
    };
    (tier, model.size_bytes, name)
}

fn heavy_model_sort_key(model: &DiscoveredModel) -> (u8, i64, String) {
    let name = model.file_name.to_ascii_lowercase();
    let tier = if name.contains("32b") {
        0
    } else if name.contains("24b") || name.contains("20b") {
        1
    } else if name.contains("13b") || name.contains("12b") {
        2
    } else {
        3
    };
    (tier, -(model.size_bytes as i64), name)
}

fn assess_model_for_planner(model: &DiscoveredModel) -> RuntimeModelAssessment {
    let name = model.file_name.to_ascii_lowercase();
    let mut suitability_tags = Vec::new();
    let mut notes = Vec::new();

    let planner_profile = if name.contains("8b") {
        suitability_tags.push("fast_planner".into());
        notes.push("good candidate for fast local planning and component tests".into());
        "fast"
    } else if name.contains("13b")
        || name.contains("12b")
        || name.contains("20b")
        || name.contains("24b")
    {
        suitability_tags.push("balanced_planner".into());
        notes.push("useful when the fast planner drifts or needs stronger judgment".into());
        "balanced"
    } else {
        suitability_tags.push("heavy_planner".into());
        notes.push("best reserved for harder rescue planning or tougher interpretation".into());
        "heavy"
    };

    if name.contains("instruct") || name.contains("hermes") || name.contains("qwen") {
        suitability_tags.push("instruction_tuned".into());
    }
    if name.contains("deepseek") || name.contains("r1") {
        notes.push(
            "may produce verbose reasoning or refusal-style traces; keep prompts tight".into(),
        );
    }
    if name.contains("gpt-oss") || name.contains("devstral") || name.contains("gemma") {
        notes.push("good candidate for alternate planner trials".into());
    }

    RuntimeModelAssessment {
        file_name: model.file_name.clone(),
        path: model.path.clone(),
        size_bytes: model.size_bytes,
        planner_profile: planner_profile.into(),
        suitability_tags,
        notes,
    }
}

fn run_version_probe(executable: &Path, working_dir: &Path) -> Result<String> {
    let output = Command::new(executable)
        .arg("--version")
        .current_dir(working_dir)
        .output()?;
    if !output.status.success() {
        bail!(
            "version probe failed for {}: {}",
            executable.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let mut text = String::from_utf8_lossy(&output.stdout).to_string();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&output.stderr).to_string();
    }
    Ok(text)
}

fn http_probe(host: &str, port: u16) -> Result<bool> {
    match TcpStream::connect((host, port)) {
        Ok(mut stream) => {
            stream.set_read_timeout(Some(Duration::from_secs(2)))?;
            stream.set_write_timeout(Some(Duration::from_secs(2)))?;
            stream.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")?;
            let mut buffer = [0u8; 256];
            let read = stream.read(&mut buffer)?;
            let response = String::from_utf8_lossy(&buffer[..read]).to_string();
            let status_line = response.lines().next().unwrap_or_default();
            Ok(status_line.starts_with("HTTP/1.") && !status_line.contains(" 503 "))
        }
        Err(err) if err.kind() == std::io::ErrorKind::ConnectionRefused => Ok(false),
        Err(err) if err.kind() == std::io::ErrorKind::TimedOut => Ok(false),
        Err(err) => Err(err.into()),
    }
}

fn build_planner_chat_request(handoff: &PlannerHandoff) -> Result<String> {
    let allowed_family_ids = handoff
        .inferred_family_candidates
        .iter()
        .map(|family| serde_json::to_value(family))
        .collect::<Result<Vec<_>, _>>()?;
    let allowed_tool_kinds = serde_json::json!([
        "directory_audit",
        "csv_report",
        "log_summary",
        "text_stats",
        "file_sorter"
    ]);
    let has_request_mode_candidates = !handoff.candidate_request_modes.is_empty();
    let has_patch_recipe_candidates = !handoff.candidate_patch_recipe_ids.is_empty();
    let has_composition_patch_candidates = !handoff.candidate_composition_patch_kinds.is_empty();
    let has_composition_helper_candidates =
        !handoff.candidate_composition_helper_primitive_ids.is_empty();
    let has_operator_bundle_candidates = !handoff.candidate_operator_bundle_ids.is_empty();
    let has_acceptance_recipe_candidates = !handoff.candidate_acceptance_recipe_ids.is_empty();
    let is_patch = matches!(handoff.mode, Some(crate::RequestMode::Patch));
    let handoff_summary = if has_request_mode_candidates {
        serde_json::json!({
            "handoff_id": handoff.handoff_id,
            "source_plan_id": handoff.source_plan_id,
            "interpreted_goal": handoff.interpreted_goal,
            "candidate_request_modes": handoff.candidate_request_modes,
            "candidate_active_projects": handoff.candidate_active_projects,
            "candidate_active_project_summaries": handoff.candidate_active_project_summaries
        })
    } else if has_patch_recipe_candidates {
        serde_json::json!({
            "handoff_id": handoff.handoff_id,
            "source_plan_id": handoff.source_plan_id,
            "mode": handoff.mode,
            "active_project": handoff.active_project,
            "interpreted_goal": handoff.interpreted_goal,
            "candidate_families": allowed_family_ids,
            "inferred_tool_kind": handoff.inferred_tool_kind,
            "available_patch_kinds": handoff.available_patch_kinds,
            "candidate_patch_recipe_ids": handoff.candidate_patch_recipe_ids
        })
    } else if has_composition_patch_candidates {
        serde_json::json!({
            "handoff_id": handoff.handoff_id,
            "source_plan_id": handoff.source_plan_id,
            "mode": handoff.mode,
            "active_project": handoff.active_project,
            "interpreted_goal": handoff.interpreted_goal,
            "candidate_families": allowed_family_ids,
            "inferred_tool_kind": handoff.inferred_tool_kind,
            "candidate_composition_family_build_primitive_classes": handoff.candidate_composition_family_build_primitive_classes,
            "candidate_composition_layers": handoff.candidate_composition_layers,
            "candidate_composition_patch_kinds": handoff.candidate_composition_patch_kinds,
            "candidate_composition_patch_primitive_classes": handoff.candidate_composition_patch_primitive_classes,
            "candidate_composition_helper_primitive_ids": handoff.candidate_composition_helper_primitive_ids,
            "candidate_composition_helper_primitive_kinds": handoff.candidate_composition_helper_primitive_kinds,
            "candidate_composition_adapter_semantics": handoff.candidate_composition_adapter_semantics
        })
    } else if has_operator_bundle_candidates {
        serde_json::json!({
            "handoff_id": handoff.handoff_id,
            "source_plan_id": handoff.source_plan_id,
            "mode": handoff.mode,
            "interpreted_goal": handoff.interpreted_goal,
            "candidate_families": allowed_family_ids,
            "candidate_operator_bundle_ids": handoff.candidate_operator_bundle_ids
        })
    } else if has_acceptance_recipe_candidates {
        serde_json::json!({
            "handoff_id": handoff.handoff_id,
            "source_plan_id": handoff.source_plan_id,
            "mode": handoff.mode,
            "interpreted_goal": handoff.interpreted_goal,
            "candidate_families": allowed_family_ids,
            "inferred_tool_kind": handoff.inferred_tool_kind,
            "candidate_acceptance_recipe_ids": handoff.candidate_acceptance_recipe_ids
        })
    } else if is_patch {
        serde_json::json!({
            "handoff_id": handoff.handoff_id,
            "source_plan_id": handoff.source_plan_id,
            "mode": handoff.mode,
            "active_project": handoff.active_project,
            "interpreted_goal": handoff.interpreted_goal,
            "candidate_families": allowed_family_ids,
            "inferred_tool_kind": handoff.inferred_tool_kind,
            "available_patch_kinds": handoff.available_patch_kinds
        })
    } else {
        serde_json::json!({
            "handoff_id": handoff.handoff_id,
            "source_plan_id": handoff.source_plan_id,
            "mode": handoff.mode,
            "interpreted_goal": handoff.interpreted_goal,
            "candidate_families": allowed_family_ids,
            "inferred_tool_kind": handoff.inferred_tool_kind,
            "allowed_tool_kinds": allowed_tool_kinds
        })
    };
    let response_shape = if has_request_mode_candidates {
        serde_json::json!({
            "approved": true,
            "recommended_request_mode": "patch_active_project",
            "recommended_active_project": "build_me_a_python_csv_report",
            "rationale": ["short reason"]
        })
    } else if has_patch_recipe_candidates {
        serde_json::json!({
            "approved": true,
            "recommended_patch_recipe_ids": ["csv_report_json_export"],
            "rationale": ["short reason"]
        })
    } else if has_composition_patch_candidates {
        serde_json::json!({
            "approved": true,
            "recommended_composition_patch_kinds": ["helper_summary_panel"],
            "recommended_composition_patch_primitive_classes": handoff.candidate_composition_patch_primitive_classes.iter().take(4).cloned().collect::<Vec<_>>(),
            "recommended_composition_family_build_primitive_classes": handoff.candidate_composition_family_build_primitive_classes.iter().take(4).cloned().collect::<Vec<_>>(),
            "recommended_composition_layers": handoff.candidate_composition_layers.iter().take(4).cloned().collect::<Vec<_>>(),
            "recommended_composition_helper_primitive_ids": if has_composition_helper_candidates {
                handoff.candidate_composition_helper_primitive_ids.iter().take(4).cloned().collect::<Vec<_>>()
            } else {
                Vec::<String>::new()
            },
            "recommended_composition_helper_primitive_kinds": if has_composition_helper_candidates {
                handoff.candidate_composition_helper_primitive_kinds.iter().take(4).cloned().collect::<Vec<_>>()
            } else {
                Vec::<String>::new()
            },
            "rationale": ["short reason"]
        })
    } else if has_operator_bundle_candidates {
        serde_json::json!({
            "approved": true,
            "recommended_operator_bundle_ids": ["dashboard_standard_surface"],
            "rationale": ["short reason"]
        })
    } else if has_acceptance_recipe_candidates {
        serde_json::json!({
            "approved": true,
            "recommended_acceptance_recipe_ids": ["directory_audit_contract"],
            "rationale": ["short reason"]
        })
    } else if is_patch {
        serde_json::json!({
            "approved": true,
            "recommended_patch_kind": "json_export",
            "rationale": ["short reason"]
        })
    } else {
        serde_json::json!({
            "approved": true,
            "recommended_family_id": "python_cli_tool",
            "recommended_tool_kind": "directory_audit",
            "rationale": ["short reason"]
        })
    };
    let user_prompt = if has_request_mode_candidates {
        format!(
            "Return a compact request mode choice JSON for this handoff summary: {}. Rules: output one-line JSON only, choose recommended_request_mode from candidate_request_modes, choose recommended_active_project from candidate_active_projects only when the mode is patch_active_project, do not invent project names, keep rationale to one short string, and match this JSON shape exactly: {}",
            serde_json::to_string(&handoff_summary)?,
            serde_json::to_string(&response_shape)?
        )
    } else if has_patch_recipe_candidates {
        format!(
            "Return a compact patch recipe choice JSON for this handoff summary: {}. Rules: output one-line JSON only, choose zero or more recommended_patch_recipe_ids from candidate_patch_recipe_ids, do not invent ids, keep rationale to one short string, and match this JSON shape exactly: {}",
            serde_json::to_string(&handoff_summary)?,
            serde_json::to_string(&response_shape)?
        )
    } else if has_composition_patch_candidates {
        format!(
            "Return a compact bounded composition review JSON for this handoff summary: {}. Rules: output one-line JSON only, choose zero or more recommended_composition_patch_kinds from candidate_composition_patch_kinds, choose zero or more recommended_composition_patch_primitive_classes from candidate_composition_patch_primitive_classes when present, choose zero or more recommended_composition_family_build_primitive_classes from candidate_composition_family_build_primitive_classes when present, choose zero or more recommended_composition_layers from candidate_composition_layers when present, choose zero or more recommended_composition_helper_primitive_ids from candidate_composition_helper_primitive_ids when present, choose zero or more recommended_composition_helper_primitive_kinds from candidate_composition_helper_primitive_kinds when present, use candidate_composition_adapter_semantics to sanity-check whether the execution shape looks too broad, under-supported, or dependency-risky, preserve dependency-safe order when possible, do not invent ids or kinds, keep rationale to one short string, and match this JSON shape exactly: {}",
            serde_json::to_string(&handoff_summary)?,
            serde_json::to_string(&response_shape)?
        )
    } else if has_operator_bundle_candidates {
        format!(
            "Return a compact operator bundle choice JSON for this handoff summary: {}. Rules: output one-line JSON only, choose zero or more recommended_operator_bundle_ids from candidate_operator_bundle_ids, do not invent ids, keep rationale to one short string, and match this JSON shape exactly: {}",
            serde_json::to_string(&handoff_summary)?,
            serde_json::to_string(&response_shape)?
        )
    } else if has_acceptance_recipe_candidates {
        format!(
            "Return a compact acceptance recipe choice JSON for this handoff summary: {}. Rules: output one-line JSON only, choose zero or more recommended_acceptance_recipe_ids from candidate_acceptance_recipe_ids, do not invent ids, keep rationale to one short string, and match this JSON shape exactly: {}",
            serde_json::to_string(&handoff_summary)?,
            serde_json::to_string(&response_shape)?
        )
    } else if is_patch {
        format!(
            "Return a compact planner patch choice JSON for this handoff summary: {}. Rules: output one-line JSON only, choose recommended_patch_kind from available_patch_kinds when one clearly fits, do not invent new patch kinds, keep rationale to one short string, and match this JSON shape exactly: {}",
            serde_json::to_string(&handoff_summary)?,
            serde_json::to_string(&response_shape)?
        )
    } else {
        format!(
            "Return a compact planner build choice JSON for this handoff summary: {}. Rules: output one-line JSON only, choose recommended_family_id from candidate_families, choose recommended_tool_kind from allowed_tool_kinds when relevant, keep rationale to one short string, and match this JSON shape exactly: {}",
            serde_json::to_string(&handoff_summary)?,
            serde_json::to_string(&response_shape)?
        )
    };
    let payload = serde_json::json!({
        "messages": [
            {
                "role": "system",
                "content": "You are the ChattyFactory local planner. Return exactly one minified JSON object and nothing else. Do not think aloud. Do not explain. Do not add markdown or code fences. The first character must be { and the last character must be }. Return only the smallest valid choice object needed for the host to continue."
            },
            {
                "role": "user",
                "content": user_prompt
            }
        ],
        "temperature": 0.2,
        "max_tokens": 72,
        "stream": false
    });

    Ok(serde_json::to_string(&payload)?)
}

fn post_json_request(host: &str, port: u16, path: &str, body: &str) -> Result<String> {
    let url = format!("http://{host}:{port}{path}");
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()?;
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()?;
    if !response.status().is_success() {
        bail!("planner HTTP request failed: {}", response.status());
    }
    Ok(response.text()?)
}

fn parse_planner_response(
    raw_response: &str,
    handoff: &PlannerHandoff,
) -> Result<PlannerParseOutcome> {
    let value: serde_json::Value = serde_json::from_str(raw_response)?;
    let finish_reason = value
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("finish_reason"))
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let message = value
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .ok_or_else(|| anyhow!("planner response did not contain chat completion message"))?;
    let content = message
        .get("content")
        .and_then(|content| content.as_str())
        .unwrap_or_default();
    let reasoning_content = message
        .get("reasoning_content")
        .and_then(|content| content.as_str())
        .unwrap_or_default();
    let combined = if !content.trim().is_empty() {
        content.to_string()
    } else {
        reasoning_content.to_string()
    };
    let (mut response, parse_mode, degraded_recovery_used) = if let Some(json_text) =
        extract_first_json_object(&combined)
            .or_else(|| extract_first_json_object(reasoning_content))
    {
        let parsed_value: serde_json::Value = serde_json::from_str(&json_text)?;
        match serde_json::from_value(parsed_value.clone()) {
            Ok(response) => (response, "strict_json".to_string(), false),
            Err(_) => match normalize_choice_response_value(&parsed_value, handoff) {
                Ok(response) => (response, "host_wrapped_choice_json".to_string(), false),
                Err(_) => (
                    normalize_planner_response_value(&parsed_value, handoff)?,
                    "normalized_json".to_string(),
                    true,
                ),
            },
        }
    } else {
        (
            infer_planner_response_from_text(&combined, handoff)?,
            "inferred_text".to_string(),
            true,
        )
    };
    if response.response_id.trim().is_empty() {
        response.response_id = crate::timestamp_id("planner-response");
    }
    if response.source_handoff_id.trim().is_empty() {
        response.source_handoff_id = handoff.handoff_id.clone();
    }
    if response.source_plan_id.trim().is_empty() {
        response.source_plan_id = handoff.source_plan_id.clone();
    }
    if response.created_at.is_none() {
        response.created_at = Some(crate::timestamp_id("created"));
    }
    Ok(PlannerParseOutcome {
        response,
        parse_mode,
        finish_reason,
        degraded_recovery_used,
    })
}

fn normalize_choice_response_value(
    parsed_value: &serde_json::Value,
    handoff: &PlannerHandoff,
) -> Result<PlannerResponse> {
    let inner = parsed_value.get("planner_choice").unwrap_or(parsed_value);

    let recommended_family_id = inner
        .get("recommended_family_id")
        .cloned()
        .and_then(|value| serde_json::from_value(value).ok())
        .or_else(|| handoff.inferred_family_candidates.first().cloned());

    let recommended_tool_kind = inner
        .get("recommended_tool_kind")
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .or_else(|| handoff.inferred_tool_kind.clone());

    let recommended_request_mode = inner
        .get("recommended_request_mode")
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let recommended_active_project = inner
        .get("recommended_active_project")
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let recommended_patch_kind = inner
        .get("recommended_patch_kind")
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let recommended_patch_recipe_ids =
        string_array(inner.get("recommended_patch_recipe_ids")).unwrap_or_default();
    let recommended_composition_patch_kinds =
        string_array(inner.get("recommended_composition_patch_kinds")).unwrap_or_default();
    let recommended_composition_patch_primitive_classes = string_array(
        inner.get("recommended_composition_patch_primitive_classes"),
    )
    .unwrap_or_default();
    let recommended_composition_family_build_primitive_classes = string_array(
        inner.get("recommended_composition_family_build_primitive_classes"),
    )
    .unwrap_or_default();
    let recommended_composition_layers =
        string_array(inner.get("recommended_composition_layers")).unwrap_or_default();
    let recommended_composition_helper_primitive_ids =
        string_array(inner.get("recommended_composition_helper_primitive_ids")).unwrap_or_default();
    let recommended_composition_helper_primitive_kinds = string_array(
        inner.get("recommended_composition_helper_primitive_kinds"),
    )
    .unwrap_or_default();
    let recommended_operator_bundle_ids =
        string_array(inner.get("recommended_operator_bundle_ids")).unwrap_or_default();
    let recommended_acceptance_recipe_ids =
        string_array(inner.get("recommended_acceptance_recipe_ids")).unwrap_or_default();

    let rationale = string_array(inner.get("rationale")).unwrap_or_else(|| {
        inner
            .get("rationale")
            .and_then(|value| value.as_str())
            .map(|value| vec![value.to_string()])
            .unwrap_or_else(|| vec!["selected the smallest honest deterministic lane".into()])
    });

    if recommended_request_mode.is_none()
        && recommended_family_id.is_none()
        && recommended_tool_kind.is_none()
        && recommended_patch_kind.is_none()
        && recommended_patch_recipe_ids.is_empty()
        && recommended_composition_patch_kinds.is_empty()
        && recommended_composition_patch_primitive_classes.is_empty()
        && recommended_composition_family_build_primitive_classes.is_empty()
        && recommended_composition_layers.is_empty()
        && recommended_composition_helper_primitive_ids.is_empty()
        && recommended_composition_helper_primitive_kinds.is_empty()
        && recommended_operator_bundle_ids.is_empty()
        && recommended_acceptance_recipe_ids.is_empty()
    {
        bail!("planner choice object did not contain actionable fields");
    }

    Ok(PlannerResponse {
        response_id: crate::timestamp_id("planner-response"),
        source_handoff_id: handoff.handoff_id.clone(),
        source_plan_id: handoff.source_plan_id.clone(),
        approved: inner
            .get("approved")
            .and_then(|value| value.as_bool())
            .unwrap_or(true),
        recommended_request_mode,
        recommended_active_project,
        recommended_family_id,
        recommended_tool_kind,
        recommended_patch_kind,
        recommended_patch_recipe_ids,
        recommended_composition_patch_kinds,
        recommended_composition_patch_primitive_classes,
        recommended_composition_family_build_primitive_classes,
        recommended_composition_layers,
        recommended_composition_helper_primitive_ids,
        recommended_composition_helper_primitive_kinds,
        recommended_operator_bundle_ids,
        recommended_operator_ids: Vec::new(),
        recommended_acceptance_recipe_ids,
        rationale,
        execution_steps: if matches!(handoff.mode, Some(crate::RequestMode::Patch)) {
            vec!["apply the recommended deterministic patch lane".into()]
        } else {
            vec!["route to the recommended family and continue deterministic build".into()]
        },
        acceptance_notes: vec!["verify the deterministic output contract after execution".into()],
        acceptance_checks_to_add: Vec::new(),
        required_markers_to_add: Vec::new(),
        acceptance_commands_to_add: Vec::new(),
        expected_outputs_to_add: Vec::new(),
        suggested_patch_kinds: Vec::new(),
        suggested_features: Vec::new(),
        created_at: Some(crate::timestamp_id("created")),
    })
}

fn normalize_planner_response_value(
    parsed_value: &serde_json::Value,
    handoff: &PlannerHandoff,
) -> Result<PlannerResponse> {
    let inner = parsed_value.get("planner_response").unwrap_or(parsed_value);

    let recommended_family_id = inner
        .get("recommended_family_id")
        .cloned()
        .or_else(|| {
            inner
                .get("structured_family_recommendation")
                .and_then(|value| value.as_array().and_then(|items| items.first()).cloned())
        })
        .and_then(|value| serde_json::from_value(value).ok());

    let recommended_patch_kind = inner
        .get("recommended_patch_kind")
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .or_else(|| {
            inner
                .get("recommended_patch_kinds")
                .and_then(|value| value.as_array())
                .and_then(|items| items.first())
                .and_then(|value| value.as_str())
                .map(str::to_string)
        });

    let rationale = string_array(inner.get("rationale"))
        .or_else(|| string_array(inner.get("repair_hints")))
        .unwrap_or_default();
    let execution_steps = string_array(inner.get("execution_steps")).unwrap_or_default();
    let acceptance_notes = string_array(inner.get("acceptance_notes"))
        .or_else(|| string_array(inner.get("acceptance_expectations")))
        .unwrap_or_default();

    Ok(PlannerResponse {
        response_id: crate::timestamp_id("planner-response"),
        source_handoff_id: handoff.handoff_id.clone(),
        source_plan_id: handoff.source_plan_id.clone(),
        approved: true,
        recommended_request_mode: None,
        recommended_active_project: None,
        recommended_family_id,
        recommended_tool_kind: inner
            .get("recommended_tool_kind")
            .and_then(|value| value.as_str())
            .map(str::to_string),
        recommended_patch_kind,
        recommended_patch_recipe_ids: string_array(inner.get("recommended_patch_recipe_ids"))
            .or_else(|| string_array(inner.get("patch_recipe_ids")))
            .unwrap_or_default(),
        recommended_composition_patch_kinds: string_array(
            inner.get("recommended_composition_patch_kinds"),
        )
        .or_else(|| string_array(inner.get("composition_patch_kinds")))
        .unwrap_or_default(),
        recommended_composition_patch_primitive_classes: string_array(
            inner.get("recommended_composition_patch_primitive_classes"),
        )
        .or_else(|| string_array(inner.get("composition_patch_primitive_classes")))
        .unwrap_or_default(),
        recommended_composition_family_build_primitive_classes: string_array(
            inner.get("recommended_composition_family_build_primitive_classes"),
        )
        .or_else(|| string_array(inner.get("composition_family_build_primitive_classes")))
        .unwrap_or_default(),
        recommended_composition_layers: string_array(
            inner.get("recommended_composition_layers"),
        )
        .or_else(|| string_array(inner.get("composition_layers")))
        .unwrap_or_default(),
        recommended_composition_helper_primitive_ids: string_array(
            inner.get("recommended_composition_helper_primitive_ids"),
        )
        .or_else(|| string_array(inner.get("composition_helper_primitive_ids")))
        .unwrap_or_default(),
        recommended_composition_helper_primitive_kinds: string_array(
            inner.get("recommended_composition_helper_primitive_kinds"),
        )
        .or_else(|| string_array(inner.get("composition_helper_primitive_kinds")))
        .unwrap_or_default(),
        recommended_operator_bundle_ids: string_array(inner.get("recommended_operator_bundle_ids"))
            .or_else(|| string_array(inner.get("operator_bundle_ids")))
            .unwrap_or_default(),
        recommended_operator_ids: string_array(inner.get("recommended_operator_ids"))
            .or_else(|| string_array(inner.get("operator_ids")))
            .unwrap_or_default(),
        recommended_acceptance_recipe_ids: string_array(
            inner.get("recommended_acceptance_recipe_ids"),
        )
        .or_else(|| string_array(inner.get("acceptance_recipe_ids")))
        .unwrap_or_default(),
        rationale,
        execution_steps,
        acceptance_notes,
        acceptance_checks_to_add: Vec::new(),
        required_markers_to_add: Vec::new(),
        acceptance_commands_to_add: Vec::new(),
        expected_outputs_to_add: Vec::new(),
        suggested_patch_kinds: string_array(inner.get("suggested_patch_kinds"))
            .or_else(|| string_array(inner.get("future_patch_kinds")))
            .or_else(|| string_array(inner.get("recommended_patch_kinds")))
            .unwrap_or_default(),
        suggested_features: string_array(inner.get("suggested_features"))
            .or_else(|| string_array(inner.get("feature_suggestions")))
            .unwrap_or_default(),
        created_at: Some(crate::timestamp_id("created")),
    })
}

fn string_array(value: Option<&serde_json::Value>) -> Option<Vec<String>> {
    let items = value?.as_array()?;
    Some(
        items
            .iter()
            .filter_map(|item| item.as_str().map(str::to_string))
            .collect(),
    )
}

fn infer_planner_response_from_text(
    text: &str,
    handoff: &PlannerHandoff,
) -> Result<PlannerResponse> {
    let lower = text.to_ascii_lowercase();
    let recommended_family_id = if lower.contains("python_cli_tool") {
        Some(crate::FamilyId::PythonCliTool)
    } else if lower.contains("rust_cli_tool") {
        Some(crate::FamilyId::RustCliTool)
    } else if lower.contains("chattycog_chattyedu_native_window_module") {
        Some(crate::FamilyId::ChattycogChattyeduNativeWindowModule)
    } else if lower.contains("chattycog_native_window_module") {
        Some(crate::FamilyId::ChattycogNativeWindowModule)
    } else if lower.contains("chattyedu_native_window_module") {
        Some(crate::FamilyId::ChattyeduNativeWindowModule)
    } else if lower.contains("chattycog_workspace_module") {
        Some(crate::FamilyId::ChattycogWorkspaceModule)
    } else if lower.contains("chattycog_webview_module") {
        Some(crate::FamilyId::ChattycogWebviewModule)
    } else if lower.contains("static_web_dashboard")
        || lower.contains("static web dashboard")
        || lower.contains("browser dashboard")
        || lower.contains("web dashboard")
    {
        Some(crate::FamilyId::StaticWebDashboard)
    } else {
        handoff.inferred_family_candidates.first().cloned()
    };

    let recommended_tool_kind = if lower.contains("directory_audit") {
        Some("directory_audit".to_string())
    } else if lower.contains("csv_report") {
        Some("csv_report".to_string())
    } else if lower.contains("log_summary") {
        Some("log_summary".to_string())
    } else if lower.contains("text_stats") {
        Some("text_stats".to_string())
    } else if lower.contains("file_sorter") {
        Some("file_sorter".to_string())
    } else {
        handoff.inferred_tool_kind.clone()
    };

    let recommended_patch_kind = if lower.contains("json_export") {
        Some("json_export".to_string())
    } else if lower.contains("column_filter") {
        Some("column_filter".to_string())
    } else if lower.contains("severity_filter") {
        Some("severity_filter".to_string())
    } else if lower.contains("file_output") {
        Some("file_output".to_string())
    } else if lower.contains("progress_banner") {
        Some("progress_banner".to_string())
    } else {
        None
    };

    Ok(PlannerResponse {
        response_id: crate::timestamp_id("planner-response"),
        source_handoff_id: handoff.handoff_id.clone(),
        source_plan_id: handoff.source_plan_id.clone(),
        approved: true,
        recommended_request_mode: None,
        recommended_active_project: None,
        recommended_family_id,
        recommended_tool_kind,
        recommended_patch_kind,
        recommended_patch_recipe_ids: Vec::new(),
        recommended_composition_patch_kinds: Vec::new(),
        recommended_composition_patch_primitive_classes: Vec::new(),
        recommended_composition_family_build_primitive_classes: Vec::new(),
        recommended_composition_layers: Vec::new(),
        recommended_composition_helper_primitive_ids: Vec::new(),
        recommended_composition_helper_primitive_kinds: Vec::new(),
        recommended_operator_bundle_ids: Vec::new(),
        recommended_operator_ids: Vec::new(),
        recommended_acceptance_recipe_ids: Vec::new(),
        rationale: vec!["derived from incomplete planner text response".into()],
        execution_steps: if matches!(handoff.mode, Some(crate::RequestMode::Patch)) {
            vec!["apply the recommended deterministic patch lane".into()]
        } else {
            vec!["route to the recommended family and continue deterministic build".into()]
        },
        acceptance_notes: vec!["verify the deterministic output contract after execution".into()],
        acceptance_checks_to_add: Vec::new(),
        required_markers_to_add: Vec::new(),
        acceptance_commands_to_add: Vec::new(),
        expected_outputs_to_add: Vec::new(),
        suggested_patch_kinds: Vec::new(),
        suggested_features: Vec::new(),
        created_at: Some(crate::timestamp_id("created")),
    })
}

fn extract_first_json_object(text: &str) -> Option<String> {
    let mut start = None;
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;
    for (index, ch) in text.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '{' => {
                if start.is_none() {
                    start = Some(index);
                }
                depth += 1;
            }
            '}' => {
                if depth > 0 {
                    depth -= 1;
                    if depth == 0 {
                        if let Some(start_index) = start {
                            return Some(text[start_index..=index].to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FamilyId, PlannerHandoff, RequestMode};

    #[test]
    fn parser_does_not_force_static_web_from_generic_dashboard_word() {
        let handoff = PlannerHandoff {
            handoff_id: "handoff-1".into(),
            request_id: "request-1".into(),
            source_plan_id: "plan-1".into(),
            mode: Some(RequestMode::NewBuild),
            interpreted_goal: "Build a project".into(),
            inferred_family_candidates: vec![FamilyId::ChattycogNativeWindowModule],
            inferred_tool_kind: Some("native_window_starter".into()),
            ..Default::default()
        };

        let parsed = infer_planner_response_from_text(
            "approved with dashboard tool kind and native shell intent",
            &handoff,
        )
        .expect("planner response should parse");

        assert_eq!(
            parsed.recommended_family_id,
            Some(FamilyId::ChattycogNativeWindowModule)
        );
    }
}
