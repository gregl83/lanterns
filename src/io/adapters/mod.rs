pub mod bluetooth;

use std::error::Error;

pub trait Discoverable {
    fn discover_devices() -> Result<Vec<Box<dyn Connectable>>, Box<dyn Error>>;
}

pub trait Connectable {
    fn connect(&self, key: &str) -> Result<(), Box<dyn Error>>;
}

pub struct Device { }

impl Connectable for Device {
    fn connect(&self, _key: &str) -> Result<(), Box<dyn Error>> {
        // fixme
        Ok(())
    }
}

// todo - make struct for bluetooth using traits