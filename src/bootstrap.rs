use crate::manifest::{BootstrapConfig, PackageManager, RunOnce, Package};
use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::path::Path;
use std::collections::HashSet;

pub fn bootstrap(config: &BootstrapConfig) -> Result<()> {
    if let Some(packages) = &config.packages {
        install_packages(packages)?;
    }
    
    if let Some(scripts) = &config.run_once {
        run_scripts(scripts)?;
    }
    
    Ok(())
}

fn install_packages(packages: &[Package]) -> Result<()> {
    let mut needs_sudo = HashSet::new();
    for package in packages {
        if requires_sudo(&package.manager) {
            needs_sudo.insert(package.manager);
        }
    }
    
    if !needs_sudo.is_empty() {
        println!("The following package managers require sudo privileges:");
        for manager in needs_sudo {
            println!("   - {}", manager);
        }
        println!("   Please run with sudo or manually install packages.");
        println!();
    }
    
    for package in packages {
        if let Err(e) = install_package(package) {
            eprintln!("Failed to install {}: {}", package.name, e);
            
            if requires_sudo(&package.manager) {
                eprintln!("Try: sudo {} install {}", package.manager, package.name);
            }
            
            continue;
        }
    }
    
    Ok(())
}

fn requires_sudo(manager: &PackageManager) -> bool {
    matches!(manager, PackageManager::Apt | PackageManager::Pacman | PackageManager::Dnf)
}

fn install_package(package: &Package) -> Result<()> {
    let (command_args, use_sudo) = match package.manager {
        PackageManager::Apt => (vec!["apt", "install", &package.name], true),
        PackageManager::Brew => (vec!["brew", "install", &package.name], false),
        PackageManager::Flatpak => (vec!["flatpak", "install", "--noninteractive", &package.name], false),
        PackageManager::Pacman => (vec!["pacman", "-S", &package.name], true),
        PackageManager::Dnf => (vec!["dnf", "install", &package.name], true),
    };
    
    println!("Installing {} with {}...", package.name, command_args[0]);
    
    let mut command = if use_sudo {
        let mut cmd = Command::new("sudo");
        cmd.args(&command_args);
        cmd
    } else {
        let mut cmd = Command::new(command_args[0]);
        cmd.args(&command_args[1..]);
        cmd
    };
    
    let status = command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("Failed to run {}", command_args.join(" ")))?;
    
    if !status.success() {
        anyhow::bail!("Package installation failed with exit code: {}", status);
    }
    
    println!("Successfully installed {}", package.name);
    Ok(())
}

fn run_scripts(scripts: &[RunOnce]) -> Result<()> {
    for script in scripts {
        if let Err(e) = run_script(script) {
            eprintln!("Failed to run script {}: {}", script.script, e);
            continue;
        }
    }
    Ok(())
}

fn run_script(script: &RunOnce) -> Result<()> {
    if let Some(description) = &script.description {
        println!("Running: {}", description);
    } else {
        println!("Running script: {}", script.script);
    }
    
    let script_path = Path::new(&script.script);
    
    if !script_path.exists() {
        anyhow::bail!("Script file not found: {}", script.script);
    }
    
    let status = if script_path.extension().map_or(false, |ext| ext == "sh") {
        Command::new("bash")
            .arg(&script.script)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    } else {
        Command::new(&script.script)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    }
    .with_context(|| format!("Failed to run script: {}", script.script))?;
    
    if !status.success() {
        anyhow::bail!("Script failed with exit code: {}", status);
    }
    
    println!("Script completed: {}", script.script);
    Ok(())
}