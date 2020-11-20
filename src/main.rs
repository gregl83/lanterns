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
        DisableMouseCapture,
        KeyCode,
    },
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
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

use crate::io::adapters::bluetooth;
use crate::util::event::{
    Events,
    Event,
};

fn main() -> Result<(), Box<dyn Error>> {
    let session = bluetooth::create_session().unwrap();
    let adapter = bluetooth::create_adapter(&session).unwrap();
    let disc_session = bluetooth::create_discovery_session(&session, &adapter).unwrap();

    println!("Searching for bluetooth devices...");

    let device_paths = bluetooth::get_device_paths(&adapter, &disc_session).unwrap();
    let devices = bluetooth::get_devices(&session, device_paths).unwrap();

    // starting tui-rs + crossterm ---

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut app = Application::new(devices);

    terminal.clear()?;

    loop {
        terminal.draw(|f| draw(f, &mut app))?;
        match events.next()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Left => app.on_left(),
                KeyCode::Up => app.on_up(),
                KeyCode::Right => app.on_right(),
                KeyCode::Down => app.on_down(),
                KeyCode::Enter => app.on_return(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
