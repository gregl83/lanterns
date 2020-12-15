use std::error::Error;
use std::thread;
use std::time::Duration;

use blurz::{BluetoothAdapter, BluetoothSession, BluetoothDiscoverySession, BluetoothDevice};

use crate::io::adapters::{
    Discoverable,
    Connectable,
};

pub fn get_device_paths(
    adapter: &BluetoothAdapter,
    discovery_session: &BluetoothDiscoverySession
) -> Result<Vec<String>, Box<dyn Error>> {
    thread::sleep(Duration::from_millis(200));
    discovery_session.start_discovery()?;
    thread::sleep(Duration::from_millis(5000));
    Ok(adapter.get_device_list()?)
}

pub struct Adapter {
    session: BluetoothSession
}

impl Adapter {
    pub fn new() -> Self {
        Adapter {
            session: BluetoothSession::create_session(None).unwrap()
        }
    }

    fn get_adapter(&self) -> BluetoothAdapter {
        BluetoothAdapter::init(&self.session).unwrap()
    }

    fn get_discovery_session(&self, adapter_id: String) -> BluetoothDiscoverySession {
        BluetoothDiscoverySession::create_session(
            &self.session,
            adapter_id
        ).unwrap()
    }
}

impl Discoverable for Adapter {
    fn discover_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        let adapter = self.get_adapter();
        let adapter_id = adapter.get_id();
        let discovery_session = self.get_discovery_session(adapter_id);

        let device_paths = get_device_paths(
            &adapter,
            &discovery_session
        ).unwrap();

        let mut devices = Vec::new();

        for device_path in device_paths {
            let device = BluetoothDevice::new(
                &self.session,
                device_path.clone()
            );

            devices.push(Device {
                id: device.get_id(),
                address: device.get_address().unwrap(),
                name: device.get_name().unwrap(),
            });
        }

        Ok(devices)
    }
}

#[derive(Debug)]
pub struct Device {
    id: String,
    address: String,
    pub name: String,
}

impl Connectable for Device {
    fn connect(&self, _key: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}