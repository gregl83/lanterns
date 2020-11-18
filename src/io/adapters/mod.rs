mod bluetooth;

use std::error::Error;

pub use bluetooth::*;

pub trait Connectable {
    fn discover_devices() -> Result<Vec<String>, Box<dyn Error>>;

    fn connect() -> Result<(), Box<dyn Error>>;
}

// todo - make struct for bluetooth using traits