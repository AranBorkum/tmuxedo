use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{state::State, tui::WindowTab};

pub async fn handle_input(key: KeyEvent, state: &mut State) {
    match state.search_mode {
        true => handle_search_mode_input(key, state).await,
        false => handle_normal_mode_input(key, state).await,
    }
}

async fn handle_search_mode_input(key: KeyEvent, state: &mut State) {
    if let KeyCode::Esc = key.code {
        state.toggle_search_mode();
        state.clear_search_string();
    }
    if let KeyCode::Char(c) = key.code {
        state.push_letter_to_search_string(c);
    }
    if let KeyCode::Backspace = key.code {
        state.pop_letter_from_search_string();
    }
    if let KeyCode::Enter = key.code {
        state.toggle_search_mode();
        state.reset_selected_installed_plugin();
        state.reset_selected_available_plugin();
    }
}

async fn handle_normal_mode_input(key: KeyEvent, state: &mut State) {
    if let KeyCode::Char('1') = key.code {
        state.set_tab(WindowTab::All);
        state.reset_selected_available_plugin();
        state.reset_selected_installed_plugin();
    }
    if let KeyCode::Char('2') = key.code {
        state.set_tab(WindowTab::Themes);
        state.reset_selected_available_plugin();
        state.reset_selected_installed_plugin();
    }
    if let KeyCode::Char('3') = key.code {
        state.set_tab(WindowTab::StatusBar);
        state.reset_selected_available_plugin();
        state.reset_selected_installed_plugin();
    }
    if let KeyCode::Char('4') = key.code {
        state.set_tab(WindowTab::Plugins);
        state.reset_selected_available_plugin();
        state.reset_selected_installed_plugin();
    }
    if let (KeyCode::Char('o'), KeyModifiers::CONTROL) = (key.code, key.modifiers)
        && state.tab != WindowTab::All
    {
        state.toggle_available();
    }
    if let KeyCode::Char('/') = key.code {
        state.clear_search_string();
        state.toggle_search_mode();
    }
    if let KeyCode::Esc = key.code {
        state.clear_search_string();
    }
    match state.toggle_available_list {
        true => install_actions(key, state).await,
        false => update_and_delete_actions(key, state).await,
    }
}

async fn install_actions(key: KeyEvent, state: &mut State) {
    if let KeyCode::Char('j') = key.code {
        state.next_available_plugin();
    }
    if let KeyCode::Char('k') = key.code {
        state.previous_available_plugin();
    }
    if let KeyCode::Char('I') = key.code {
        state.install_plugin().await;
    }
}

async fn update_and_delete_actions(key: KeyEvent, state: &mut State) {
    if let KeyCode::Char('j') = key.code {
        state.next_installed_plugin();
    }
    if let KeyCode::Char('k') = key.code {
        state.previous_installed_plugin();
    }
    if let KeyCode::Char('U') = key.code {
        state.update_plugin().await;
    }
    if let KeyCode::Char('X') = key.code {
        state.remove_plugin();
    }
}
