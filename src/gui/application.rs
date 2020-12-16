use std::{
    io::Stdout,
    rc::Rc,
    cell::RefCell,
    borrow::BorrowMut
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame,
};
use crossterm::event::KeyCode;

use crate::gui::store::Store;
use crate::gui::screen::Screenable;
use crate::io::adapters::bluetooth::Device;
use crate::util::StatefulList;

pub struct Application {
    store: Rc<RefCell<Store>>,
    screens: Vec<Box<dyn Screenable>>,
    screen_index: usize,
    pub should_quit: bool,
}

impl Application {
    pub fn new(store: Rc<RefCell<Store>>, screens: Vec<Box<dyn Screenable>>) -> Self {
        Application {
            store,
            screens,
            screen_index: 0,
            should_quit: false,
        }
    }

    pub fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let screen: &mut dyn Screenable = self.screens[self.screen_index].borrow_mut();
        screen.draw(f);
    }

    pub fn on_tick(&mut self) { }

    pub fn on_key(&mut self, key_code: KeyCode) {
        let screen: &mut dyn Screenable = self.screens[self.screen_index].borrow_mut();
        screen.on_key(key_code);
    }
}