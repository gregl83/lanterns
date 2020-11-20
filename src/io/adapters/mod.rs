mod bluetooth;

use std::error::Error;

pub use bluetooth::*;

pub trait Discoverable {
    fn discover_devices() -> Result<Vec<dyn Connectable>, Box<dyn Error>>;
}

pub trait Connectable {
    type Key;

    fn connect(key: key) -> Result<(), Box<dyn Error>>;
}

pub struct Device { }

impl Connectable for Device {
    type Key = str;

    fn connect(key: Key) -> Result<(), Box<dyn Error>> {
        // fixme
    }
}

// todo - make struct for bluetooth using traits