use std::process::Command;

use semver::Version;

pub fn get_current_node_version() -> Result<Version, String> {
    let output = Command::new("node")
        .arg("-v")
        .output()
        .map_err(|_| "Failed to execute `node -v` command".to_string())?;

    if output.status.success() {
        let version_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let version_str = version_str.trim_start_matches('v'); // Remove leading 'v'

        Version::parse(&version_str)
            .map_err(|err| format!("Failed to parse current Node.js version: {}", err))
    } else {
        Err("Failed to retrieve current Node.js version".to_string())
    }
}
