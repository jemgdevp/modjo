# Contribuir a modjo

¡De una, bienvenido! Valoramos un montón las contribuciones de la comunidad. Ya sea que estés corrigiendo un bug, agregando una funcionalidad nueva o mejorando la documentación, tu ayuda es clave.

Seguí estas instrucciones para contribuir de forma efectiva.

---

## Cómo contribuir

### 1. Hacé un fork del repositorio

Dale clic al botón **Fork** arriba a la derecha para crear una copia del proyecto en tu cuenta de GitHub.

### 2. Cloná tu fork

```bash
git clone https://github.com/TU_USUARIO/modjo.git
cd modjo
```

### 3. Creá una rama

Creá una rama nueva para tu cambio. Usá un nombre descriptivo:

```bash
git checkout -b feature/nombre-de-tu-funcionalidad
# o
git checkout -b fix/descripcion-del-bug
```

### 4. Hacé tus cambios

Implementá tus cambios en el código. Asegurate de:

- Seguir el estilo y las convenciones que ya usa el proyecto.
- No introducir warnings al compilar.
- Ejecutar `cargo build` para verificar que todo compile.
- Si tu cambio lo amerita, ejecutá `cargo test` para validar que nada se rompió.

### 5. Confirmá tus cambios

Hacé commits con mensajes descriptivos en español o inglés:

```bash
git add .
git commit -m "feat: agregar soporte para autenticación OAuth2"
```

Usá el estándar [Conventional Commits](https://www.conventionalcommits.org/) cuando sea posible:
- `feat:` para funcionalidades nuevas
- `fix:` para correcciones de bugs
- `docs:` para cambios en documentación
- `refactor:` para reestructuración de código
- `style:` para cambios de formato
- `test:` para pruebas

### 6. Subí tus cambios

```bash
git push origin feature/nombre-de-tu-funcionalidad
```

### 7. Creá un Pull Request

Andá al repositorio original en GitHub y hacé clic en **New Pull Request**. Describí claramente:

- Qué problema resuelve tu cambio.
- Cómo lo resolviste.
- Cualquier consideración especial para revisar.

---

## Antes de enviar tu PR

- [ ] `cargo build` compila sin errores.
- [ ] `cargo test` pasa todas las pruebas.
- [ ] No dejaste código comentado ni `println!` de depuración.
- [ ] Si tocaste la UI, verificaste que se ve bien en una terminal de 80x24 o más.
- [ ] Si agregaste dependencias nuevas, están justificadas en la descripción del PR.

---

## Reportar bugs

Si encontraste un bug, abrí un [issue](https://github.com/jemgdevp/modjo/issues) con:

- Versión de modjo (`modjo --version`).
- Sistema operativo y versión.
- Terminal que estás usando.
- Pasos para reproducir el bug.
- Comportamiento esperado vs. lo que pasó.

---

## Código de Conducta

Este proyecto se rige por nuestro [Código de Conducta](./CODE_OF_CONDUCT.md). Al participar, te comprometés a respetarlo. No se aceptan comportamientos que vayan en contra de un ambiente sano y respetuoso.

---

¡Gracias por contribuir a modjo! 
