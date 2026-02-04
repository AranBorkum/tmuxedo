use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "my-app", args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[arg(short, long, default_value_t = false, conflicts_with_all = ["update"])]
    pub tui: bool,

    #[arg(short, long, default_value_t = false, conflicts_with_all = ["tui"])]
    pub update: bool,

    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Install plugin
    Install {
        /// Path to install from
        #[arg(short = 'p', long, value_name = "PATH")]
        path: String,

        /// Branch to use (optional)
        #[arg(short = 'b', long, value_name = "BRANCH", requires = "path")]
        branch: Option<String>,
    },
}
