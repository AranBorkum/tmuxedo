use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::{bindings, state::State};

pub fn render_keymap(f: &mut Frame, rect: Rect, state: &State) {
    let block = Block::default().title("Keymap").borders(Borders::ALL);

    let line = bindings::get(state)
        .into_iter()
        .flat_map(|key| {
            vec![
                Span::raw(" <"),
                Span::styled(key.key(), Style::default().fg(Color::Yellow)),
                Span::raw(": "),
                Span::raw(key.repr()),
                Span::raw("> "),
            ]
        })
        .collect::<Vec<_>>();

    let line = Line::from(line);
    let line = Text::from(vec![line]);

    let paragraph = Paragraph::new(line).block(block).style(Style::default());

    f.render_widget(paragraph, rect);
}
