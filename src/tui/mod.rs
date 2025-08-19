use std::io;

use crossterm::event::{self, KeyEvent};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
};

use crate::{
    state::State,
    tui::{
        installed_list::render_installed_list, keymap::render_keymap, list::render_list,
        tabs::render_tabs,
    },
};

mod installed_list;
mod keymap;
mod list;
mod tabs;

pub async fn run_tmuxedo_tui<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut state = State::default().await;
    let mut index = 0;

    loop {
        match index {
            0 => index += 1,
            1 => {
                state.check_for_plugin_updated().await;
                index += 1;
            }
            _ => {}
        }

        terminal.draw(|f| {
            render(f, &state);
        })?;
        if event::poll(std::time::Duration::from_millis(100))?
            && let event::Event::Key(key) = event::read()?
        {
            if let event::KeyCode::Char('q') = key.code {
                break;
            }
            if let event::KeyCode::Char('1') = key.code {
                state.set_tab(WindowTab::All);
                state.reset_selected_available_plugin();
                state.reset_selected_installed_plugin();
            }
            if let event::KeyCode::Char('2') = key.code {
                state.set_tab(WindowTab::Themes);
                state.reset_selected_available_plugin();
                state.reset_selected_installed_plugin();
            }
            if let event::KeyCode::Char('3') = key.code {
                state.set_tab(WindowTab::StatusBar);
                state.reset_selected_available_plugin();
                state.reset_selected_installed_plugin();
            }
            if let event::KeyCode::Char('4') = key.code {
                state.set_tab(WindowTab::Plugins);
                state.reset_selected_available_plugin();
                state.reset_selected_installed_plugin();
            }
            if let (event::KeyCode::Char('o'), event::KeyModifiers::CONTROL) =
                (key.code, key.modifiers)
            {
                state.toggle_available();
            }
            match state.toggle_available_list {
                true => available_plugins_actions(key, &mut state).await,
                false => installed_plugins_actions(key, &mut state).await,
            }
        }
    }

    Ok(())
}

async fn available_plugins_actions(key: KeyEvent, state: &mut State) {
    if let event::KeyCode::Char('j') = key.code {
        state.next_available_plugin();
    }
    if let event::KeyCode::Char('k') = key.code {
        state.previous_available_plugin();
    }
    if let event::KeyCode::Char('I') = key.code {
        state.install_plugin().await;
    }
}

async fn installed_plugins_actions(key: KeyEvent, state: &mut State) {
    if let event::KeyCode::Char('j') = key.code {
        state.next_installed_plugin();
    }
    if let event::KeyCode::Char('k') = key.code {
        state.previous_installed_plugin();
    }
    if let event::KeyCode::Char('U') = key.code {
        state.update_plugin().await;
    }
    if let event::KeyCode::Char('X') = key.code {
        state.remove_plugin();
    }
}

fn render(f: &mut Frame, state: &State) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Max(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    render_tabs(f, chunks[0], state);
    render_installed_list(f, chunks[1], state);
    if state.tab != WindowTab::All {
        render_list(f, chunks[2], state);
    }
    render_keymap(f, chunks[3], state);
}

#[derive(PartialEq)]
pub enum WindowTab {
    All,
    Themes,
    StatusBar,
    Plugins,
}

impl WindowTab {
    pub fn repr(&self) -> String {
        match self {
            Self::All => String::from("All (1)"),
            Self::Themes => String::from("Themes (2)"),
            Self::StatusBar => String::from("Status Bar (3)"),
            Self::Plugins => String::from("Plugins (4)"),
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Self::All => 0,
            Self::Themes => 1,
            Self::StatusBar => 2,
            Self::Plugins => 3,
        }
    }
}
