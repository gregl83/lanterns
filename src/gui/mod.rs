mod application;

pub use application::Application;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Spans},
    widgets::{
        Block, Borders, List, ListItem
    },
    Frame,
};

fn draw_bluetooth_device_selection<B: Backend>(f: &mut Frame<B>, app: &mut Application, chunk: Rect) {
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

fn draw_message_input<B: Backend>(f: &mut Frame<B>, app: &mut Application, chunk: Rect) {
    let items = List::new(Vec::new())
        .block(Block::default().borders(Borders::ALL).title("Message"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, chunk, &mut app.devices.state);
}

fn draw_info_bar<B: Backend>(f: &mut Frame<B>, app: &mut Application, chunk: Rect) {
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

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut Application) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(6),
        ].as_ref())
        .split(f.size());

    draw_bluetooth_device_selection(f, app, chunks[0]);
    draw_info_bar(f, app, chunks[1]);
    draw_message_input(f, app, chunks[2]);
}