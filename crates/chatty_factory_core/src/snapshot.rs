use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

use crate::{timestamp_id, ContextBundle, ProjectSnapshot, ProjectSpec, SnapshotGateResult};

pub fn build_project_snapshot(project_dir: &Path, spec: &ProjectSpec) -> Result<ProjectSnapshot> {
    let mut discovered_files = Vec::new();
    collect_relative_files(project_dir, project_dir, &mut discovered_files)?;
    discovered_files.sort();

    Ok(ProjectSnapshot {
        snapshot_id: timestamp_id("project-snapshot"),
        project_name: spec.project_name.clone(),
        project_dir: project_dir.display().to_string(),
        family_id: spec.family_id.clone(),
        tool_kind: spec.tool_kind.clone(),
        entrypoints: spec.entrypoints.clone(),
        expected_files: spec.expected_files.clone(),
        discovered_files,
        created_at: Some(timestamp_id("created")),
    })
}

pub fn build_context_bundle(
    request_id: &str,
    spec: &ProjectSpec,
    snapshot: &ProjectSnapshot,
) -> ContextBundle {
    ContextBundle {
        context_id: timestamp_id("context-bundle"),
        request_id: request_id.to_string(),
        project_name: spec.project_name.clone(),
        family_id: spec.family_id.clone(),
        tool_kind: spec.tool_kind.clone(),
        request_summary: spec.request_summary.clone(),
        entrypoints: spec.entrypoints.clone(),
        expected_files: spec.expected_files.clone(),
        snapshot_file_count: snapshot.discovered_files.len(),
        snapshot_preview: snapshot.discovered_files.iter().take(24).cloned().collect(),
        created_at: Some(timestamp_id("created")),
    }
}

pub fn gate_patch_project_snapshot(
    request_id: &str,
    snapshot: &ProjectSnapshot,
    spec: &ProjectSpec,
) -> Result<SnapshotGateResult> {
    let mut checked_paths = Vec::new();
    let mut missing_paths = Vec::new();

    for path in spec.entrypoints.iter().chain(spec.expected_files.iter()) {
        if !is_safe_relpath(path) {
            bail!(
                "snapshot gate failed: unsafe project-relative path declared: {}",
                path
            );
        }
        if checked_paths.iter().any(|existing| existing == path) {
            continue;
        }
        checked_paths.push(path.clone());
        if !snapshot
            .discovered_files
            .iter()
            .any(|existing| existing == path)
        {
            missing_paths.push(path.clone());
        }
    }

    let mut rationale = vec![
        "patch gating uses ProjectSpec entrypoints and expected files as the grounded scope".into(),
        format!("snapshot_file_count={}", snapshot.discovered_files.len()),
    ];
    if missing_paths.is_empty() {
        rationale
            .push("all declared patch-scope files were present in the project snapshot".into());
    } else {
        rationale.push(
            "one or more declared patch-scope files were missing from the project snapshot".into(),
        );
    }

    Ok(SnapshotGateResult {
        gate_id: timestamp_id("snapshot-gate"),
        request_id: request_id.to_string(),
        project_name: spec.project_name.clone(),
        status: if missing_paths.is_empty() {
            "passed".into()
        } else {
            "failed".into()
        },
        checked_paths,
        missing_paths,
        rationale,
        created_at: Some(timestamp_id("created")),
    })
}

fn collect_relative_files(
    project_root: &Path,
    current_dir: &Path,
    out: &mut Vec<String>,
) -> Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name == "target" || name == ".git" || name == "__pycache__" {
                continue;
            }
            collect_relative_files(project_root, &path, out)?;
        } else if path.is_file() {
            let rel = path
                .strip_prefix(project_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            out.push(rel);
        }
    }
    Ok(())
}

fn is_safe_relpath(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.starts_with('/') || trimmed.starts_with('\\') || trimmed.contains(':') {
        return false;
    }
    let path = PathBuf::from(trimmed);
    !path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
}
