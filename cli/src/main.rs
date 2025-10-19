mod commands;
mod config;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gpui-ui")]
#[command(about = "A CLI tool for managing GPUI UI components", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize gpui-ui in your project
    Init {
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
    /// Add a component to your project
    Add {
        /// Component name(s) to add
        components: Vec<String>,

        /// Overwrite existing files
        #[arg(short, long)]
        force: bool,
    },
    /// List all available components
    List {
        /// Show detailed component information
        #[arg(short, long)]
        verbose: bool,
    },
    /// Update components to the latest version
    Update {
        /// Component name(s) to update (updates all if none specified)
        components: Vec<String>,
    },
    /// Show information about a component
    Info {
        /// Component name
        component: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { yes } => {
            commands::init::run(yes).await?;
        }
        Commands::Add { components, force } => {
            commands::add::run(components, force).await?;
        }
        Commands::List { verbose } => {
            commands::list::run(verbose).await?;
        }
        Commands::Update { components } => {
            commands::update::run(components).await?;
        }
        Commands::Info { component } => {
            commands::info::run(component).await?;
        }
    }

    Ok(())
}
