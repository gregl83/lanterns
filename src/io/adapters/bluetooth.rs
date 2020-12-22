use std::error::Error;
use std::thread;
use std::time::Duration;

use blurz::{
    BluetoothAdapter,
    BluetoothSession,
    BluetoothDiscoverySession,
    BluetoothDevice,
    BluetoothGATTService,
    BluetoothGATTCharacteristic
};

use crate::io::adapters::{
    Discoverable,
    Connectable,
};

const BLUETOOTH_DISCOVERY_TIME: u64 = 2000;

pub fn get_device_paths(
    adapter: &BluetoothAdapter,
    discovery_session: &BluetoothDiscoverySession
) -> Result<Vec<String>, Box<dyn Error>> {
    discovery_session.start_discovery()?;
    thread::sleep(Duration::from_millis(BLUETOOTH_DISCOVERY_TIME));
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
        )?;

        let mut devices = Vec::new();

        for device_path in device_paths {
            let device = BluetoothDevice::new(
                &self.session,
                device_path.clone()
            );

            // todo - warning for unnamed devices
            if let Ok(name) = device.get_name() {
                devices.push(Device {
                    id: device.get_id(),
                    address: device.get_address().unwrap(),
                    name
                });
            }
        }

        discovery_session.stop_discovery();

        Ok(devices)
    }
}

impl Connectable for Adapter {
    fn connect(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let adapter = self.get_adapter();
        let adapter_id = adapter.get_id();
        let discovery_session = self.get_discovery_session(adapter_id);
        let device_paths = get_device_paths(
            &adapter,
            &discovery_session
        )?;

        'device_loop: for d in device_paths {
            let device = BluetoothDevice::new(&self.session, d.clone());

            println!(
                "Device: Id: {} Address: {:?} Rssi: {:?} Name: {:?}",
                device.get_id(),
                device.get_address(),
                device.get_rssi(),
                device.get_name()
            );

            if device.get_id() != key {
                continue;
            }

            if let Err(e) = device.pair() {
                println!("  Error on pairing: {:?}", e);
            }

            println!("  Is paired: {:?}", device.is_paired());

            //device.connect(5000);

            println!("  Is connected: {:?}", device.is_connected());

            match device.is_ready_to_receive() {
                Some(v) => println!("  Is ready to receive: {:?}", v),
                None => println!("  Error is_ready_to_receive()")
            }

            let all_gatt_services = device.get_gatt_services();

            match all_gatt_services {
                Ok(gatt_services) => {
                    for service in gatt_services {
                        let gatt_service = BluetoothGATTService::new(&self.session, service);

                        println!("  Gatt service Id: {} UUID: {:?} Device : {:?} Is primary: {:?}",
                                 gatt_service.get_id(),
                                 gatt_service.get_uuid(),
                                 gatt_service.get_device(),
                                 gatt_service.is_primary());

                        match gatt_service.get_gatt_characteristics() {
                            Ok(ref gat_chars) => {
                                for characteristics in gat_chars {
                                    let gatt_char = BluetoothGATTCharacteristic::new(&self.session, characteristics.to_owned());

                                    println!("    Characteristic Name: {} UUID: {:?} Flags: {:?}",
                                             characteristics, gatt_char.get_uuid(),
                                             gatt_char.get_flags());
                                }
                            },
                            Err(e) => println!("    Error get_gatt_characteristics(): {:?}", e)
                        }
                    }
                },
                Err(e) => println!("{:?}", e)
            }

            if let Err(e) = device.disconnect() {
                println!("  Error on disconnect: {:?}", e);
            }

            adapter.remove_device(device.get_id())?;
        }

        discovery_session.stop_discovery();

        Ok(())
    }

    fn get_name(&self) -> &str {
        ""
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