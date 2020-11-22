mod application;

pub use application::Application;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Spans},
    widgets::{
        Block, Borders, List, ListItem
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut Application) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app
        .devices
        .items
        .iter()
        .map(|device| {
            let device_name = device.get_name();
            ListItem::new(vec![Spans::from(device_name)]).style(Style::default())
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
    f.render_stateful_widget(items, chunks[0], &mut app.devices.state);
}