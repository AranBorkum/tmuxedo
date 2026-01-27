use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

pub const BANNER: &str = r#"████████╗███╗   ███╗██╗   ██╗██╗  ██╗███████╗██████╗  ██████╗ 
╚══██╔══╝████╗ ████║██║   ██║╚██╗██╔╝██╔════╝██╔══██╗██╔═══██╗
   ██║   ██╔████╔██║██║   ██║ ╚███╔╝ █████╗  ██║  ██║██║   ██║
   ██║   ██║╚██╔╝██║██║   ██║ ██╔██╗ ██╔══╝  ██║  ██║██║   ██║
   ██║   ██║ ╚═╝ ██║╚██████╔╝██╔╝ ██╗███████╗██████╔╝╚██████╔╝
   ╚═╝   ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═════╝  ╚═════╝ "#;

pub fn render_banner(f: &mut Frame, chunk: Rect) {
    let block = Paragraph::new(BANNER)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(block, chunk);
}
