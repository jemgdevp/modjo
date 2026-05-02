# Registro de cambios

Todos los cambios notables de modjo se documentan en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-419/1.1.0/) y este proyecto adhiere a [Versionado Semántico](https://semver.org/lang/es/).

---

## [0.0.1] — MVP Inicial

### Agregado
- Interfaz TUI con `ratatui` + `crossterm`.
- Modos de edición Normal/Inserción al estilo Vim.
- Envío de peticiones HTTP asíncronas (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS) mediante `reqwest` + `tokio`.
- Selector de método HTTP integrado.
- Paneles para URL, headers, cuerpo de la petición y respuesta.
- Resaltado de sintaxis JSON en el panel de respuesta (claves, strings, números, booleanos, null).
- Alternancia entre formato JSON pretty-print y raw.
- Sidebar con pestañas de historial (máx. 150 entradas) y colecciones guardadas.
- Persistencia local en carpeta `.modjo/` con archivos JSON y TOML.
- Variables de entorno con interpolación `{{nombre_var}}` desde `.modjo/env.toml`.
- Cinco temas: OC-2 Dark, OC-2 Light, Catppuccin, Nord, Dracula.
- Soporte de idioma dual: español colombiano e inglés.
- Soporte completo para mouse: clic para enfocar, hover para resaltar, arrastre para redimensionar paneles, scroll para navegar.
- Paleta de comandos (`Ctrl+P`) con todas las acciones disponibles.
- Portapapeles interno con yank/paste (`y`/`p`).
- Pila de deshacer con hasta 100 entradas (`u` / `Ctrl+Z`).
- Exportación e importación de snapshots (`modjo-export.json` / `modjo-import.json`).
- Notificaciones toast con auto-desvanecimiento.
- Animación de portapapeles (snip/woosh) al copiar.
- Splash screen animada al iniciar.
- Reporte de errores con `color-eyre`.
- CI/CD con GitHub Actions (build + test en push/PR a `main`).
- Documentación: README, CONTRIBUTING, CODE_OF_CONDUCT, CHANGELOG y SECURITY.
