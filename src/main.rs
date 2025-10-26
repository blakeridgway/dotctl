mod manifest;
mod sync;

use anyhow::Result;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let manifest_path = Path::new("dotfiles.toml");
    let content = fs::read_to_string(manifest_path)?;
    
    let manifest = manifest::DotfilesManifest::from_toml(&content)?;
    let entries = manifest::parse_manifest(&manifest)?;
    
    sync::sync_dotfiles(&entries)?;
    
    Ok(())
}