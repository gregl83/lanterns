#[allow(dead_code)]
mod util;
mod io;
mod gui;

use std::{
    error::Error,
    io::{
        stdout,
        Write
    },
};

use crossterm::{
    event::{
        EnableMouseCapture,
    },
    execute,
    terminal::{
        enable_raw_mode,
        EnterAlternateScreen,
    },
};

use tui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::gui::{
    Application,
    draw,
};

use blurz::BluetoothDevice;

use crate::io::adapters::{
    create_bluetooth_session,
    create_bluetooth_adapter,
    create_bluetooth_discovery_session,
    get_bluetooth_device_paths,
};
use crate::util::event::Events;

fn main() -> Result<(), Box<dyn Error>> {
    let session = create_bluetooth_session().unwrap();
    let adapter = create_bluetooth_adapter(&session).unwrap();
    let disc_session = create_bluetooth_discovery_session(&session, &adapter).unwrap();

    println!("Searching for bluetooth devices...");

    let device_paths = get_bluetooth_device_paths(&adapter, &disc_session).unwrap();
    let device_names: Vec<String> = device_paths.iter().map(|device_path| {
        let device = BluetoothDevice::new(&session, device_path.clone());
        let device_name = device.get_name().unwrap();
        device_name.clone()
    }).collect();
    let device_names_str: Vec<&str> = device_names.iter().map(|device_name| {
        device_name.as_str()
    }).collect();

    // starting tui-rs + crossterm ---

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // fixme: bind events to application handling

    let _events = Events::new();

    let mut app = Application::new(device_names_str);

    terminal.clear()?;

    loop {
        terminal.draw(|f| draw(f, &mut app))?;
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
