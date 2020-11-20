use crate::util::StatefulList;

pub struct Application<'a> {
    pub devices: StatefulList<&'a str>,
    pub should_quit: bool,
}

impl<'a> Application<'a> {
    pub fn new(devices: Vec<&'a str>) -> Application<'a> {
        Application {
            devices: StatefulList::with_items(devices),
            should_quit: false,
        }
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
        let device = self.devices.current();

        // todo - set device
    }
}