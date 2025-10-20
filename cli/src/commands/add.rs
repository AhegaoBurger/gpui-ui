use anyhow::{bail, Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

use crate::config::Config;
use crate::registry::Registry;
use crate::utils;

pub async fn run(components: Vec<String>, force: bool) -> Result<()> {
    if components.is_empty() {
        bail!("Please specify at least one component to add. Run 'gpui-ui list' to see available components.");
    }

    // Check if project is initialized
    if !Config::exists() {
        bail!("gpui-ui is not initialized in this directory. Run 'gpui-ui init' first.");
    }

    let mut config = Config::load()?;
    let registry = Registry::new();

    println!("{}", "Adding components...".cyan().bold());
    println!();

    // Get component source directory
    let source_dir = utils::get_component_source_dir()
        .context("Failed to locate component source directory")?;

    // Get destination directory from config
    let dest_dir = PathBuf::from(&config.component_path);

    let mut added_components = Vec::new();

    for component_name in components {
        // Check if component exists in registry
        let component = match registry.get_component(&component_name) {
            Ok(c) => c,
            Err(_) => {
                println!(
                    "  {} Component '{}' not found. Run 'gpui-ui list' to see available components.",
                    "✗".red(),
                    component_name.red()
                );
                continue;
            }
        };

        println!("  {} Adding {}", "→".cyan(), component.name.bold());

        // Resolve dependencies
        let all_components = registry
            .resolve_dependencies(&component_name)
            .context(format!("Failed to resolve dependencies for {}", component_name))?;

        // Create progress bar
        let total_files: usize = all_components
            .iter()
            .filter_map(|name| registry.get_component(name).ok())
            .map(|c| c.files.len())
            .sum();

        let pb = ProgressBar::new(total_files as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("    [{bar:40.cyan/blue}] {pos}/{len} files")
                .unwrap()
                .progress_chars("=>-"),
        );

        // Copy all files including dependencies
        for comp_name in &all_components {
            let comp = registry.get_component(comp_name)?;

            for file in &comp.files {
                let source_path = source_dir.join(file);
                let dest_path = dest_dir.join(file);

                // Check if source exists
                if !source_path.exists() {
                    println!(
                        "    {} Source file not found: {}",
                        "⚠".yellow(),
                        source_path.display()
                    );
                    continue;
                }

                // Check if destination exists and we're not forcing
                if dest_path.exists() && !force {
                    if comp_name == &component_name {
                        // Only warn for the main component, not dependencies
                        println!(
                            "    {} {} already exists (skipping, use --force to overwrite)",
                            "⚠".yellow(),
                            file
                        );
                    }
                    pb.inc(1);
                    continue;
                }

                // Copy the file
                utils::copy_file(&source_path, &dest_path, force)
                    .context(format!("Failed to copy {}", file))?;

                pb.inc(1);
            }

            // Add to installed components list if not already there
            if !config
                .components
                .iter()
                .any(|c| c.name == comp.name)
            {
                added_components.push(comp.name.clone());
            }
        }

        pb.finish_and_clear();
        println!("    {} {} installed successfully", "✓".green(), component.name);

        // If this is the main component (not a dependency), record it
        if !config
            .components
            .iter()
            .any(|c| c.name == component.name)
        {
            config.add_component(component.name.clone(), component.version.clone());
        }
    }

    // Save updated config
    config.save()?;

    println!();
    println!("{}", "Done!".green().bold());

    if !added_components.is_empty() {
        println!();
        println!("Components added:");
        for comp in added_components {
            println!("  {} {}", "▸".cyan(), comp);
        }
    }

    println!();
    println!("Next steps:");
    println!("  1. Import components in your code:");
    println!("     {}", "mod components;".dimmed());
    println!("  2. Use the components in your GPUI app:");
    println!("     {}", "use components::ui::Button;".dimmed());

    Ok(())
}
