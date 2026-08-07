use crate::{app, plugins, state::State, tui};
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
        plugin: String,
    },

    /// Remove a plugin
    Remove {
        /// Plugin name to remove (omit to select interactively with fzf)
        #[arg(short, long)]
        plugin: Option<String>,
    },
}

async fn update(plugin: Option<String>, state: &mut State) -> Result<(), Box<dyn Error>> {
    match plugin {
        Some(plugin) => app::update(plugin, state).await,
        None => app::update_all().await,
    }
}

async fn install(plugin: String, state: &mut State) -> Result<(), Box<dyn Error>> {
    app::install(plugin, state).await
}

async fn remove(plugin: Option<String>, state: &mut State) -> Result<(), Box<dyn Error>> {
    let plugin = match plugin {
        Some(plugin) => Some(plugin),
        None => plugins::select_plugin_via_fzf()?,
    };

    match plugin {
        Some(plugin) => app::remove(plugin, state).await,
        None => Ok(()),
    }
}

pub async fn run() {
    let cli = Cli::parse();
    let mut state = State::default().await;

    let result = match cli.command {
        Some(Commands::Tui) => tui::run().await,
        Some(Commands::Update { plugin }) => update(plugin, &mut state).await,
        Some(Commands::Install { plugin }) => install(plugin, &mut state).await,
        Some(Commands::Remove { plugin }) => remove(plugin, &mut state).await,
        None => app::run().await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
