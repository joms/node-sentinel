mod cli;
mod node;
mod nvmrc;

use std::process::ExitCode;

use cli::parse_cli_args;
use node::get_current_node_version;
use nvmrc::{
    confirm_switch_node_version, get_nvm_node_version, is_node_version_matching, is_nvm_installed,
    resolve_nvm_version,
};

fn main() -> ExitCode {
    let args = parse_cli_args();
    let is_nvm_installed = is_nvm_installed();

    if !is_nvm_installed {
        eprintln!("❌ NVM is not installed.");
        return ExitCode::from(1);
    }

    let nvm_version = match get_nvm_node_version(args.directory.as_deref()) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::from(1);
        }
    };

    let required_version = match resolve_nvm_version(&nvm_version) {
        Ok(version) => version,
        Err(_e) => {
            eprintln!("An unexpected error occured");
            return ExitCode::from(1);
        }
    };

    let node_version = match get_current_node_version() {
        Ok(version) => version,
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::from(1);
        }
    };

    let is_correct_node_version = is_node_version_matching(&required_version, &node_version);

    if args.check_only {
        if is_correct_node_version {
            eprintln!(
                "✅ You are using the correct Node.js version.\n\
             Current version: {node_version}\n\
             Required version: {nvm_version}"
            );
            return ExitCode::from(0);
        } else {
            eprintln!(
                "❌ You are using a different Node.js version than required.\n\
             Current version: {node_version}\n\
             Required version: {nvm_version}"
            );
            return ExitCode::from(1);
        }
    }

    if !args.auto_switch {
        if !confirm_switch_node_version(&nvm_version) {
            return ExitCode::from(0);
        }
    }

    println!("nvm use {}", nvm_version);
    ExitCode::from(0)
}
