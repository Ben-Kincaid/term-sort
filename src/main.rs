use crate::app::{App, View};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    backtrace::Backtrace,
    io,
    panic::{self, PanicInfo},
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    Terminal,
};

pub mod app;
pub mod handlers;
pub mod sort;
pub mod ui;

fn setup_terminal(stdout: &mut io::Stdout) -> Result<(), io::Error> {
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(())
}

fn cleanup_terminal() -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn handle_panic(info: &PanicInfo<'_>) -> Result<(), io::Error> {
    let msg = info.payload().downcast_ref::<&'static str>().unwrap();
    cleanup_terminal()?;
    println!("{}", *msg);
    println!("{:?}", Backtrace::capture());
    Ok(())
}

fn ui() -> Result<(), io::Error> {
    // Cleanup terminal on panic
    panic::set_hook(Box::new(|info| {
        handle_panic(info).unwrap();
    }));

    // Setup terminal
    let mut stdout = io::stdout();
    setup_terminal(&mut stdout)?;

    // Create backend/terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize the application
    let mut app = App::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(12);

    // Draw loop
    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let w = std::cmp::min(size.width, 100);
            let h = std::cmp::min(size.height, 30);
            let viewport = Rect {
                width: w,
                height: h,
                x: (size.width - w) / 2,
                y: (size.height - h) / 2,
                ..size
            };

            app.ui_width = viewport.width / 2 - 3;
            let current_view = app.current_view();
            match current_view {
                View::Menu => ui::draw_menu(&mut f, &mut app, viewport),
                _ => ui::draw_single_sort(&mut f, &mut app, viewport),
            }
        })?;

        // Handle user input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key {
                    event::KeyEvent {
                        code: event::KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    } => {
                        break;
                    }
                    _ if key.code == KeyCode::Char('q') => match app.current_view {
                        app::View::Menu => break,
                        _ => app.set_current_view(app::View::Menu),
                    },
                    _ => app.handle_input(key)?,
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            if let Some(sort) = &mut app.sort {
                if sort.is_active() {
                    sort.step();
                }
            }
            last_tick = Instant::now();
        }
    }

    cleanup_terminal()?;
    terminal.show_cursor()?;

    Ok(())
}

fn main() -> Result<(), io::Error> {
    ui()
}
