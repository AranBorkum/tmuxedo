use crate::{app, state::State, tui};
use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(name = "tmuxedo")]
#[command(about = "A tmux configuration manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the interactive TUI
    Tui,

    /// Update tmuxedo configuration
    Update {
        /// Plugin name or URL to update
        #[arg(short, long)]
        plugin: Option<String>,
    },

    /// Install a plugin
    Install {
        /// Plugin name or URL to install
        #[arg(short, long)]
        plugin: Option<String>,
    },

    /// Remove a plugin
    Remove {
        /// Plugin name to remove
        #[arg(short, long)]
        plugin: String,
    },
}

async fn update(plugin: Option<String>, state: &mut State) -> Result<(), Box<dyn Error>> {
    match plugin {
        Some(plugin) => app::update(plugin, state).await,
        None => app::update_all().await,
    }
}

async fn install(plugin: Option<String>, state: &mut State) -> Result<(), Box<dyn Error>> {
    match plugin {
        Some(plugin) => app::install(plugin, state).await,
        None => app::install_all().await,
    }
}

pub async fn run() {
    let cli = Cli::parse();
    let mut state = State::default().await;

    let result = match cli.command {
        Some(Commands::Tui) => tui::run().await,
        Some(Commands::Update { plugin }) => update(plugin, &mut state).await,
        Some(Commands::Install { plugin }) => install(plugin, &mut state).await,
        Some(Commands::Remove { plugin }) => app::remove(plugin, &mut state).await,
        None => app::run().await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
