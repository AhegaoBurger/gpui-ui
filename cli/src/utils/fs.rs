use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Get the root directory of the gpui-ui project (where components/ is located)
pub fn get_project_root() -> Result<PathBuf> {
    // The CLI binary is typically built in target/debug/ or target/release/
    // So we need to go up to find the project root
    let current_exe = std::env::current_exe()
        .context("Failed to get current executable path")?;

    // Try to find the project root by looking for the components directory
    let mut path = current_exe.parent().context("No parent directory")?;

    // Go up until we find the components directory or hit the root
    loop {
        let components_path = path.join("components");
        if components_path.exists() && components_path.is_dir() {
            return Ok(path.to_path_buf());
        }

        if let Some(parent) = path.parent() {
            path = parent;
        } else {
            break;
        }
    }

    // If we're running via cargo run, try using the CARGO_MANIFEST_DIR
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let manifest_path = PathBuf::from(manifest_dir);
        // We're in cli/, so go up one level
        if let Some(parent) = manifest_path.parent() {
            let components_path = parent.join("components");
            if components_path.exists() {
                return Ok(parent.to_path_buf());
            }
        }
    }

    anyhow::bail!("Could not find gpui-ui project root (looking for components/ directory)")
}

/// Copy a file from source to destination, creating parent directories if needed
pub fn copy_file(source: &Path, dest: &Path, force: bool) -> Result<()> {
    // Check if destination exists
    if dest.exists() && !force {
        anyhow::bail!(
            "File {} already exists. Use --force to overwrite.",
            dest.display()
        );
    }

    // Create parent directories
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create directory {}", parent.display()))?;
    }

    // Copy the file
    fs::copy(source, dest).context(format!(
        "Failed to copy {} to {}",
        source.display(),
        dest.display()
    ))?;

    Ok(())
}

/// Read the contents of a file as a string
pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .context(format!("Failed to read file {}", path.display()))
}

/// Check if a file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// Get the component source directory
pub fn get_component_source_dir() -> Result<PathBuf> {
    let root = get_project_root()?;
    Ok(root.join("components/src"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_project_root() {
        // This test will only work when running from the project directory
        let result = get_project_root();
        if let Ok(root) = result {
            assert!(root.join("components").exists());
        }
    }
}

