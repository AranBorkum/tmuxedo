use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
};

use crate::{state::State, tui::WindowTab};

pub fn render_tabs(f: &mut Frame, rect: Rect, state: &State) {
    let tab_titles = [
        WindowTab::All.repr(),
        WindowTab::Themes.repr(),
        WindowTab::StatusBar.repr(),
        WindowTab::Plugins.repr(),
    ];

    let titles: Vec<Line> = tab_titles
        .iter()
        .map(|t| {
            Line::from(Span::styled(
                format!(" {t} "),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::BOTTOM))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .select(state.tab.index())
        .divider(Span::raw(" | "));

    f.render_widget(tabs, rect);
}
