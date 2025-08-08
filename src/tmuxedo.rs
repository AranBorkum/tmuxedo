use std::{fs, path::PathBuf};

use dirs::home_dir;
use walkdir::WalkDir;

use crate::tmux::{TmuxCommand, run_tmux_command};

fn get_tmuxedo_base_directory() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".config/tmux/tmuxedo");

    match fs::create_dir_all(&path) {
        Ok(_) => println!("Ensured directory exists at: {}", path.display()),
        Err(e) => eprintln!("Error creating directory: {}", e),
    }

    path
}

pub fn source_all_tmuxedo_files() {
    let tmuxedo_dir = get_tmuxedo_base_directory();
    for entry in WalkDir::new(&tmuxedo_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        println!("Found file: {}", entry.path().display());
        run_tmux_command(TmuxCommand::SourceFile, entry.path().display().to_string());
    }
}
