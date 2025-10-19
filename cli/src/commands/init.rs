use anyhow::{bail, Result};
use colored::Colorize;
use std::fs;
use std::io::{self, Write};

use crate::config::Config;

pub async fn run(yes: bool) -> Result<()> {
    println!("{}", "Initializing gpui-ui...".cyan().bold());

    // Check if config already exists
    if Config::exists() {
        bail!("gpui-ui.json already exists in this directory. Use 'gpui-ui add' to add components.");
    }

    // Confirm with user unless --yes flag is set
    if !yes {
        print!("This will create a gpui-ui.json file in the current directory. Continue? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{}", "Initialization cancelled.".yellow());
            return Ok(());
        }
    }

    // Create default config
    let config = Config::new();
    config.save()?;

    // Create component directory
    fs::create_dir_all(&config.component_path)?;

    println!("{}", "✓ Created gpui-ui.json".green());
    println!("{}", format!("✓ Created {} directory", config.component_path).green());
    println!();
    println!("Next steps:");
    println!("  1. Run {} to see available components", "gpui-ui list".cyan());
    println!("  2. Run {} to add components", "gpui-ui add <component>".cyan());

    Ok(())
}
