use anyhow::{bail, Result};
use colored::Colorize;

use crate::config::Config;
use crate::registry::Registry;

pub async fn run(components: Vec<String>) -> Result<()> {
    // Check if project is initialized
    if !Config::exists() {
        bail!("gpui-ui is not initialized in this directory. Run 'gpui-ui init' first.");
    }

    let config = Config::load()?;
    let registry = Registry::new();

    if components.is_empty() {
        // Update all installed components
        if config.components.is_empty() {
            println!("{}", "No components installed.".yellow());
            println!("Run {} to add components", "gpui-ui add <component>".cyan());
            return Ok(());
        }

        println!("{}", "Updating all components...".cyan().bold());
        println!();

        for installed in &config.components {
            // Check if component exists in registry
            match registry.get_component(&installed.name) {
                Ok(component) => {
                    if component.version == installed.version {
                        println!(
                            "  {} {} is already up to date (v{})",
                            "✓".green(),
                            installed.name,
                            installed.version
                        );
                    } else {
                        println!(
                            "  {} {} {} → {}",
                            "→".cyan(),
                            installed.name,
                            installed.version.dimmed(),
                            component.version.green()
                        );
                        // Note: Actual update logic would use the add command with --force
                        println!(
                            "    {} Run {} to update",
                            "ℹ".blue(),
                            format!("gpui-ui add {} --force", installed.name).cyan()
                        );
                    }
                }
                Err(_) => {
                    println!(
                        "  {} {} not found in registry (may have been removed)",
                        "⚠".yellow(),
                        installed.name
                    );
                }
            }
        }
    } else {
        // Update specific components
        println!("{}", "Checking for updates...".cyan().bold());
        println!();

        for component_name in components {
            // Check if installed
            let installed = config
                .components
                .iter()
                .find(|c| c.name == component_name);

            if installed.is_none() {
                println!(
                    "  {} {} is not installed",
                    "✗".red(),
                    component_name.red()
                );
                continue;
            }

            let installed = installed.unwrap();

            // Check if exists in registry
            match registry.get_component(&component_name) {
                Ok(component) => {
                    if component.version == installed.version {
                        println!(
                            "  {} {} is already up to date (v{})",
                            "✓".green(),
                            component_name,
                            installed.version
                        );
                    } else {
                        println!(
                            "  {} {} {} → {}",
                            "→".cyan(),
                            component_name,
                            installed.version.dimmed(),
                            component.version.green()
                        );
                        println!(
                            "    {} Run {} to update",
                            "ℹ".blue(),
                            format!("gpui-ui add {} --force", component_name).cyan()
                        );
                    }
                }
                Err(_) => {
                    println!(
                        "  {} {} not found in registry",
                        "✗".red(),
                        component_name.red()
                    );
                }
            }
        }
    }

    println!();
    println!("{}", "Note:".yellow().bold());
    println!(
        "Use {} to update and overwrite existing files",
        "gpui-ui add <component> --force".cyan()
    );

    Ok(())
}
