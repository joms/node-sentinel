use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

use semver::{Version, VersionReq};

pub fn is_nvm_installed() -> bool {
    std::env::var("NVM_DIR").is_ok()
}

pub fn get_nvm_node_version(dir: Option<&str>) -> Result<String, String> {
    let directory = dir.unwrap_or(".");
    let nvmrc_path = Path::new(directory).join(".nvmrc");

    if nvmrc_path.exists() {
        fs::read_to_string(&nvmrc_path)
            .map(|content| content.trim().to_string())
            .map_err(|err| format!("Error reading .nvmrc file: {}", err))
    } else {
        Err(".nvmrc file not found in the specified directory.".to_string())
    }
}

pub fn switch_node_version(version: &str) -> Result<(), String> {
    let nvm_script_path = "$HOME/.nvm/nvm.sh";
    let shell_cmd = format!(". {}; nvm use {}", nvm_script_path, version);

    println!("{shell_cmd}");

    let output = Command::new("sh")
        .arg("-c")
        .arg(shell_cmd)
        .output()
        .map_err(|e| format!("Failed to execute shell command: {}", e))?;

    if output.status.success() {
        println!("Switched to Node version {}", version);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!(
            "Node version switch failed:\nstdout: {}\nstderr: {}",
            stdout, stderr
        ))
    }
}

pub fn confirm_switch_node_version(version: &str) -> bool {
    print!("Switch to node version {}? (y/n): ", version);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() == "y"
}

fn get_lts_version() -> Result<Version, String> {
    let output = Command::new("nvm")
        .args(&["list", "--no-colors"])
        .output()
        .map_err(|_| "Failed to execute `nvm list --no-colors` command".to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Find the current LTS version marked with "->"
        for line in stdout.lines() {
            if line.contains("->") && line.contains("lts/") {
                // Example line: "->     v16.13.0        lts/gallium"
                if let Some(version_str) = line.split_whitespace().nth(1) {
                    let version_str = version_str.trim_start_matches('v');
                    return Version::parse(version_str)
                        .map_err(|err| format!("Failed to parse LTS version: {}", err));
                }
            }
        }
        Err("Could not determine current LTS version".to_string())
    } else {
        Err("Failed to retrieve current LTS version".to_string())
    }
}

fn get_latest_version() -> Result<Version, String> {
    let output = Command::new("nvm")
        .args(&["list", "remote", "--no-colors"])
        .output()
        .map_err(|_| "Failed to execute `nvm list remote --no-colors` command".to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Find the last version listed
        for line in stdout.lines().rev() {
            let line = line.trim();
            if line.starts_with('v') {
                let version_str = line.trim_start_matches('v');
                return Version::parse(version_str)
                    .map_err(|err| format!("Failed to parse latest version: {}", err));
            }
        }
        Err("Could not determine latest Node.js version".to_string())
    } else {
        Err("Failed to retrieve latest Node.js version".to_string())
    }
}

pub fn resolve_nvm_version(version_str: &str) -> Result<VersionReq, String> {
    match version_str {
        "lts" => {
            let lts_version = get_lts_version()?;
            // Create a VersionReq that matches exactly the LTS version
            let version_req_str = format!("={}", lts_version);
            VersionReq::parse(&version_req_str)
                .map_err(|err| format!("Failed to create VersionReq for LTS version: {}", err))
        }
        "latest" => {
            let latest_version = get_latest_version()?;
            let version_req_str = format!("={}", latest_version);
            VersionReq::parse(&version_req_str)
                .map_err(|err| format!("Failed to create VersionReq for latest version: {}", err))
        }
        _ => {
            // Determine how many components are in the version string
            let parts: Vec<&str> = version_str.split('.').collect();
            let version_req_str = match parts.len() {
                1 => {
                    // If only major version is specified, use a caret requirement
                    format!("^{}", version_str)
                }
                2 => {
                    // If major and minor are specified, use a caret requirement
                    format!("^{}", version_str)
                }
                3 => {
                    // If major, minor, and patch are specified, match the exact version
                    format!("={}", version_str)
                }
                _ => {
                    // Invalid version format
                    return Err("Invalid version format in .nvmrc".to_string());
                }
            };
            VersionReq::parse(&version_req_str)
                .map_err(|err| format!("Failed to parse required Node.js version: {}", err))
        }
    }
}

pub fn is_node_version_matching(
    required_node_version: &VersionReq,
    current_node_version: &Version,
) -> bool {
    required_node_version.matches(current_node_version)
}
