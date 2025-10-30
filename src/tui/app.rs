//! Application state and event loop
//!
//! This module contains the main application state and event handling logic.

use anyhow::Result;
use std::time::Duration;

use super::events::{is_quit_event, Event, EventHandler};
use super::mock_data::{generate_mock_data, MockAgent, MockDataConfig, MockJob, MockQueueData};
use super::terminal::Terminal;
use super::ui;

/// Available tabs in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Jobs,
    Agents,
    Config,
    Logs,
}

impl Tab {
    /// Get tab name as string
    pub fn as_str(&self) -> &str {
        match self {
            Tab::Jobs => "Jobs",
            Tab::Agents => "Agents",
            Tab::Config => "Config",
            Tab::Logs => "Logs",
        }
    }

    /// Get all tabs
    pub fn all() -> Vec<Tab> {
        vec![Tab::Jobs, Tab::Agents, Tab::Config, Tab::Logs]
    }

    /// Get next tab
    pub fn next(&self) -> Tab {
        match self {
            Tab::Jobs => Tab::Agents,
            Tab::Agents => Tab::Config,
            Tab::Config => Tab::Logs,
            Tab::Logs => Tab::Jobs,
        }
    }

    /// Get previous tab
    pub fn previous(&self) -> Tab {
        match self {
            Tab::Jobs => Tab::Logs,
            Tab::Agents => Tab::Jobs,
            Tab::Config => Tab::Agents,
            Tab::Logs => Tab::Config,
        }
    }
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Refresh interval for updating data
    pub refresh_interval: Duration,
    /// Panel proportions (jobs, agents, queue) - percentages that sum to 100
    pub panel_proportions: (u16, u16, u16),
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_secs(1),
            panel_proportions: (60, 20, 20),
        }
    }
}

/// Main application state
pub struct App {
    /// Application configuration
    config: AppConfig,
    /// Mock data configuration
    mock_config: MockDataConfig,
    /// Current job list
    jobs: Vec<MockJob>,
    /// Current agent list
    agents: Vec<MockAgent>,
    /// Queue depth data
    queue_data: MockQueueData,
    /// Whether the application should quit
    should_quit: bool,
    /// Current active tab
    current_tab: Tab,
    /// Selected job index (for scrolling/selection)
    selected_job: usize,
    /// Selected agent index (for scrolling/selection)
    selected_agent: usize,
}

impl App {
    /// Create a new application with default configuration
    pub fn new() -> Self {
        Self::with_config(AppConfig::default())
    }

    /// Create a new application with custom configuration
    pub fn with_config(config: AppConfig) -> Self {
        let mock_config = MockDataConfig::default();
        let (jobs, agents, queue_data) = generate_mock_data(&mock_config);

        Self {
            config,
            mock_config,
            jobs,
            agents,
            queue_data,
            should_quit: false,
            current_tab: Tab::Jobs,
            selected_job: 0,
            selected_agent: 0,
        }
    }

    /// Get current tab
    pub fn current_tab(&self) -> Tab {
        self.current_tab
    }

    /// Switch to next tab
    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    /// Switch to previous tab
    pub fn previous_tab(&mut self) {
        self.current_tab = self.current_tab.previous();
    }

    /// Get selected job index
    pub fn selected_job(&self) -> usize {
        self.selected_job
    }

    /// Get selected agent index
    pub fn selected_agent(&self) -> usize {
        self.selected_agent
    }

    /// Move selection up
    pub fn select_previous(&mut self) {
        match self.current_tab {
            Tab::Jobs => {
                if self.selected_job > 0 {
                    self.selected_job -= 1;
                }
            }
            Tab::Agents => {
                if self.selected_agent > 0 {
                    self.selected_agent -= 1;
                }
            }
            _ => {}
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        match self.current_tab {
            Tab::Jobs => {
                if self.selected_job < self.jobs.len().saturating_sub(1) {
                    self.selected_job += 1;
                }
            }
            Tab::Agents => {
                if self.selected_agent < self.agents.len().saturating_sub(1) {
                    self.selected_agent += 1;
                }
            }
            _ => {}
        }
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Request the application to quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Update application state (refresh mock data)
    pub fn update(&mut self) {
        // Regenerate mock data to simulate changes
        let (jobs, agents, _) = generate_mock_data(&self.mock_config);
        self.jobs = jobs;
        self.agents = agents;

        // Update queue data incrementally
        let mut rng = rand::thread_rng();
        self.queue_data.update(&mut rng);
    }

    /// Handle an event
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => {
                if is_quit_event(&key) {
                    self.quit();
                } else {
                    use crossterm::event::KeyCode;
                    match key.code {
                        KeyCode::Tab => self.next_tab(),
                        KeyCode::BackTab => self.previous_tab(),
                        KeyCode::Right => self.next_tab(),
                        KeyCode::Left => self.previous_tab(),
                        KeyCode::Up => self.select_previous(),
                        KeyCode::Down => self.select_next(),
                        KeyCode::Char('1') => self.current_tab = Tab::Jobs,
                        KeyCode::Char('2') => self.current_tab = Tab::Agents,
                        KeyCode::Char('3') => self.current_tab = Tab::Config,
                        KeyCode::Char('4') => self.current_tab = Tab::Logs,
                        _ => {}
                    }
                }
            }
            Event::Resize(_, _) => {
                // Terminal resize is handled automatically by ratatui
            }
            Event::Tick => {
                // Auto-refresh on tick
                self.update();
            }
        }
    }

    /// Run the main event loop
    pub fn run(&mut self, terminal: &mut Terminal) -> Result<()> {
        let event_handler = EventHandler::new(self.config.refresh_interval);

        while !self.should_quit() {
            // Render the UI
            terminal.draw(|frame| {
                ui::render(
                    frame,
                    &self.jobs,
                    &self.agents,
                    &self.queue_data,
                    self.current_tab,
                    self.selected_job,
                    self.selected_agent,
                );
            })?;

            // Handle events
            let event = event_handler.next()?;
            self.handle_event(event);
        }

        Ok(())
    }

    /// Get jobs reference (for testing and integration tests)
    pub fn jobs(&self) -> &[MockJob] {
        &self.jobs
    }

    /// Get agents reference (for testing and integration tests)
    pub fn agents(&self) -> &[MockAgent] {
        &self.agents
    }

    /// Get queue data reference (for testing and integration tests)
    pub fn queue_data(&self) -> &MockQueueData {
        &self.queue_data
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert!(!app.should_quit());
        assert!(!app.jobs().is_empty());
        assert!(!app.agents().is_empty());
        assert_eq!(app.queue_data().history.len(), 60);
    }

    #[test]
    fn test_app_quit() {
        let mut app = App::new();
        assert!(!app.should_quit());

        app.quit();
        assert!(app.should_quit());
    }

    #[test]
    fn test_app_update() {
        let mut app = App::new();
        let initial_jobs = app.jobs().len();
        let initial_agents = app.agents().len();

        app.update();

        // Should still have the same count of jobs and agents
        assert_eq!(app.jobs().len(), initial_jobs);
        assert_eq!(app.agents().len(), initial_agents);
    }

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.refresh_interval, Duration::from_secs(1));
        assert_eq!(config.panel_proportions, (60, 20, 20));
    }

    #[test]
    fn test_app_with_custom_config() {
        let config = AppConfig {
            refresh_interval: Duration::from_millis(500),
            panel_proportions: (70, 15, 15),
        };

        let app = App::with_config(config.clone());
        assert_eq!(app.config.refresh_interval, config.refresh_interval);
        assert_eq!(app.config.panel_proportions, config.panel_proportions);
    }

    #[test]
    fn test_handle_quit_event() {
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

        let mut app = App::new();

        // Test 'q' key
        let quit_event = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        app.handle_event(quit_event);
        assert!(app.should_quit());
    }

    #[test]
    fn test_handle_tick_event() {
        let mut app = App::new();
        let initial_queue_len = app.queue_data().history.len();

        app.handle_event(Event::Tick);

        // Should maintain queue history length
        assert_eq!(app.queue_data().history.len(), initial_queue_len);
    }
}
