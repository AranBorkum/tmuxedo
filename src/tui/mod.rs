use std::{error::Error, io};

use ratatui::{Terminal, prelude::CrosstermBackend};

use crossterm::event::{self};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
};

use crate::{
    state::State,
    tui::{
        input::handle_input, ui_banner::render_banner, ui_installed_list::render_installed_list,
        ui_keymap::render_keymap, ui_list::render_list, ui_search_box::render_search_box,
        ui_tabs::render_tabs,
    },
};

mod input;
mod ui_banner;
mod ui_installed_list;
mod ui_keymap;
mod ui_list;
mod ui_search_box;
mod ui_tabs;

pub async fn run() -> Result<(), Box<dyn Error>> {
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
            if let event::KeyCode::Char('q') = key.code
                && !state.search_mode
            {
                break;
            }
            handle_input(key, &mut state).await;
        }
    }

    Ok(())
}

fn render(f: &mut Frame, state: &State) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(6),
                Constraint::Length(2),
                Constraint::Max(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    render_banner(f, chunks[0]);
    render_tabs(f, chunks[1], state);
    render_installed_list(f, chunks[2], state);
    if state.tab != WindowTab::All {
        render_list(f, chunks[3], state);
    }
    render_keymap(f, chunks[4], state);
    if state.search_mode {
        render_search_box(f, state);
    }
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
