pub mod bluetooth;

use std::error::Error;

pub trait Discoverable {
    fn discover_devices(&mut self) -> Result<(), Box<dyn Error>>;

    fn borrow_devices(&self) -> &Vec<Box<dyn Connectable>>;
}

pub trait Connectable {
    fn connect(&self, key: &str) -> Result<(), Box<dyn Error>>;

    fn get_name(&self) -> &str;
}