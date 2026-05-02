use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind};

use crate::app::{
    AccionMenu, DragState, EstadoApp, HoverZone, ModalActivo, ModoEntrada, SidebarActiva, TipoDrag,
    ZonaFoco,
};

pub enum AccionApp {
    Salir,
    EnviarRequest,
    GuardarColeccion,
    GuardarTodo,
    Exportar,
    Importar,
    EjecutarAyudaIa,
    EjecutarAccionMenu(AccionMenu),
}

pub fn manejar_evento(app: &mut EstadoApp, event: Event) -> Option<AccionApp> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            if app.modal_activo != ModalActivo::Ninguno {
                return manejar_tecla_modal(app, key.code, key.modifiers);
            }
            if app.modo == ModoEntrada::Insertar {
                return manejar_tecla_insertar(app, key.code, key.modifiers);
            }
            manejar_tecla_normal(app, key.code, key.modifiers)
        }
        Event::Mouse(mouse) => manejar_mouse(app, mouse),
        _ => None,
    }
}

// ── Mouse handler completo ──────────────────────────────────────────────

fn manejar_mouse(
    app: &mut EstadoApp,
    mouse: crossterm::event::MouseEvent,
) -> Option<AccionApp> {
    match mouse.kind {
        // ── Movimiento del mouse (hover tracking) ──
        MouseEventKind::Moved => {
            app.actualizar_hover(mouse.row, mouse.column, app.terminal_height);
            None
        }

        // ── Click izquierdo ──
        MouseEventKind::Down(MouseButton::Left) => {
            // Si hay drag activo, lo cancelamos
            app.drag_state = None;

            // Modal abierto — usar hover zone para detectar item
            if app.modal_activo == ModalActivo::SelectorMetodo {
                if let HoverZone::ModalItem(idx) = app.mouse.hover {
                    let max = crate::app::metodos_http().len();
                    if idx < max {
                        app.metodo_idx_selector = idx;
                        app.confirmar_selector_metodo();
                    }
                }
                return None;
            }
            if app.modal_activo == ModalActivo::SelectorMenuSuperior {
                if let HoverZone::ModalItem(idx) = app.mouse.hover {
                    let max = app.menu_superior_activo.opciones().len();
                    if idx < max {
                        app.menu_selector_idx = idx;
                        if let Some(accion) = app.aplicar_opcion_menu_actual() {
                            return Some(AccionApp::EjecutarAccionMenu(accion));
                        }
                    }
                }
                return None;
            }

            // Click en zona de resize
            if app.mouse.hover == HoverZone::ResizeSidebar {
                app.drag_state = Some(DragState {
                    tipo: TipoDrag::SidebarAncho,
                    inicio_x: mouse.column,
                    inicio_y: mouse.row,
                    valor_inicial: app.ancho_sidebar,
                });
                return None;
            }
            if app.mouse.hover == HoverZone::ResizeHeaders {
                app.drag_state = Some(DragState {
                    tipo: TipoDrag::AltoHeaders,
                    inicio_x: mouse.column,
                    inicio_y: mouse.row,
                    valor_inicial: app.alto_headers,
                });
                return None;
            }
            if app.mouse.hover == HoverZone::ResizeBody {
                app.drag_state = Some(DragState {
                    tipo: TipoDrag::AltoBody,
                    inicio_x: mouse.column,
                    inicio_y: mouse.row,
                    valor_inicial: app.alto_body,
                });
                return None;
            }

            // Click en sidebar tabs
            if app.mouse.hover == HoverZone::SidebarTabHist {
                app.sidebar = SidebarActiva::Historial;
                app.sidebar_index = 0;
                app.foco = ZonaFoco::Sidebar;
                app.focus_flash = Some(std::time::Instant::now());
                return None;
            }
            if app.mouse.hover == HoverZone::SidebarTabCol {
                app.sidebar = SidebarActiva::Colecciones;
                app.sidebar_index = 0;
                app.foco = ZonaFoco::Sidebar;
                app.focus_flash = Some(std::time::Instant::now());
                return None;
            }

            // Click en sidebar item — select and load
            if let HoverZone::SidebarItem(idx) = app.mouse.hover {
                app.foco = ZonaFoco::Sidebar;
                app.focus_flash = Some(std::time::Instant::now());
                if idx < app.sidebar_len() {
                    app.sidebar_index = idx;
                    app.activar_item_sidebar();
                    app.sync_cursor_to_end();
                }
                return None;
            }

            // Click en paneles principales
            match app.mouse.hover {
                HoverZone::BtnMetodo => {
                    app.foco = ZonaFoco::Metodo;
                    app.focus_flash = Some(std::time::Instant::now());
                    app.sync_cursor_to_end();
                }
                HoverZone::BtnUrl => {
                    app.foco = ZonaFoco::Url;
                    app.focus_flash = Some(std::time::Instant::now());
                    app.sync_cursor_to_end();
                }
                HoverZone::BtnHeaders => {
                    app.foco = ZonaFoco::Headers;
                    app.focus_flash = Some(std::time::Instant::now());
                    app.sync_cursor_to_end();
                }
                HoverZone::BtnBody => {
                    app.foco = ZonaFoco::Body;
                    app.focus_flash = Some(std::time::Instant::now());
                    app.sync_cursor_to_end();
                }
                HoverZone::BtnResponse => {
                    app.foco = ZonaFoco::Response;
                    app.focus_flash = Some(std::time::Instant::now());
                }
                _ => {}
            }
            None
        }

        // ── Drag (mover resize) ──
        MouseEventKind::Drag(MouseButton::Left) => {
            if let Some(ref drag) = app.drag_state {
                match drag.tipo {
                    TipoDrag::SidebarAncho => {
                        let nuevo = drag.valor_inicial as i16
                            + (mouse.column as i16 - drag.inicio_x as i16);
                        app.ancho_sidebar = nuevo.clamp(22, 55) as u16;
                    }
                    TipoDrag::AltoHeaders => {
                        let delta = mouse.row as i16 - drag.inicio_y as i16;
                        let nuevo_h = (drag.valor_inicial as i16 + delta).clamp(5, 16) as u16;
                        let diff = nuevo_h as i16 - app.alto_headers as i16;
                        app.alto_headers = nuevo_h;
                        app.alto_body = (app.alto_body as i16 - diff).clamp(5, 16) as u16;
                    }
                    TipoDrag::AltoBody => {
                        let delta = mouse.row as i16 - drag.inicio_y as i16;
                        let nuevo_b = (drag.valor_inicial as i16 + delta).clamp(5, 16) as u16;
                        let diff = nuevo_b as i16 - app.alto_body as i16;
                        app.alto_body = nuevo_b;
                        app.alto_headers = (app.alto_headers as i16 - diff).clamp(5, 16) as u16;
                    }
                }
            }
            None
        }

        // ── Soltar ──
        MouseEventKind::Up(MouseButton::Left) => {
            app.drag_state = None;
            None
        }

        // ── Scroll ──
        MouseEventKind::ScrollUp => {
            if app.foco == ZonaFoco::Response {
                app.response_scroll_up(3);
            } else if app.foco == ZonaFoco::Sidebar || app.foco == ZonaFoco::MenuSuperior {
                app.prev_sidebar_item();
            } else {
                app.tab_activo = app.tab_activo.saturating_sub(1);
                app.foco_desde_tab();
            }
            None
        }
        MouseEventKind::ScrollDown => {
            if app.foco == ZonaFoco::Response {
                let max = app.response_body_lines().saturating_sub(5);
                app.response_scroll_down(3, max);
            } else if app.foco == ZonaFoco::Sidebar || app.foco == ZonaFoco::MenuSuperior {
                app.next_sidebar_item();
            } else {
                app.tab_activo = (app.tab_activo + 1).min(3);
                app.foco_desde_tab();
            }
            None
        }

        _ => None,
    }
}

// ── Teclado modo normal ────────────────────────────────────────────────

fn manejar_tecla_normal(
    app: &mut EstadoApp,
    code: KeyCode,
    modifiers: KeyModifiers,
) -> Option<AccionApp> {
    match (code, modifiers) {
        (KeyCode::Char('q'), _) => Some(AccionApp::Salir),
        (KeyCode::Char('1'), _) => {
            app.tab_activo = 0;
            app.foco_desde_tab();
            app.sync_cursor_to_end();
            app.mostrar_toast("tab: request");
            None
        }
        (KeyCode::Char('2'), _) => {
            app.tab_activo = 1;
            app.foco_desde_tab();
            app.sync_cursor_to_end();
            app.mostrar_toast("tab: headers");
            None
        }
        (KeyCode::Char('3'), _) => {
            app.tab_activo = 2;
            app.foco_desde_tab();
            app.sync_cursor_to_end();
            app.mostrar_toast("tab: body");
            None
        }
        (KeyCode::Char('4'), _) => {
            app.tab_activo = 3;
            app.foco_desde_tab();
            app.mostrar_toast("tab: response");
            None
        }
        (KeyCode::Char('i'), _) => {
            app.modo = ModoEntrada::Insertar;
            app.sync_cursor_to_end();
            app.mostrar_toast("insert mode");
            None
        }
        (KeyCode::Char('a'), _) if modifiers.is_empty() => {
            app.modo = ModoEntrada::Insertar;
            if let Some(texto) = app.texto_foco_mut() {
                app.cursor.posicion = texto.len();
            }
            app.mostrar_toast("insert at end");
            None
        }
        (KeyCode::Char('I'), _) => {
            app.modo = ModoEntrada::Insertar;
            app.cursor.posicion = 0;
            app.mostrar_toast("insert at start");
            None
        }
        (KeyCode::Char('A'), _) => {
            app.modo = ModoEntrada::Insertar;
            app.cursor_end();
            app.mostrar_toast("append");
            None
        }
        (KeyCode::Char('0'), _) => {
            app.cursor_home();
            None
        }
        (KeyCode::Char('$'), _) => {
            app.cursor_end();
            None
        }
        (KeyCode::Char('m'), _) => {
            app.abrir_selector_metodo();
            None
        }
        (KeyCode::Char('?'), _) => {
            app.abrir_ayuda_rapida();
            None
        }
        (KeyCode::Tab, _) => {
            app.foco = app.foco.siguiente();
            app.sync_cursor_to_end();
            app.focus_flash = Some(std::time::Instant::now());
            app.mostrar_toast(format!("{:?}", app.foco));
            None
        }
        (KeyCode::BackTab, _) => {
            app.tab_activo = if app.tab_activo == 0 { 3 } else { app.tab_activo - 1 };
            app.foco_desde_tab();
            app.sync_cursor_to_end();
            None
        }
        (KeyCode::Char('h'), _) if modifiers.is_empty() => {
            app.sidebar = SidebarActiva::Historial;
            app.foco = ZonaFoco::Sidebar;
            app.sidebar_index = 0;
            app.focus_flash = Some(std::time::Instant::now());
            None
        }
        (KeyCode::Char('l'), _) if modifiers.is_empty() => {
            app.sidebar = SidebarActiva::Colecciones;
            app.foco = ZonaFoco::Sidebar;
            app.sidebar_index = 0;
            app.focus_flash = Some(std::time::Instant::now());
            None
        }
        (KeyCode::Up | KeyCode::Char('k'), _) => {
            match app.foco {
                ZonaFoco::Sidebar => app.prev_sidebar_item(),
                ZonaFoco::MenuSuperior => app.mover_menu_superior(-1),
                ZonaFoco::Response => app.response_scroll_up(1),
                _ => {
                    app.tab_activo = app.tab_activo.saturating_sub(1);
                    app.foco_desde_tab();
                }
            }
            None
        }
        (KeyCode::Down | KeyCode::Char('j'), _) => {
            match app.foco {
                ZonaFoco::Sidebar => app.next_sidebar_item(),
                ZonaFoco::MenuSuperior => app.mover_menu_superior(1),
                ZonaFoco::Response => {
                    let max = app.response_body_lines().saturating_sub(5);
                    app.response_scroll_down(1, max);
                }
                _ => {
                    app.tab_activo = (app.tab_activo + 1).min(3);
                    app.foco_desde_tab();
                }
            }
            None
        }
        (KeyCode::Left, _) => {
            if app.foco == ZonaFoco::MenuSuperior {
                app.mover_menu_superior(-1);
            } else {
                app.tab_activo = app.tab_activo.saturating_sub(1);
                app.foco_desde_tab();
                app.sync_cursor_to_end();
            }
            None
        }
        (KeyCode::Right, _) => {
            if app.foco == ZonaFoco::MenuSuperior {
                app.mover_menu_superior(1);
            } else {
                app.tab_activo = (app.tab_activo + 1).min(3);
                app.foco_desde_tab();
                app.sync_cursor_to_end();
            }
            None
        }
        (KeyCode::Enter, _) => {
            match app.foco {
                ZonaFoco::Sidebar => {
                    app.activar_item_sidebar();
                    app.sync_cursor_to_end();
                }
                ZonaFoco::MenuSuperior => app.abrir_selector_menu_superior(),
                _ => {}
            }
            None
        }
        (KeyCode::Char('r'), _) => Some(AccionApp::EnviarRequest),
        (KeyCode::Char('w'), _) => Some(AccionApp::GuardarTodo),
        (KeyCode::Char('c'), _) => Some(AccionApp::GuardarColeccion),
        (KeyCode::Char('o'), _) => Some(AccionApp::Importar),
        (KeyCode::Char('e'), _) => Some(AccionApp::Exportar),
        (KeyCode::Char('a'), KeyModifiers::CONTROL) => Some(AccionApp::EjecutarAyudaIa),
        (KeyCode::Char('T'), _) => {
            app.abrir_selector_tema();
            None
        }
        (KeyCode::Char('L'), KeyModifiers::CONTROL) => {
            app.redimensionar_paneles(2, 0, 0);
            None
        }
        (KeyCode::Char('L'), _) => {
            app.abrir_selector_idioma();
            None
        }
        (KeyCode::Char('y'), _) => {
            let texto = texto_activo(app);
            app.portapapeles_interno = texto.clone();
            app.animar_clipboard(&texto);
            app.mostrar_toast("yanked!");
            None
        }
        (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
            app.abrir_command_palette();
            None
        }
        (KeyCode::Char('p'), _) => {
            let clip = app.portapapeles_interno.clone();
            if !clip.is_empty() {
                app.push_undo();
                pegar_texto_activo(app, &clip);
                app.animar_clipboard(&clip);
                app.mostrar_toast("pasted!");
            }
            None
        }
        (KeyCode::Char('u'), _) => {
            app.undo();
            None
        }
        (KeyCode::Char('H'), KeyModifiers::CONTROL) => {
            app.redimensionar_paneles(-2, 0, 0);
            None
        }
        (KeyCode::Char('K'), KeyModifiers::CONTROL) => {
            app.redimensionar_paneles(0, 1, -1);
            None
        }
        (KeyCode::Char('J'), KeyModifiers::CONTROL) => {
            app.redimensionar_paneles(0, -1, 1);
            None
        }
        (KeyCode::Char('s'), KeyModifiers::CONTROL) => Some(AccionApp::GuardarTodo),
        (KeyCode::Char('z'), KeyModifiers::CONTROL) => {
            app.undo();
            None
        }
        (KeyCode::Char('f'), _) => {
            app.toggle_response_format();
            None
        }
        (KeyCode::PageUp, _) => {
            if app.foco == ZonaFoco::Response {
                app.response_page_up(10);
            } else {
                app.tab_activo = app.tab_activo.saturating_sub(1);
                app.foco_desde_tab();
            }
            None
        }
        (KeyCode::PageDown, _) => {
            if app.foco == ZonaFoco::Response {
                let max = app.response_body_lines().saturating_sub(5);
                app.response_page_down(10, max);
            } else {
                app.tab_activo = (app.tab_activo + 1).min(3);
                app.foco_desde_tab();
            }
            None
        }
        (KeyCode::Home, _) if app.foco == ZonaFoco::Response => {
            app.response_scroll = 0;
            None
        }
        (KeyCode::End, _) if app.foco == ZonaFoco::Response => {
            let max = app.response_body_lines().saturating_sub(5);
            app.response_scroll = max;
            None
        }
        _ => None,
    }
}

// ── Teclado modo insertar ──────────────────────────────────────────────

fn manejar_tecla_insertar(
    app: &mut EstadoApp,
    code: KeyCode,
    modifiers: KeyModifiers,
) -> Option<AccionApp> {
    match (code, modifiers) {
        (KeyCode::Esc, _) => {
            app.modo = ModoEntrada::Normal;
            app.mostrar_toast("normal mode");
            None
        }
        (KeyCode::Left, _) => {
            app.cursor_left();
            None
        }
        (KeyCode::Right, _) => {
            app.cursor_right();
            None
        }
        (KeyCode::Up, _) => {
            app.cursor_up();
            None
        }
        (KeyCode::Down, _) => {
            app.cursor_down();
            None
        }
        (KeyCode::Home, _) => {
            app.cursor_line_start();
            None
        }
        (KeyCode::End, _) => {
            app.cursor_line_end();
            None
        }
        (KeyCode::Backspace, _) => {
            app.push_undo();
            app.borrar_char();
            None
        }
        (KeyCode::Delete, _) => {
            app.push_undo();
            app.borrar_adelante();
            None
        }
        (KeyCode::Enter, _) => {
            app.push_undo();
            app.insertar_char('\n');
            None
        }
        (KeyCode::Tab, _) => {
            app.modo = ModoEntrada::Normal;
            app.foco = app.foco.siguiente();
            app.sync_cursor_to_end();
            app.focus_flash = Some(std::time::Instant::now());
            app.mostrar_toast(format!("{:?}", app.foco));
            None
        }
        (KeyCode::Char('z'), KeyModifiers::CONTROL) => {
            app.undo();
            None
        }
        (KeyCode::Char(ch), _) => {
            app.push_undo();
            app.insertar_char_con_autocomplete(ch);
            None
        }
        _ => None,
    }
}

// ── Teclado modal ──────────────────────────────────────────────────────

fn manejar_tecla_modal(
    app: &mut EstadoApp,
    code: KeyCode,
    _modifiers: KeyModifiers,
) -> Option<AccionApp> {
    match app.modal_activo {
        ModalActivo::Ninguno => None,
        ModalActivo::AyudaRapida => {
            if matches!(code, KeyCode::Esc | KeyCode::Enter | KeyCode::Char('?')) {
                app.cerrar_modal();
            }
            None
        }
        ModalActivo::SelectorMetodo => match code {
            KeyCode::Esc => {
                app.cerrar_modal();
                None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.mover_selector_metodo(-1);
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.mover_selector_metodo(1);
                None
            }
            KeyCode::Enter => {
                app.confirmar_selector_metodo();
                None
            }
            _ => None,
        },
        ModalActivo::SelectorMenuSuperior => match code {
            KeyCode::Esc => {
                app.cerrar_modal();
                None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.mover_selector_menu(-1);
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.mover_selector_menu(1);
                None
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.mover_menu_superior(-1);
                app.menu_selector_idx = 0;
                None
            }
            KeyCode::Right | KeyCode::Char('l') => {
                app.mover_menu_superior(1);
                app.menu_selector_idx = 0;
                None
            }
            KeyCode::Enter => app
                .aplicar_opcion_menu_actual()
                .map(AccionApp::EjecutarAccionMenu),
            _ => None,
        },
        ModalActivo::CommandPalette => match code {
            KeyCode::Esc => {
                app.cerrar_modal();
                None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let total = crate::app::comandos_disponibles().len();
                if total > 0 {
                    app.menu_selector_idx = if app.menu_selector_idx == 0 {
                        total - 1
                    } else {
                        app.menu_selector_idx - 1
                    };
                }
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let total = crate::app::comandos_disponibles().len();
                if total > 0 {
                    app.menu_selector_idx = (app.menu_selector_idx + 1) % total;
                }
                None
            }
            KeyCode::Enter => {
                let comandos = crate::app::comandos_disponibles();
                if let Some(cmd) = comandos.get(app.menu_selector_idx) {
                    let accion = cmd.accion;
                    app.cerrar_modal();
                    return Some(AccionApp::EjecutarAccionMenu(accion));
                }
                None
            }
            _ => None,
        },
        ModalActivo::SelectorTema => match code {
            KeyCode::Esc => {
                app.cerrar_modal();
                None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let total = crate::ui::theme::TemaVariant::todos().len();
                if total > 0 {
                    app.menu_selector_idx = if app.menu_selector_idx == 0 {
                        total - 1
                    } else {
                        app.menu_selector_idx - 1
                    };
                }
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let total = crate::ui::theme::TemaVariant::todos().len();
                if total > 0 {
                    app.menu_selector_idx = (app.menu_selector_idx + 1) % total;
                }
                None
            }
            KeyCode::Enter => {
                let temas = crate::ui::theme::TemaVariant::todos();
                if let Some(tema) = temas.get(app.menu_selector_idx) {
                    app.tema = *tema;
                    crate::ui::theme::set_active_theme(*tema);
                    let _ = crate::storage::save_all(app);
                    app.mostrar_toast(format!("theme: {}", tema.nombre()));
                }
                app.cerrar_modal();
                None
            }
            _ => None,
        },
        ModalActivo::SelectorIdioma => match code {
            KeyCode::Esc => {
                app.cerrar_modal();
                None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let total = crate::app::Idioma::todos().len();
                if total > 0 {
                    app.menu_selector_idx = if app.menu_selector_idx == 0 {
                        total - 1
                    } else {
                        app.menu_selector_idx - 1
                    };
                }
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let total = crate::app::Idioma::todos().len();
                if total > 0 {
                    app.menu_selector_idx = (app.menu_selector_idx + 1) % total;
                }
                None
            }
            KeyCode::Enter => {
                let idiomas = crate::app::Idioma::todos();
                if let Some(idioma) = idiomas.get(app.menu_selector_idx) {
                    app.idioma = *idioma;
                    let _ = crate::storage::save_all(app);
                    app.mostrar_toast(format!("language: {}", idioma.nombre()));
                }
                app.cerrar_modal();
                None
            }
            _ => None,
        },
    }
}

fn texto_activo(app: &EstadoApp) -> String {
    match app.foco {
        ZonaFoco::Metodo => app.request.method.clone(),
        ZonaFoco::Url => app.request.url.clone(),
        ZonaFoco::Body => app.request.body.clone(),
        ZonaFoco::Headers => app
            .request
            .headers
            .first()
            .map(|(_, v)| v.clone())
            .unwrap_or_default(),
        ZonaFoco::Response => app
            .response
            .as_ref()
            .map(|r| r.body.clone())
            .unwrap_or_default(),
        ZonaFoco::MenuSuperior => String::new(),
        ZonaFoco::Sidebar => String::new(),
    }
}

fn pegar_texto_activo(app: &mut EstadoApp, payload: &str) {
    match app.foco {
        ZonaFoco::Metodo => {
            let pos = app.cursor.posicion.min(app.request.method.len());
            app.request.method.insert_str(pos, payload);
            app.cursor.posicion = pos + payload.len();
        }
        ZonaFoco::Url => {
            let pos = app.cursor.posicion.min(app.request.url.len());
            app.request.url.insert_str(pos, payload);
            app.cursor.posicion = pos + payload.len();
        }
        ZonaFoco::Body => {
            let pos = app.cursor.posicion.min(app.request.body.len());
            app.request.body.insert_str(pos, payload);
            app.cursor.posicion = pos + payload.len();
        }
        ZonaFoco::Headers => {
            if app.request.headers.is_empty() {
                app.request.headers.push((String::new(), String::new()));
            }
            if let Some((_, v)) = app.request.headers.first_mut() {
                let pos = app.cursor.posicion.min(v.len());
                v.insert_str(pos, payload);
                app.cursor.posicion = pos + payload.len();
            }
        }
        _ => {}
    }
}
