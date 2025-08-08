use std::process::Command;

pub enum TmuxCommand {
    SourceFile,
    Run,
}

impl TmuxCommand {
    pub fn command(&self) -> String {
        match self {
            Self::SourceFile => String::from("source-file"),
            Self::Run => String::from("run"),
        }
    }
}

pub fn run_tmux_command(command: TmuxCommand, arg: String) {
    let status = Command::new("tmux")
        .arg(command.command())
        .arg(arg)
        .status();

    match status {
        Ok(status) if status.success() => println!("Tmux command ran successfully."),
        Ok(status) => eprintln!("Tmux command failed with status: {}", status),
        Err(e) => eprintln!("Failed to run tmux command: {}", e),
    }
}
