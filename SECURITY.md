# Seguridad

## Reporte de vulnerabilidades

Tomamos la seguridad de modjo muy en serio. Si descubrís una vulnerabilidad, te agradecemos que la reportés de forma responsable para que podamos corregirla antes de que sea divulgada públicamente.

**No abras un issue público** para reportar vulnerabilidades de seguridad.

En su lugar, escribinos un correo a:

**murksopps@gmail.com**

Incluí en tu reporte:

- Una descripción clara del problema y el impacto potencial.
- Pasos para reproducir la vulnerabilidad.
- Versión de modjo afectada.
- Cualquier sugerencia para la solución (opcional).

---

## Tiempo de respuesta

Nos comprometemos a:

- Acusar recibo de tu reporte dentro de los **3 días hábiles**.
- Mantenerte informado del progreso hacia la solución.
- Notificarte cuando la vulnerabilidad haya sido corregida.

---

## Alcance

Este documento aplica al núcleo de modjo (la aplicación TUI) y sus dependencias directas. Vulnerabilidades en dependencias de terceros deben reportarse primero a los proyectos correspondientes.

---

## Versiones soportadas

| Versión | Soporte |
|---|---|
| 0.0.1 (actual) | Reportes de seguridad aceptados |

---

## Mejores prácticas al usar modjo

- **Variables de entorno**: No guardés tokens o secretos en `.modjo/env.toml` si el directorio está versionado con git. Agregá `.modjo/` a tu `.gitignore` si manejás datos sensibles.
- **Snapshots de exportación**: El archivo `modjo-export.json` puede contener headers y cuerpos de peticiones. Revisá su contenido antes de compartirlo.
- **HTTPS**: modjo no impone HTTPS, pero siempre preferí usar URLs con `https://` en producción.
- **Cuerpo de peticiones**: No compartas capturas de pantalla de la terminal que muestren tokens, contraseñas o datos sensibles en los paneles de headers o cuerpo.

---

## Divulgación responsable

Una vez corregida la vulnerabilidad, se publicará un aviso en la sección de [Releases](https://github.com/jemgdevp/modjo/releases) incluyendo:

- Descripción del problema.
- Versiones afectadas.
- Solución aplicada.
- Crédito al reportante (si así lo desea).
