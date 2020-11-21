use std::error::{Error};
use std::time::Duration;
use std::thread;

use blurz::{BluetoothAdapter, BluetoothSession, BluetoothDiscoverySession, BluetoothDevice};

use crate::io::adapters::{
    Discoverable,
    Connectable,
};

fn create_session() -> Result<BluetoothSession, Box<dyn Error>> {
    Ok(BluetoothSession::create_session(None)?)
}

fn create_adapter(session: &BluetoothSession) -> Result<BluetoothAdapter, Box<dyn Error>> {
    let adapter = BluetoothAdapter::init(session)?;
    adapter.set_powered(true)?;
    Ok(adapter)
}

fn create_discovery_session<'a>(
    session: &'a BluetoothSession,
    adapter: &'a BluetoothAdapter
) -> Result<BluetoothDiscoverySession<'a>, Box<dyn Error>> {
    Ok(BluetoothDiscoverySession::create_session(
        session,
        adapter.get_id()
    )?)
}

pub fn get_device_paths(
    adapter: &BluetoothAdapter,
    discovery_session: &BluetoothDiscoverySession
) -> Result<Vec<String>, Box<dyn Error>> {
    thread::sleep(Duration::from_millis(200));
    discovery_session.start_discovery()?;
    thread::sleep(Duration::from_millis(5000));
    Ok(adapter.get_device_list()?)
}

pub struct Adapter<'a> {
    session: BluetoothSession,
    adapter: BluetoothAdapter<'a>,
    discovery_session: BluetoothDiscoverySession<'a>,
    devices: Vec<Box<dyn Connectable>>,
}

impl<'a> Adapter<'a> {
    pub fn new() -> Self {
        let session = create_session().unwrap();
        let adapter = create_adapter(&session).unwrap();
        let discovery_session = create_discovery_session(&session, &adapter).unwrap();
        Adapter {
            session,
            adapter,
            discovery_session,
            devices: Vec::new()
        }
    }
}

impl<'a> Discoverable for Adapter<'a> {
    fn discover_devices(&mut self) -> Result<(), Box<dyn Error>> {
        let device_paths = get_device_paths(&self.adapter, &self.discovery_session).unwrap();

        self.devices.clear();

        for device_path in device_paths {
            self.devices.push(
                Box::new(
                    Device {
                        device: BluetoothDevice::new(&self.session, device_path.clone())
                    }
                )
            );
        }

        Ok(())
    }

    fn borrow_devices(&self) -> &Vec<Box<dyn Connectable>> {
        &self.devices
    }
}

struct Device<'a> {
    device: BluetoothDevice<'a>
}

impl<'a> Connectable for Device<'a> {
    fn connect(&self, _key: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn get_name(&self) -> &str {
        self.get_name()
    }
}