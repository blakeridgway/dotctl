mod manifest;
mod sync;
mod bootstrap;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "dotctl")]
#[command(about = "Dotfile management tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sync,
    Bootstrap,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let manifest_path = Path::new("dotfiles.toml");
    let content = fs::read_to_string(manifest_path)?;
    
    let manifest = manifest::DotfilesManifest::from_toml(&content)?;
    
    match cli.command {
        Commands::Sync => {
            let entries = manifest::parse_manifest(&manifest)?;
            sync::sync_dotfiles(&entries)?;
        }
        Commands::Bootstrap => {
            if let Some(bootstrap_config) = &manifest.bootstrap {
                bootstrap::bootstrap(bootstrap_config)?;
            } else {
                println!("No bootstrap configuration found in dotfiles.toml");
            }
        }
    }
    
    Ok(())
}