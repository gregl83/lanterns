use std::error::Error;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use blurz::{BluetoothAdapter, BluetoothSession, BluetoothDiscoverySession, BluetoothDevice};

use crate::io::adapters::{
    Discoverable,
    Connectable,
};
use std::borrow::Borrow;

fn create_session() -> Result<BluetoothSession, Box<dyn Error>> {
    Ok(BluetoothSession::create_session(None)?)
}

fn create_adapter<'a>(session: Rc<BluetoothSession>) -> Result<BluetoothAdapter<'a>, Box<dyn Error>> {
    let adapter = BluetoothAdapter::init(&session)?;
    adapter.set_powered(true)?;
    Ok(adapter)
}

fn create_discovery_session<'a>(
    session: Rc<BluetoothSession>,
    adapter: Rc<BluetoothAdapter>
) -> Result<BluetoothDiscoverySession<'a>, Box<dyn Error>> {
    Ok(BluetoothDiscoverySession::create_session(
        &session,
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
    session: Rc<BluetoothSession>,
    adapter: Rc<BluetoothAdapter<'a>>,
    discovery_session: Rc<BluetoothDiscoverySession<'a>>,
    devices: Vec<Box<dyn Connectable>>,
}

impl<'a> Adapter<'a> {
    pub fn new() -> Self {
        let session = Rc::new(
            create_session().unwrap()
        );

        let adapter = Rc::new(
            create_adapter(session.clone()).unwrap()
        );

        let discovery_session = Rc::new(
            create_discovery_session(session.clone(), adapter.clone()).unwrap()
        );

        Adapter {
            session,
            adapter,
            discovery_session,
            devices: Vec::new()
        }
    }
}

impl<'a> Discoverable<'a> for Adapter<'a> {
    fn discover_devices(&'a mut self) -> Result<(), Box<dyn Error>> {
        let device_paths = get_device_paths(&self.adapter, &self.discovery_session).unwrap();

        self.devices.clear();

        for device_path in device_paths {
            self.devices.push(
                Box::new(
                    Device {
                        device: BluetoothDevice::new(
                            &Rc::clone(&self.session),
                            device_path.clone()
                        )
                    }
                )
            );
        }

        Ok(())
    }

    fn borrow_devices(&'a self) -> &Vec<Box<dyn Connectable>> {
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