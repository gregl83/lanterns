mod application;
mod router;
mod screens;
mod state;
mod modules;

pub use application::Application;
use modules::connection::draw_bluetooth_device_selection;
use modules::message::draw_message_input;
use modules::info::draw_info_bar;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

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