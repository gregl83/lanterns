use std::error::Error;
use std::time::Duration;
use std::thread;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_session::BluetoothSession as Session;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;

fn print_devices() -> Result<(), Box<dyn Error>> {
    let bt_session = &Session::create_session(None)?;
    let adapter: Adapter = Adapter::init(bt_session)?;
    adapter.set_powered(true)?;

    let session = DiscoverySession::create_session(
        &bt_session,
        adapter.get_id()
    )?;
    thread::sleep(Duration::from_millis(200));
    session.start_discovery()?;
    thread::sleep(Duration::from_millis(2000));
    let devices = adapter.get_device_list()?;

    for device in devices {
        let device = Device::new(bt_session, device.clone());

        println!("{}", device.get_name()?)
    }

    Ok(())
}

fn main() {
    match print_devices() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}