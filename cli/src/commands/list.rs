use anyhow::Result;
use colored::Colorize;

pub async fn run(verbose: bool) -> Result<()> {
    println!("{}", "Available components:".cyan().bold());
    println!();

    // TODO: Fetch from registry
    let components = vec![
        ("button", "A customizable button component with multiple variants"),
        ("input", "Text input with validation support"),
        ("card", "Card container with header, content, and footer"),
        ("dialog", "Modal dialog with overlay"),
        ("select", "Dropdown select with single/multi-select"),
        ("checkbox", "Checkbox input component"),
        ("badge", "Badge component for labels and tags"),
        ("toast", "Toast notification component"),
    ];

    for (name, description) in components {
        println!("  {} {}", "▸".cyan(), name.bold());
        if verbose {
            println!("    {}", description.dimmed());
        }
    }

    println!();
    println!("Run {} to add a component", "gpui-ui add <component>".cyan());

    Ok(())
}
