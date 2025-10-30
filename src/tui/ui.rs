//! UI rendering for the TUI dashboard
//!
//! This module contains the rendering logic for the 3-panel dashboard.

use chrono::Local;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Sparkline, Table, Tabs},
    Frame,
};

use super::app::Tab;
use super::mock_data::{AgentStatus, JobStatus, MockAgent, MockJob, MockQueueData};

/// Main render function for the dashboard
pub fn render(
    frame: &mut Frame,
    jobs: &[MockJob],
    agents: &[MockAgent],
    queue_data: &MockQueueData,
    current_tab: Tab,
    selected_job: usize,
    selected_agent: usize,
) {
    let size = frame.size();

    // Create main layout with header, tabs, content, and footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    // Render header
    render_header(frame, main_chunks[0]);

    // Render tabs
    render_tabs(frame, main_chunks[1], current_tab);

    // Render content based on current tab
    match current_tab {
        Tab::Jobs => render_jobs_tab(
            frame,
            main_chunks[2],
            jobs,
            agents,
            queue_data,
            selected_job,
        ),
        Tab::Agents => render_agents_tab(frame, main_chunks[2], agents, selected_agent),
        Tab::Config => render_config_tab(frame, main_chunks[2]),
        Tab::Logs => render_logs_tab(frame, main_chunks[2]),
    }

    // Render footer
    render_footer(frame, main_chunks[3]);
}

/// Render the header with title and system info
fn render_header(frame: &mut Frame, area: Rect) {
    let now = Local::now();
    let time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let header = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let header_text = vec![Line::from(vec![
        Span::styled(
            " Raibid CI Dashboard ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled("DGX Spark Agent Pool", Style::default().fg(Color::Gray)),
        Span::raw(" | "),
        Span::styled(time_str, Style::default().fg(Color::Yellow)),
    ])];

    let paragraph = ratatui::widgets::Paragraph::new(header_text)
        .block(header)
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Render the tab bar
fn render_tabs(frame: &mut Frame, area: Rect, current_tab: Tab) {
    let all_tabs = Tab::all();
    let tab_titles: Vec<Line> = all_tabs.iter().map(|t| Line::from(t.as_str())).collect();

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .select(match current_tab {
            Tab::Jobs => 0,
            Tab::Agents => 1,
            Tab::Config => 2,
            Tab::Logs => 3,
        })
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}

/// Render the footer with keybindings
fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let footer_text = vec![Line::from(vec![
        Span::styled(
            "Tab",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("/"),
        Span::styled(
            "Left/Right",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Switch Tab | "),
        Span::styled(
            "1-4",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Jump to Tab | "),
        Span::styled(
            "Up/Down",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Navigate | "),
        Span::styled(
            "q",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Quit"),
    ])];

    let paragraph = Paragraph::new(footer_text)
        .block(footer)
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Render the jobs panel with status table
fn render_jobs_panel(frame: &mut Frame, area: Rect, jobs: &[MockJob]) {
    let block = Block::default()
        .title(format!(" Jobs ({}) ", jobs.len()))
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    // Create table header
    let header = Row::new(vec![
        "ID", "Repo", "Branch", "Status", "Progress", "Duration",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .bottom_margin(1);

    // Create table rows
    let rows: Vec<Row> = jobs
        .iter()
        .map(|job| {
            let status_style = match job.status {
                JobStatus::Success => Style::default().fg(Color::Green),
                JobStatus::Failed => Style::default().fg(Color::Red),
                JobStatus::Running => Style::default().fg(Color::Yellow),
                JobStatus::Pending => Style::default().fg(Color::Gray),
            };

            let progress_bar = if job.progress > 0 {
                format!("{:3}% {}", job.progress, progress_indicator(job.progress))
            } else {
                "  -".to_string()
            };

            let duration_str = if let Some(duration) = job.duration {
                format_duration(duration)
            } else {
                match job.status {
                    JobStatus::Running => {
                        let elapsed = (chrono::Utc::now() - job.start_time).num_seconds() as u64;
                        format_duration(elapsed)
                    }
                    _ => "-".to_string(),
                }
            };

            Row::new(vec![
                Cell::from(job.id.clone()),
                Cell::from(job.repo.clone()),
                Cell::from(job.branch.clone()),
                Cell::from(format!("{} {}", job.status.icon(), job.status.as_str()))
                    .style(status_style),
                Cell::from(progress_bar),
                Cell::from(duration_str),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(10), // ID
        Constraint::Length(20), // Repo
        Constraint::Length(18), // Branch
        Constraint::Length(12), // Status
        Constraint::Length(18), // Progress
        Constraint::Min(8),     // Duration
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .column_spacing(1);

    frame.render_widget(table, area);
}

/// Render the agents panel with agent list and resource usage
fn render_agents_panel(frame: &mut Frame, area: Rect, agents: &[MockAgent]) {
    let block = Block::default()
        .title(format!(" Agents ({}) ", agents.len()))
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let items: Vec<ListItem> = agents
        .iter()
        .map(|agent| {
            let status_style = match agent.status {
                AgentStatus::Idle => Style::default().fg(Color::Green),
                AgentStatus::Busy => Style::default().fg(Color::Yellow),
                AgentStatus::Starting => Style::default().fg(Color::Blue),
                AgentStatus::Stopping => Style::default().fg(Color::Gray),
            };

            let uptime_str = format_uptime(agent.uptime);

            let content = vec![
                Line::from(vec![
                    Span::styled(
                        agent.name.clone(),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" - "),
                    Span::styled(agent.status.as_str(), status_style),
                ]),
                Line::from(vec![
                    Span::raw("  CPU: "),
                    Span::styled(
                        format!("{:3}% ", agent.cpu),
                        Style::default().fg(cpu_color(agent.cpu)),
                    ),
                    Span::raw(resource_bar(agent.cpu)),
                ]),
                Line::from(vec![
                    Span::raw("  MEM: "),
                    Span::styled(
                        format!("{:3}% ", agent.memory),
                        Style::default().fg(memory_color(agent.memory)),
                    ),
                    Span::raw(resource_bar(agent.memory)),
                ]),
                Line::from(vec![
                    Span::raw("  UP:  "),
                    Span::styled(uptime_str, Style::default().fg(Color::Gray)),
                ]),
                Line::from(""), // Empty line for spacing
            ];

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items).block(block);

    frame.render_widget(list, area);
}

/// Render the queue panel with sparkline chart
#[allow(dead_code)]
fn render_queue_panel(frame: &mut Frame, area: Rect, queue_data: &MockQueueData) {
    let _block = Block::default()
        .title(format!(" Queue Depth ({}) ", queue_data.current))
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    // Split the queue panel into info and sparkline sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),     // Info section
            Constraint::Length(10), // Sparkline chart
        ])
        .split(area);

    // Render queue info
    let max_depth = *queue_data.history.iter().max().unwrap_or(&0);
    let avg_depth = if !queue_data.history.is_empty() {
        queue_data.history.iter().sum::<u64>() / queue_data.history.len() as u64
    } else {
        0
    };

    let info_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Current: "),
            Span::styled(
                queue_data.current.to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Max:     "),
            Span::styled(max_depth.to_string(), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("  Average: "),
            Span::styled(avg_depth.to_string(), Style::default().fg(Color::Cyan)),
        ]),
    ];

    let info_paragraph = ratatui::widgets::Paragraph::new(info_text).block(Block::default());

    frame.render_widget(info_paragraph, chunks[0]);

    // Render sparkline
    let sparkline_block = Block::default()
        .title(" History (60s) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));

    let sparkline = Sparkline::default()
        .block(sparkline_block)
        .data(&queue_data.history)
        .style(Style::default().fg(Color::Cyan))
        .max(30); // Max queue depth for scaling

    frame.render_widget(sparkline, chunks[1]);
}

/// Generate a simple progress indicator
fn progress_indicator(progress: u8) -> String {
    let filled = (progress / 10) as usize;
    let empty = 10 - filled;
    format!("[{}{}]", "█".repeat(filled), " ".repeat(empty))
}

/// Generate a simple resource usage bar
fn resource_bar(percentage: u8) -> String {
    let filled = (percentage / 10) as usize;
    let empty = 10 - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

/// Get color for CPU usage
fn cpu_color(cpu: u8) -> Color {
    match cpu {
        0..=50 => Color::Green,
        51..=80 => Color::Yellow,
        _ => Color::Red,
    }
}

/// Get color for memory usage
fn memory_color(memory: u8) -> Color {
    match memory {
        0..=60 => Color::Green,
        61..=85 => Color::Yellow,
        _ => Color::Red,
    }
}

/// Format uptime in human-readable form
fn format_uptime(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {:02}m {:02}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {:02}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

/// Format duration in human-readable form (same as uptime but different context)
fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {:02}m {:02}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {:02}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

/// Render the Jobs tab (detailed view)
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
fn render_jobs_tab(
    frame: &mut Frame,
    area: Rect,
    jobs: &[MockJob],
    agents: &[MockAgent],
    queue_data: &MockQueueData,
    _selected: usize,
) {
    // Create 3-panel layout for content
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Jobs panel
            Constraint::Percentage(20), // Agents panel
            Constraint::Percentage(20), // Queue panel
        ])
        .split(area);

    // Render panels
    render_jobs_panel(frame, content_chunks[0], jobs);
    render_agents_panel(frame, content_chunks[1], agents);
    render_queue_panel(frame, content_chunks[2], queue_data);
}

/// Render the Agents tab (detailed view)
#[allow(dead_code)]
fn render_agents_tab(frame: &mut Frame, area: Rect, agents: &[MockAgent], _selected: usize) {
    // For now, delegate to the agents panel implementation
    render_agents_panel(frame, area, agents);
}

/// Render the Config tab (placeholder)
#[allow(dead_code)]
fn render_config_tab(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Configuration ")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let config_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "API Server:  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("http://localhost:8080"),
        ]),
        Line::from(vec![
            Span::styled(
                "Gitea URL:   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("http://gitea.local:3000"),
        ]),
        Line::from(vec![
            Span::styled(
                "Redis URL:   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("redis://localhost:6379"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Job Queue:   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("raibid:jobs"),
        ]),
        Line::from(vec![
            Span::styled(
                "Max Agents:  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("10"),
        ]),
        Line::from(vec![
            Span::styled(
                "Scale Down:  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("300s idle"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Platform:    ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("NVIDIA DGX Spark (ARM64)"),
        ]),
        Line::from(vec![
            Span::styled(
                "CPU Cores:   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("20 (10x Cortex-X925, 10x Cortex-A725)"),
        ]),
        Line::from(vec![
            Span::styled(
                "Memory:      ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("128GB LPDDR5x"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Note: Configuration editing coming in future release",
            Style::default().fg(Color::Gray),
        )]),
    ];

    let paragraph = Paragraph::new(config_text)
        .block(block)
        .alignment(ratatui::layout::Alignment::Left);

    frame.render_widget(paragraph, area);
}

/// Render the Logs tab (placeholder)
#[allow(dead_code)]
fn render_logs_tab(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" System Logs ")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let log_entries = vec![
        Line::from(vec![
            Span::styled("[2025-10-29 14:32:15]", Style::default().fg(Color::Gray)),
            Span::raw(" "),
            Span::styled("INFO", Style::default().fg(Color::Green)),
            Span::raw("  Agent dgx-agent-001 started successfully"),
        ]),
        Line::from(vec![
            Span::styled("[2025-10-29 14:31:58]", Style::default().fg(Color::Gray)),
            Span::raw(" "),
            Span::styled("INFO", Style::default().fg(Color::Green)),
            Span::raw("  Job job-4523 completed successfully"),
        ]),
        Line::from(vec![
            Span::styled("[2025-10-29 14:31:42]", Style::default().fg(Color::Gray)),
            Span::raw(" "),
            Span::styled("WARN", Style::default().fg(Color::Yellow)),
            Span::raw("  High queue depth detected: 15 jobs pending"),
        ]),
        Line::from(vec![
            Span::styled("[2025-10-29 14:31:20]", Style::default().fg(Color::Gray)),
            Span::raw(" "),
            Span::styled("INFO", Style::default().fg(Color::Green)),
            Span::raw("  Job job-4521 dispatched to agent dgx-agent-002"),
        ]),
        Line::from(vec![
            Span::styled("[2025-10-29 14:30:55]", Style::default().fg(Color::Gray)),
            Span::raw(" "),
            Span::styled("ERROR", Style::default().fg(Color::Red)),
            Span::raw(" Job job-4518 failed: build timeout exceeded"),
        ]),
        Line::from(vec![
            Span::styled("[2025-10-29 14:30:33]", Style::default().fg(Color::Gray)),
            Span::raw(" "),
            Span::styled("INFO", Style::default().fg(Color::Green)),
            Span::raw("  KEDA autoscaler triggered: scaling to 5 agents"),
        ]),
        Line::from(vec![
            Span::styled("[2025-10-29 14:30:10]", Style::default().fg(Color::Gray)),
            Span::raw(" "),
            Span::styled("INFO", Style::default().fg(Color::Green)),
            Span::raw("  Redis connection pool initialized"),
        ]),
        Line::from(vec![
            Span::styled("[2025-10-29 14:29:55]", Style::default().fg(Color::Gray)),
            Span::raw(" "),
            Span::styled("INFO", Style::default().fg(Color::Green)),
            Span::raw("  Flux sync completed: 3 deployments updated"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Note: Real-time log streaming coming in future release",
            Style::default().fg(Color::Gray),
        )]),
    ];

    let paragraph = Paragraph::new(log_entries)
        .block(block)
        .alignment(ratatui::layout::Alignment::Left);

    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_indicator() {
        assert_eq!(progress_indicator(0), "[          ]");
        assert_eq!(progress_indicator(50), "[█████     ]");
        assert_eq!(progress_indicator(100), "[██████████]");
    }

    #[test]
    fn test_resource_bar() {
        assert_eq!(resource_bar(0), "[░░░░░░░░░░]");
        assert_eq!(resource_bar(50), "[█████░░░░░]");
        assert_eq!(resource_bar(100), "[██████████]");
    }

    #[test]
    fn test_cpu_color() {
        assert_eq!(cpu_color(30), Color::Green);
        assert_eq!(cpu_color(70), Color::Yellow);
        assert_eq!(cpu_color(90), Color::Red);
    }

    #[test]
    fn test_memory_color() {
        assert_eq!(memory_color(30), Color::Green);
        assert_eq!(memory_color(70), Color::Yellow);
        assert_eq!(memory_color(90), Color::Red);
    }

    #[test]
    fn test_format_uptime() {
        assert_eq!(format_uptime(30), "30s");
        assert_eq!(format_uptime(90), "1m 30s");
        assert_eq!(format_uptime(3661), "1h 01m 01s");
    }
}
