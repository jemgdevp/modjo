use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind};

use crate::app::{AccionMenu, EstadoApp, ModalActivo, ModoEntrada, SidebarActiva, ZonaFoco};

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
                return manejar_tecla_modal(app, key.code);
            }
            if app.modo == ModoEntrada::Insertar {
                return manejar_tecla_insertar(app, key.code);
            }
            manejar_tecla_normal(app, key.code, key.modifiers)
        }
        Event::Mouse(mouse) => {
            if matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
                if app.modal_activo == ModalActivo::SelectorMetodo {
                    if mouse.row > 8 {
                        let idx = (mouse.row - 9) as usize;
                        let max = crate::app::metodos_http().len();
                        if idx < max {
                            app.metodo_idx_selector = idx;
                            app.confirmar_selector_metodo();
                        }
                    }
                    return None;
                }
                if app.modal_activo == ModalActivo::SelectorMenuSuperior {
                    if mouse.row > 8 {
                        let idx = (mouse.row - 9) as usize;
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
                if mouse.row <= 2 {
                    if mouse.row == 1 {
                        if let Some(menu) = menu_desde_columna(mouse.column) {
                            app.menu_superior_activo = menu;
                            app.foco = ZonaFoco::MenuSuperior;
                            app.abrir_selector_menu_superior();
                            return None;
                        }
                    } else if mouse.row == 2 {
                        if mouse.column > 6 && mouse.column < 16 {
                            app.tab_activo = 0;
                        } else if mouse.column >= 16 && mouse.column < 28 {
                            app.tab_activo = 1;
                        } else if mouse.column >= 28 && mouse.column < 38 {
                            app.tab_activo = 2;
                        } else if mouse.column >= 38 && mouse.column < 53 {
                            app.tab_activo = 3;
                        }
                        app.foco_desde_tab();
                        app.mensaje_estado = "Tab cambiado con mouse.".to_string();
                    }
                } else if mouse.column < app.ancho_sidebar {
                    app.foco = ZonaFoco::Sidebar;
                } else if mouse.row < 8 {
                    app.foco = if mouse.column < app.ancho_sidebar + 14 {
                        ZonaFoco::Metodo
                    } else {
                        ZonaFoco::Url
                    };
                } else if mouse.row < 8 + app.alto_headers {
                    app.foco = ZonaFoco::Headers;
                } else if mouse.row < 8 + app.alto_headers + app.alto_body {
                    app.foco = ZonaFoco::Body;
                } else {
                    app.foco = ZonaFoco::Response;
                }
            }
            None
        }
        _ => None,
    }
}

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
            app.mensaje_estado = "Tab Request activo.".to_string();
            None
        }
        (KeyCode::Char('2'), _) => {
            app.tab_activo = 1;
            app.foco_desde_tab();
            app.mensaje_estado = "Tab Headers activo.".to_string();
            None
        }
        (KeyCode::Char('3'), _) => {
            app.tab_activo = 2;
            app.foco_desde_tab();
            app.mensaje_estado = "Tab Body activo.".to_string();
            None
        }
        (KeyCode::Char('4'), _) => {
            app.tab_activo = 3;
            app.foco_desde_tab();
            app.mensaje_estado = "Tab Response activo.".to_string();
            None
        }
        (KeyCode::Char('i'), _) => {
            app.modo = ModoEntrada::Insertar;
            app.mensaje_estado = "Modo INSERTAR activo. Escribe tranquilo.".to_string();
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
            app.mensaje_estado = format!("Cambio de ventana: {:?}", app.foco);
            None
        }
        (KeyCode::BackTab, _) => {
            app.tab_activo = if app.tab_activo == 0 { 3 } else { app.tab_activo - 1 };
            app.foco_desde_tab();
            None
        }
        (KeyCode::Char('h'), _) => {
            app.sidebar = SidebarActiva::Historial;
            app.foco = ZonaFoco::Sidebar;
            app.sidebar_index = 0;
            None
        }
        (KeyCode::Char('l'), _) => {
            app.sidebar = SidebarActiva::Colecciones;
            app.foco = ZonaFoco::Sidebar;
            app.sidebar_index = 0;
            None
        }
        (KeyCode::Up, _) => {
            match app.foco {
                ZonaFoco::Sidebar => app.prev_sidebar_item(),
                ZonaFoco::MenuSuperior => app.mover_menu_superior(-1),
                _ => app.tab_activo = app.tab_activo.saturating_sub(1),
            }
            None
        }
        (KeyCode::Down, _) => {
            match app.foco {
                ZonaFoco::Sidebar => app.next_sidebar_item(),
                ZonaFoco::MenuSuperior => app.mover_menu_superior(1),
                _ => app.tab_activo = (app.tab_activo + 1).min(3),
            }
            app.foco_desde_tab();
            None
        }
        (KeyCode::Left, _) => {
            if app.foco == ZonaFoco::MenuSuperior {
                app.mover_menu_superior(-1);
            } else {
                app.tab_activo = app.tab_activo.saturating_sub(1);
                app.foco_desde_tab();
            }
            None
        }
        (KeyCode::Right, _) => {
            if app.foco == ZonaFoco::MenuSuperior {
                app.mover_menu_superior(1);
            } else {
                app.tab_activo = (app.tab_activo + 1).min(3);
                app.foco_desde_tab();
            }
            None
        }
        (KeyCode::Enter, _) => {
            match app.foco {
                ZonaFoco::Sidebar => app.activar_item_sidebar(),
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
        (KeyCode::Char('a'), _) => Some(AccionApp::EjecutarAyudaIa),
        (KeyCode::Char('y'), _) => {
            app.portapapeles_interno = texto_activo(app);
            app.mensaje_estado = "Copiado ninja (yank) al portapapeles interno.".to_string();
            None
        }
        (KeyCode::Char('p'), _) => {
            let clip = app.portapapeles_interno.clone();
            if !clip.is_empty() {
                pegar_texto_activo(app, &clip);
                app.mensaje_estado = "Pegado ninja listo.".to_string();
            }
            None
        }
        (KeyCode::Char('H'), KeyModifiers::CONTROL) => {
            app.redimensionar_paneles(-2, 0, 0);
            None
        }
        (KeyCode::Char('L'), KeyModifiers::CONTROL) => {
            app.redimensionar_paneles(2, 0, 0);
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
        _ => None,
    }
}

fn manejar_tecla_insertar(app: &mut EstadoApp, code: KeyCode) -> Option<AccionApp> {
    match code {
        KeyCode::Esc => {
            app.modo = ModoEntrada::Normal;
            app.mensaje_estado = "Modo NORMAL. Usa r para ejecutar.".to_string();
            None
        }
        KeyCode::Backspace => {
            editar_texto_activo(app, Mutacion::Borrar);
            None
        }
        KeyCode::Enter => {
            if app.foco == ZonaFoco::Body {
                editar_texto_activo(app, Mutacion::Agregar('\n'));
            }
            None
        }
        KeyCode::Char(ch) => {
            editar_texto_activo(app, Mutacion::Agregar(ch));
            None
        }
        _ => None,
    }
}

fn manejar_tecla_modal(app: &mut EstadoApp, code: KeyCode) -> Option<AccionApp> {
    match app.modal_activo {
        ModalActivo::Ninguno => None,
        ModalActivo::AyudaRapida => {
            if matches!(code, KeyCode::Esc | KeyCode::Enter) {
                app.cerrar_modal();
                app.mensaje_estado = "Modal de ayuda cerrado.".to_string();
            }
            None
        }
        ModalActivo::SelectorMetodo => match code {
            KeyCode::Esc => {
                app.cerrar_modal();
                app.mensaje_estado = "Selector de método cancelado.".to_string();
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
    }
}

fn menu_desde_columna(columna: u16) -> Option<crate::app::MenuSuperior> {
    let tabla = [
        (1, 6, crate::app::MenuSuperior::Help),
        (8, 13, crate::app::MenuSuperior::File),
        (15, 20, crate::app::MenuSuperior::Edit),
        (22, 32, crate::app::MenuSuperior::Selection),
        (34, 39, crate::app::MenuSuperior::View),
        (41, 44, crate::app::MenuSuperior::Go),
        (46, 50, crate::app::MenuSuperior::Run),
        (52, 57, crate::app::MenuSuperior::Todo),
        (59, 62, crate::app::MenuSuperior::Ai),
    ];
    tabla
        .iter()
        .find(|(inicio, fin, _)| columna >= *inicio && columna <= *fin)
        .map(|(_, _, menu)| *menu)
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
        ZonaFoco::Metodo => app.request.method.push_str(payload),
        ZonaFoco::Url => app.request.url.push_str(payload),
        ZonaFoco::Body => app.request.body.push_str(payload),
        ZonaFoco::Headers => {
            if app.request.headers.is_empty() {
                app.request.headers.push((String::new(), String::new()));
            }
            app.request.headers[0].1.push_str(payload);
        }
        _ => {}
    }
}

enum Mutacion {
    Agregar(char),
    Borrar,
}

fn editar_texto_activo(app: &mut EstadoApp, mutacion: Mutacion) {
    match app.foco {
        ZonaFoco::Metodo => aplicar_mutacion(&mut app.request.method, mutacion),
        ZonaFoco::Url => aplicar_mutacion(&mut app.request.url, mutacion),
        ZonaFoco::Body => aplicar_mutacion(&mut app.request.body, mutacion),
        ZonaFoco::Headers => {
            if app.request.headers.is_empty() {
                app.request.headers.push((String::new(), String::new()));
            }
            aplicar_mutacion(&mut app.request.headers[0].1, mutacion);
        }
        _ => {}
    }
}

fn aplicar_mutacion(target: &mut String, mutacion: Mutacion) {
    match mutacion {
        Mutacion::Agregar(ch) => target.push(ch),
        Mutacion::Borrar => {
            target.pop();
        }
    }
}
