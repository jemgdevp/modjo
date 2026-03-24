use std::io;
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{event::{self, KeyCode, Event}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

fn main() -> Result<(), io::Error> {
    // 1. Preparar la terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Bucle principal de la app
    loop {
        terminal.draw(|f| {
            // Aquí dibujaremos la UI más adelante
            let size = f.area();
            let block = ratatui::widgets::Block::default()
                .title(" Modjo")
                .borders(ratatui::widgets::Borders::ALL);
            f.render_widget(block, size);
        })?;

        // 3. Manejo de eventos (Teclado)
        if let Event::Key(key) = event::read()? && key.code == KeyCode::Char('q') {
            break;
        }
    }

    // 4. Restaurar la terminal al salir
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
