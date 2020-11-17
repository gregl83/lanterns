#[allow(dead_code)]
mod util;

use std::io;
use std::error::Error;

use termion::{
    event::Key,
    input::MouseTerminal,
    raw::IntoRawMode,
    screen::AlternateScreen
};
use tui::{
    backend::TermionBackend,
    layout::{
        Constraint,
        Direction,
        Layout,
    },
    style::{
        Color,
        Modifier,
        Style,
    },
    text::{
        Spans,
    },
    widgets::{
        Block,
        Borders,
        List,
        ListItem,
    },
    Terminal,
};

use crate::util::{
    event::{
        Event,
        Events
    },
    StatefulList,
};

use blurz::BluetoothDevice;

use lanterns::{
    create_bluetooth_session,
    create_bluetooth_adapter,
    create_bluetooth_discovery_session,
    get_bluetooth_devices,
};

struct App<'a> {
    devices: StatefulList<&'a str>,
}

impl<'a> App<'a> {
    fn new(devices: Vec<&'a str>) -> App<'a> {
        App {
            devices: StatefulList::with_items(devices),
        }
    }

    fn advance(&mut self) {
        // todo - on frame
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let session = create_bluetooth_session().unwrap();
    let adapter = create_bluetooth_adapter(&session).unwrap();
    let disc_session = create_bluetooth_discovery_session(&session, &adapter).unwrap();

    println!("Searching for bluetooth devices...");

    let devices = get_bluetooth_devices(&adapter, &disc_session).unwrap();

    let mut device_names = vec![];
    for device in devices {
        let device = BluetoothDevice::new(&session, device.clone());
        let name = device.get_name().unwrap();
        device_names.push(name.as_str());
    }

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // App
    let mut app = App::new(device_names);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = app
                .devices
                .items
                .iter()
                .map(|i: &&str| {
                    ListItem::new(vec![Spans::from(*i)]).style(Style::default())
                })
                .collect();
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Devices"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(items, chunks[0], &mut app.devices.state);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.devices.unselect();
                }
                Key::Down => {
                    app.devices.next();
                }
                Key::Up => {
                    app.devices.previous();
                }
                _ => {}
            },
            Event::Tick => {
                app.advance();
            }
        }
    }

    Ok(())
}
