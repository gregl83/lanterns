use crate::util::StatefulList;
use crate::io::adapters::bluetooth::Device;

pub struct Application {
    pub devices: StatefulList<Device>,
    pub should_quit: bool,
}

impl Application {
    pub fn new(devices: Vec<Device>) -> Self {
        Application {
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