use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use super::theme;

pub fn draw_select_modal(
    frame: &mut Frame<'_>,
    titulo: &str,
    opciones: &[&str],
    seleccion: usize,
) {
    let area_modal = centered_rect(frame.area(), 55, 50);
    frame.render_widget(Clear, area_modal);

    let items = opciones
        .iter()
        .map(|opcion| ListItem::new(Line::from(*opcion)))
        .collect::<Vec<_>>();

    let lista = List::new(items)
        .highlight_style(theme::sidebar_selected())
        .highlight_symbol(">> ")
        .block(
            Block::default()
                .title(format!(" {titulo} "))
                .title_style(theme::title())
                .borders(Borders::ALL)
                .border_style(theme::border(true)),
        );

    let mut estado = ListState::default();
    if !opciones.is_empty() {
        estado.select(Some(seleccion.min(opciones.len() - 1)));
    }
    frame.render_stateful_widget(lista, area_modal, &mut estado);
}

pub fn draw_info_modal(frame: &mut Frame<'_>, titulo: &str, contenido: &str) {
    let area_modal = centered_rect(frame.area(), 70, 60);
    frame.render_widget(Clear, area_modal);
    let texto = Paragraph::new(contenido)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(format!(" {titulo} "))
                .title_style(theme::title())
                .borders(Borders::ALL)
                .border_style(theme::border(true)),
        );
    frame.render_widget(texto, area_modal);
}

fn centered_rect(area_total: Rect, ancho_pct: u16, alto_pct: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - alto_pct) / 2),
            Constraint::Percentage(alto_pct),
            Constraint::Percentage((100 - alto_pct) / 2),
        ])
        .split(area_total);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - ancho_pct) / 2),
            Constraint::Percentage(ancho_pct),
            Constraint::Percentage((100 - ancho_pct) / 2),
        ])
        .split(vertical[1])[1]
}
