use std::error::{Error};
use std::time::Duration;
use std::thread;

use blurz::{
    BluetoothAdapter,
    BluetoothSession,
    BluetoothDiscoverySession,
};

pub fn create_bluetooth_session() -> Result<BluetoothSession, Box<dyn Error>> {
    Ok(BluetoothSession::create_session(None)?)
}

pub fn create_bluetooth_adapter(session: &BluetoothSession) -> Result<BluetoothAdapter, Box<dyn Error>> {
    let adapter = BluetoothAdapter::init(session)?;
    adapter.set_powered(true)?;
    Ok(adapter)
}

pub fn create_bluetooth_discovery_session<'a>(
    session: &'a BluetoothSession,
    adapter: &'a BluetoothAdapter
) -> Result<BluetoothDiscoverySession<'a>, Box<dyn Error>> {
    Ok(BluetoothDiscoverySession::create_session(
        session,
        adapter.get_id()
    )?)
}

pub fn get_bluetooth_device_paths(
    adapter: &BluetoothAdapter,
    disc_session: &BluetoothDiscoverySession
) -> Result<Vec<String>, Box<dyn Error>> {
    thread::sleep(Duration::from_millis(200));
    disc_session.start_discovery()?;
    thread::sleep(Duration::from_millis(5000));

    Ok(adapter.get_device_list()?)
}