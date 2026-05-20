use crate::{plugins, state, tmuxedo};
use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    tmuxedo::ensure_structure();
    tmuxedo::source_all_tmuxedo_files().await;
    plugins::run();
    Ok(())
}

pub async fn install(plugin: String, state: &mut state::State) -> Result<(), Box<dyn Error>> {
    println!("Installing {}", plugin);
    let _ = state.install_plugin_by_name(plugin).await;
    Ok(())
}

pub async fn install_all() -> Result<(), Box<dyn Error>> {
    println!("Installing all plugins");
    let _ = plugins::clone().await;
    plugins::run();
    Ok(())
}

pub async fn update(plugin: String, state: &mut state::State) -> Result<(), Box<dyn Error>> {
    println!("Updating {}", plugin);
    let _ = state.update_plugin_by_name(plugin).await;
    Ok(())
}

pub async fn update_all() -> Result<(), Box<dyn Error>> {
    println!("Updating all plugins");
    let _ = plugins::pull().await;
    plugins::run();
    Ok(())
}

pub async fn remove(plugin: String, state: &mut state::State) -> Result<(), Box<dyn Error>> {
    println!("Removing plugin {}", plugin);
    state.remove_plugin_by_name(plugin);
    plugins::run();
    Ok(())
}
