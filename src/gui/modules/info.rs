use tui::{
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, List
    },
};

pub fn draw_info_bar() -> List<'static> {
    List::new(Vec::new())
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}