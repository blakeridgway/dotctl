use crate::manifest::{DotfileEntry, EntryType};
use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

pub fn sync_dotfiles(entries: &[DotfileEntry]) -> Result<()> {
    for entry in entries {
        process_entry(entry)?;
    }
    Ok(())
}

fn process_entry(entry: &DotfileEntry) -> Result<()> {
    if entry.target.exists() {
        backup_target(&entry.target)?;
    }

    if let Some(parent) = entry.target.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    match entry.entry_type {
        EntryType::Copy => copy_file(entry),
        EntryType::Symlink => create_symlink(entry),
        EntryType::Template => render_template(entry),
    }
}


fn backup_target(target: &Path) -> Result<()> {
    let backup_path = target.with_extension(format!(
        "backup.{}",
        chrono::Utc::now().format("%Y%m%d%H%M%S")
    ));

    fs::rename(target, &backup_path)
        .with_context(|| format!("Failed to backup {} to {}", target.display(), backup_path.display()))?;

    println!("Backed up {} to {}", target.display(), backup_path.display());
    Ok(())
}

fn create_symlink(entry: &DotfileEntry) -> Result<()> {
    symlink(&entry.source, &entry.target)
        .with_context(|| format!("Failed to create symlink from {} to {}", 
            entry.source.display(), entry.target.display()))?;
    
    println!("Created symlink: {} -> {}", entry.target.display(), entry.source.display());
    Ok(())
}

fn copy_file(entry: &DotfileEntry) -> Result<()> {
    fs::copy(&entry.source, &entry.target)
        .with_context(|| format!("Failed to copy {} to {}", 
            entry.source.display(), entry.target.display()))?;
    
    println!("Copied file: {} -> {}", entry.source.display(), entry.target.display());
    Ok(())
}

fn render_template(entry: &DotfileEntry) -> Result<()> {
    let content = fs::read_to_string(&entry.source)
        .with_context(|| format!("Failed to read template: {}", entry.source.display()))?;
    
    let rendered = content
        .replace("{{HOME}}", &std::env::var("HOME").unwrap_or_else(|_| "~".to_string()));
    
    fs::write(&entry.target, rendered)
        .with_context(|| format!("Failed to write rendered template: {}", entry.target.display()))?;
    
    println!("Rendered template: {} -> {}", entry.source.display(), entry.target.display());
    Ok(())
}