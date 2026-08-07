use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{state::State, tui::WindowTab};

pub async fn handle_input(key: KeyEvent, state: &mut State) {
    match state.search_mode {
        true => handle_search_mode_input(key, state).await,
        false => handle_normal_mode_input(key, state).await,
    }
}

async fn handle_search_mode_input(key: KeyEvent, state: &mut State) {
    match key.code {
        KeyCode::Esc => exit_search(state),
        KeyCode::Char(c) => state.push_letter_to_search_string(c),
        KeyCode::Backspace => state.pop_letter_from_search_string(),
        KeyCode::Enter => submit_search_string(state),
        _ => {}
    }
}

async fn handle_normal_mode_input(key: KeyEvent, state: &mut State) {
    match key.code {
        KeyCode::Char('1') => switch_tab(WindowTab::All, state),
        KeyCode::Char('2') => switch_tab(WindowTab::Themes, state),
        KeyCode::Char('3') => switch_tab(WindowTab::StatusBar, state),
        KeyCode::Char('4') => switch_tab(WindowTab::Plugins, state),
        KeyCode::Char('/') => open_search_bar(state),
        KeyCode::Esc => clear_search_string(state),
        _ => {}
    }

    if let (KeyCode::Char('o'), KeyModifiers::CONTROL) = (key.code, key.modifiers) {
        toggle_available(state)
    }

    match state.toggle_available_list {
        true => install_actions(key, state).await,
        false => update_and_delete_actions(key, state).await,
    }
}

async fn install_actions(key: KeyEvent, state: &mut State) {
    match key.code {
        KeyCode::Char('j') => state.next_available_plugin(),
        KeyCode::Char('k') => state.previous_available_plugin(),
        KeyCode::Char('I') => state.install_plugin().await,
        _ => {}
    }
}

async fn update_and_delete_actions(key: KeyEvent, state: &mut State) {
    match key.code {
        KeyCode::Char('j') => state.next_installed_plugin(),
        KeyCode::Char('k') => state.previous_installed_plugin(),
        KeyCode::Char('U') => state.update_plugin().await,
        KeyCode::Char('X') => state.remove_plugin(),
        _ => {}
    }
}

fn exit_search(state: &mut State) {
    state.toggle_search_mode();
    state.clear_search_string();
}

fn submit_search_string(state: &mut State) {
    state.toggle_search_mode();
    state.reset_selected_installed_plugin();
    state.reset_selected_available_plugin();
}

fn switch_tab(tab: WindowTab, state: &mut State) {
    state.set_tab(tab);
    state.reset_selected_available_plugin();
    state.reset_selected_installed_plugin();
}

fn open_search_bar(state: &mut State) {
    state.clear_search_string();
    state.toggle_search_mode();
}

fn clear_search_string(state: &mut State) {
    if !state.search_mode {
        state.clear_search_string();
    }
}

fn toggle_available(state: &mut State) {
    if state.tab != WindowTab::All {
        state.toggle_available();
    }
}
