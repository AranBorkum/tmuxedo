use std::{
    fmt::{self, Display},
    fs::{self, File},
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
    process::{ExitStatus, Stdio},
    vec,
};

use regex::Regex;
use tokio::{io, process::Command, task};
use walkdir::WalkDir;

use crate::{TmuxCommand, tmuxedo::Path};

#[derive(Debug, Eq, Clone)]
pub struct Plugin {
    pub path: String,
    pub commit_hash: String,
    pub is_up_to_date: bool,
}

impl Plugin {
    pub fn set_commit_hash(&mut self, commit_hash: String) {
        self.commit_hash = commit_hash;
    }
}

impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.is_up_to_date == other.is_up_to_date
    }
}

impl Hash for Plugin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.is_up_to_date.hash(state);
    }
}

impl Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

pub async fn git_clone(plugin: &String, branch: Option<String>) -> io::Result<ExitStatus> {
    let path = Path::Plugins.get();

    let status = match branch {
        Some(b) => {
            Command::new("git")
                .arg("clone")
                .arg("-b")
                .arg(b)
                .arg("--single-branch")
                .arg("--recursive")
                .arg(format!("https://git::@github.com/{plugin}"))
                .current_dir(path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .await?
        }
        None => {
            Command::new("git")
                .arg("clone")
                .arg("--single-branch")
                .arg("--recursive")
                .arg(format!("https://git::@github.com/{plugin}"))
                .current_dir(path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .await?
        }
    };

    if !status.success() {
        eprintln!("Git failed: {plugin}");
    }

    Ok(status)
}

pub async fn git_pull(plugin: &String) -> io::Result<ExitStatus> {
    let mut path = Path::Plugins.get();
    path.push(plugin);

    let pull_status = Command::new("git")
        .arg("pull")
        .current_dir(&path)
        .stdout(Stdio::null())
        .status()
        .await?;
    let submodule_status = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .arg("--recursive")
        .current_dir(&path)
        .stdout(Stdio::null())
        .status()
        .await?;

    if !pull_status.success() {
        eprintln!("Git failed: {plugin}");
    }
    if !submodule_status.success() {
        eprintln!("Git failed: {plugin}");
    }

    Ok(pull_status)
}

pub async fn check_for_update(plugin: &str) -> io::Result<(String, String)> {
    let mut path = Path::Plugins.get();
    path.push(plugin.split("/").collect::<Vec<_>>()[1]);

    let output = Command::new("git")
        .arg("pull")
        .arg("--dry-run")
        .current_dir(&path)
        .stdout(Stdio::piped())
        .output()
        .await?;

    let text = String::from_utf8(output.stderr).expect("REASON");
    let re = Regex::new(r"([a-f0-9]{7})\.\.([a-f0-9]{7})").unwrap();

    let mut commit = String::new();
    if let Some(caps) = re.captures(&text.to_string()) {
        commit = caps[2].to_string();
    }
    Ok((plugin.to_owned(), commit))
}

pub fn remove_dir(path: String) -> io::Result<()> {
    let mut dir = Path::Plugins.get();
    dir.push(path);
    fs::remove_dir_all(dir)?;
    Ok(())
}

pub async fn clone() -> io::Result<()> {
    let path = Path::PluginsConfig.get();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut handles = vec![];

    for line_result in reader.lines() {
        let line = line_result?;
        let repo_and_branch: Vec<_> = line.split_whitespace().collect();

        let repo = if !repo_and_branch.is_empty() {
            repo_and_branch[0].to_string()
        } else {
            continue;
        };
        let branch: Option<String> = if repo_and_branch.len() == 2 {
            Some(repo_and_branch[1].to_string())
        } else {
            None
        };

        let handle = task::spawn(async move {
            let _ = git_clone(&repo, branch).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task failed: {e:?}");
        }
    }

    Ok(())
}

pub async fn pull() -> io::Result<()> {
    let path = Path::Plugins.get();

    let mut handles = vec![];

    for entry_result in fs::read_dir(path)? {
        let entry = entry_result?;
        let path = entry.path().display().to_string();

        let handle = task::spawn(async move {
            let _ = git_pull(&path).await;
        });

        handles.push(handle);
    }
    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task failed: {e:?}");
        }
    }

    Ok(())
}

pub fn run_plugins() {
    let path = Path::Plugins.get();

    let plugins: Vec<_> = WalkDir::new(&path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().display().to_string().ends_with(".tmux"))
        .collect();

    if plugins.is_empty() {
        return;
    }

    for entry in plugins {
        let arguments = vec![entry.path().display().to_string()];
        TmuxCommand::RunShell.run(arguments);
    }
}
