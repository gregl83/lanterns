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

pub struct Communicate {
    store: Store
}

impl Communicate {
    pub fn new(store: Store) -> Self {
        Communicate {
            store,
        }
    }
}

impl Screenable for Communicate {
    fn draw(&mut self, state: &mut StatefulList<Device>, f: &mut Frame<CrosstermBackend<Stdout>>) {}
}