use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Backend,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState, Paragraph},
};

use crate::state::State;

pub fn render_installed_list<B: Backend>(f: &mut Frame, rect: Rect, state: &State) {
    let top_bottom = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(rect);

    let title = if !state.toggle_available_list {
        Paragraph::new("Installed Plugins").style(Style::default().add_modifier(Modifier::BOLD))
    } else {
        Paragraph::new("Installed Plugins")
    };
    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_installed_plugin));

    let list_items: Vec<ListItem> = state
        .get_installed_plugins()
        .iter()
        .enumerate()
        .map(|(_i, s)| ListItem::new(format!(" * {}", s.clone())))
        .collect();

    let list = if !state.toggle_available_list {
        List::new(list_items).highlight_style(Style::default().fg(Color::Yellow))
    } else {
        List::new(list_items)
    };

    f.render_widget(title, top_bottom[0]);
    f.render_stateful_widget(list, top_bottom[1], &mut list_state);
}
