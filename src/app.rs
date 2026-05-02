//! Estado central de la aplicación TUI.
//!
//! Este módulo concentra:
//! - Modelos serializables de request/response.
//! - Estado de interacción de la interfaz.
//! - Utilidades de formato y transformación (por ejemplo interpolación de variables).

use std::{collections::BTreeMap, fs, path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Idioma {
    En,
    Es,
}

impl Default for Idioma {
    fn default() -> Self {
        Self::En
    }
}

impl Idioma {
    pub fn todos() -> &'static [Self] {
        &[Self::En, Self::Es]
    }

    pub fn nombre(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Es => "Español (Colombia)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub mensaje: String,
    pub creado_en: std::time::Instant,
    pub duracion: Duration,
}

impl Toast {
    pub fn nuevo(mensaje: impl Into<String>) -> Self {
        Self {
            mensaje: mensaje.into(),
            creado_en: std::time::Instant::now(),
            duracion: Duration::from_millis(2000),
        }
    }

    #[allow(dead_code)]
    pub fn nuevo_con_duracion(mensaje: impl Into<String>, duracion: Duration) -> Self {
        Self {
            mensaje: mensaje.into(),
            creado_en: std::time::Instant::now(),
            duracion,
        }
    }

    pub fn activo(&self) -> bool {
        self.creado_en.elapsed() < self.duracion
    }
}

#[derive(Debug, Clone)]
pub struct ClipboardAnim {
    #[allow(dead_code)]
    pub texto: String,
    pub creado_en: std::time::Instant,
}

impl ClipboardAnim {
    pub fn nuevo(texto: impl Into<String>) -> Self {
        Self {
            texto: texto.into(),
            creado_en: std::time::Instant::now(),
        }
    }

    pub fn activo(&self) -> bool {
        self.creado_en.elapsed() < Duration::from_millis(800)
    }

    pub fn progreso(&self) -> f32 {
        let elapsed = self.creado_en.elapsed().as_millis() as f32;
        (elapsed / 800.0).min(1.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CursorState {
    pub posicion: usize,
}

#[derive(Debug, Clone)]
pub struct UndoEntry {
    pub campo: CampoEditado,
    pub contenido_anterior: String,
    pub cursor_anterior: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampoEditado {
    Metodo,
    Url,
    Body,
    HeaderValor(usize),
}

#[derive(Debug, Clone)]
pub struct DragState {
    pub tipo: TipoDrag,
    pub inicio_x: u16,
    pub inicio_y: u16,
    pub valor_inicial: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoDrag {
    SidebarAncho,
    AltoHeaders,
    AltoBody,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HoverZone {
    #[default]
    Ninguno,
    SidebarTabHist,
    SidebarTabCol,
    SidebarItem(usize),
    BtnMetodo,
    BtnUrl,
    BtnHeaders,
    BtnBody,
    BtnResponse,
    ResizeSidebar,
    ResizeHeaders,
    ResizeBody,
    StatusMode,
    ModalItem(usize),
}

#[derive(Debug, Clone, Default)]
pub struct MouseState {
    pub row: u16,
    pub col: u16,
    pub hover: HoverZone,
}

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
    CommandPalette,
    SelectorTema,
    SelectorIdioma,
}

#[derive(Debug, Clone)]
pub struct ComandoPalette {
    pub etiqueta: &'static str,
    pub atajo: &'static str,
    pub accion: AccionMenu,
}

pub fn comandos_disponibles() -> Vec<ComandoPalette> {
    vec![
        ComandoPalette { etiqueta: "Run request", atajo: "r", accion: AccionMenu::EjecutarRequest },
        ComandoPalette { etiqueta: "Save to collection", atajo: "w", accion: AccionMenu::GuardarTodo },
        ComandoPalette { etiqueta: "Export snapshot", atajo: "e", accion: AccionMenu::Exportar },
        ComandoPalette { etiqueta: "Import snapshot", atajo: "o", accion: AccionMenu::Importar },
        ComandoPalette { etiqueta: "Select HTTP method", atajo: "m", accion: AccionMenu::SelectorMetodo },
        ComandoPalette { etiqueta: "Toggle JSON format", atajo: "f", accion: AccionMenu::ToggleJsonFormat },
        ComandoPalette { etiqueta: "Change theme", atajo: "T", accion: AccionMenu::SelectorTema },
        ComandoPalette { etiqueta: "Change language", atajo: "L", accion: AccionMenu::SelectorIdioma },
        ComandoPalette { etiqueta: "AI suggestion", atajo: "ctrl+a", accion: AccionMenu::AyudaIa },
        ComandoPalette { etiqueta: "Help", atajo: "?", accion: AccionMenu::AbrirAyudaRapida },
    ]
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
    // --- Nuevos campos para cursor, undo, animaciones ---
    #[serde(skip, default)]
    pub cursor: CursorState,
    #[serde(skip, default)]
    pub undo_stack: Vec<UndoEntry>,
    #[serde(skip, default)]
    pub toast: Option<Toast>,
    #[serde(skip, default)]
    pub clipboard_anim: Option<ClipboardAnim>,
    #[serde(skip, default)]
    pub tick: u64,
    #[serde(skip, default)]
    pub drag_state: Option<DragState>,
    #[serde(skip, default)]
    pub sidebar_hover: Option<usize>,
    #[serde(skip, default)]
    pub focus_flash: Option<std::time::Instant>,
    #[serde(skip, default)]
    pub mouse: MouseState,
    #[serde(skip, default)]
    pub terminal_height: u16,
    // --- Theme ---
    #[serde(default)]
    pub tema: crate::ui::theme::TemaVariant,
    // --- Language ---
    #[serde(default)]
    pub idioma: Idioma,
    // --- Response panel state ---
    #[serde(skip, default)]
    pub response_scroll: u16,
    #[serde(skip, default = "response_formatted_default")]
    pub response_formatted: bool,
    #[serde(skip, default)]
    pub response_body_pretty: String,
    #[serde(skip, default)]
    pub response_body_raw: String,
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
            mensaje_estado: "ready. press i to type.".to_string(),
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
            cursor: CursorState::default(),
            undo_stack: Vec::new(),
            toast: None,
            clipboard_anim: None,
            tick: 0,
            drag_state: None,
            sidebar_hover: None,
            focus_flash: None,
            mouse: MouseState::default(),
            terminal_height: 24,
            tema: crate::ui::theme::TemaVariant::default(),
            idioma: Idioma::default(),
            response_scroll: 0,
            response_formatted: true,
            response_body_pretty: String::new(),
            response_body_raw: String::new(),
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

fn response_formatted_default() -> bool {
    true
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
            self.mensaje_estado = format!("error: {err}");
            return;
        }

        self.response = data.response;
        self.last_error = None;
        self.response_scroll = 0;

        // Cache both versions of the body
        if let Some(ref resp) = self.response {
            self.response_body_pretty = formatear_json_o_texto(&resp.body);
            self.response_body_raw = resp.body.clone();
        }

        // Deduplicate: if same method+url exists, update it instead of inserting
        let existing_idx = self.history.iter().position(|entry| {
            entry.method == self.request.method && entry.url == self.request.url
        });

        let new_entry = HistoryEntry {
            method: self.request.method.clone(),
            url: self.request.url.clone(),
            status: self.response.as_ref().and_then(|r| r.status),
        };

        if let Some(idx) = existing_idx {
            self.history.remove(idx);
        }
        self.history.insert(0, new_entry);

        if self.history.len() > 150 {
            self.history.truncate(150);
        }
        let _ = crate::storage::save_history(self);
        self.mensaje_estado = "response received".to_string();
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

    /// Devuelve el texto activo según foco.
    pub fn texto_foco(&self) -> &str {
        match self.foco {
            ZonaFoco::Metodo => &self.request.method,
            ZonaFoco::Url => &self.request.url,
            ZonaFoco::Body => &self.request.body,
            ZonaFoco::Headers => self
                .request
                .headers
                .first()
                .map(|(_, v)| v.as_str())
                .unwrap_or(""),
            _ => "",
        }
    }

    /// Devuelve una referencia mutable al texto activo según foco.
    pub fn texto_foco_mut(&mut self) -> Option<&mut String> {
        match self.foco {
            ZonaFoco::Metodo => Some(&mut self.request.method),
            ZonaFoco::Url => Some(&mut self.request.url),
            ZonaFoco::Body => Some(&mut self.request.body),
            ZonaFoco::Headers => self.request.headers.first_mut().map(|(_, v)| v),
            _ => None,
        }
    }

    /// Campo editado actual según foco.
    pub fn campo_actual(&self) -> Option<CampoEditado> {
        match self.foco {
            ZonaFoco::Metodo => Some(CampoEditado::Metodo),
            ZonaFoco::Url => Some(CampoEditado::Url),
            ZonaFoco::Body => Some(CampoEditado::Body),
            ZonaFoco::Headers => Some(CampoEditado::HeaderValor(0)),
            _ => None,
        }
    }

    /// Empuja estado al undo stack antes de una mutación.
    pub fn push_undo(&mut self) {
        let campo = self.campo_actual();
        let texto = match self.foco {
            ZonaFoco::Metodo => Some(self.request.method.clone()),
            ZonaFoco::Url => Some(self.request.url.clone()),
            ZonaFoco::Body => Some(self.request.body.clone()),
            ZonaFoco::Headers => self.request.headers.first().map(|(_, v)| v.clone()),
            _ => None,
        };
        if let (Some(campo), Some(texto)) = (campo, texto) {
            self.undo_stack.push(UndoEntry {
                campo,
                contenido_anterior: texto,
                cursor_anterior: self.cursor.posicion,
            });
            if self.undo_stack.len() > 100 {
                self.undo_stack.remove(0);
            }
        }
    }

    /// Deshace la última mutación.
    pub fn undo(&mut self) {
        if let Some(entry) = self.undo_stack.pop() {
            match entry.campo {
                CampoEditado::Metodo => self.request.method = entry.contenido_anterior,
                CampoEditado::Url => self.request.url = entry.contenido_anterior,
                CampoEditado::Body => self.request.body = entry.contenido_anterior,
                CampoEditado::HeaderValor(i) => {
                    if let Some((_, v)) = self.request.headers.get_mut(i) {
                        *v = entry.contenido_anterior;
                    }
                }
            }
            self.cursor.posicion = entry.cursor_anterior;
            self.toast = Some(Toast::nuevo("undo"));
        }
    }

    /// Mueve cursor a la izquierda.
    pub fn cursor_left(&mut self) {
        self.cursor.posicion = self.cursor.posicion.saturating_sub(1);
    }

    /// Mueve cursor a la derecha.
    pub fn cursor_right(&mut self) {
        let len = self.texto_foco().len();
        if self.cursor.posicion < len {
            self.cursor.posicion += 1;
        }
    }

    /// Mueve cursor al inicio.
    pub fn cursor_home(&mut self) {
        self.cursor.posicion = 0;
    }

    /// Mueve cursor al final.
    pub fn cursor_end(&mut self) {
        self.cursor.posicion = self.texto_foco().len();
    }

    /// Sincroniza cursor al final del texto (al cambiar de foco).
    pub fn sync_cursor_to_end(&mut self) {
        self.cursor.posicion = self.texto_foco().len();
    }

    /// Mueve cursor a la línea anterior (multi-línea).
    pub fn cursor_up(&mut self) {
        let texto = self.texto_foco().to_string();
        let pos = self.cursor.posicion.min(texto.len());
        // Encontrar inicio de línea actual
        let line_start = texto[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
        if line_start == 0 {
            // Ya estamos en la primera línea
            self.cursor.posicion = 0;
            return;
        }
        // Calcular columna actual
        let col = pos - line_start;
        // Encontrar inicio de línea anterior
        let prev_line_end = line_start - 1; // el \n anterior
        let prev_line_start = texto[..prev_line_end].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let prev_line_len = prev_line_end - prev_line_start;
        let new_col = col.min(prev_line_len);
        self.cursor.posicion = prev_line_start + new_col;
    }

    /// Mueve cursor a la siguiente línea (multi-línea).
    pub fn cursor_down(&mut self) {
        let texto = self.texto_foco().to_string();
        let pos = self.cursor.posicion.min(texto.len());
        // Encontrar inicio de línea actual
        let line_start = texto[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let col = pos - line_start;
        // Encontrar fin de línea actual
        let line_end = texto[pos..].find('\n').map(|i| pos + i).unwrap_or(texto.len());
        if line_end >= texto.len() {
            // Ya estamos en la última línea
            return;
        }
        // Inicio de siguiente línea
        let next_line_start = line_end + 1;
        let next_line_end = texto[next_line_start..]
            .find('\n')
            .map(|i| next_line_start + i)
            .unwrap_or(texto.len());
        let next_line_len = next_line_end - next_line_start;
        let new_col = col.min(next_line_len);
        self.cursor.posicion = next_line_start + new_col;
    }

    /// Mueve cursor al inicio de la línea actual.
    pub fn cursor_line_start(&mut self) {
        let texto = self.texto_foco().to_string();
        let pos = self.cursor.posicion.min(texto.len());
        self.cursor.posicion = texto[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
    }

    /// Mueve cursor al final de la línea actual.
    pub fn cursor_line_end(&mut self) {
        let texto = self.texto_foco().to_string();
        let pos = self.cursor.posicion.min(texto.len());
        self.cursor.posicion = texto[pos..]
            .find('\n')
            .map(|i| pos + i)
            .unwrap_or(texto.len());
    }

    /// Inserta carácter con auto-completado de brackets/quotes.
    pub fn insertar_char_con_autocomplete(&mut self, ch: char) {
        let closing = match ch {
            '{' => Some('}'),
            '[' => Some(']'),
            '(' => Some(')'),
            '"' => Some('"'),
            '\'' => Some('\''),
            '`' => Some('`'),
            _ => None,
        };

        self.insertar_char(ch);

        if let Some(c) = closing {
            // No duplicar si ya hay el mismo carácter justo después
            let texto = self.texto_foco().to_string();
            let pos = self.cursor.posicion;
            let next_char = texto.chars().nth(pos);
            if next_char != Some(c) {
                self.insertar_char(c);
                // Retroceder cursor para quedar entre los brackets
                self.cursor.posicion = self.cursor.posicion.saturating_sub(1);
            }
        }
    }

    /// Inserta carácter en posición del cursor.
    pub fn insertar_char(&mut self, ch: char) {
        let pos = self.cursor.posicion;
        match self.foco {
            ZonaFoco::Metodo => {
                let p = pos.min(self.request.method.len());
                self.request.method.insert(p, ch);
                self.cursor.posicion = p + 1;
            }
            ZonaFoco::Url => {
                let p = pos.min(self.request.url.len());
                self.request.url.insert(p, ch);
                self.cursor.posicion = p + 1;
            }
            ZonaFoco::Body => {
                let p = pos.min(self.request.body.len());
                self.request.body.insert(p, ch);
                self.cursor.posicion = p + 1;
            }
            ZonaFoco::Headers => {
                if self.request.headers.is_empty() {
                    self.request.headers.push((String::new(), String::new()));
                }
                if let Some((_, v)) = self.request.headers.first_mut() {
                    let p = pos.min(v.len());
                    v.insert(p, ch);
                    self.cursor.posicion = p + 1;
                }
            }
            _ => {}
        }
    }

    /// Borra carácter antes del cursor (backspace).
    pub fn borrar_char(&mut self) {
        let pos = self.cursor.posicion;
        if pos == 0 {
            return;
        }
        match self.foco {
            ZonaFoco::Metodo => {
                if pos <= self.request.method.len() {
                    self.request.method.remove(pos - 1);
                    self.cursor.posicion = pos - 1;
                }
            }
            ZonaFoco::Url => {
                if pos <= self.request.url.len() {
                    self.request.url.remove(pos - 1);
                    self.cursor.posicion = pos - 1;
                }
            }
            ZonaFoco::Body => {
                if pos <= self.request.body.len() {
                    self.request.body.remove(pos - 1);
                    self.cursor.posicion = pos - 1;
                }
            }
            ZonaFoco::Headers => {
                if let Some((_, v)) = self.request.headers.first_mut() {
                    if pos <= v.len() {
                        v.remove(pos - 1);
                        self.cursor.posicion = pos - 1;
                    }
                }
            }
            _ => {}
        }
    }

    /// Borra carácter en la posición del cursor (delete).
    pub fn borrar_adelante(&mut self) {
        let pos = self.cursor.posicion;
        match self.foco {
            ZonaFoco::Metodo => {
                if pos < self.request.method.len() {
                    self.request.method.remove(pos);
                }
            }
            ZonaFoco::Url => {
                if pos < self.request.url.len() {
                    self.request.url.remove(pos);
                }
            }
            ZonaFoco::Body => {
                if pos < self.request.body.len() {
                    self.request.body.remove(pos);
                }
            }
            ZonaFoco::Headers => {
                if let Some((_, v)) = self.request.headers.first_mut() {
                    if pos < v.len() {
                        v.remove(pos);
                    }
                }
            }
            _ => {}
        }
    }

    /// Muestra un toast temporal.
    pub fn mostrar_toast(&mut self, mensaje: impl Into<String>) {
        self.toast = Some(Toast::nuevo(mensaje));
    }

    /// Inicia animación de clipboard.
    pub fn animar_clipboard(&mut self, texto: impl Into<String>) {
        self.clipboard_anim = Some(ClipboardAnim::nuevo(texto));
    }

    /// Toggle entre JSON formateado y raw.
    pub fn toggle_response_format(&mut self) {
        self.response_formatted = !self.response_formatted;
        self.mostrar_toast(if self.response_formatted {
            "json: pretty"
        } else {
            "json: raw"
        });
    }

    /// Devuelve el body del response según el formato seleccionado.
    pub fn response_body_display(&self) -> &str {
        if self.response_formatted {
            &self.response_body_pretty
        } else {
            &self.response_body_raw
        }
    }

    /// Scroll up en el response.
    pub fn response_scroll_up(&mut self, lines: u16) {
        self.response_scroll = self.response_scroll.saturating_sub(lines);
    }

    /// Scroll down en el response.
    pub fn response_scroll_down(&mut self, lines: u16, max_lines: u16) {
        if max_lines > 0 {
            self.response_scroll = (self.response_scroll + lines).min(max_lines);
        }
    }

    /// Page up en el response.
    pub fn response_page_up(&mut self, page_size: u16) {
        self.response_scroll = self.response_scroll.saturating_sub(page_size);
    }

    /// Page down en el response.
    pub fn response_page_down(&mut self, page_size: u16, max_lines: u16) {
        if max_lines > 0 {
            self.response_scroll = (self.response_scroll + page_size).min(max_lines);
        }
    }

    /// Devuelve el número de líneas del body del response.
    pub fn response_body_lines(&self) -> u16 {
        self.response_body_display().lines().count() as u16
    }

    /// Actualiza la zona de hover basada en la posición del mouse.
    pub fn actualizar_hover(&mut self, row: u16, col: u16, terminal_height: u16) {
        self.mouse.row = row;
        self.mouse.col = col;

        // Modal abierto — los modales están centrados, calculamos offset dinámicamente
        if self.modal_activo != ModalActivo::Ninguno {
            // El modal empieza aproximadamente en terminal_height/2 - alto_modal/2
            // Para simplificar, asumimos que el contenido del modal empieza ~4 filas
            // después del borde superior del modal
            let modal_y_offset = terminal_height.saturating_sub(20) / 2 + 1; // rough top of modal content
            if row >= modal_y_offset + 3 {
                let idx = (row - modal_y_offset - 3) as usize;
                self.mouse.hover = HoverZone::ModalItem(idx);
            } else {
                self.mouse.hover = HoverZone::Ninguno;
            }
            return;
        }

        let sidebar_w = self.ancho_sidebar;
        // Layout: method+url = 3 filas (rows 0-2), headers empieza en row 3
        let fila_borde_headers = 3 + self.alto_headers;
        let fila_borde_body = fila_borde_headers + self.alto_body;

        // Resize zones
        if col >= sidebar_w.saturating_sub(1) && col <= sidebar_w + 1 && row >= 1 {
            self.mouse.hover = HoverZone::ResizeSidebar;
            return;
        }
        if row == fila_borde_headers && col >= sidebar_w {
            self.mouse.hover = HoverZone::ResizeHeaders;
            return;
        }
        if row == fila_borde_body && col >= sidebar_w {
            self.mouse.hover = HoverZone::ResizeBody;
            return;
        }

        // Status bar (última fila)
        if terminal_height > 0 && row >= terminal_height.saturating_sub(1) {
            self.mouse.hover = HoverZone::StatusMode;
            return;
        }

        // Sidebar: tabs en row 0, items desde row 1
        if col < sidebar_w {
            if row == 0 {
                if col < sidebar_w / 2 {
                    self.mouse.hover = HoverZone::SidebarTabHist;
                } else {
                    self.mouse.hover = HoverZone::SidebarTabCol;
                }
            } else if row >= 1 {
                let idx = (row - 1) as usize;
                if idx < self.sidebar_len() {
                    self.mouse.hover = HoverZone::SidebarItem(idx);
                    self.sidebar_hover = Some(idx);
                } else {
                    self.mouse.hover = HoverZone::Ninguno;
                    self.sidebar_hover = None;
                }
            } else {
                self.mouse.hover = HoverZone::Ninguno;
            }
            return;
        }

        // Main panels: method+url = rows 0-2, headers desde row 3
        if row < 3 {
            if col < sidebar_w + 12 {
                self.mouse.hover = HoverZone::BtnMetodo;
            } else {
                self.mouse.hover = HoverZone::BtnUrl;
            }
        } else if row < 3 + self.alto_headers {
            self.mouse.hover = HoverZone::BtnHeaders;
        } else if row < 3 + self.alto_headers + self.alto_body {
            self.mouse.hover = HoverZone::BtnBody;
        } else {
            self.mouse.hover = HoverZone::BtnResponse;
        }
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
        self.mensaje_estado = "selector de metodo abierto".to_string();
    }

    pub fn abrir_command_palette(&mut self) {
        self.modal_activo = ModalActivo::CommandPalette;
        self.menu_selector_idx = 0;
        self.mensaje_estado = "command palette".to_string();
    }

    pub fn abrir_selector_tema(&mut self) {
        self.modal_activo = ModalActivo::SelectorTema;
        self.menu_selector_idx = crate::ui::theme::TemaVariant::todos()
            .iter()
            .position(|t| *t == self.tema)
            .unwrap_or(0);
    }

    pub fn abrir_selector_idioma(&mut self) {
        self.modal_activo = ModalActivo::SelectorIdioma;
        self.menu_selector_idx = Idioma::todos()
            .iter()
            .position(|l| *l == self.idioma)
            .unwrap_or(0);
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
    ToggleJsonFormat,
    SelectorTema,
    SelectorIdioma,
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
        AccionMenu::AbrirAyudaRapida => "help",
        AccionMenu::GuardarTodo => "save all",
        AccionMenu::Importar => "import snapshot",
        AccionMenu::Exportar => "export snapshot",
        AccionMenu::SelectorMetodo => "select HTTP method",
        AccionMenu::EjecutarRequest => "run request",
        AccionMenu::AyudaIa => "AI suggestion",
        AccionMenu::ToggleJsonFormat => "toggle JSON format",
        AccionMenu::SelectorTema => "change theme",
        AccionMenu::SelectorIdioma => "change language",
    }
}

/// Alias conservado por compatibilidad con llamadas existentes.
#[allow(dead_code)]
pub fn interpolar_env(input: &str, env: &BTreeMap<String, String>) -> String {
    interpolar_variables_entorno(input, env)
}
