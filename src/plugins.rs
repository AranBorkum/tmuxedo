use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    process::Stdio,
    vec,
};

use tokio::{process::Command, task};
use walkdir::WalkDir;

use crate::{TmuxCommand, tmuxedo::Path};

async fn git_clone(plugin: &String) -> io::Result<()> {
    let path = Path::Plugins.get();
    let status = Command::new("git")
        .arg("clone")
        .arg("--single-branch")
        .arg("--recursive")
        .arg(format!("https://git::@github.com/{}", plugin))
        .current_dir(path)
        .status()
        .await?;

    if !status.success() {
        eprintln!("Git failed: {}", plugin);
    }

    Ok(())
}

async fn git_pull(plugin: &String) -> io::Result<()> {
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
        eprintln!("Git failed: {}", plugin);
    }
    if !submodule_status.success() {
        eprintln!("Git failed: {}", plugin);
    }

    Ok(())
}

pub async fn clone() -> io::Result<()> {
    let path = Path::PluginsConfig.get();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut handles = vec![];

    for line_result in reader.lines() {
        let line = line_result?;
        let line_clone = line.clone();

        let handle = task::spawn(async move {
            let _ = git_clone(&line_clone).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task failed: {:?}", e);
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
            eprintln!("Task failed: {:?}", e);
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
