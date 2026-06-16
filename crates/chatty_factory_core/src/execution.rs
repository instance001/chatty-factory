use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::{timestamp_id, ExecutionPolicy, ExecutionReceipt, ExecutionSmokeCheck, FamilyId};

pub fn build_execution_policy(
    request_id: &str,
    output_root: &Path,
    project_dir: &Path,
    family_id: Option<&FamilyId>,
    entrypoints: &[String],
) -> Result<ExecutionPolicy> {
    if !project_dir.exists() {
        bail!(
            "execution policy failed: project directory does not exist: {}",
            project_dir.display()
        );
    }
    if !project_dir.starts_with(output_root) {
        bail!(
            "execution policy failed: project directory is outside output root: {}",
            project_dir.display()
        );
    }
    for entrypoint in entrypoints {
        if !is_safe_relpath(entrypoint) {
            bail!(
                "execution policy failed: entrypoint path is not a safe relative path: {}",
                entrypoint
            );
        }
    }

    let (allowed_commands, substrate_smoke_checks, notes) = match family_id {
        Some(FamilyId::PythonCliTool) => (
            vec!["py".into(), "python".into(), "python3".into()],
            vec![
                "project_root_confined".into(),
                "entrypoints_exist".into(),
                "python_py_compile".into(),
            ],
            vec!["python-backed outputs get a deterministic syntax smoke pass".into()],
        ),
        Some(FamilyId::ChattycogNativeWindowModule)
        | Some(FamilyId::ChattyeduNativeWindowModule)
        | Some(FamilyId::ChattycogChattyeduNativeWindowModule) => (
            vec!["cargo".into()],
            vec![
                "project_root_confined".into(),
                "entrypoints_exist".into(),
                "cargo_manifest_guardrails".into(),
                "cargo_metadata_offline".into(),
            ],
            vec![
                "native Rust dashboard outputs get nested workspace and cargo metadata guardrails"
                    .into(),
            ],
        ),
        Some(FamilyId::RustCliTool) => (
            vec!["cargo".into()],
            vec![
                "project_root_confined".into(),
                "entrypoints_exist".into(),
                "cargo_manifest_guardrails".into(),
                "cargo_metadata_offline".into(),
            ],
            vec!["rust cli outputs get nested workspace and cargo metadata guardrails".into()],
        ),
        Some(FamilyId::StaticWebDashboard)
        | Some(FamilyId::ChattycogWebviewModule)
        | Some(FamilyId::ChattycogWorkspaceModule) => (
            Vec::new(),
            vec!["project_root_confined".into(), "entrypoints_exist".into()],
            vec![
                "web families are currently guarded by path confinement and entrypoint checks"
                    .into(),
            ],
        ),
        None => (
            Vec::new(),
            vec!["project_root_confined".into(), "entrypoints_exist".into()],
            vec!["unknown family execution policy fell back to generic confinement checks".into()],
        ),
    };

    Ok(ExecutionPolicy {
        policy_id: timestamp_id("execution-policy"),
        request_id: request_id.to_string(),
        family_id: family_id.cloned(),
        project_dir: project_dir.display().to_string(),
        allowed_root: output_root.display().to_string(),
        allowed_entrypoints: entrypoints.to_vec(),
        allowed_commands,
        substrate_smoke_checks,
        notes,
        created_at: Some(timestamp_id("created")),
    })
}

pub fn run_execution_policy(policy: &ExecutionPolicy) -> Result<ExecutionReceipt> {
    let project_dir = PathBuf::from(&policy.project_dir);
    let allowed_root = PathBuf::from(&policy.allowed_root);
    let mut checks = Vec::new();

    checks.push(run_project_root_confined_check(
        &project_dir,
        &allowed_root,
    )?);
    checks.push(run_entrypoints_exist_check(
        &project_dir,
        &policy.allowed_entrypoints,
    )?);

    match policy.family_id.as_ref() {
        Some(FamilyId::PythonCliTool) => {
            checks.push(run_python_py_compile_check(
                &project_dir,
                &policy.allowed_entrypoints,
            )?);
        }
        Some(FamilyId::ChattycogNativeWindowModule)
        | Some(FamilyId::ChattyeduNativeWindowModule)
        | Some(FamilyId::ChattycogChattyeduNativeWindowModule) => {
            checks.push(run_cargo_manifest_guardrails_check(&project_dir)?);
            checks.push(run_cargo_metadata_offline_check(&project_dir)?);
        }
        Some(FamilyId::RustCliTool) => {
            checks.push(run_cargo_manifest_guardrails_check(&project_dir)?);
            checks.push(run_cargo_metadata_offline_check(&project_dir)?);
        }
        _ => {}
    }

    if let Some(failed) = checks.iter().find(|check| check.status != "passed") {
        bail!(
            "execution policy failed: {} ({})",
            failed.summary,
            failed.kind
        );
    }

    Ok(ExecutionReceipt {
        receipt_id: timestamp_id("execution-receipt"),
        request_id: policy.request_id.clone(),
        policy_id: policy.policy_id.clone(),
        family_id: policy.family_id.clone(),
        project_dir: policy.project_dir.clone(),
        status: "passed".into(),
        smoke_checks: checks,
        notes: policy.notes.clone(),
        created_at: Some(timestamp_id("created")),
    })
}

fn run_project_root_confined_check(
    project_dir: &Path,
    allowed_root: &Path,
) -> Result<ExecutionSmokeCheck> {
    if !project_dir.starts_with(allowed_root) {
        bail!(
            "project root is outside the allowed output root: {}",
            project_dir.display()
        );
    }
    Ok(ExecutionSmokeCheck {
        check_id: "project_root_confined".into(),
        kind: "policy".into(),
        status: "passed".into(),
        summary: "project root is confined under the output root".into(),
        details: vec![
            format!("project_dir={}", project_dir.display()),
            format!("allowed_root={}", allowed_root.display()),
        ],
    })
}

fn run_entrypoints_exist_check(
    project_dir: &Path,
    entrypoints: &[String],
) -> Result<ExecutionSmokeCheck> {
    for entrypoint in entrypoints {
        let path = project_dir.join(entrypoint);
        if !path.exists() {
            bail!("expected entrypoint is missing: {}", entrypoint);
        }
    }
    Ok(ExecutionSmokeCheck {
        check_id: "entrypoints_exist".into(),
        kind: "file_check".into(),
        status: "passed".into(),
        summary: "all declared entrypoints exist".into(),
        details: entrypoints
            .iter()
            .map(|entrypoint| format!("entrypoint={entrypoint}"))
            .collect(),
    })
}

fn run_python_py_compile_check(
    project_dir: &Path,
    entrypoints: &[String],
) -> Result<ExecutionSmokeCheck> {
    let script = entrypoints
        .iter()
        .find(|entrypoint| entrypoint.ends_with(".py"))
        .cloned()
        .unwrap_or_else(|| "main.py".into());

    let candidates = [
        vec!["py", "-3", "-m", "py_compile", script.as_str()],
        vec!["py", "-m", "py_compile", script.as_str()],
        vec!["python", "-m", "py_compile", script.as_str()],
        vec!["python3", "-m", "py_compile", script.as_str()],
    ];

    let mut attempted = Vec::new();
    let mut last_error = None;
    for candidate in candidates {
        attempted.push(candidate.join(" "));
        let mut cmd = Command::new(candidate[0]);
        cmd.args(&candidate[1..]).current_dir(project_dir);
        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    return Ok(ExecutionSmokeCheck {
                        check_id: "python_py_compile".into(),
                        kind: "syntax_smoke".into(),
                        status: "passed".into(),
                        summary: format!("python syntax smoke passed for {}", script),
                        details: vec![format!("command={}", candidate.join(" "))],
                    });
                }
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                last_error = Some(format!("{} => {}", candidate.join(" "), stderr));
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                last_error = Some(format!("{} => interpreter not found", candidate.join(" ")));
            }
            Err(err) => {
                last_error = Some(format!("{} => {}", candidate.join(" "), err));
            }
        }
    }

    bail!(
        "python syntax smoke failed for {}: {}",
        script,
        last_error.unwrap_or_else(|| format!("attempted={}", attempted.join(" | ")))
    )
}

fn run_cargo_manifest_guardrails_check(project_dir: &Path) -> Result<ExecutionSmokeCheck> {
    let cargo_toml = project_dir.join("Cargo.toml");
    let text = fs::read_to_string(&cargo_toml)
        .with_context(|| format!("failed to read {}", cargo_toml.display()))?;
    if text.contains("@package") {
        bail!("cargo manifest contains invalid '@package' header");
    }
    if !text.contains("[package]") {
        bail!("cargo manifest is missing [package]");
    }
    if !text.contains("[workspace]") {
        bail!("cargo manifest is missing [workspace] for nested project safety");
    }
    Ok(ExecutionSmokeCheck {
        check_id: "cargo_manifest_guardrails".into(),
        kind: "manifest_guardrail".into(),
        status: "passed".into(),
        summary: "cargo manifest passed nested-project guardrails".into(),
        details: vec![format!("manifest={}", cargo_toml.display())],
    })
}

fn run_cargo_metadata_offline_check(project_dir: &Path) -> Result<ExecutionSmokeCheck> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps")
        .arg("--offline")
        .current_dir(project_dir)
        .output()
        .context("failed to spawn cargo for metadata smoke")?;
    if !output.status.success() {
        bail!(
            "cargo metadata offline smoke failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(ExecutionSmokeCheck {
        check_id: "cargo_metadata_offline".into(),
        kind: "toolchain_smoke".into(),
        status: "passed".into(),
        summary: "cargo metadata offline smoke passed".into(),
        details: vec!["command=cargo metadata --format-version 1 --no-deps --offline".into()],
    })
}

fn is_safe_relpath(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.starts_with('/') || trimmed.starts_with('\\') || trimmed.contains(':') {
        return false;
    }
    let path = Path::new(trimmed);
    !path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
}
