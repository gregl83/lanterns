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
use crate::gui::screen::Screenable;
use crate::io::adapters::bluetooth::Device;

pub struct Connection {
    store: Rc<RefCell<Store>>
}

impl Connection {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Connection {
            store
        }
    }
}

impl Screenable for Connection {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {}
}