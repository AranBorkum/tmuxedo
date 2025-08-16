use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    process::{ExitStatus, Stdio},
    vec,
};

use tokio::{io, process::Command, task};
use walkdir::WalkDir;

use crate::{TmuxCommand, tmuxedo::Path};

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
            todo!()
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
    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().display().to_string().ends_with(".tmux"))
    {
        let arguments = vec![entry.path().display().to_string()];
        TmuxCommand::RunShell.run(arguments);
    }
}
