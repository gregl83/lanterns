use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::util::StatefulList;
use crate::gui::screens::Screenable;
use crate::gui::screens::Dashboard;
use crate::io::adapters::bluetooth::Device;
use tui::backend::CrosstermBackend;
use std::io::Stdout;
use std::borrow::{Borrow, BorrowMut};

pub struct Application {
    screens: Vec<Box<dyn Screenable>>,
    screen_index: usize,
    pub devices: StatefulList<Device>,
    pub should_quit: bool,
}

impl Application {
    pub fn new(screens: Vec<Box<dyn Screenable>>, devices: Vec<Device>) -> Self {
        Application {
            screens,
            screen_index: 0,
            devices: StatefulList::new(devices),
            should_quit: false,
        }
    }

    pub fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let screen: &mut dyn Screenable = self.screens[self.screen_index].borrow_mut();
        screen.draw(self.devices.borrow_mut(), f);
    }

    pub fn key_handler(&mut self) {
        // todo - load screen based on conditions

        // todo - handle default keys then pass to screen
    }

    pub fn on_tick(&mut self) { }

    pub fn on_up(&mut self) {
        self.devices.previous();
    }

    pub fn on_right(&mut self) { }

    pub fn on_down(&mut self) {
        self.devices.next();
    }

    pub fn on_left(&mut self) {
        self.devices.unselect();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_return(&mut self) {
        let _device = self.devices.current().unwrap();

        // todo - set device
    }
}