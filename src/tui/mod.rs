//! TUI (Terminal User Interface) module
//!
//! This module provides the Ratatui-based terminal UI for monitoring
//! CI jobs, agents, and queue status.

pub mod app;
pub mod events;
pub mod mock_data;
pub mod terminal;
pub mod ui;

pub use app::App;
pub use terminal::Terminal;

use anyhow::Result;

/// Run the TUI application
pub fn run() -> Result<()> {
    // Initialize terminal
    let mut terminal = Terminal::new()?;

    // Create app state
    let mut app = App::new();

    // Run the event loop
    app.run(&mut terminal)?;

    Ok(())
}
