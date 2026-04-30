pub mod theme;
pub mod splash;
pub mod components;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::{
    EstadoApp, ModalActivo, ModoEntrada, SidebarActiva, ZonaFoco, etiqueta_accion_menu,
    metodos_http,
};

pub fn render(frame: &mut Frame<'_>, app: &EstadoApp) {
    let principal = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(10), Constraint::Length(3)])
        .split(frame.area());

    render_top_menu(frame, app, principal[0]);

    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(app.ancho_sidebar), Constraint::Min(10)])
        .split(principal[1]);

    render_sidebar(frame, app, root[0]);
    render_main(frame, app, root[1]);
    render_statusbar(frame, app, principal[2]);
    render_modales(frame, app);
}

fn render_top_menu(frame: &mut Frame<'_>, app: &EstadoApp, area: ratatui::layout::Rect) {
    let menu = crate::app::MenuSuperior::todos()
        .iter()
        .map(|menu| {
            if *menu == app.menu_superior_activo {
                format!("[{}]", menu.etiqueta())
            } else {
                format!(" {} ", menu.etiqueta())
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    let tabs = [
        style_tab("1 Request", app.tab_activo == 0),
        style_tab("2 Headers", app.tab_activo == 1),
        style_tab("3 Body", app.tab_activo == 2),
        style_tab("4 Response", app.tab_activo == 3),
    ]
    .join("  ");
    let title = format!("{menu}\n{tabs}");
    let p = Paragraph::new(title).style(theme::normal()).block(
        Block::default()
            .title(" Modjo ")
            .title_style(theme::title())
            .borders(Borders::ALL)
            .border_style(theme::border(true)),
    );
    frame.render_widget(p, area);
}

fn render_sidebar(frame: &mut Frame<'_>, app: &EstadoApp, area: ratatui::layout::Rect) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(area);

    let title = match app.sidebar {
        SidebarActiva::Historial => " Historial (h) ",
        SidebarActiva::Colecciones => " Colecciones (l) ",
    };

    let tabs = Paragraph::new("h:Historial  l:Colecciones\nEnter: cargar")
        .style(theme::muted())
        .block(
            Block::default()
                .title(title)
                .title_style(theme::title())
                .borders(Borders::ALL)
                .border_style(theme::border(app.foco == ZonaFoco::Sidebar)),
        );
    frame.render_widget(tabs, sections[0]);

    let items: Vec<ListItem<'_>> = match app.sidebar {
        SidebarActiva::Historial => app
            .history
            .iter()
            .map(|entry| {
                ListItem::new(Line::from(format!(
                    "{} {} [{}]",
                    entry.method,
                    entry.url,
                    entry.status.unwrap_or(0)
                )))
            })
            .collect(),
        SidebarActiva::Colecciones => app
            .collections
            .iter()
            .map(|entry| ListItem::new(Line::from(entry.name.clone())))
            .collect(),
    };

    let list = List::new(items)
        .highlight_style(theme::sidebar_selected())
        .highlight_symbol(">> ")
        .block(
            Block::default()
                .title(" Items ")
                .title_style(theme::title())
                .borders(Borders::ALL)
                .border_style(theme::border(app.foco == ZonaFoco::Sidebar)),
        );

    let mut state = ratatui::widgets::ListState::default();
    if app.sidebar_len() > 0 {
        state.select(Some(app.sidebar_index.min(app.sidebar_len() - 1)));
    }
    frame.render_stateful_widget(list, sections[1], &mut state);
}

fn render_main(frame: &mut Frame<'_>, app: &EstadoApp, area: ratatui::layout::Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(app.alto_headers),
            Constraint::Length(app.alto_body),
            Constraint::Min(6),
        ])
        .split(area);

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(14), Constraint::Min(10)])
        .split(rows[0]);

    let method = Paragraph::new(app.request.method.as_str())
        .style(active_input_style(app.foco == ZonaFoco::Metodo))
        .block(
            Block::default()
                .title(" Metodo ")
                .title_style(theme::title())
                .borders(Borders::ALL)
                .border_style(theme::border(app.foco == ZonaFoco::Metodo)),
        );
    frame.render_widget(method, top[0]);

    let url = Paragraph::new(app.request.url.as_str())
        .style(active_input_style(app.foco == ZonaFoco::Url))
        .block(
            Block::default()
                .title(" URL ")
                .title_style(theme::title())
                .borders(Borders::ALL)
                .border_style(theme::border(app.foco == ZonaFoco::Url)),
        );
    frame.render_widget(url, top[1]);

    let header_text = app
        .request
        .headers
        .iter()
        .map(|(k, v)| format!("{k}: {v}"))
        .collect::<Vec<_>>()
        .join("\n");
    let headers = Paragraph::new(header_text)
        .style(active_input_style(app.foco == ZonaFoco::Headers))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(" Headers (edita valor del primero) ")
                .title_style(theme::title())
                .borders(Borders::ALL)
                .border_style(theme::border(app.foco == ZonaFoco::Headers)),
        );
    frame.render_widget(headers, rows[1]);

    let body = Paragraph::new(app.request.body.as_str())
        .style(active_input_style(app.foco == ZonaFoco::Body))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(" Body ")
                .title_style(theme::title())
                .borders(Borders::ALL)
                .border_style(theme::border(app.foco == ZonaFoco::Body)),
        );
    frame.render_widget(body, rows[2]);

    let response_text = if let Some(err) = &app.last_error {
        format!("Error:\n{err}")
    } else if let Some(resp) = &app.response {
        format!(
            "Status: {} {}\nTime: {} ms\nSize: {} bytes\n\n{}",
            resp.status.unwrap_or(0),
            resp.status_text,
            resp.duration_ms,
            resp.size_bytes,
            resp.body
        )
    } else {
        "r ejecuta request, w guarda, o importa, e exporta, i insertar, y/p copiar-pegar ninja."
            .to_string()
    };

    let response = Paragraph::new(response_text)
        .style(active_input_style(app.foco == ZonaFoco::Response))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(if app.loading {
                    " Respuesta (enviando...) "
                } else {
                    " Respuesta "
                })
                .title_style(theme::title())
                .borders(Borders::ALL)
                .border_style(theme::border(app.foco == ZonaFoco::Response)),
        );
    frame.render_widget(response, rows[3]);
}

fn render_statusbar(frame: &mut Frame<'_>, app: &EstadoApp, area: ratatui::layout::Rect) {
    let modo = match app.modo {
        ModoEntrada::Normal => "NORMAL",
        ModoEntrada::Insertar => "INSERTAR",
    };
    let texto = format!(
        " {} | r:run w:save c:collection o:import e:export m:method-select ?:help a:ai i:insert Esc:normal Tab:cambiar Ctrl+h/j/k/l:resize y/p:copiar-pegar || {}",
        modo, app.mensaje_estado
    );
    let status = Paragraph::new(texto).style(theme::focused()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border(true)),
    );
    frame.render_widget(status, area);
}

fn render_modales(frame: &mut Frame<'_>, app: &EstadoApp) {
    match app.modal_activo {
        ModalActivo::Ninguno => {}
        ModalActivo::SelectorMetodo => {
            components::draw_select_modal(
                frame,
                "Selector de Método HTTP (j/k o flechas, Enter confirma, Esc cierra)",
                metodos_http(),
                app.metodo_idx_selector,
            );
        }
        ModalActivo::SelectorMenuSuperior => {
            let opciones = app
                .menu_superior_activo
                .opciones()
                .iter()
                .map(|accion| etiqueta_accion_menu(*accion))
                .collect::<Vec<_>>();
            components::draw_select_modal(
                frame,
                &format!(
                    "Menu {} (h/l cambia menu, j/k navega, Enter confirma)",
                    app.menu_superior_activo.etiqueta()
                ),
                &opciones,
                app.menu_selector_idx,
            );
        }
        ModalActivo::AyudaRapida => {
            components::draw_info_modal(
                frame,
                "Ayuda Rápida",
                "Navegacion:\n- Tab cambia foco\n- 1/2/3/4 cambia tabs\n- h/l cambia historial-colecciones\n\nEdicion:\n- i entra en insertar\n- Esc vuelve a normal\n- y/p copia y pega interno\n\nAcciones:\n- r ejecutar request\n- w guardar\n- c guardar coleccion\n- o importar\n- e exportar\n- m selector de metodo\n\nPaneles:\n- Ctrl+h/l ancho sidebar\n- Ctrl+j/k alto headers-body\n\nMouse:\n- Click en tabs superiores\n- Click en paneles para foco\n\nPresiona Esc para cerrar este modal.",
            );
        }
    }
}

fn style_tab(nombre: &str, activo: bool) -> String {
    if activo {
        format!("[{nombre}]")
    } else {
        format!(" {nombre} ")
    }
}

fn active_input_style(is_active: bool) -> Style {
    if is_active {
        theme::focused()
            .bg(theme::PANEL_BG)
            .add_modifier(Modifier::REVERSED)
    } else {
        theme::normal().bg(theme::PANEL_BG)
    }
}
