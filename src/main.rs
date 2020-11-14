use std::error::Error;
use std::time::Duration;
use std::thread;

use blurz::{
    BluetoothAdapter,
    BluetoothSession,
    BluetoothDiscoverySession,
    BluetoothDevice,
};

fn create_bluetooth_session() -> Result<BluetoothSession, Box<dyn Error>> {
    Ok(BluetoothSession::create_session(None)?)
}

fn create_bluetooth_adapter(session: &BluetoothSession) -> Result<BluetoothAdapter, Box<dyn Error>> {
    let adapter = BluetoothAdapter::init(session)?;
    adapter.set_powered(true)?;
    Ok(adapter)
}

fn create_bluetooth_discovery_session<'a>(
    session: &'a BluetoothSession,
    adapter: &'a BluetoothAdapter
) -> Result<BluetoothDiscoverySession<'a>, Box<dyn Error>> {
    Ok(BluetoothDiscoverySession::create_session(
        session,
        adapter.get_id()
    )?)
}

fn get_devices(
    adapter: &BluetoothAdapter,
    disc_session: &BluetoothDiscoverySession
) -> Result<Vec<String>, Box<dyn Error>> {
    thread::sleep(Duration::from_millis(200));
    disc_session.start_discovery()?;
    thread::sleep(Duration::from_millis(5000));

    Ok(adapter.get_device_list()?)
}

fn main() {
    let session = create_bluetooth_session().unwrap();
    let adapter = create_bluetooth_adapter(&session).unwrap();
    let disc_session = create_bluetooth_discovery_session(&session, &adapter).unwrap();

    println!("Searching for bluetooth devices...");

    let devices = get_devices(&adapter, &disc_session).unwrap();

    println!("Found:");

    for device in devices {
        let device = BluetoothDevice::new(&session, device.clone());
        let name = device.get_name().unwrap();
        println!("\t{}", name);
    }
}