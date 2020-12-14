use crate::util::StatefulList;
use crate::gui::screens::Screenable;
use crate::gui::screens::Dashboard;
use crate::io::adapters::bluetooth::Device;

pub struct Application {
    screen: Box<dyn Screenable>,
    pub devices: StatefulList<Device>,
    pub should_quit: bool,
}

impl Application {
    pub fn new(devices: Vec<Device>) -> Self {
        Application {
            screen: Box::new(Dashboard {}),
            devices: StatefulList::new(devices),
            should_quit: false,
        }
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