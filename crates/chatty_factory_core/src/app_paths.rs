use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

const APP_BASE_ENV: &str = "CHATTY_FACTORY_BASE_PATH";

pub fn workspace_root() -> Result<PathBuf> {
    if let Ok(value) = std::env::var(APP_BASE_ENV) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Ok(PathBuf::from(trimmed));
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        for dir in exe.ancestors().filter(|path| path.is_dir()) {
            if dir.join("Cargo.toml").is_file()
                && dir.join("crates").is_dir()
                && dir.join("templates").is_dir()
            {
                return Ok(dir.to_path_buf());
            }
        }

        if let Some(dir) = exe.parent() {
            return Ok(dir.to_path_buf());
        }
    }

    std::env::current_dir().context("could not determine ChattyFactory workspace root")
}

pub fn ensure_workspace_dirs(root: &Path) -> Result<()> {
    let dirs = [
        root.join("output"),
        root.join("runtime"),
        root.join("models"),
        root.join("operator_registry"),
        root.join("extensions"),
    ];

    for dir in dirs {
        std::fs::create_dir_all(&dir).with_context(|| format!("mkdir {}", dir.display()))?;
    }

    Ok(())
}

pub fn resolve_and_prepare_workspace_root() -> Result<PathBuf> {
    let root = workspace_root()?;
    ensure_workspace_dirs(&root)?;
    Ok(root)
}
