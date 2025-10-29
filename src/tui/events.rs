//! Event types for TUI application

/// Application-level events
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// Key press event
    KeyPress(char),

    /// Quit event
    Quit,

    /// Refresh data
    Refresh,

    /// Resize event
    Resize(u16, u16),
}
