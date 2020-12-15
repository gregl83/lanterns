#[allow(dead_code)]
mod util;
mod io;
mod gui;
mod logger;

use std::{
    error::Error,
    io::{
        stdout,
        Write
    },
};

use log::{LevelFilter};

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

use crate::logger::Log;
use crate::gui::application::Application;
use crate::gui::screens::{
    Screenable,
    Dashboard,
    Connection
};
use crate::io::adapters::{
    Discoverable,
    bluetooth
};
use crate::util::event::{
    Events,
    Event,
};

static LOGGER: Log = Log;

fn main() -> Result<(), Box<dyn Error>> {
    // todo - dynamic level filter (debug mode)
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug)).unwrap();

    let adapter = bluetooth::Adapter::new();

    println!("Searching for bluetooth devices...");

    let devices = adapter.discover_devices().unwrap();

    // starting tui-rs + crossterm ---

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let screens: Vec<Box<dyn Screenable>> = vec![
        Box::new(Dashboard {}),
        Box::new(Connection {}),
    ];

    let mut app = Application::new(screens, devices);

    // todo - state management using Mutex

    terminal.clear()?;

    loop {
        terminal.draw(|f| app.draw(f))?;
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
