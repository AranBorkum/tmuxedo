use crate::plugins::{clone, pull, remove_dir};
use dirs::home_dir;
use ini::Ini;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
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
    TmuxConfig,
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
            Self::TmuxConfig => path.push("tmux.conf"),
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
        "bind C-u run-shell 'tmuxedo --update'",
        "bind C-t display-popup -E 'tmuxedo --tui'",
    ];
    let tmux_defaults: Vec<&str> = vec!["run-shell 'tmuxedo'"];
    ensure_dir_exists(&Path::Tmuxedo.get());
    ensure_dir_exists(&Path::Plugins.get());
    let _ = ensure_file_exists(&Path::PluginsConfig.get(), plugins_defaults);
    let _ = ensure_file_exists(&Path::TmuxedoConfig.get(), tmuxedo_defaults);
    let _ = ensure_file_exists(&Path::TmuxConfig.get(), tmux_defaults);
}

pub fn prune_mismatched_remote_origins() -> io::Result<()> {
    let path = Path::PluginsConfig.get();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let repo_and_branch: Vec<_> = line.split_whitespace().collect();

        let repo = if !repo_and_branch.is_empty() {
            repo_and_branch[0].to_string()
        } else {
            continue;
        };

        if let Ok(origin_matches) = plugin_origin_matches_repo_name(&repo)
            && !origin_matches
        {
            let dir_name = repo.split("/").collect::<Vec<_>>()[1];
            let mut plugin_path = Path::Plugins.get();
            plugin_path.push(dir_name);
            let _ = remove_dir(plugin_path.display().to_string());
        }
    }

    Ok(())
}

fn plugin_origin_matches_repo_name(repo_name: &String) -> Result<bool, Box<dyn Error>> {
    let dir_name = repo_name.split("/").collect::<Vec<_>>()[1];
    let mut path = Path::Plugins.get();
    path.push(dir_name);
    path.push(".git/config");

    let conf = Ini::load_from_file(path)?;
    let mut result: bool = false;

    if let Some(section) = conf.section(Some("remote \"origin\""))
        && let Some(val) = section.get("url")
        && let Some(repo) = val.strip_prefix("https://git::@github.com/")
    {
        result = repo == repo_name;
    }

    Ok(result)
}
