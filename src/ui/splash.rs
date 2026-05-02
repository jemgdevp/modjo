use std::{thread, time::Duration};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn mostrar_splash_carga(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let bg = Color::Rgb(16, 16, 16);
    let border = Color::Rgb(40, 40, 40);
    let primary = Color::Rgb(250, 178, 131);
    let text_weak = Color::Rgb(112, 112, 112);

    for (idx, frame) in frames.iter().enumerate() {
        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(35),
                    Constraint::Length(10),
                    Constraint::Min(1),
                ])
                .split(f.area());

            let pulse = (idx as u8).min(5);
            let accent = if pulse % 2 == 0 { primary } else { Color::Rgb(220, 155, 110) };

            let bloque = Block::default()
                .title(Span::styled(
                    " modjo ",
                    Style::default()
                        .fg(primary)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(border))
                .style(Style::default().bg(bg));

            let arte = vec![
                Line::from(Span::styled(
                    "███╗   ███╗ ██████╗ ██████╗      ██╗ ██████╗ ",
                    Style::default().fg(accent),
                )),
                Line::from(Span::styled(
                    "████╗ ████║██╔═══██╗██╔══██╗     ██║██╔═══██╗",
                    Style::default().fg(accent),
                )),
                Line::from(Span::styled(
                    "██╔████╔██║██║   ██║██║  ██║     ██║██║   ██║",
                    Style::default().fg(accent),
                )),
                Line::from(Span::styled(
                    "██║╚██╔╝██║██║   ██║██║  ██║██   ██║██║   ██║",
                    Style::default().fg(accent),
                )),
                Line::from(Span::styled(
                    "██║ ╚═╝ ██║╚██████╔╝██████╔╝╚█████╔╝╚██████╔╝",
                    Style::default().fg(accent),
                )),
                Line::from(vec![
                    Span::styled(format!(" {} ", frame), Style::default().fg(primary).add_modifier(Modifier::BOLD)),
                    Span::styled("initializing...", Style::default().fg(text_weak)),
                ]),
            ];
            let p = Paragraph::new(arte).block(bloque);
            f.render_widget(p, chunks[1]);
        });
        thread::sleep(Duration::from_millis(70));
    }
}
