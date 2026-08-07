mod app;
mod bindings;
mod cli;
mod plugins;
mod register;
mod state;
mod tmux;
mod tmuxedo;
mod tui;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();
    cli::run().await;
}
