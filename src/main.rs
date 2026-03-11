use std::{error::Error, io, process::Command};

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{
    cli::Cli,
    plugins::run_plugins,
    tmuxedo::{ensure_structure, source_all_tmuxedo_files},
    tui::run_tmuxedo_tui,
};

mod bindings;
mod cli;
mod plugins;
mod register;
mod state;
mod tmuxedo;
mod tui;
mod utils;

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

async fn run_app(cli: &Cli) -> Result<(), Box<dyn Error>> {
    ensure_structure();
    source_all_tmuxedo_files(cli.update).await;
    run_plugins();

    Ok(())
}

async fn run_tui() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_tmuxedo_tui(&mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let _ = match cli.tui {
        true => run_tui().await,
        false => run_app(&cli).await,
    };
}
