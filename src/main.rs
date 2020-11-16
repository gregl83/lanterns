#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

//use blurz::BluetoothDevice;

use lanterns::{
    create_bluetooth_session,
    create_bluetooth_adapter,
    create_bluetooth_discovery_session,
    get_bluetooth_devices,
};

fn main() -> Result<(), Box<dyn Error>> {
    let session = create_bluetooth_session().unwrap();
    let adapter = create_bluetooth_adapter(&session).unwrap();
    let disc_session = create_bluetooth_discovery_session(&session, &adapter).unwrap();

    println!("Searching for bluetooth devices...");

    let devices = get_bluetooth_devices(&adapter, &disc_session).unwrap();

    println!("Found:");

    // for device in devices {
    //     let device = BluetoothDevice::new(&session, device.clone());
    //     let name = device.get_name().unwrap();
    //     println!("\t{}", name);
    // }

    // --

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                        .as_ref(),
                )
                .split(f.size());

            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block, chunks[2]);
        })?;

        if let Event::Input(input) = events.next()? {
            if let Key::Char('q') = input {
                break;
            }
        }
    }

    Ok(())
}
