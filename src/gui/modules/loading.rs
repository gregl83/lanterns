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
        Text,
        Spans,
        Span
    }
};

pub fn draw_loading_message(height: u16, offset: u16, message: &'static str) -> Paragraph<'static> {
    let midpoint = height / 2;
    let start_point = if height > 10 { midpoint - offset } else { 0 };
    let padding_top = "\r\n".repeat(start_point as usize);
    let mut text = Text::raw(padding_top).lines.clone();
    text.append(&mut vec![
        Spans::from(vec![
            Span::raw(message),
        ]),
    ]);
    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}