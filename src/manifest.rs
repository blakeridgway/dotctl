use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::{Context, Result};
use shellexpand;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct DotfilesManifest {
    pub symlink: Option<HashMap<String, String>>,
    pub copy: Option<HashMap<String, String>>,
    pub template: Option<HashMap<String, String>>,
    pub bootstrap: Option<BootstrapConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BootstrapConfig {
    pub packages: Option<Vec<Package>>,
    pub run_once: Option<Vec<RunOnce>>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub manager: PackageManager,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RunOnce {
    pub script: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    Apt,
    Brew,
    Flatpak,
    Pacman,
    Dnf,
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PackageManager::Apt => "apt",
            PackageManager::Brew => "brew",
            PackageManager::Flatpak => "flatpak",
            PackageManager::Pacman => "pacman",
            PackageManager::Dnf => "dnf",
        })
    }
}

impl DotfilesManifest {
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}

#[derive(Debug)]
pub struct DotfileEntry {
    pub source: PathBuf,
    pub target: PathBuf,
    pub entry_type: EntryType,
}

#[derive(Debug)]
pub enum EntryType {
    Symlink,
    Copy,
    Template,
}

pub fn parse_manifest(manifest: &DotfilesManifest) -> Result<Vec<DotfileEntry>> {
    let mut entries = Vec::new();
    
    if let Some(symlinks) = &manifest.symlink {
        for (target, source) in symlinks {
            entries.push(parse_entry(target, source, EntryType::Symlink)?);
        }
    }
    
    if let Some(copies) = &manifest.copy {
        for (target, source) in copies {
            entries.push(parse_entry(target, source, EntryType::Copy)?);
        }
    }
    
    if let Some(templates) = &manifest.template {
        for (target, source) in templates {
            entries.push(parse_entry(target, source, EntryType::Template)?);
        }
    }
    
    Ok(entries)
}

fn parse_entry(target: &str, source: &str, entry_type: EntryType) -> Result<DotfileEntry> {
    let expanded_target = shellexpand::full(target)
        .with_context(|| format!("Failed to expand target path: {}", target))?;
    
    let target_path = PathBuf::from(expanded_target.as_ref());
    let source_path = PathBuf::from(source);
    
    Ok(DotfileEntry {
        source: source_path,
        target: target_path,
        entry_type,
    })
}