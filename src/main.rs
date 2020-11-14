use std::error::Error;
use std::time::Duration;
use std::thread;

use blurz::{
    BluetoothAdapter,
    BluetoothSession,
    BluetoothDiscoverySession,
    BluetoothDevice,
};

fn create_bluetooth_session() -> Result<(
    BluetoothAdapter,
    BluetoothSession,
    BluetoothDiscoverySession
), dyn Error> {
    let session = BluetoothSession::create_session(None)?;

    let adapter = BluetoothAdapter::init(&session)?;
    adapter.set_powered(true)?;

    let disc_session = BluetoothDiscoverySession::create_session(
        &session,
        adapter.get_id()
    )?;

    Ok((adapter, session, disc_session))
}

fn get_devices(
    adapter: &BluetoothAdapter,
    disc_session: &BluetoothDiscoverySession
) -> Result<Vec<String>, dyn Error> {
    thread::sleep(Duration::from_millis(200));
    disc_session.start_discovery()?;
    thread::sleep(Duration::from_millis(2000));

    Ok(adapter.get_device_list()?)
}

fn main() {
    let (
        adapter,
        session,
        disc_session
    ) = create_bluetooth_session()?;

    let devices = get_devices(&adapter, &disc_session)?;

    for device in devices {
        let device = Device::new(&session, device.clone());

        println!("{}", device.get_name()?)
    }
}