//! Estado central de la aplicación TUI.
//!
//! Este módulo concentra:
//! - Modelos serializables de request/response.
//! - Estado de interacción de la interfaz.
//! - Utilidades de formato y transformación (por ejemplo interpolación de variables).

use std::{collections::BTreeMap, fs, path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ZonaFoco {
    MenuSuperior,
    Sidebar,
    Metodo,
    Url,
    Headers,
    Body,
    Response,
}

impl ZonaFoco {
    pub fn siguiente(self) -> Self {
        match self {
            Self::MenuSuperior => Self::Sidebar,
            Self::Sidebar => Self::Metodo,
            Self::Metodo => Self::Url,
            Self::Url => Self::Headers,
            Self::Headers => Self::Body,
            Self::Body => Self::Response,
            Self::Response => Self::MenuSuperior,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SidebarActiva {
    Historial,
    Colecciones,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModoEntrada {
    Normal,
    Insertar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalActivo {
    Ninguno,
    SelectorMetodo,
    SelectorMenuSuperior,
    AyudaRapida,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuSuperior {
    Help,
    File,
    Edit,
    Selection,
    View,
    Go,
    Run,
    Todo,
    Ai,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestModel {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Default for RequestModel {
    fn default() -> Self {
        Self {
            method: "GET".to_string(),
            url: "https://httpbin.org/get".to_string(),
            headers: vec![("Accept".to_string(), "application/json".to_string())],
            body: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseModel {
    pub status: Option<u16>,
    pub status_text: String,
    pub duration_ms: u128,
    pub size_bytes: usize,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub method: String,
    pub url: String,
    pub status: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionEntry {
    pub name: String,
    pub request: RequestModel,
}

#[derive(Debug, Clone)]
pub struct PendingRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl From<&EstadoApp> for PendingRequest {
    fn from(value: &EstadoApp) -> Self {
        Self {
            method: value.request.method.clone(),
            url: interpolar_variables_entorno(&value.request.url, &value.env_vars),
            headers: value
                .request
                .headers
                .iter()
                .map(|(k, v)| (k.clone(), interpolar_variables_entorno(v, &value.env_vars)))
                .collect(),
            body: interpolar_variables_entorno(&value.request.body, &value.env_vars),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResponseData {
    pub response: Option<ResponseModel>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotExport {
    pub history: Vec<HistoryEntry>,
    pub collections: Vec<CollectionEntry>,
    pub env_vars: BTreeMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstadoApp {
    /// Request que se está editando actualmente.
    pub request: RequestModel,
    /// Última respuesta obtenida.
    pub response: Option<ResponseModel>,
    /// Historial local de ejecuciones recientes.
    pub history: Vec<HistoryEntry>,
    /// Colecciones guardadas por el usuario.
    pub collections: Vec<CollectionEntry>,
    /// Variables de entorno para interpolar tokens como `{{base_url}}`.
    pub env_vars: BTreeMap<String, String>,
    /// Zona con foco de interacción principal.
    pub foco: ZonaFoco,
    /// Lista activa en la barra lateral.
    pub sidebar: SidebarActiva,
    /// Índice seleccionado en la barra lateral.
    pub sidebar_index: usize,
    /// Indica si hay un request en curso.
    pub loading: bool,
    /// Error reciente, si aplica.
    pub last_error: Option<String>,
    /// Mensaje breve mostrado en barra de estado.
    pub mensaje_estado: String,
    /// Índice del tab superior activo.
    pub tab_activo: usize,
    /// Ancho dinámico de la barra lateral.
    pub ancho_sidebar: u16,
    /// Alto dinámico del panel de headers.
    pub alto_headers: u16,
    /// Alto dinámico del panel de body.
    pub alto_body: u16,
    #[serde(skip, default = "modo_default")]
    pub modo: ModoEntrada,
    #[serde(skip, default = "modal_default")]
    pub modal_activo: ModalActivo,
    #[serde(skip, default = "metodo_idx_default")]
    pub metodo_idx_selector: usize,
    #[serde(skip, default = "menu_superior_default")]
    pub menu_superior_activo: MenuSuperior,
    #[serde(skip, default = "menu_selector_idx_default")]
    pub menu_selector_idx: usize,
    #[serde(skip, default)]
    pub portapapeles_interno: String,
}

impl Default for EstadoApp {
    fn default() -> Self {
        Self {
            request: RequestModel::default(),
            response: None,
            history: Vec::new(),
            collections: Vec::new(),
            env_vars: BTreeMap::new(),
            foco: ZonaFoco::MenuSuperior,
            sidebar: SidebarActiva::Historial,
            sidebar_index: 0,
            loading: false,
            last_error: None,
            mensaje_estado: "Listo, parcero. Pulsa i para escribir.".to_string(),
            tab_activo: 0,
            ancho_sidebar: 30,
            alto_headers: 8,
            alto_body: 8,
            modo: ModoEntrada::Normal,
            modal_activo: ModalActivo::Ninguno,
            metodo_idx_selector: 0,
            menu_superior_activo: MenuSuperior::Help,
            menu_selector_idx: 0,
            portapapeles_interno: String::new(),
        }
    }
}

fn modo_default() -> ModoEntrada {
    ModoEntrada::Normal
}

fn modal_default() -> ModalActivo {
    ModalActivo::Ninguno
}

fn metodo_idx_default() -> usize {
    0
}

fn menu_superior_default() -> MenuSuperior {
    MenuSuperior::Help
}

fn menu_selector_idx_default() -> usize {
    0
}

impl EstadoApp {
    /// Carga el estado persistido y, si no existe, crea uno por defecto.
    pub fn cargar_o_predeterminado() -> color_eyre::Result<Self> {
        let mut estado = Self::default();
        let directorio_datos = data_dir();
        fs::create_dir_all(&directorio_datos)?;
        estado.history = crate::storage::load_history().unwrap_or_default();
        estado.collections = crate::storage::load_collections().unwrap_or_default();
        estado.env_vars = crate::storage::load_env_vars().unwrap_or_default();
        Ok(estado)
    }

    /// Alias conservado por compatibilidad con llamadas existentes.
    #[allow(dead_code)]
    pub fn load_or_default() -> color_eyre::Result<Self> {
        Self::cargar_o_predeterminado()
    }

    /// Aplica una respuesta entrante al estado y actualiza historial.
    pub fn aplicar_respuesta(&mut self, data: ResponseData) {
        self.loading = false;
        if let Some(err) = data.error {
            self.last_error = Some(err.clone());
            self.mensaje_estado = format!("Uy, fallo la vuelta: {err}");
            return;
        }

        self.response = data.response;
        self.last_error = None;
        self.history.insert(
            0,
            HistoryEntry {
                method: self.request.method.clone(),
                url: self.request.url.clone(),
                status: self.response.as_ref().and_then(|r| r.status),
            },
        );
        if self.history.len() > 150 {
            self.history.truncate(150);
        }
        let _ = crate::storage::save_history(self);
        self.mensaje_estado = "Respuesta lista. Todo bien, mi llave.".to_string();
    }

    /// Guarda el request actual en colecciones con un nombre derivado.
    pub fn guardar_actual_en_coleccion(&mut self) {
        let nombre = format!(
            "{} {}",
            self.request.method,
            self.request.url.chars().take(36).collect::<String>()
        );
        self.collections.insert(
            0,
            CollectionEntry {
                name: nombre,
                request: self.request.clone(),
            },
        );
        if self.collections.len() > 150 {
            self.collections.truncate(150);
        }
        self.mensaje_estado = "Guardado en colecciones, de una.".to_string();
    }

    /// Carga el elemento seleccionado desde historial o colecciones.
    pub fn activar_item_sidebar(&mut self) {
        match self.sidebar {
            SidebarActiva::Historial => {
                if let Some(item) = self.history.get(self.sidebar_index) {
                    self.request.method = item.method.clone();
                    self.request.url = item.url.clone();
                }
            }
            SidebarActiva::Colecciones => {
                if let Some(item) = self.collections.get(self.sidebar_index) {
                    self.request = item.request.clone();
                }
            }
        }
        self.mensaje_estado = "Elemento cargado en el editor.".to_string();
    }

    /// Devuelve el tamaño de la lista activa en la barra lateral.
    pub fn sidebar_len(&self) -> usize {
        match self.sidebar {
            SidebarActiva::Historial => self.history.len(),
            SidebarActiva::Colecciones => self.collections.len(),
        }
    }

    /// Selecciona el item anterior en la barra lateral.
    pub fn prev_sidebar_item(&mut self) {
        let len = self.sidebar_len();
        if len == 0 {
            self.sidebar_index = 0;
        } else if self.sidebar_index == 0 {
            self.sidebar_index = len - 1;
        } else {
            self.sidebar_index -= 1;
        }
    }

    /// Selecciona el item siguiente en la barra lateral.
    pub fn next_sidebar_item(&mut self) {
        let len = self.sidebar_len();
        if len == 0 {
            self.sidebar_index = 0;
        } else {
            self.sidebar_index = (self.sidebar_index + 1) % len;
        }
    }

    /// Crea un snapshot serializable del estado persistente.
    pub fn snapshot(&self) -> SnapshotExport {
        SnapshotExport {
            history: self.history.clone(),
            collections: self.collections.clone(),
            env_vars: self.env_vars.clone(),
        }
    }

    /// Reemplaza historial/colecciones/variables con datos importados.
    pub fn cargar_snapshot(&mut self, snapshot: SnapshotExport) {
        self.history = snapshot.history;
        self.collections = snapshot.collections;
        self.env_vars = snapshot.env_vars;
        self.mensaje_estado = "Importacion hecha. Quedo melo.".to_string();
    }

    /// Ajusta dimensiones de paneles dentro de límites seguros.
    pub fn redimensionar_paneles(&mut self, dx_sidebar: i16, dh_headers: i16, dh_body: i16) {
        self.ancho_sidebar = ajustar_rango_u16(self.ancho_sidebar, dx_sidebar, 22, 55);
        self.alto_headers = ajustar_rango_u16(self.alto_headers, dh_headers, 5, 16);
        self.alto_body = ajustar_rango_u16(self.alto_body, dh_body, 5, 16);
    }

    /// Sincroniza el foco principal de UI con el tab superior activo.
    pub fn foco_desde_tab(&mut self) {
        self.foco = match self.tab_activo {
            0 => ZonaFoco::Url,
            1 => ZonaFoco::Headers,
            2 => ZonaFoco::Body,
            _ => ZonaFoco::Response,
        };
    }

    pub fn abrir_selector_metodo(&mut self) {
        self.modal_activo = ModalActivo::SelectorMetodo;
        self.metodo_idx_selector = metodos_http()
            .iter()
            .position(|m| *m == self.request.method)
            .unwrap_or(0);
        self.mensaje_estado = "Selector de método abierto.".to_string();
    }

    pub fn abrir_ayuda_rapida(&mut self) {
        self.modal_activo = ModalActivo::AyudaRapida;
    }

    pub fn abrir_selector_menu_superior(&mut self) {
        self.modal_activo = ModalActivo::SelectorMenuSuperior;
        self.menu_selector_idx = 0;
        self.mensaje_estado = format!(
            "Menú {} abierto.",
            self.menu_superior_activo.etiqueta()
        );
    }

    pub fn cerrar_modal(&mut self) {
        self.modal_activo = ModalActivo::Ninguno;
    }

    pub fn mover_selector_metodo(&mut self, delta: i32) {
        let opciones = metodos_http();
        if opciones.is_empty() {
            return;
        }
        if delta < 0 {
            self.metodo_idx_selector = if self.metodo_idx_selector == 0 {
                opciones.len() - 1
            } else {
                self.metodo_idx_selector - 1
            };
        } else {
            self.metodo_idx_selector = (self.metodo_idx_selector + 1) % opciones.len();
        }
    }

    pub fn confirmar_selector_metodo(&mut self) {
        if let Some(metodo) = metodos_http().get(self.metodo_idx_selector) {
            self.request.method = (*metodo).to_string();
            self.mensaje_estado = format!("Método actualizado a {}.", metodo);
        }
        self.cerrar_modal();
    }

    pub fn mover_menu_superior(&mut self, delta: i32) {
        let menus = MenuSuperior::todos();
        let mut idx = menus
            .iter()
            .position(|m| *m == self.menu_superior_activo)
            .unwrap_or(0);
        if delta < 0 {
            idx = if idx == 0 { menus.len() - 1 } else { idx - 1 };
        } else {
            idx = (idx + 1) % menus.len();
        }
        self.menu_superior_activo = menus[idx];
    }

    pub fn mover_selector_menu(&mut self, delta: i32) {
        let opciones = self.menu_superior_activo.opciones();
        if opciones.is_empty() {
            self.menu_selector_idx = 0;
            return;
        }
        if delta < 0 {
            self.menu_selector_idx = if self.menu_selector_idx == 0 {
                opciones.len() - 1
            } else {
                self.menu_selector_idx - 1
            };
        } else {
            self.menu_selector_idx = (self.menu_selector_idx + 1) % opciones.len();
        }
    }

    pub fn aplicar_opcion_menu_actual(&mut self) -> Option<AccionMenu> {
        let opciones = self.menu_superior_activo.opciones();
        let opcion = opciones.get(self.menu_selector_idx).copied()?;
        self.cerrar_modal();
        Some(opcion)
    }
}

fn ajustar_rango_u16(base: u16, delta: i16, min: u16, max: u16) -> u16 {
    let next = if delta >= 0 {
        base.saturating_add(delta as u16)
    } else {
        base.saturating_sub((-delta) as u16)
    };
    next.clamp(min, max)
}

/// Intenta formatear JSON; si falla, devuelve el texto original.
pub fn formatear_json_o_texto(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .and_then(|value| serde_json::to_string_pretty(&value))
        .unwrap_or_else(|_| body.to_string())
}

/// Alias conservado por compatibilidad con llamadas existentes.
pub fn pretty_json_or_raw(body: &str) -> String {
    formatear_json_o_texto(body)
}

/// Convierte duración a milisegundos enteros.
pub fn duration_to_ms(duration: Duration) -> u128 {
    duration.as_millis()
}

/// Directorio local de datos de Modjo (`./.modjo`).
pub fn data_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".modjo")
}

/// Interpola tokens `{{clave}}` usando variables de entorno cargadas.
pub fn interpolar_variables_entorno(entrada: &str, variables: &BTreeMap<String, String>) -> String {
    let mut texto_interpolado = entrada.to_string();
    for (clave, valor) in variables {
        texto_interpolado = texto_interpolado.replace(&format!("{{{{{clave}}}}}"), valor);
    }
    texto_interpolado
}

pub fn metodos_http() -> &'static [&'static str] {
    &["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccionMenu {
    AbrirAyudaRapida,
    GuardarTodo,
    Importar,
    Exportar,
    SelectorMetodo,
    EjecutarRequest,
    AyudaIa,
}

impl MenuSuperior {
    pub fn todos() -> &'static [Self] {
        &[
            Self::Help,
            Self::File,
            Self::Edit,
            Self::Selection,
            Self::View,
            Self::Go,
            Self::Run,
            Self::Todo,
            Self::Ai,
        ]
    }

    pub fn etiqueta(self) -> &'static str {
        match self {
            Self::Help => "Help",
            Self::File => "File",
            Self::Edit => "Edit",
            Self::Selection => "Selection",
            Self::View => "View",
            Self::Go => "Go",
            Self::Run => "Run",
            Self::Todo => "Todo",
            Self::Ai => "AI",
        }
    }

    pub fn opciones(self) -> &'static [AccionMenu] {
        match self {
            Self::Help => &[AccionMenu::AbrirAyudaRapida],
            Self::File => &[
                AccionMenu::GuardarTodo,
                AccionMenu::Importar,
                AccionMenu::Exportar,
            ],
            Self::Edit => &[AccionMenu::SelectorMetodo],
            Self::Selection => &[AccionMenu::SelectorMetodo],
            Self::View => &[AccionMenu::AbrirAyudaRapida],
            Self::Go => &[AccionMenu::SelectorMetodo],
            Self::Run => &[AccionMenu::EjecutarRequest],
            Self::Todo => &[AccionMenu::AbrirAyudaRapida],
            Self::Ai => &[AccionMenu::AyudaIa],
        }
    }
}

pub fn etiqueta_accion_menu(accion: AccionMenu) -> &'static str {
    match accion {
        AccionMenu::AbrirAyudaRapida => "Abrir ayuda rápida",
        AccionMenu::GuardarTodo => "Guardar todo",
        AccionMenu::Importar => "Importar snapshot",
        AccionMenu::Exportar => "Exportar snapshot",
        AccionMenu::SelectorMetodo => "Seleccionar método HTTP",
        AccionMenu::EjecutarRequest => "Ejecutar request",
        AccionMenu::AyudaIa => "Sugerencia IA",
    }
}

/// Alias conservado por compatibilidad con llamadas existentes.
#[allow(dead_code)]
pub fn interpolar_env(input: &str, env: &BTreeMap<String, String>) -> String {
    interpolar_variables_entorno(input, env)
}
