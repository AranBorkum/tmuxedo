use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::state::State;

pub fn render_search_box(f: &mut Frame, state: &State) {
    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    let search_window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(40),
            Constraint::Min(0),
        ])
        .split(chunk[1]);

    let text = Paragraph::new(state.search_string.clone())
        .style(Style::default())
        .block(
            Block::default()
                .title(Span::from("Search"))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(text, search_window[1]);
}
