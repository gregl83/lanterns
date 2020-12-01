pub mod bluetooth;

use std::error::Error;
use bluetooth::Device;

pub trait Discoverable {
    fn discover_devices(&self) -> Result<Vec<Device>, Box<dyn Error>>;
}

pub trait Connectable {
    fn connect(&self, key: &str) -> Result<(), Box<dyn Error>>;

    fn get_name(&self) -> &str;
}