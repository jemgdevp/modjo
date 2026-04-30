use ratatui::style::{Color, Modifier, Style};

pub const ORANGE: Color = Color::Rgb(255, 140, 0);
pub const DEEP_ORANGE: Color = Color::Rgb(220, 110, 0);
pub const PANEL_BG: Color = Color::Rgb(22, 22, 22);
pub const TEXT: Color = Color::Rgb(235, 235, 235);
pub const MUTED: Color = Color::Rgb(150, 150, 150);

pub fn title() -> Style {
    Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)
}

pub fn focused() -> Style {
    Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)
}

pub fn normal() -> Style {
    Style::default().fg(TEXT)
}

pub fn muted() -> Style {
    Style::default().fg(MUTED)
}

pub fn sidebar_selected() -> Style {
    Style::default().fg(Color::Black).bg(ORANGE)
}

pub fn border(active: bool) -> Style {
    if active {
        Style::default().fg(ORANGE)
    } else {
        Style::default().fg(DEEP_ORANGE)
    }
}
