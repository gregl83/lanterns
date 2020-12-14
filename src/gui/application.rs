use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::util::StatefulList;
use crate::gui::screens::Screenable;
use crate::gui::screens::Dashboard;
use crate::io::adapters::bluetooth::Device;
use super::modules::connection::draw_bluetooth_device_selection;
use super::modules::message::draw_message_input;
use super::modules::info::draw_info_bar;

pub struct Application {
    screen: Box<dyn Screenable>,
    pub devices: StatefulList<Device>,
    pub should_quit: bool,
}

impl Application {
    pub fn new(devices: Vec<Device>) -> Self {
        Application {
            screen: Box::new(Dashboard {}),
            devices: StatefulList::new(devices),
            should_quit: false,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(6),
            ].as_ref())
            .split(f.size());

        f.render_stateful_widget(
            draw_bluetooth_device_selection(self.devices.items.iter()),
            chunks[0],
            &mut self.devices.state
        );
        f.render_stateful_widget(
            draw_info_bar(),
            chunks[1],
            &mut self.devices.state
        );
        f.render_stateful_widget(
            draw_message_input(),
            chunks[2],
            &mut self.devices.state
        );
    }

    pub fn key_handler(&mut self) {
        // todo - load screen based on conditions

        // todo - handle default keys then pass to screen
    }

    pub fn on_tick(&mut self) { }

    pub fn on_up(&mut self) {
        self.devices.previous();
    }

    pub fn on_right(&mut self) { }

    pub fn on_down(&mut self) {
        self.devices.next();
    }

    pub fn on_left(&mut self) {
        self.devices.unselect();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_return(&mut self) {
        let _device = self.devices.current().unwrap();

        // todo - set device
    }
}