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

pub struct Call {
    store: Rc<RefCell<Store>>
}

impl Call {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Call {
            store,
        }
    }
}

impl Screenable for Call {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {}
}