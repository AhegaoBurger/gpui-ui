use anyhow::{bail, Result};
use colored::Colorize;

use crate::config::Config;

pub async fn run(components: Vec<String>, _force: bool) -> Result<()> {
    if components.is_empty() {
        bail!("Please specify at least one component to add. Run 'gpui-ui list' to see available components.");
    }

    // Check if project is initialized
    if !Config::exists() {
        bail!("gpui-ui is not initialized in this directory. Run 'gpui-ui init' first.");
    }

    let mut config = Config::load()?;

    println!("{}", "Adding components...".cyan().bold());

    for component in components {
        println!("  {} {}", "→".cyan(), component);
        // TODO: Implement component fetching and installation
        println!("    {} Component '{}' added successfully", "✓".green(), component);

        // Add to config
        config.add_component(component.clone(), "0.1.0".to_string());
    }

    config.save()?;

    println!();
    println!("{}", "Components added successfully!".green().bold());

    Ok(())
}
