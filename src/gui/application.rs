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

    pub fn on_up(&mut self) {
        //self.devices.previous();
    }

    pub fn on_right(&mut self) { }

    pub fn on_down(&mut self) {
        //self.devices.next();
    }

    pub fn on_left(&mut self) {
        //self.devices.unselect();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {
                let screen: &mut dyn Screenable = self.screens[self.screen_index].borrow_mut();
                screen.on_key(c);
            }
        }
    }

    pub fn on_return(&mut self) {
        // let _device = self.devices.current().unwrap();

        // fixme - propagate and set device
    }
}