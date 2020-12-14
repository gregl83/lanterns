use std::slice::Iter;

use tui::{
    style::{Color, Modifier, Style},
    text::{Spans},
    widgets::{
        Block, Borders, List, ListItem
    },
};

use crate::io::adapters::bluetooth::Device;

pub fn draw_bluetooth_device_selection(devices: Iter<'_, Device>) -> List<'static> {
    let items: Vec<ListItem> = devices
        .map(|device| {
            ListItem::new(vec![Spans::from(device.name.clone())]).style(Style::default())
        })
        .collect();
    List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Devices"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}