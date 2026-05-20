use crate::plugins::clone;
use crate::utils::{ensure_dir_exists, ensure_file_exists};
use dirs::home_dir;
use std::path::PathBuf;
use std::vec;
use walkdir::WalkDir;

use crate::tmux::TmuxCommand;

pub enum Path {
    Tmuxedo,
    Plugins,
    PluginsConfig,
    LocalPluginsConfig,
    TmuxedoConfig,
    TmuxConfig,
}

impl Path {
    pub fn get(&self) -> PathBuf {
        let mut path = home_dir().expect("Could not find home directory");
        match self {
            Self::Tmuxedo => path.push(".config/tmux/tmuxedo"),
            Self::Plugins => path.push(".local/share/tmuxedo/plugins"),
            Self::PluginsConfig => path.push(".config/tmux/tmuxedo/plugins.conf"),
            Self::LocalPluginsConfig => path.push(".config/tmux/tmuxedo/plugins-local.conf"),
            Self::TmuxedoConfig => path.push(".config/tmux/tmuxedo/tmuxedo.conf"),
            Self::TmuxConfig => path.push(".config/tmux/tmux.conf"),
        };
        path
    }
}

pub async fn source_all_tmuxedo_files() {
    let tmuxedo_dir = Path::Tmuxedo.get();
    for entry in WalkDir::new(&tmuxedo_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        if entry.path() == Path::PluginsConfig.get() {
            let _ = clone().await;
        } else {
            if !entry
                .path()
                .display()
                .to_string()
                .ends_with("plugins-local.conf")
            {
                let arguments = vec![entry.path().display().to_string()];
                TmuxCommand::SourceFile.run(arguments)
            }
        }
    }
}

pub fn ensure_structure() {
    let plugins_defaults: Vec<&str> = vec![""];
    let tmuxedo_defaults: Vec<&str> = vec![
        "unbind r",
        "bind r run-shell tmuxedo",
        "bind C-u run-shell 'tmuxedo --update'",
        "bind C-t display-popup -w 80% -h 80% -E 'tmuxedo --tui'",
    ];
    let tmux_defaults: Vec<&str> = vec!["run-shell 'tmuxedo'"];
    ensure_dir_exists(&Path::Tmuxedo.get());
    ensure_dir_exists(&Path::Plugins.get());
    let _ = ensure_file_exists(&Path::PluginsConfig.get(), plugins_defaults.clone());
    let _ = ensure_file_exists(&Path::LocalPluginsConfig.get(), plugins_defaults.clone());
    let _ = ensure_file_exists(&Path::TmuxedoConfig.get(), tmuxedo_defaults);
    let _ = ensure_file_exists(&Path::TmuxConfig.get(), tmux_defaults);
}
