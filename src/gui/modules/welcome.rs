use tui::{
    layout::Alignment,
    style::{
        Color,
        Modifier,
        Style
    },
    widgets::{
        Block,
        Borders,
        List,
        Paragraph,
        Wrap
    },
    text::{
        Spans,
        Span
    }
};

pub fn draw_welcome_message(height: u16) -> Paragraph<'static> {
    let text = vec![
        Spans::from(vec![
            Span::raw("Lanterns"),
        ]),
        Spans::from(vec![
            Span::raw("")
        ]),
        Spans::from(vec![
            Span::styled("Private Communications No Holds Barred",Style::default().add_modifier(Modifier::ITALIC)),
        ]),
    ];
    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}