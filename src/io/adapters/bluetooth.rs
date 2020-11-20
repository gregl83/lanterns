use std::error::{Error};
use std::time::Duration;
use std::thread;

use blurz::{BluetoothAdapter, BluetoothSession, BluetoothDiscoverySession, BluetoothDevice};

pub fn create_session() -> Result<BluetoothSession, Box<dyn Error>> {
    Ok(BluetoothSession::create_session(None)?)
}

pub fn create_adapter(session: &BluetoothSession) -> Result<BluetoothAdapter, Box<dyn Error>> {
    let adapter = BluetoothAdapter::init(session)?;
    adapter.set_powered(true)?;
    Ok(adapter)
}

pub fn create_discovery_session<'a>(
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
    disc_session: &BluetoothDiscoverySession
) -> Result<Vec<String>, Box<dyn Error>> {
    thread::sleep(Duration::from_millis(200));
    disc_session.start_discovery()?;
    thread::sleep(Duration::from_millis(5000));

    Ok(adapter.get_device_list()?)
}

pub fn get_devices<'a>(
    session: &'a BluetoothSession,
    device_paths: Vec<String>
) -> Result<Vec<BluetoothDevice<'a>>, Box<dyn Error>> {
    Ok(device_paths.iter().map(|device_path| {
        BluetoothDevice::new(&session, device_path.clone())
    }).collect())
}