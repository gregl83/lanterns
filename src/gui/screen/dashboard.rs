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

use crate::gui::store::Store;
use crate::util::StatefulList;
use crate::gui::screen::Screenable;
use crate::gui::modules::connection::draw_bluetooth_device_selection;
use crate::gui::modules::message::draw_message_input;
use crate::gui::modules::info::draw_info_bar;
use crate::io::adapters::bluetooth::Device;

pub struct Dashboard {
    store: Rc<RefCell<Store>>
}

impl Dashboard {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Dashboard {
            store,
        }
    }
}

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