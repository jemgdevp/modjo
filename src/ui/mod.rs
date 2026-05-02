pub mod theme;
pub mod splash;
pub mod components;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::{
    EstadoApp, HoverZone, ModalActivo, ModoEntrada, SidebarActiva, ZonaFoco,
    etiqueta_accion_menu, metodos_http,
};

const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

fn spinner_frame(tick: u64) -> &'static str {
    SPINNER[(tick as usize) % SPINNER.len()]
}

fn rounded_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
}

pub fn render(frame: &mut Frame<'_>, app: &EstadoApp) {
    let principal = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(frame.area());

    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(app.ancho_sidebar), Constraint::Min(10)])
        .split(principal[0]);

    render_sidebar(frame, app, root[0]);
    render_main(frame, app, root[1]);
    render_statusbar(frame, app, principal[1]);
    render_toast(frame, app);
    render_clipboard_anim(frame, app);
    render_resize_hints(frame, app);
    render_modales(frame, app);
}

// ── Sidebar ────────────────────────────────────────────────────────────

fn render_sidebar(frame: &mut Frame<'_>, app: &EstadoApp, area: Rect) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(3)])
        .split(area);

    // Tabs (no border, just a line)
    let hist_hover = app.mouse.hover == HoverZone::SidebarTabHist;
    let col_hover = app.mouse.hover == HoverZone::SidebarTabCol;

    let hist_style = if app.sidebar == SidebarActiva::Historial {
        theme::focused().add_modifier(Modifier::BOLD)
    } else if hist_hover {
        theme::base()
    } else {
        theme::muted()
    };
    let col_style = if app.sidebar == SidebarActiva::Colecciones {
        theme::focused().add_modifier(Modifier::BOLD)
    } else if col_hover {
        theme::base()
    } else {
        theme::muted()
    };

    let hist_prefix = if app.sidebar == SidebarActiva::Historial {
        " "
    } else if hist_hover {
        " "
    } else {
        "  "
    };
    let col_prefix = if app.sidebar == SidebarActiva::Colecciones {
        " "
    } else if col_hover {
        " "
    } else {
        "  "
    };

    let tab_line = Line::from(vec![
        Span::styled(hist_prefix, theme::focused()),
        Span::styled("history ", hist_style),
        Span::styled(col_prefix, theme::focused()),
        Span::styled("collections", col_style),
    ]);
    let tabs = Paragraph::new(tab_line);
    frame.render_widget(tabs, sections[0]);

    // Items with hover
    let items: Vec<ListItem<'_>> = match app.sidebar {
        SidebarActiva::Historial => app
            .history
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let method_color = theme::method_color(&entry.method);
                let status_color = match entry.status {
                    Some(s) if (200..300).contains(&s) => theme::success_color(),
                    Some(s) if (400..500).contains(&s) => theme::warning_color(),
                    Some(s) if s >= 500 => theme::error_color(),
                    _ => theme::text_weak_color(),
                };
                let is_hovered = app.mouse.hover == HoverZone::SidebarItem(i);
                let is_selected = app.sidebar_index == i && app.foco == ZonaFoco::Sidebar;

                let prefix = if is_selected {
                    " "
                } else if is_hovered {
                    "▸ "
                } else {
                    "  "
                };
                let item_style = if is_hovered && !is_selected {
                    Style::default().fg(theme::text_strong())
                } else {
                    Style::default()
                };

                ListItem::new(Line::from(vec![
                    Span::styled(prefix, if is_hovered { theme::focused() } else { theme::muted() }),
                    Span::styled(
                        format!("{:<8}", entry.method),
                        Style::default()
                            .fg(method_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        entry.url.chars().take(28).collect::<String>(),
                        if is_hovered { theme::normal() } else { theme::base() },
                    ),
                    Span::styled(
                        format!(" [{}]", entry.status.unwrap_or(0)),
                        Style::default().fg(status_color),
                    ),
                ]))
                .style(item_style)
            })
            .collect(),
        SidebarActiva::Colecciones => app
            .collections
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let method_color = theme::method_color(&entry.request.method);
                let is_hovered = app.mouse.hover == HoverZone::SidebarItem(i);
                let is_selected = app.sidebar_index == i && app.foco == ZonaFoco::Sidebar;

                let prefix = if is_selected {
                    " "
                } else if is_hovered {
                    "▸ "
                } else {
                    "  "
                };
                let item_style = if is_hovered && !is_selected {
                    Style::default().fg(theme::text_strong())
                } else {
                    Style::default()
                };

                ListItem::new(Line::from(vec![
                    Span::styled(prefix, if is_hovered { theme::focused() } else { theme::muted() }),
                    Span::styled(
                        format!("{:<8}", entry.request.method),
                        Style::default()
                            .fg(method_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        entry.name.clone(),
                        if is_hovered { theme::normal() } else { theme::base() },
                    ),
                ]))
                .style(item_style)
            })
            .collect(),
    };

    let list = List::new(items)
        .highlight_style(theme::sidebar_selected())
        .highlight_symbol(" ");

    let mut state = ratatui::widgets::ListState::default();
    if app.sidebar_len() > 0 {
        state.select(Some(app.sidebar_index.min(app.sidebar_len() - 1)));
    }
    frame.render_stateful_widget(list, sections[1], &mut state);
}

// ── Main content ───────────────────────────────────────────────────────

fn render_main(frame: &mut Frame<'_>, app: &EstadoApp, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(app.alto_headers),
            Constraint::Length(app.alto_body),
            Constraint::Min(6),
        ])
        .split(area);

    // Method + URL
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Min(10)])
        .split(rows[0]);

    // Method box
    let method_color = theme::method_color(&app.request.method);
    let method = Paragraph::new(Span::styled(
        &app.request.method,
        Style::default()
            .fg(method_color)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(ratatui::layout::Alignment::Center)
    .block(
        rounded_block()
            .title(Span::styled(" method ", theme::title()))
            .border_style(panel_border(app, ZonaFoco::Metodo, HoverZone::BtnMetodo)),
    );
    frame.render_widget(method, top[0]);

    // URL box
    let url_block = render_editable_text(app, &app.request.url, "url", ZonaFoco::Url, HoverZone::BtnUrl);
    frame.render_widget(url_block, top[1]);

    // Headers
    let header_lines: Vec<Line<'_>> = app
        .request
        .headers
        .iter()
        .map(|(k, v)| {
            Line::from(vec![
                Span::styled(k.clone(), Style::default().fg(theme::teal_color())),
                Span::styled(": ", theme::muted()),
                Span::styled(v.clone(), theme::base()),
            ])
        })
        .collect();
    let headers = Paragraph::new(header_lines)
        .wrap(Wrap { trim: false })
        .block(
            rounded_block()
                .title(Span::styled(" headers ", theme::title()))
                .border_style(panel_border(app, ZonaFoco::Headers, HoverZone::BtnHeaders)),
        );
    frame.render_widget(headers, rows[1]);

    // Body
    let body_block = render_editable_text(app, &app.request.body, "body", ZonaFoco::Body, HoverZone::BtnBody);
    frame.render_widget(body_block, rows[2]);

    // Response
    render_response(frame, app, rows[3]);
}

fn render_editable_text<'a>(
    app: &EstadoApp,
    texto: &'a str,
    titulo: &str,
    zona: ZonaFoco,
    hover: HoverZone,
) -> Paragraph<'a> {
    let is_focused = app.foco == zona;
    let is_inserting = is_focused && app.modo == ModoEntrada::Insertar;

    let bg = if is_focused {
        theme::surface_color()
    } else {
        theme::bg_base()
    };

    // Show placeholder when empty
    if texto.is_empty() && !is_inserting {
        let placeholder = match zona {
            ZonaFoco::Body => "{ }",
            ZonaFoco::Url => "https://...",
            ZonaFoco::Headers => "Key: Value",
            _ => "",
        };
        if !placeholder.is_empty() {
            return Paragraph::new(Span::styled(
                placeholder,
                Style::default().fg(theme::text_weaker_color()),
            ))
            .style(normal_input_style(is_focused))
            .block(
                rounded_block()
                    .title(Span::styled(format!(" {titulo} "), theme::title()))
                    .border_style(panel_border(app, zona, hover)),
            );
        }
    }

    if is_inserting {
        let pos = app.cursor.posicion.min(texto.len());

        // Build lines with cursor
        let all_lines: Vec<Line<'_>> = texto
            .split('\n')
            .enumerate()
            .map(|(line_idx, line_text)| {
                let line_start = texto
                    .split('\n')
                    .take(line_idx)
                    .map(|l| l.len() + 1)
                    .sum::<usize>();

                if pos >= line_start && pos <= line_start + line_text.len() {
                    // Cursor is on this line
                    let col = pos - line_start;
                    let before: String = line_text.chars().take(col).collect();
                    let cursor_char = line_text.chars().nth(col).unwrap_or(' ');
                    let after: String = line_text.chars().skip(col + 1).collect();

                    let blink = (app.tick / 8) % 2 == 0;
                    let cursor_style = if blink {
                        Style::default()
                            .fg(theme::bg_base())
                            .bg(theme::primary_color())
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .fg(theme::bg_base())
                            .bg(theme::primary_dim_color())
                    };

                    Line::from(vec![
                        Span::styled(before, Style::default().fg(theme::text_strong()).bg(bg)),
                        Span::styled(cursor_char.to_string(), cursor_style.bg(bg)),
                        Span::styled(after, Style::default().fg(theme::text_strong()).bg(bg)),
                    ])
                } else {
                    Line::from(Span::styled(
                        line_text,
                        Style::default().fg(theme::text_strong()).bg(bg),
                    ))
                }
            })
            .collect();

        Paragraph::new(all_lines).block(
            rounded_block()
                .title(Line::from(vec![
                    Span::styled(format!(" {titulo} "), theme::title()),
                    Span::styled(
                        format!(" {} ", spinner_frame(app.tick)),
                        Style::default().fg(theme::warning_color()),
                    ),
                ]))
                .border_style(Style::default().fg(theme::warning_color())),
        )
    } else {
        Paragraph::new(texto)
            .style(normal_input_style(is_focused))
            .wrap(Wrap { trim: false })
            .block(
                rounded_block()
                    .title(Span::styled(format!(" {titulo} "), theme::title()))
                    .border_style(panel_border(app, zona, hover)),
            )
    }
}

fn render_response(frame: &mut Frame<'_>, app: &EstadoApp, area: Rect) {
    let response_text = if let Some(err) = &app.last_error {
        vec![
            Line::from(Span::styled(
                " error ",
                Style::default()
                    .fg(theme::error_color())
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(err.clone(), theme::base())),
        ]
    } else if let Some(resp) = &app.response {
        let status_color = match resp.status {
            Some(s) if (200..300).contains(&s) => theme::success_color(),
            Some(s) if (400..500).contains(&s) => theme::warning_color(),
            Some(s) if s >= 500 => theme::error_color(),
            _ => theme::text_weak_color(),
        };

        let mut lines = vec![
            Line::from(vec![
                Span::styled("status ", theme::muted()),
                Span::styled(
                    format!("{} {}", resp.status.unwrap_or(0), resp.status_text),
                    Style::default()
                        .fg(status_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("   ", theme::muted()),
                Span::styled("time ", theme::muted()),
                Span::styled(format!("{}ms", resp.duration_ms), theme::teal_color()),
                Span::styled("   ", theme::muted()),
                Span::styled("size ", theme::muted()),
                Span::styled(format!("{}B", resp.size_bytes), theme::teal_color()),
            ]),
            Line::from(""),
        ];

        // Add syntax-highlighted body lines with scroll offset
        let body_lines: Vec<Line<'_>> = app
            .response_body_display()
            .lines()
            .map(|l| highlight_json_line(l))
            .collect();
        let scroll = app.response_scroll as usize;
        let visible_height = area.height.saturating_sub(4) as usize; // header + padding
        let total_lines = body_lines.len();

        let visible: Vec<Line<'_>> = body_lines
            .into_iter()
            .skip(scroll)
            .take(visible_height)
            .collect();
        lines.extend(visible);

        // Scroll indicator
        if total_lines > visible_height {
            let pct = if total_lines > 0 {
                ((scroll + visible_height) * 100 / total_lines).min(100)
            } else {
                100
            };
            lines.push(Line::from(Span::styled(
                format!(
                    "  ── {}% ({}/{}) ──",
                    pct,
                    scroll.min(total_lines),
                    total_lines
                ),
                Style::default().fg(theme::text_weaker_color()),
            )));
        }

        lines
    } else {
        vec![
            Line::from(Span::styled(
                " ready ",
                Style::default()
                    .fg(theme::success_color())
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled("r  run request", theme::muted())),
            Line::from(Span::styled("w  save to collection", theme::muted())),
            Line::from(Span::styled("m  method selector", theme::muted())),
            Line::from(Span::styled("i  insert mode", theme::muted())),
            Line::from(Span::styled("?  help", theme::muted())),
            Line::from(Span::styled("u  undo", theme::muted())),
            Line::from(Span::styled("y/p  yank/paste", theme::muted())),
            Line::from(Span::styled("f  toggle json format", theme::muted())),
            Line::from(Span::styled("pgup/pgdn  scroll response", theme::muted())),
        ]
    };

    let resp_title = if app.loading {
        Line::from(vec![
            Span::styled(
                format!(" {} ", spinner_frame(app.tick)),
                Style::default()
                    .fg(theme::warning_color())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("sending... ", theme::muted()),
        ])
    } else {
        let format_hint = if app.response.is_some() {
            if app.response_formatted {
                " json "
            } else {
                " raw "
            }
        } else {
            ""
        };
        Line::from(vec![
            Span::styled(" response ", theme::title()),
            Span::styled(format_hint, Style::default().fg(theme::text_weaker_color())),
        ])
    };

    let border = if app.loading {
        Style::default().fg(theme::warning_color())
    } else {
        panel_border(app, ZonaFoco::Response, HoverZone::BtnResponse)
    };

    let response = Paragraph::new(response_text)
        .style(normal_input_style(app.foco == ZonaFoco::Response))
        .wrap(Wrap { trim: false })
        .block(rounded_block().title(resp_title).border_style(border));
    frame.render_widget(response, area);
}

fn highlight_json_line(line: &str) -> Line<'_> {
    let trimmed = line.trim_start();
    let indent: String = line.chars().take(line.len() - trimmed.len()).collect();

    if trimmed.is_empty() {
        return Line::from(Span::styled(indent, theme::base()));
    }

    let mut spans = vec![Span::styled(indent, theme::base())];

    // Detect if line starts with a key (has a colon)
    if let Some(colon_pos) = trimmed.find(':') {
        let key_part = &trimmed[..colon_pos];
        let rest = &trimmed[colon_pos..];

        // Key
        let key_clean = key_part.trim_matches('"').trim_matches(',');
        let prefix = if key_part.starts_with('"') { "\"" } else { "" };
        let suffix = if key_part.ends_with(',') { "," } else { "" };
        spans.push(Span::styled(
            format!("{prefix}{key_clean}{prefix}{suffix}"),
            Style::default().fg(theme::teal_color()), // keys in teal
        ));

        // Colon and space
        if rest.starts_with(':') {
            spans.push(Span::styled(": ", theme::muted()));
            let value = rest[1..].trim();

            // Value highlighting
            if value.is_empty() {
                // nothing
            } else if value == "{" || value == "[" || value == "}," || value == "]," {
                spans.push(Span::styled(value, theme::muted()));
            } else if value == "null" {
                spans.push(Span::styled(value, Style::default().fg(theme::text_weaker_color())));
            } else if value == "true" || value == "false" {
                spans.push(Span::styled(
                    value,
                    Style::default().fg(theme::warning_color()).add_modifier(Modifier::BOLD),
                ));
            } else if value.starts_with('"') {
                // String value
                let s = value.trim_matches('"');
                spans.push(Span::styled(
                    format!("\"{}\"", s),
                    Style::default().fg(theme::success_color()),
                ));
            } else if value.parse::<f64>().is_ok() {
                // Number
                spans.push(Span::styled(
                    value,
                    Style::default().fg(theme::primary_color()).add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(value, theme::base()));
            }
        }
    } else {
        // Pure structural: {, }, [, ], etc.
        spans.push(Span::styled(trimmed, theme::muted()));
    }

    Line::from(spans)
}

// ── Resize hints ───────────────────────────────────────────────────────

fn render_resize_hints(frame: &mut Frame<'_>, app: &EstadoApp) {
    // Show resize cursor hint when hovering resize zones
    let (zone, label) = match app.mouse.hover {
        HoverZone::ResizeSidebar => (true, " ↔ resize "),
        HoverZone::ResizeHeaders => (true, " ↕ resize "),
        HoverZone::ResizeBody => (true, " ↕ resize "),
        _ => (false, ""),
    };
    if !zone {
        return;
    }
    let area = frame.area();
    let hint_rect = Rect {
        x: area.x + app.ancho_sidebar.saturating_sub(label.len() as u16 / 2),
        y: area.y,
        width: label.len() as u16,
        height: 1,
    };
    let p = Paragraph::new(Span::styled(
        label,
        Style::default()
            .fg(theme::warning_color())
            .add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(p, hint_rect);
}

// ── Status bar ─────────────────────────────────────────────────────────

fn render_statusbar(frame: &mut Frame<'_>, app: &EstadoApp, area: Rect) {
    let mode_hover = app.mouse.hover == HoverZone::StatusMode;

    let modo = match app.modo {
        ModoEntrada::Normal => {
            let bg = if mode_hover {
                Color::Rgb(255, 200, 150)
            } else {
                theme::primary_color()
            };
            Span::styled(
                " NORMAL ",
                Style::default()
                    .fg(theme::bg_base())
                    .bg(bg)
                    .add_modifier(Modifier::BOLD),
            )
        }
        ModoEntrada::Insertar => {
            let bg = if mode_hover {
                Color::Rgb(255, 230, 100)
            } else {
                theme::warning_color()
            };
            Span::styled(
                " INSERT ",
                Style::default()
                    .fg(theme::bg_base())
                    .bg(bg)
                    .add_modifier(Modifier::BOLD),
            )
        }
    };

    let undo_count = if app.undo_stack.is_empty() {
        String::new()
    } else {
        format!(" [{}u]", app.undo_stack.len())
    };

    let clip_indicator = if app.clipboard_anim.as_ref().map_or(false, |c| c.activo()) {
        let alpha = 1.0 - app.clipboard_anim.as_ref().unwrap().progreso();
        let brightness = (200.0 * alpha) as u8 + 55;
        Span::styled(
            format!(" {} ", spinner_frame(app.tick)),
            Style::default().fg(Color::Rgb(brightness, brightness, 55)),
        )
    } else if !app.portapapeles_interno.is_empty() {
        Span::styled(" y ", Style::default().fg(theme::text_weaker_color()))
    } else {
        Span::raw("")
    };

    let line = Line::from(vec![
        modo,
        Span::styled("  ", theme::status_style()),
        Span::styled(
            &app.mensaje_estado,
            Style::default()
                .fg(theme::text_base_color())
                .bg(theme::bg_weak()),
        ),
        Span::styled(
            &undo_count,
            Style::default().fg(theme::text_weaker_color()).bg(theme::bg_weak()),
        ),
        Span::styled(" ", theme::status_style()),
        clip_indicator,
    ]);

    let status = Paragraph::new(line).style(theme::status_style());
    frame.render_widget(status, area);
}

use ratatui::style::Color;

// ── Toast overlay ──────────────────────────────────────────────────────

fn render_toast(frame: &mut Frame<'_>, app: &EstadoApp) {
    if let Some(ref toast) = app.toast {
        if !toast.activo() {
            return;
        }
        let elapsed = toast.creado_en.elapsed().as_millis() as f32;
        let progress = (elapsed / toast.duracion.as_millis() as f32).min(1.0);

        let alpha = if progress > 0.7 {
            ((1.0 - progress) / 0.3 * 255.0) as u8
        } else {
            255
        };

        let texto = format!(" {} ", toast.mensaje);
        let ancho = texto.len() as u16 + 2;
        let area_total = frame.area();

        let toast_rect = Rect {
            x: area_total.x + area_total.width.saturating_sub(ancho + 2),
            y: area_total.y + 1,
            width: ancho.min(area_total.width),
            height: 1,
        };

        let fg = Color::Rgb(alpha, alpha, alpha);
        let p = Paragraph::new(Span::styled(
            &texto,
            Style::default().fg(fg).add_modifier(Modifier::BOLD),
        ));
        frame.render_widget(p, toast_rect);
    }
}

// ── Clipboard ninja animation ──────────────────────────────────────────

fn render_clipboard_anim(frame: &mut Frame<'_>, app: &EstadoApp) {
    if let Some(ref clip) = app.clipboard_anim {
        if !clip.activo() {
            return;
        }
        let progress = clip.progreso();
        let bounce = if progress < 0.3 {
            (progress / 0.3 * 3.0) as u16
        } else if progress < 0.6 {
            3 - ((progress - 0.3) / 0.3 * 3.0) as u16
        } else {
            0
        };

        let ninja = if progress < 0.5 { "*snip*" } else { "*woosh" };
        let texto = format!("{} ", ninja);
        let area_total = frame.area();

        let anim_rect = Rect {
            x: area_total.x + area_total.width.saturating_sub(texto.len() as u16 + 3),
            y: area_total.y + 2 + bounce,
            width: texto.len() as u16 + 1,
            height: 1,
        };

        let alpha = ((1.0 - progress) * 255.0) as u8;
        let p = Paragraph::new(Span::styled(
            &texto,
            Style::default()
                .fg(Color::Rgb(55, alpha, 55))
                .add_modifier(Modifier::BOLD),
        ));
        frame.render_widget(p, anim_rect);
    }
}

// ── Modales ────────────────────────────────────────────────────────────

fn render_modales(frame: &mut Frame<'_>, app: &EstadoApp) {
    match app.modal_activo {
        ModalActivo::Ninguno => {}
        ModalActivo::SelectorMetodo => {
            let hover_idx = match app.mouse.hover {
                HoverZone::ModalItem(i) => Some(i),
                _ => None,
            };
            components::draw_select_modal(
                frame,
                "select http method",
                metodos_http(),
                app.metodo_idx_selector,
                hover_idx,
            );
        }
        ModalActivo::SelectorMenuSuperior => {
            let opciones = app
                .menu_superior_activo
                .opciones()
                .iter()
                .map(|accion| etiqueta_accion_menu(*accion))
                .collect::<Vec<_>>();
            let hover_idx = match app.mouse.hover {
                HoverZone::ModalItem(i) => Some(i),
                _ => None,
            };
            components::draw_select_modal(
                frame,
                &format!("menu {}", app.menu_superior_activo.etiqueta()),
                &opciones,
                app.menu_selector_idx,
                hover_idx,
            );
        }
        ModalActivo::CommandPalette => {
            let comandos = crate::app::comandos_disponibles();
            let labels: Vec<String> = comandos
                .iter()
                .map(|c| format!("{:<28} {}", c.etiqueta, c.atajo))
                .collect();
            let refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
            let hover_idx = match app.mouse.hover {
                HoverZone::ModalItem(i) => Some(i),
                _ => None,
            };
            components::draw_command_palette(
                frame,
                &refs,
                app.menu_selector_idx,
                hover_idx,
            );
        }
        ModalActivo::SelectorTema => {
            let temas = crate::ui::theme::TemaVariant::todos();
            let labels: Vec<String> = temas
                .iter()
                .map(|t| {
                    let mark = if *t == app.tema { "●" } else { " " };
                    format!("{} {}", mark, t.nombre())
                })
                .collect();
            let refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
            let hover_idx = match app.mouse.hover {
                HoverZone::ModalItem(i) => Some(i),
                _ => None,
            };
            components::draw_select_modal(
                frame,
                "select theme",
                &refs,
                app.menu_selector_idx,
                hover_idx,
            );
        }
        ModalActivo::SelectorIdioma => {
            let idiomas = crate::app::Idioma::todos();
            let labels: Vec<String> = idiomas
                .iter()
                .map(|l| {
                    let mark = if *l == app.idioma { "●" } else { " " };
                    format!("{} {}", mark, l.nombre())
                })
                .collect();
            let refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
            let hover_idx = match app.mouse.hover {
                HoverZone::ModalItem(i) => Some(i),
                _ => None,
            };
            components::draw_select_modal(
                frame,
                "select language",
                &refs,
                app.menu_selector_idx,
                hover_idx,
            );
        }
        ModalActivo::AyudaRapida => {
            components::draw_info_modal(
                frame,
                "help",
                "navigation\n\
                 ─────────────────────────\n\
                 tab         cycle focus\n\
                 1/2/3/4     switch tabs\n\
                 h/l         history/collections\n\
                 j/k, ↑↓     navigate / scroll\n\
                 pgup/pgdn   page scroll response\n\
                 home/end    top/bottom response\n\
                 scroll      mouse wheel\n\
                 hover       mouse move\n\
                 \n\
                 editing\n\
                 ─────────────────────────\n\
                 i           insert mode\n\
                 a           append (end)\n\
                 I           insert at start\n\
                 A           append\n\
                 0/$         home/end cursor\n\
                 ← →         cursor movement\n\
                 esc         back to normal\n\
                 ctrl+z/u    undo\n\
                 y/p         yank/paste\n\
                 \n\
                 actions\n\
                 ─────────────────────────\n\
                 ctrl+p      command palette\n\
                 r           run request\n\
                 w           save collection\n\
                 o           import\n\
                 e           export\n\
                 m           method selector\n\
                 f           toggle json format\n\
                 ?           this help",
            );
        }
    }
}

// ── Style helpers ──────────────────────────────────────────────────────

fn panel_border(app: &EstadoApp, zona: ZonaFoco, hover: HoverZone) -> Style {
    let active = app.foco == zona;
    let hovered = app.mouse.hover == hover;
    let flash = app
        .focus_flash
        .as_ref()
        .map(|t| t.elapsed().as_millis() < 300)
        .unwrap_or(false);

    if active && flash {
        Style::default()
            .fg(theme::primary_color())
            .add_modifier(Modifier::BOLD)
    } else if active {
        Style::default().fg(theme::primary_color())
    } else if hovered {
        Style::default().fg(theme::text_weak_color())
    } else {
        Style::default().fg(theme::border_weak())
    }
}

fn normal_input_style(is_active: bool) -> Style {
    if is_active {
        theme::focused().bg(theme::surface_color())
    } else {
        theme::normal().bg(theme::bg_base())
    }
}
