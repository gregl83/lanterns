use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
};

use crate::gui::application::Application;
use super::modules::connection::draw_bluetooth_device_selection;
use super::modules::message::draw_message_input;
use super::modules::info::draw_info_bar;
use crate::util::StatefulList;
use crate::io::adapters::bluetooth::Device;

pub trait Screenable {
    fn draw(&mut self, state: &mut StatefulList<Device>, f: &mut Frame<CrosstermBackend<Stdout>>);

    fn on_key(&mut self) { }
}

pub struct Dashboard {}

impl Screenable for Dashboard {
    fn draw(&mut self, state: &mut StatefulList<Device>, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(6),
            ].as_ref())
            .split(f.size());

        f.render_stateful_widget(
            draw_bluetooth_device_selection(state.items.iter()),
            chunks[0],
            &mut state.state
        );
        f.render_stateful_widget(
            draw_info_bar(),
            chunks[1],
            &mut state.state
        );
        f.render_stateful_widget(
            draw_message_input(),
            chunks[2],
            &mut state.state
        );
    }
}

pub struct Connection {}

impl Screenable for Connection {
    fn draw(&mut self, state: &mut StatefulList<Device>, f: &mut Frame<CrosstermBackend<Stdout>>) {}
}

pub struct Communicate {}

impl Screenable for Communicate {
    fn draw(&mut self, state: &mut StatefulList<Device>, f: &mut Frame<CrosstermBackend<Stdout>>) {}
}