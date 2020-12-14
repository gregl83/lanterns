use crate::gui::Application;

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, List
    },
    Frame,
};

pub fn draw_info_bar<B: Backend>(f: &mut Frame<B>, app: &mut Application, chunk: Rect) {
    let items = List::new(Vec::new())
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, chunk, &mut app.devices.state);
}