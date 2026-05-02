use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use super::theme;

fn esc_indicator() -> Span<'static> {
    Span::styled(
        " esc ✕ ",
        Style::default()
            .fg(theme::text_weaker_color())
            .add_modifier(Modifier::ITALIC),
    )
}

pub fn draw_select_modal(
    frame: &mut Frame<'_>,
    titulo: &str,
    opciones: &[&str],
    seleccion: usize,
    hover_idx: Option<usize>,
) {
    let area_modal = centered_rect(frame.area(), 50, 45);
    frame.render_widget(Clear, area_modal);

    let items = opciones
        .iter()
        .enumerate()
        .map(|(i, opcion)| {
            let is_selected = i == seleccion;
            let is_hovered = hover_idx == Some(i);

            let prefix = if is_selected {
                " "
            } else if is_hovered {
                "▸ "
            } else {
                "  "
            };

            let style = if is_selected {
                Style::default()
                    .fg(theme::text_strong())
                    .bg(theme::surface_color())
                    .add_modifier(Modifier::BOLD)
            } else if is_hovered {
                Style::default().fg(theme::text_strong()).bg(theme::bg_weak())
            } else {
                Style::default().fg(theme::text_base_color())
            };

            ListItem::new(Line::from(vec![
                Span::styled(prefix, if is_hovered { theme::focused() } else { theme::muted() }),
                Span::styled(*opcion, style),
            ]))
        })
        .collect::<Vec<_>>();

    let border_color = if hover_idx.is_some() {
        theme::text_weak_color()
    } else {
        theme::border_weak()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(Line::from(vec![
            Span::styled(
                format!(" {titulo} "),
                Style::default()
                    .fg(theme::primary_color())
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .title(
            Line::from(esc_indicator()).alignment(Alignment::Right),
        )
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme::bg_base()));

    let lista = List::new(items)
        .highlight_style(
            Style::default()
                .fg(theme::text_strong())
                .bg(theme::surface_color())
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ")
        .block(block);

    let mut estado = ListState::default();
    if !opciones.is_empty() {
        estado.select(Some(seleccion.min(opciones.len() - 1)));
    }
    frame.render_stateful_widget(lista, area_modal, &mut estado);
}

pub fn draw_command_palette(
    frame: &mut Frame<'_>,
    opciones: &[&str],
    seleccion: usize,
    hover_idx: Option<usize>,
) {
    let area_modal = centered_rect(frame.area(), 55, 55);
    frame.render_widget(Clear, area_modal);

    let items = opciones
        .iter()
        .enumerate()
        .map(|(i, opcion)| {
            let is_selected = i == seleccion;
            let is_hovered = hover_idx == Some(i);

            let prefix = if is_selected {
                " "
            } else if is_hovered {
                "▸ "
            } else {
                "  "
            };

            let style = if is_selected {
                Style::default()
                    .fg(theme::text_strong())
                    .bg(theme::surface_color())
                    .add_modifier(Modifier::BOLD)
            } else if is_hovered {
                Style::default().fg(theme::text_strong()).bg(theme::bg_weak())
            } else {
                Style::default().fg(theme::text_base_color())
            };

            // Split label and shortcut
            let parts: Vec<&str> = opcion.rsplitn(2, "  ").collect();
            if parts.len() == 2 {
                let shortcut = parts[0];
                let label = parts[1];
                ListItem::new(Line::from(vec![
                    Span::styled(prefix, if is_hovered { theme::focused() } else { theme::muted() }),
                    Span::styled(format!("{:<28}", label), style),
                    Span::styled(
                        shortcut,
                        Style::default().fg(theme::text_weaker_color()),
                    ),
                ]))
            } else {
                ListItem::new(Line::from(vec![
                    Span::styled(prefix, if is_hovered { theme::focused() } else { theme::muted() }),
                    Span::styled(*opcion, style),
                ]))
            }
        })
        .collect::<Vec<_>>();

    let border_color = if hover_idx.is_some() {
        theme::text_weak_color()
    } else {
        theme::border_weak()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(Line::from(vec![
            Span::styled(
                " command palette ",
                Style::default()
                    .fg(theme::primary_color())
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .title(
            Line::from(esc_indicator()).alignment(Alignment::Right),
        )
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme::bg_base()));

    let lista = List::new(items)
        .highlight_style(
            Style::default()
                .fg(theme::text_strong())
                .bg(theme::surface_color())
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ")
        .block(block);

    let mut estado = ListState::default();
    if !opciones.is_empty() {
        estado.select(Some(seleccion.min(opciones.len() - 1)));
    }
    frame.render_stateful_widget(lista, area_modal, &mut estado);
}

pub fn draw_info_modal(frame: &mut Frame<'_>, titulo: &str, contenido: &str) {
    let area_modal = centered_rect(frame.area(), 55, 65);
    frame.render_widget(Clear, area_modal);

    let lines: Vec<Line<'_>> = contenido
        .lines()
        .map(|l| {
            if l.starts_with("───") {
                Line::from(Span::styled(l, Style::default().fg(theme::border_weak())))
            } else if l.is_empty() {
                Line::from("")
            } else {
                Line::from(Span::styled(l, Style::default().fg(theme::text_base_color())))
            }
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(Line::from(vec![
            Span::styled(
                format!(" {titulo} "),
                Style::default()
                    .fg(theme::primary_color())
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .title(
            Line::from(esc_indicator()).alignment(Alignment::Right),
        )
        .border_style(Style::default().fg(theme::border_weak()))
        .style(Style::default().bg(theme::bg_base()));

    let texto = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .block(block);
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
