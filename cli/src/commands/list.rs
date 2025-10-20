use anyhow::Result;
use colored::Colorize;

use crate::config::Config;
use crate::registry::Registry;

pub async fn run(verbose: bool) -> Result<()> {
    let registry = Registry::new();
    let components = registry.list_components();

    println!("{}", "Available components:".cyan().bold());
    println!();

    for component in components {
        print!("  {} {}", "▸".cyan(), component.name.bold());

        // Show if already installed
        if Config::exists() {
            if let Ok(config) = Config::load() {
                if config.components.iter().any(|c| c.name == component.name) {
                    print!(" {}", "(installed)".green().dimmed());
                }
            }
        }

        println!();

        if verbose {
            println!("    {}", component.description.dimmed());
            println!("    {} {}", "Version:".dimmed(), component.version.dimmed());

            if !component.dependencies.is_empty() {
                println!(
                    "    {} {}",
                    "Dependencies:".dimmed(),
                    component.dependencies.join(", ").dimmed()
                );
            }
            println!();
        }
    }

    if !verbose {
        println!();
        println!(
            "Run {} for detailed information about each component",
            "gpui-ui list --verbose".cyan()
        );
    }

    println!();
    println!("Run {} to add a component", "gpui-ui add <component>".cyan());

    Ok(())
}
