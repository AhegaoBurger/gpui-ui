use anyhow::Result;
use colored::Colorize;

use crate::config::Config;
use crate::registry::Registry;

pub async fn run(component_name: String) -> Result<()> {
    let registry = Registry::new();

    let component = registry.get_component(&component_name)?;

    println!();
    println!("{} {}", "Component:".cyan().bold(), component.name.bold());
    println!();

    println!("{}", component.description);
    println!();

    println!("{} {}", "Version:".cyan(), component.version);

    if !component.dependencies.is_empty() {
        println!();
        println!("{}", "Dependencies:".cyan());
        for dep in &component.dependencies {
            println!("  {} {}", "▸".cyan(), dep);
        }
    }

    println!();
    println!("{}", "Files:".cyan());
    for file in &component.files {
        println!("  {} {}", "▸".cyan(), file);
    }

    // Check if installed
    if Config::exists() {
        if let Ok(config) = Config::load() {
            if let Some(installed) = config.components.iter().find(|c| c.name == component.name) {
                println!();
                println!(
                    "{} {} {}",
                    "Status:".cyan(),
                    "Installed".green().bold(),
                    format!("(v{})", installed.version).dimmed()
                );
                println!("  {} {}", "Installed at:".dimmed(), installed.installed_at.dimmed());
            } else {
                println!();
                println!("{} {}", "Status:".cyan(), "Not installed".yellow());
            }
        }
    }

    println!();
    println!(
        "Run {} to install this component",
        format!("gpui-ui add {}", component.name).cyan()
    );
    println!();

    Ok(())
}
