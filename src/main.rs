mod app;
mod http;
mod input;
mod storage;
mod ui;

// Punto de entrada de Modjo.
//
// Responsabilidades de este módulo:
// - Inicializar y restaurar la terminal.
// - Orquestar el ciclo principal (eventos, render y acciones).
// - Despachar requests HTTP asíncronos sin bloquear la interfaz.

use std::{io, path::PathBuf, time::Duration};

use app::{AccionMenu, EstadoApp, PendingRequest, ResponseData};
use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut estado_app = EstadoApp::cargar_o_predeterminado()?;
    let (canal_respuesta_tx, mut canal_respuesta_rx) = mpsc::unbounded_channel::<ResponseData>();

    // Entramos en modo alterno para que la UI tome control completo de la terminal.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    ui::splash::mostrar_splash_carga(&mut terminal);

    let resultado_ejecucion = ejecutar_bucle_principal(
        &mut terminal,
        &mut estado_app,
        canal_respuesta_tx,
        &mut canal_respuesta_rx,
    )
    .await;

    // Restauración segura de la terminal al salir, incluso si hubo error en el loop.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    storage::save_all(&estado_app)?;
    resultado_ejecucion
}

/// Ciclo principal de la aplicación.
///
/// Procesa en cada iteración:
/// 1) Respuestas async pendientes.
/// 2) Render completo del frame.
/// 3) Eventos de teclado/mouse y acciones asociadas.
async fn ejecutar_bucle_principal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    estado_app: &mut EstadoApp,
    canal_respuesta_tx: mpsc::UnboundedSender<ResponseData>,
    canal_respuesta_rx: &mut mpsc::UnboundedReceiver<ResponseData>,
) -> Result<()> {
    loop {
        while let Ok(respuesta) = canal_respuesta_rx.try_recv() {
            estado_app.aplicar_respuesta(respuesta);
        }

        terminal.draw(|frame| ui::render(frame, estado_app))?;

        if event::poll(Duration::from_millis(50))? {
            let evento = event::read()?;
            if let Some(accion) = input::manejar_evento(estado_app, evento) {
                match accion {
                    input::AccionApp::Salir => break,
                    input::AccionApp::EnviarRequest => {
                        if !estado_app.loading {
                            let request_pendiente = PendingRequest::from(&*estado_app);
                            estado_app.loading = true;
                            estado_app.last_error = None;
                            estado_app.mensaje_estado =
                                "Enviando request... aguanta un toque.".to_string();
                            let respuesta_tx = canal_respuesta_tx.clone();
                            tokio::spawn(async move {
                                let respuesta = http::client::send_request(request_pendiente).await;
                                let _ = respuesta_tx.send(respuesta);
                            });
                        }
                    }
                    input::AccionApp::GuardarColeccion => {
                        estado_app.guardar_actual_en_coleccion();
                        storage::save_collections(estado_app)?;
                    }
                    input::AccionApp::GuardarTodo => {
                        storage::save_all(estado_app)?;
                        estado_app.mensaje_estado =
                            "Guardado completo en .modjo, al pelo.".to_string();
                    }
                    input::AccionApp::Exportar => {
                        let ruta_exportacion = PathBuf::from("modjo-export.json");
                        match storage::exportar_snapshot(estado_app, &ruta_exportacion) {
                            Ok(()) => {
                                estado_app.mensaje_estado = format!(
                                    "Exportado en {}. Quedo fino.",
                                    ruta_exportacion.display()
                                );
                            }
                            Err(error) => {
                                estado_app.mensaje_estado =
                                    format!("No se pudo exportar snapshot: {error}");
                            }
                        }
                    }
                    input::AccionApp::Importar => {
                        let ruta_importacion = PathBuf::from("modjo-import.json");
                        match storage::importar_snapshot(&ruta_importacion) {
                            Ok(snapshot_importado) => {
                                estado_app.cargar_snapshot(snapshot_importado);
                                if let Err(error) = storage::save_all(estado_app) {
                                    estado_app.mensaje_estado =
                                        format!("Importado, pero no se pudo guardar: {error}");
                                }
                            }
                            Err(error) => {
                                estado_app.mensaje_estado = format!(
                                    "No se pudo importar {}: {}",
                                    ruta_importacion.display(),
                                    error
                                );
                            }
                        }
                    }
                    input::AccionApp::EjecutarAyudaIa => {
                        estado_app.mensaje_estado = format!(
                            "IA: prueba {} {} con Accept: application/json y valida status + body.",
                            estado_app.request.method, estado_app.request.url
                        );
                    }
                    input::AccionApp::EjecutarAccionMenu(accion_menu) => match accion_menu {
                        AccionMenu::AbrirAyudaRapida => estado_app.abrir_ayuda_rapida(),
                        AccionMenu::GuardarTodo => {
                            storage::save_all(estado_app)?;
                            estado_app.mensaje_estado =
                                "Guardado completo en .modjo, al pelo.".to_string();
                        }
                        AccionMenu::Importar => {
                            let ruta_importacion = PathBuf::from("modjo-import.json");
                            match storage::importar_snapshot(&ruta_importacion) {
                                Ok(snapshot_importado) => {
                                    estado_app.cargar_snapshot(snapshot_importado);
                                    if let Err(error) = storage::save_all(estado_app) {
                                        estado_app.mensaje_estado = format!(
                                            "Importado, pero no se pudo guardar: {error}"
                                        );
                                    }
                                }
                                Err(error) => {
                                    estado_app.mensaje_estado = format!(
                                        "No se pudo importar {}: {}",
                                        ruta_importacion.display(),
                                        error
                                    );
                                }
                            }
                        }
                        AccionMenu::Exportar => {
                            let ruta_exportacion = PathBuf::from("modjo-export.json");
                            match storage::exportar_snapshot(estado_app, &ruta_exportacion) {
                                Ok(()) => {
                                    estado_app.mensaje_estado = format!(
                                        "Exportado en {}. Quedo fino.",
                                        ruta_exportacion.display()
                                    );
                                }
                                Err(error) => {
                                    estado_app.mensaje_estado =
                                        format!("No se pudo exportar snapshot: {error}");
                                }
                            }
                        }
                        AccionMenu::SelectorMetodo => estado_app.abrir_selector_metodo(),
                        AccionMenu::EjecutarRequest => {
                            if !estado_app.loading {
                                let request_pendiente = PendingRequest::from(&*estado_app);
                                estado_app.loading = true;
                                estado_app.last_error = None;
                                estado_app.mensaje_estado =
                                    "Enviando request... aguanta un toque.".to_string();
                                let respuesta_tx = canal_respuesta_tx.clone();
                                tokio::spawn(async move {
                                    let respuesta =
                                        http::client::send_request(request_pendiente).await;
                                    let _ = respuesta_tx.send(respuesta);
                                });
                            }
                        }
                        AccionMenu::AyudaIa => {
                            estado_app.mensaje_estado = format!(
                                "IA: prueba {} {} con Accept: application/json y valida status + body.",
                                estado_app.request.method, estado_app.request.url
                            );
                        }
                    },
                }
            }
        }
    }

    Ok(())
}
