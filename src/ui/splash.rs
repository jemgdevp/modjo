use std::{thread, time::Duration};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub fn mostrar_splash_carga(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    for (idx, frame) in frames.iter().enumerate() {
        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(35), Constraint::Length(10), Constraint::Min(1)])
                .split(f.area());

            let brillo = 180 + (idx as u8 * 7);
            let naranja = ratatui::style::Color::Rgb(255, brillo.min(245), 40);
            let bloque = Block::default()
                .title(" MODJO BOOT ")
                .borders(Borders::ALL)
                .border_style(ratatui::style::Style::default().fg(naranja));

            let arte = vec![
                Line::from("███╗   ███╗ ██████╗ ██████╗      ██╗ ██████╗").fg(naranja),
                Line::from("████╗ ████║██╔═══██╗██╔══██╗     ██║██╔═══██╗").fg(naranja),
                Line::from("██╔████╔██║██║   ██║██║  ██║     ██║██║   ██║").fg(naranja),
                Line::from("██║╚██╔╝██║██║   ██║██║  ██║██   ██║██║   ██║").fg(naranja),
                Line::from("██║ ╚═╝ ██║╚██████╔╝██████╔╝╚█████╔╝╚██████╔╝").fg(naranja),
                Line::from(format!("{frame} Inicializando motor TUI... estilo parcero")),
            ];
            let p = Paragraph::new(arte).block(bloque);
            f.render_widget(p, chunks[1]);
        });
        thread::sleep(Duration::from_millis(70));
    }
}
