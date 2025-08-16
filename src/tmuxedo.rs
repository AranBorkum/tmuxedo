use crate::plugins::{clone, pull};
use dirs::home_dir;
use std::io::{self, Write};
use std::vec;
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};
use walkdir::WalkDir;

use crate::TmuxCommand;

pub enum Path {
    Tmuxedo,
    Plugins,
    PluginsConfig,
    TmuxedoConfig,
}

impl Path {
    pub fn get(&self) -> PathBuf {
        let mut path = home_dir().expect("Could not find home directory");
        path.push(".config/tmux");
        match self {
            Self::Tmuxedo => path.push("tmuxedo"),
            Self::Plugins => path.push("plugins"),
            Self::PluginsConfig => path.push("tmuxedo/plugins.conf"),
            Self::TmuxedoConfig => path.push("tmuxedo/tmuxedo.conf"),
        };

        path
    }
}

pub async fn source_all_tmuxedo_files(update: bool) {
    let tmuxedo_dir = Path::Tmuxedo.get();
    for entry in WalkDir::new(&tmuxedo_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        if entry.path().display().to_string().ends_with("plugins.conf") {
            let _ = match update {
                true => pull().await,
                false => clone().await,
            };
        } else {
            let arguments = vec![entry.path().display().to_string()];
            TmuxCommand::SourceFile.run(arguments)
        }
    }
}

fn ensure_dir_exists(path: &PathBuf) {
    match fs::create_dir_all(path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error creating directory: {e}"),
    }
}

fn ensure_file_exists(path: &PathBuf, content: Vec<&str>) -> io::Result<()> {
    if !path.exists() {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;

        for line in content {
            writeln!(file, "{}", String::from(line))?;
        }
    }

    Ok(())
}

pub fn ensure_structure() {
    let plugins_defaults: Vec<&str> = vec![""];
    let tmuxedo_defaults: Vec<&str> = vec![
        "unbind r",
        "bind r run-shell tmuxedo",
        "bind C-u run-shell tmuxedo --update",
        "bind C-t display-popup -E 'tmuxedo --tui'",
    ];
    ensure_dir_exists(&Path::Tmuxedo.get());
    ensure_dir_exists(&Path::Plugins.get());
    let _ = ensure_file_exists(&Path::PluginsConfig.get(), plugins_defaults);
    let _ = ensure_file_exists(&Path::TmuxedoConfig.get(), tmuxedo_defaults);
}
