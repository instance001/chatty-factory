use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::contracts::{ProjectBrowserState, ProjectCatalogEntry, ProjectSession, ProjectSpec};
use crate::{persist_json_pretty, timestamp_id};

pub fn discover_projects(
    output_root: &Path,
    preferred_session: Option<&ProjectSession>,
) -> Result<Vec<ProjectCatalogEntry>> {
    if !output_root.exists() {
        return Ok(Vec::new());
    }

    let mut projects = Vec::new();
    for entry in fs::read_dir(output_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.join("ProjectSpec.json").exists() {
            let spec_path = path.join("ProjectSpec.json");
            let spec: ProjectSpec = serde_json::from_str(&fs::read_to_string(&spec_path)?)?;
            let modified = fs::metadata(&spec_path)
                .and_then(|metadata| metadata.modified())
                .ok();
            let recency_hint = modified
                .and_then(|stamp| stamp.elapsed().ok())
                .map(|elapsed| {
                    let seconds = elapsed.as_secs();
                    if seconds < 3600 {
                        "recent".to_string()
                    } else if seconds < 86_400 {
                        "today".to_string()
                    } else {
                        "older".to_string()
                    }
                })
                .unwrap_or_else(|| "unknown".to_string());

            projects.push(ProjectCatalogEntry {
                project_name: entry.file_name().to_string_lossy().to_string(),
                substrate: spec.substrate,
                tool_kind: spec.tool_kind,
                request_summary: spec.request_summary,
                recency_hint,
            });
        }
    }

    let preferred_name = preferred_session.map(|session| session.project_name.as_str());
    projects.sort_by(|left, right| {
        session_rank(&left.project_name, preferred_name)
            .cmp(&session_rank(&right.project_name, preferred_name))
            .then_with(|| {
                recency_rank(&left.recency_hint)
                    .cmp(&recency_rank(&right.recency_hint))
                    .then_with(|| left.project_name.cmp(&right.project_name))
            })
    });

    Ok(projects)
}

pub fn active_project_summary_line(project: &ProjectCatalogEntry) -> String {
    let surface = if project.substrate.trim().is_empty() {
        "unknown_surface"
    } else {
        project.substrate.as_str()
    };
    let tool = project.tool_kind.as_deref().unwrap_or("none");
    let summary = project
        .request_summary
        .as_deref()
        .unwrap_or("no summary recorded");
    format!(
        "{} | surface={} | tool={} | recency={} | summary={}",
        project.project_name, surface, tool, project.recency_hint, summary
    )
}

pub fn load_project_session(
    runtime_root: &Path,
    file_name: &str,
) -> Result<Option<ProjectSession>> {
    let session_path = runtime_root.join(file_name);
    if !session_path.exists() {
        return Ok(None);
    }
    let session = serde_json::from_str(&fs::read_to_string(session_path)?)?;
    Ok(Some(session))
}

pub fn persist_project_session(
    runtime_root: &Path,
    current_file_name: &str,
    history_dir_name: &str,
    session: &ProjectSession,
) -> Result<()> {
    persist_json_pretty(&runtime_root.join(current_file_name), session)?;
    persist_json_pretty(
        &runtime_root.join(history_dir_name).join(format!(
            "{}-{}.json",
            session.project_name, session.updated_at
        )),
        session,
    )?;
    Ok(())
}

pub fn build_project_browser_state(
    output_root: &Path,
    runtime_root: &Path,
) -> Result<ProjectBrowserState> {
    let selected_project_session =
        load_project_session(runtime_root, "selected_project_session.json")?;
    let active_project_session = load_project_session(runtime_root, "active_project_session.json")?;
    let preferred_session = selected_project_session
        .as_ref()
        .or(active_project_session.as_ref());
    let projects = discover_projects(output_root, preferred_session)?;
    Ok(ProjectBrowserState {
        state_id: timestamp_id("project-browser-state"),
        selected_project_session,
        active_project_session,
        projects,
        updated_at: Some(timestamp_id("updated")),
    })
}

pub fn persist_project_browser_state(
    output_root: &Path,
    runtime_root: &Path,
) -> Result<ProjectBrowserState> {
    let state = build_project_browser_state(output_root, runtime_root)?;
    persist_json_pretty(&runtime_root.join("project_browser_state.json"), &state)?;
    Ok(state)
}

fn session_rank(project_name: &str, active_name: Option<&str>) -> u8 {
    match active_name {
        Some(active_name) if project_name == active_name => 0,
        _ => 1,
    }
}

fn recency_rank(value: &str) -> u8 {
    match value {
        "recent" => 0,
        "today" => 1,
        "older" => 2,
        _ => 3,
    }
}
