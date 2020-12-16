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
use crate::gui::modules::connection::draw_bluetooth_device_selection;
use crate::gui::modules::message::draw_message_input;
use crate::gui::modules::info::draw_info_bar;
use crate::io::adapters::bluetooth::Device;

pub struct Connection {
    store: Rc<RefCell<Store>>,
    pub devices: StatefulList<Device>,
}

impl Connection {
    pub fn new(store: Rc<RefCell<Store>>, devices: Vec<Device>) -> Self {
        Connection {
            store,
            devices: StatefulList::new(devices)
        }
    }
}

impl Screenable for Connection {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
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

    fn on_key(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Left => self.devices.unselect(),
            KeyCode::Up => self.devices.previous(),
            KeyCode::Down => self.devices.next(),
            KeyCode::Enter => {
                let _device = self.devices.current().unwrap();
            },
            _ => {}
        }
    }
}