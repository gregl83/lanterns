use std::{
    io::Stdout,
    rc::Rc,
    cell::RefCell,
    fmt::Error
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

pub struct Communicate {
    store: Rc<RefCell<Store>>
}

impl Communicate {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Communicate {
            store,
        }
    }
}

impl Screenable for Communicate {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<(), Error> {
        Ok(())
    }
}