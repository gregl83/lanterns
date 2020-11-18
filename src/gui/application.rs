use crate::util::StatefulList;

pub struct Application<'a> {
    pub devices: StatefulList<&'a str>,
}

impl<'a> Application<'a> {
    pub fn new(devices: Vec<&'a str>) -> Application<'a> {
        Application {
            devices: StatefulList::with_items(devices),
        }
    }

    pub fn advance(&mut self) {
        // todo - on frame
    }
}