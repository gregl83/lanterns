use std::error::Error;
use std::io;

use blurz::BluetoothDevice;

use self::util::{
    event::{Event, Events},
    StatefulList,
};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use lanterns::{
    create_bluetooth_session,
    create_bluetooth_adapter,
    create_bluetooth_discovery_session,
    get_bluetooth_devices,
    App,
};

fn main() -> Result<(), Box<dyn Error>> {
    let session = create_bluetooth_session().unwrap();
    let adapter = create_bluetooth_adapter(&session).unwrap();
    let disc_session = create_bluetooth_discovery_session(&session, &adapter).unwrap();

    println!("Searching for bluetooth devices...");

    let devices = get_bluetooth_devices(&adapter, &disc_session).unwrap();

    println!("Found:");

    for device in devices {
        let device = BluetoothDevice::new(&session, device.clone());
        let name = device.get_name().unwrap();
        println!("\t{}", name);
    }

    // ---

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = app
                .items
                .items
                .iter()
                .map(|i| {
                    let mut lines = vec![Spans::from(i.0)];
                    for _ in 0..i.1 {
                        lines.push(Spans::from(Span::styled(
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                            Style::default().add_modifier(Modifier::ITALIC),
                        )));
                    }
                    ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
                })
                .collect();
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(items, chunks[0], &mut app.items.state);

            let events: Vec<ListItem> = app
                .events
                .iter()
                .map(|&(evt, level)| {
                    let s = match level {
                        "CRITICAL" => Style::default().fg(Color::Red),
                        "ERROR" => Style::default().fg(Color::Magenta),
                        "WARNING" => Style::default().fg(Color::Yellow),
                        "INFO" => Style::default().fg(Color::Blue),
                        _ => Style::default(),
                    };
                    let header = Spans::from(vec![
                        Span::styled(format!("{:<9}", level), s),
                        Span::raw(" "),
                        Span::styled(
                            "2020-01-01 10:00:00",
                            Style::default().add_modifier(Modifier::ITALIC),
                        ),
                    ]);
                    let log = Spans::from(vec![Span::raw(evt)]);
                    ListItem::new(vec![
                        Spans::from("-".repeat(chunks[1].width as usize)),
                        header,
                        Spans::from(""),
                        log,
                    ])
                })
                .collect();
            let events_list = List::new(events)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .start_corner(Corner::BottomLeft);
            f.render_widget(events_list, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.items.unselect();
                }
                Key::Down => {
                    app.items.next();
                }
                Key::Up => {
                    app.items.previous();
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