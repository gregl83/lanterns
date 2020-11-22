pub mod bluetooth;

use std::error::Error;

pub trait Discoverable<'a> {
    fn discover_devices(&'a mut self) -> Result<(), Box<dyn Error>>;

    fn borrow_devices(&'a self) -> &Vec<Box<dyn Connectable>>;
}

pub trait Connectable {
    fn connect(&self, key: &str) -> Result<(), Box<dyn Error>>;

    fn get_name(&self) -> &str;
}