use std::process::Command;

pub enum TmuxCommand {
    SourceFile,
    RunShell,
}

impl TmuxCommand {
    fn command(&self) -> String {
        match self {
            Self::SourceFile => String::from("source-file"),
            Self::RunShell => String::from("run-shell"),
        }
    }

    pub fn run(&self, args: Vec<String>) {
        let status = Command::new("tmux").arg(self.command()).args(args).status();

        match status {
            Ok(status) if status.success() => {}
            Ok(status) => eprintln!("Tmux command failed with status: {status}"),
            Err(e) => eprintln!("Failed to run tmux command: {e}"),
        }
    }
}
