use std::{path::PathBuf, process::Command};

use dirs::home_dir;

use crate::tmux::{run_tmux_command, TmuxCommand};

fn install_tpm(path: &PathBuf) {
    let status = Command::new("git")
        .arg("clone")
        .arg("https://github.com/tmux-plugins/tpm")
        .arg(path.display().to_string())
        .status();

    match status {
        Ok(status) if status.success() => println!("Git command ran successfully."),
        Ok(status) => eprintln!("Git command failed with status: {}", status),
        Err(e) => eprintln!("Failed to run git command: {}", e),
    }
}

fn ensure_tpm_installed(path: &PathBuf) {
    if path.exists() && path.is_dir() {
        println!("Tmux package manager is already installed.");
    } else {
        println!("No tmux package manager installed, installing now.");
        install_tpm(&path);
    }
}

pub fn run_tpm() {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".tmux/plugins/tpm");
    ensure_tpm_installed(&path);
    path.push("tpm");
    run_tmux_command(TmuxCommand::Run, path.display().to_string());
}
