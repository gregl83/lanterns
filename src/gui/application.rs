use std::{
    io::Stdout,
    rc::Rc,
    cell::RefCell,
    borrow::BorrowMut,
    collections::HashMap
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame,
};
use crossterm::event::KeyCode;

use crate::gui::store::Store;
use crate::gui::screen::Screenable;
use crate::gui::router::{
    Route,
    Router,
};
use crate::io::adapters::bluetooth::Device;
use crate::util::StatefulList;

pub struct Application {
    store: Rc<RefCell<Store>>,
    router: Router,
    pub should_quit: bool,
}

impl Application {
    pub fn new(store: Rc<RefCell<Store>>, router: Router) -> Self {
        Application {
            store,
            router,
            should_quit: false,
        }
    }

    pub fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let screen = self.router.get_screen();
        match screen.draw(f) {
            Ok(_) => {},
            Err(e) => self.should_quit = true,
        }
    }

    pub fn on_tick(&mut self) { }

    pub fn on_key(&mut self, key_code: KeyCode) {
        let screen = self.router.get_screen();
        screen.on_key(key_code);
    }
}