use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
};

use crate::gui::store::Store;
use crate::util::StatefulList;
use crate::gui::screen::Screenable;
use crate::io::adapters::bluetooth::Device;

pub struct Connection {
    store: Store
}

impl Connection {
    pub fn new(store: Store) -> Self {
        Connection {
            store,
        }
    }
}

impl Screenable for Connection {
    fn draw(&mut self, state: &mut StatefulList<Device>, f: &mut Frame<CrosstermBackend<Stdout>>) {}
}