# modjo

El cliente TUI definitivo para explorar y probar APIs. Toda la potencia de Postman, el flujo de Apidog y la velocidad de la terminal. Hecho en Colombia con Rust.

[![CI/CD](https://github.com/jemgdevp/modjo/actions/workflows/rust.yml/badge.svg)](https://github.com/jemgdevp/modjo/actions/workflows/rust.yml)
[![Licencia MIT](https://img.shields.io/badge/licencia-MIT-verde.svg)](./LICENSE)
[![Versión](https://img.shields.io/badge/versi%C3%B3n-0.0.1-azul.svg)](./Cargo.toml)

---

## Documentación

Explorá nuestra documentación completa para sacarle el jugo a modjo:

- Sitio oficial: [modjo.jemg.dev](https://modjo.jemg.dev/docs)
- Guía de instalación, atajos, temas y más en la [wiki](https://github.com/jemgdevp/modjo/wiki)

---

## Características

- **Interfaz TUI intuitiva**: Navegá tus proyectos de API con una interfaz de terminal limpia y rápida. Modos Normal/Inserción estilo Vim.
- **Pruebas de API completas**: Creá, administrá y ejecutá peticiones HTTP con soporte para métodos GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS, headers personalizados y cuerpo a la medida.
- **Manejo de entornos**: Definí variables de entorno con `{{nombre_var}}` en URL, headers y cuerpo, y cambialas desde `.modjo/env.toml`.
- **Persistencia local**: Toda la información se guarda en la carpeta `.modjo/` de tu proyecto (historial, colecciones, variables de entorno). Nada sale de tu máquina.
- **Resaltado de sintaxis JSON**: Las respuestas JSON se muestran con colores por tipo de dato (claves, strings, números, booleanos, null).
- **Soporte para mouse**: Clic para enfocar paneles, hover para resaltar, arrastre para redimensionar, scroll para navegar respuestas.
- **Temas y dualidad de idiomas**: Cinco temas (OC-2 Dark, OC-2 Light, Catppuccin, Nord, Dracula) e interfaz en español colombiano o inglés.
- **Exportación e importación**: Compartí snapshots de tus peticiones con archivos `modjo-export.json` / `modjo-import.json`.
- **Paleta de comandos**: `Ctrl+P` abre una lista de todas las acciones disponibles con sus atajos.
- **Deshacer y portapapeles interno**: Ctrl+Z / `u` para deshacer, `y` y `p` para copiar y pegar.
- **Multiplataforma**: Corre en Windows, macOS y Linux. Solo necesitás Rust y una terminal con soporte Unicode.

---

## ¿Por qué modjo?

| Razón | Descripción |
|---|---|
| **Velocidad** | Arranque y ejecución al instante. Sin navegador, sin Electron, sin esperas. |
| **Flexibilidad** | Variables de entorno, temas, atajos personalizables y flujo modal. |
| **Colaboración** | Snapshots exportables para compartir peticiones con tu equipo. |
| **Código abierto** | MIT. La comunidad decide el rumbo. |
| **Hecho en Colombia** | Por dev colombiano, pa' todo el mundo. |

---

## Instalación

### Desde crates.io (próximamente)

```bash
cargo install modjo
```

### Desde el código fuente

Necesitás [Rust](https://rustup.rs/) (edición 2024) instalado.

```bash
git clone https://github.com/jemgdevp/modjo.git
cd modjo
cargo build --release
./target/release/modjo
```

### Binarios precompilados

Descargá el binario para tu sistema desde la página de [Releases](https://github.com/jemgdevp/modjo/releases).

---

## Uso

```bash
cargo run
# o si ya está instalado:
modjo
```

### Flujo principal

1. Seleccioná el **método HTTP** con `Ctrl+M` y escribí la **URL**.
2. Agregá **headers** y **cuerpo** en los paneles correspondientes.
3. Presioná `r` para enviar la petición.
4. Revisá **estado, tiempo, tamaño** y la respuesta en el panel de respuesta.
5. Presioná `c` para guardar la petición actual en colecciones.
6. Presioná `Enter` sobre un ítem del sidebar para cargarlo de vuelta.

### Atajos de teclado

#### Modo Normal (Vim-like)

| Tecla | Acción |
|---|---|
| `q` | Salir de modjo |
| `i` | Entrar en modo inserción |
| `Tab` | Rotar el foco entre paneles |
| `1` / `2` / `3` / `4` | Ir a la pestaña de Sidebar, Request, Body o Response |
| `r` | Enviar la petición actual |
| `c` | Guardar petición actual en colecciones |
| `w` | Guardar todo (historial + colecciones + variables) |
| `o` | Importar snapshot |
| `e` | Exportar snapshot |
| `m` | Abrir selector de método HTTP |
| `f` | Alternar formato JSON (pretty / raw) |
| `T` | Abrir selector de tema |
| `L` | Abrir selector de idioma |
| `?` | Mostrar ayuda rápida |
| `h` / `l` | Navegar items del sidebar (historial / colecciones) |
| `→` / `←` | Igual que h/l |
| `↑` / `↓` | Navegar entre items del sidebar |
| `Enter` | Cargar el ítem seleccionado del sidebar |
| `y` / `p` | Copiar / Pegar (portapapeles interno) |
| `u` / `Ctrl+Z` | Deshacer |
| `Ctrl+P` | Abrir paleta de comandos |
| `Ctrl+A` | Sugerencia de IA |

#### Modo Inserción

| Tecla | Acción |
|---|---|
| `Esc` | Volver a modo Normal |
| `Tab` | Salir del campo de texto actual |
| `Ctrl+Z` | Deshacer última edición |
| `Enter` | Insertar nueva línea en el campo |
| Teclas imprimibles | Insertar texto (autocompletado de corchetes y comillas) |

### Soporte de mouse

- **Clic izquierdo**: Selecciona el panel activo (sidebar, URL, headers, cuerpo, respuesta).
- **Hover**: Resalta los elementos interactivos de la interfaz.
- **Arrastre**: Redimensiona el ancho del sidebar y la altura de los paneles de headers/cuerpo/respuesta.
- **Scroll**: Navega el panel de respuesta y la lista del sidebar.

---

## Persistencia

Los datos de la aplicación se guardan en la carpeta `.modjo/` del directorio actual:

```
.modjo/
├── history.json       # Historial de peticiones (máx. 150)
├── collections.json   # Colecciones guardadas
└── env.toml           # Variables de entorno
```

### Variables de entorno

Interpolá variables en URL, headers y cuerpo con la sintaxis `{{nombre_var}}`.

Ejemplo de `.modjo/env.toml`:

```toml
api_url = "https://api.ejemplo.co/v2"
token = "bearer-abc123"
```

Luego usalas así en la URL:

```
{{api_url}}/usuarios?auth={{token}}
```

---

## Estructura del proyecto

```
src/
├── main.rs             # Punto de entrada, ciclo principal y splash screen
├── app.rs              # Estado de la aplicación, modelos y lógica de negocio
├── input.rs            # Manejo de teclado (3 modos) y mouse (hover/clic/scroll/drag)
├── http/
│   ├── mod.rs          # Declaración del módulo HTTP
│   └── client.rs       # Cliente HTTP asíncrono (reqwest + tokio)
├── storage/
│   └── mod.rs          # Persistencia en .modjo/ (JSON/TOML)
└── ui/
    ├── mod.rs          # Renderizado principal de la interfaz
    ├── components.rs   # Componentes de modales y overlays
    ├── splash.rs       # Animación de carga al iniciar
    └── theme.rs        # Sistema de 5 temas
```

## CI/CD

Este proyecto usa GitHub Actions para compilar y probar en cada push y PR contra `main`. Ver [.github/workflows/rust.yml](.github/workflows/rust.yml).

---

## Contribuir

¿Encontraste un bug, tenés una idea o querés meterle mano al código? ¡Bienvenido! Leé [CONTRIBUTING.md](./CONTRIBUTING.md) y el [Código de Conducta](./CODE_OF_CONDUCT.md).

---

## Licencia

modjo es software libre bajo la licencia MIT. Leé el archivo [LICENSE](./LICENSE) para más detalles.

---

Creado con  por [Juan Esteban Manrique Giraldo](https://github.com/jemgdevp) desde Colombia para el mundo.
