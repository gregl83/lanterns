use crate::gui::Application;

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Color, Modifier, Style},
    text::{Spans},
    widgets::{
        Block, Borders, List, ListItem
    },
    Frame,
};

pub fn draw_bluetooth_device_selection<B: Backend>(f: &mut Frame<B>, app: &mut Application, chunk: Rect) {
    let items: Vec<ListItem> = app
        .devices
        .items
        .iter()
        .map(|device| {
            ListItem::new(vec![Spans::from(device.name.clone())]).style(Style::default())
        })
        .collect();
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Devices"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, chunk, &mut app.devices.state);
}