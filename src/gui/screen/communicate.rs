use std::{
    process::exit,
    io::Stdout,
    rc::Rc,
    cell::RefCell,
    time::Duration,
    thread,
    fmt::Error
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
use std::io::ErrorKind;

const BLUETOOTH_ERROR_TIME: u64 = 2000;

pub struct Communicate {
    store: Rc<RefCell<Store>>,
    adapter: Rc<RefCell<Adapter>>,
    pub devices: StatefulList<Device>,
    rendered: bool,
    failed: bool,
}

impl Communicate {
    pub fn new(store: Rc<RefCell<Store>>, adapter: Rc<RefCell<Adapter>>) -> Self {
        Communicate {
            store,
            adapter,
            devices: StatefulList::new(Vec::new()),
            rendered: false,
            failed: false,
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
                "Searching for Bluetooth devices."
            ),
            chunks[0]
        );
    }

    fn draw_discover_error(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
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
                "Please make sure Bluetooth is enabled."
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

impl Screenable for Communicate {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<(), Error> {
        if self.failed {
            thread::sleep(Duration::from_millis(BLUETOOTH_ERROR_TIME));
            return Err(Error)
        }

        if self.devices.items.is_empty() {
            self.draw_discover(f);

            // wait iteration for discover module to render frame prior to blocking call
            if self.rendered == true {
                let devices = {
                    self.adapter.borrow_mut().discover_devices()
                };
                match devices {
                    Ok(devices) => self.devices = StatefulList::new(devices),
                    Err(e) => {
                        self.failed = true;
                        self.draw_discover_error(f)
                    },
                }
            }
            self.rendered = true;
        } else {
            self.draw_devices(f);
        }

        Ok(())
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