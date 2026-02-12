#![allow(dead_code)]

mod ai;
mod app;
mod data;
mod game;
mod input;
mod modes;
mod ui;

use std::io;
use std::panic;
use std::time::{Duration, Instant};

use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::App;

const TARGET_FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS);

fn main() -> io::Result<()> {
    // Set up panic hook to restore terminal on crash
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        original_hook(info);
    }));

    // Check if the terminal supports keyboard enhancement (key release events)
    let has_key_release = crossterm::terminal::supports_keyboard_enhancement().unwrap_or(false);

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    if has_key_release {
        execute!(
            stdout,
            EnterAlternateScreen,
            EnableMouseCapture,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
        )?;
    } else {
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    }
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    // Run the app
    let result = run_app(&mut terminal, has_key_release);

    // Restore terminal
    restore_terminal()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    has_key_release: bool,
) -> io::Result<()> {
    let mut app = App::new(has_key_release);
    let mut last_frame = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now.duration_since(last_frame);
        last_frame = now;

        // Cap dt to prevent huge jumps (e.g., after a debugger pause)
        let dt = dt.min(Duration::from_millis(100));

        // Update app state
        if !app.update(dt) {
            break;
        }

        // Render
        terminal.draw(|frame| {
            let area = frame.area();
            app.render(area, frame.buffer_mut());
        })?;

        // Frame timing
        let elapsed = now.elapsed();
        if elapsed < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - elapsed);
        }
    }

    Ok(())
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    // PopKeyboardEnhancementFlags is safe to call even if we didn't push;
    // it will just be ignored by terminals that don't support it.
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        PopKeyboardEnhancementFlags
    )?;
    Ok(())
}
