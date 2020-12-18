use std::{
    io::Stdout,
    rc::Rc,
    cell::RefCell
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
};
use crossterm::event::KeyCode;

use crate::gui::store::Store;
use crate::util::StatefulList;
use crate::gui::screen::Screenable;
use crate::gui::modules::loading::draw_loading_message;
use crate::gui::modules::connection::draw_bluetooth_device_selection;
use crate::gui::modules::message::draw_message_input;
use crate::gui::modules::info::draw_info_bar;
use crate::io::adapters::{
    Discoverable,
    bluetooth::{
        Device,
        Adapter
    }
};

pub struct Connection {
    store: Rc<RefCell<Store>>,
    adapter: Adapter,
    pub devices: StatefulList<Device>,
    initialized: bool
}

impl Connection {
    pub fn new(store: Rc<RefCell<Store>>, adapter: Adapter) -> Self {
        Connection {
            store,
            adapter,
            devices: StatefulList::new(Vec::new()),
            initialized: false
        }
    }

    fn draw_discover(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
            ].as_ref())
            .split(f.size());

        f.render_widget(
            draw_loading_message(
                f.size().height,
                5,
                "Searching For Bluetooth Devices"
            ),
            chunks[0]
        );
    }

    fn draw_devices(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
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
}

impl Screenable for Connection {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        if self.initialized == true {
            self.draw_devices(f);
        } else {
            self.draw_discover(f);
            // fixme - move discovery to new thread
            // self.devices = StatefulList::new(self.adapter.discover_devices().unwrap());
            self.initialized = true;
        }
    }

    fn on_key(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Left => self.devices.unselect(),
            KeyCode::Up => self.devices.previous(),
            KeyCode::Down => self.devices.next(),
            KeyCode::Enter => {
                //let _device = self.devices.current().unwrap();
            },
            _ => {}
        }
    }
}