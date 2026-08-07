use crate::{plugins, state, tmuxedo};
use log::info;
use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    tmuxedo::ensure_structure();
    tmuxedo::source_all_tmuxedo_files().await;
    plugins::run();
    let _ = plugins::run_local_plugins();
    Ok(())
}

pub async fn install(plugin: String, state: &mut state::State) -> Result<(), Box<dyn Error>> {
    info!("Installing {}", plugin);
    let _ = state.install_plugin_by_name(plugin).await;
    plugins::run();
    Ok(())
}

pub async fn update(plugin: String, state: &mut state::State) -> Result<(), Box<dyn Error>> {
    info!("Updating {}", plugin);
    let _ = state.update_plugin_by_name(plugin).await;
    Ok(())
}

pub async fn update_all() -> Result<(), Box<dyn Error>> {
    info!("Updating all plugins");
    let _ = plugins::pull().await;
    plugins::run();
    Ok(())
}

pub async fn remove(plugin: String, state: &mut state::State) -> Result<(), Box<dyn Error>> {
    info!("Removing plugin {}", plugin);
    state.remove_plugin_by_name(plugin);
    plugins::run();
    Ok(())
}
