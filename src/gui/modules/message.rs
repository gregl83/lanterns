use tui::{
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, List
    },
};

pub fn draw_message_input() -> List<'static> {
    List::new(Vec::new())
        .block(Block::default().borders(Borders::ALL).title("Message"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}