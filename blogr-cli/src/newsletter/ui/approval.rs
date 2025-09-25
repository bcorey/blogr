//! Newsletter subscriber approval UI using Ratatui

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, TableState, Wrap},
    Frame,
};
use std::collections::HashSet;

use crate::newsletter::{NewsletterDatabase, Subscriber, SubscriberStatus};

pub type AppResult<T> = anyhow::Result<T>;

#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalResult {
    Continue,
    Quit,
    #[allow(dead_code)]
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalMode {
    List,
    Help,
    Confirm,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfirmAction {
    Approve,
    Decline,
    Delete,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubscriberFilter {
    All,
    Pending,
    Approved,
    Declined,
}

impl std::fmt::Display for SubscriberFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriberFilter::All => write!(f, "All"),
            SubscriberFilter::Pending => write!(f, "Pending"),
            SubscriberFilter::Approved => write!(f, "Approved"),
            SubscriberFilter::Declined => write!(f, "Declined"),
        }
    }
}

impl SubscriberFilter {
    #[allow(dead_code)]
    fn to_status(&self) -> Option<SubscriberStatus> {
        match self {
            SubscriberFilter::All => None,
            SubscriberFilter::Pending => Some(SubscriberStatus::Pending),
            SubscriberFilter::Approved => Some(SubscriberStatus::Approved),
            SubscriberFilter::Declined => Some(SubscriberStatus::Declined),
        }
    }
}

pub struct ApprovalApp {
    /// Whether the app should continue running
    pub running: bool,
    /// Current mode
    pub mode: ApprovalMode,
    /// Current filter
    pub filter: SubscriberFilter,
    /// All subscribers
    pub subscribers: Vec<Subscriber>,
    /// Filtered subscribers based on current filter
    pub filtered_subscribers: Vec<usize>, // indices into subscribers
    /// Current table state
    pub table_state: TableState,
    /// Selected subscriber indices (for bulk operations)
    pub selected: HashSet<usize>,
    /// Search query
    pub search_query: String,
    /// Status message
    pub status_message: Option<String>,
    /// Confirmation action
    pub confirm_action: Option<ConfirmAction>,
    /// Database reference
    database: NewsletterDatabase,
}

impl ApprovalApp {
    pub fn new(database: NewsletterDatabase) -> Result<Self> {
        let subscribers = database.get_subscribers(None)?;
        let filtered_subscribers: Vec<usize> = (0..subscribers.len()).collect();

        let mut app = Self {
            running: true,
            mode: ApprovalMode::List,
            filter: SubscriberFilter::Pending, // Start with pending by default
            subscribers,
            filtered_subscribers,
            table_state: TableState::default(),
            selected: HashSet::new(),
            search_query: String::new(),
            status_message: None,
            confirm_action: None,
            database,
        };

        app.apply_filter()?;
        app.select_first();

        Ok(app)
    }

    /// Handle key events
    pub fn handle_key_event(&mut self, key: KeyEvent) -> AppResult<ApprovalResult> {
        match self.mode {
            ApprovalMode::List => self.handle_list_key_event(key),
            ApprovalMode::Help => self.handle_help_key_event(key),
            ApprovalMode::Confirm => self.handle_confirm_key_event(key),
        }
    }

    fn handle_list_key_event(&mut self, key: KeyEvent) -> AppResult<ApprovalResult> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
                Ok(ApprovalResult::Quit)
            }
            KeyCode::Char('h') | KeyCode::F(1) => {
                self.mode = ApprovalMode::Help;
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('r') => {
                self.refresh_data()?;
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.previous_subscriber();
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.next_subscriber();
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char(' ') => {
                self.toggle_selection();
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('a') => {
                self.select_all();
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('n') => {
                self.clear_selection();
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('A') => {
                if !self.selected.is_empty() {
                    self.confirm_action = Some(ConfirmAction::Approve);
                    self.mode = ApprovalMode::Confirm;
                } else if let Some(current) = self.current_subscriber_index() {
                    self.selected.insert(current);
                    self.confirm_action = Some(ConfirmAction::Approve);
                    self.mode = ApprovalMode::Confirm;
                }
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('D') => {
                if !self.selected.is_empty() {
                    self.confirm_action = Some(ConfirmAction::Decline);
                    self.mode = ApprovalMode::Confirm;
                } else if let Some(current) = self.current_subscriber_index() {
                    self.selected.insert(current);
                    self.confirm_action = Some(ConfirmAction::Decline);
                    self.mode = ApprovalMode::Confirm;
                }
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('X') => {
                if !self.selected.is_empty() {
                    self.confirm_action = Some(ConfirmAction::Delete);
                    self.mode = ApprovalMode::Confirm;
                } else if let Some(current) = self.current_subscriber_index() {
                    self.selected.insert(current);
                    self.confirm_action = Some(ConfirmAction::Delete);
                    self.mode = ApprovalMode::Confirm;
                }
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('1') => {
                self.filter = SubscriberFilter::All;
                self.apply_filter()?;
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('2') => {
                self.filter = SubscriberFilter::Pending;
                self.apply_filter()?;
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('3') => {
                self.filter = SubscriberFilter::Approved;
                self.apply_filter()?;
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('4') => {
                self.filter = SubscriberFilter::Declined;
                self.apply_filter()?;
                Ok(ApprovalResult::Continue)
            }
            _ => Ok(ApprovalResult::Continue),
        }
    }

    fn handle_help_key_event(&mut self, key: KeyEvent) -> AppResult<ApprovalResult> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('h') | KeyCode::F(1) => {
                self.mode = ApprovalMode::List;
                Ok(ApprovalResult::Continue)
            }
            _ => Ok(ApprovalResult::Continue),
        }
    }

    fn handle_confirm_key_event(&mut self, key: KeyEvent) -> AppResult<ApprovalResult> {
        match key.code {
            KeyCode::Char('y') | KeyCode::Enter => {
                self.execute_confirm_action()?;
                self.mode = ApprovalMode::List;
                Ok(ApprovalResult::Continue)
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.confirm_action = None;
                self.mode = ApprovalMode::List;
                Ok(ApprovalResult::Continue)
            }
            _ => Ok(ApprovalResult::Continue),
        }
    }

    fn execute_confirm_action(&mut self) -> Result<()> {
        if let Some(action) = &self.confirm_action {
            let selected_indices: Vec<usize> = self.selected.iter().cloned().collect();
            let count = selected_indices.len();

            match action {
                ConfirmAction::Approve => {
                    for &index in &selected_indices {
                        if let Some(subscriber) = self.subscribers.get(index) {
                            if let Some(id) = subscriber.id {
                                self.database
                                    .update_subscriber_status(id, SubscriberStatus::Approved)?;
                            }
                        }
                    }
                    self.status_message = Some(format!("Approved {} subscriber(s)", count));
                }
                ConfirmAction::Decline => {
                    for &index in &selected_indices {
                        if let Some(subscriber) = self.subscribers.get(index) {
                            if let Some(id) = subscriber.id {
                                self.database
                                    .update_subscriber_status(id, SubscriberStatus::Declined)?;
                            }
                        }
                    }
                    self.status_message = Some(format!("Declined {} subscriber(s)", count));
                }
                ConfirmAction::Delete => {
                    for &index in &selected_indices {
                        if let Some(subscriber) = self.subscribers.get(index) {
                            self.database.remove_subscriber(&subscriber.email)?;
                        }
                    }
                    self.status_message = Some(format!("Deleted {} subscriber(s)", count));
                }
            }

            self.selected.clear();
            self.confirm_action = None;
            self.refresh_data()?;
        }

        Ok(())
    }

    fn refresh_data(&mut self) -> Result<()> {
        self.subscribers = self.database.get_subscribers(None)?;
        self.apply_filter()?;
        Ok(())
    }

    fn apply_filter(&mut self) -> Result<()> {
        self.filtered_subscribers.clear();

        for (index, subscriber) in self.subscribers.iter().enumerate() {
            let matches_filter = match self.filter {
                SubscriberFilter::All => true,
                SubscriberFilter::Pending => subscriber.status == SubscriberStatus::Pending,
                SubscriberFilter::Approved => subscriber.status == SubscriberStatus::Approved,
                SubscriberFilter::Declined => subscriber.status == SubscriberStatus::Declined,
            };

            let matches_search = if self.search_query.is_empty() {
                true
            } else {
                subscriber
                    .email
                    .to_lowercase()
                    .contains(&self.search_query.to_lowercase())
            };

            if matches_filter && matches_search {
                self.filtered_subscribers.push(index);
            }
        }

        // Reset table state if needed
        if self.filtered_subscribers.is_empty() {
            self.table_state.select(None);
        } else if let Some(selected) = self.table_state.selected() {
            if selected >= self.filtered_subscribers.len() {
                self.table_state.select(Some(0));
            }
        }

        Ok(())
    }

    fn select_first(&mut self) {
        if !self.filtered_subscribers.is_empty() {
            self.table_state.select(Some(0));
        }
    }

    fn previous_subscriber(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_subscribers.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        if !self.filtered_subscribers.is_empty() {
            self.table_state.select(Some(i));
        }
    }

    fn next_subscriber(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.filtered_subscribers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.filtered_subscribers.is_empty() {
            self.table_state.select(Some(i));
        }
    }

    fn current_subscriber_index(&self) -> Option<usize> {
        self.table_state
            .selected()
            .and_then(|i| self.filtered_subscribers.get(i).copied())
    }

    fn toggle_selection(&mut self) {
        if let Some(index) = self.current_subscriber_index() {
            if self.selected.contains(&index) {
                self.selected.remove(&index);
            } else {
                self.selected.insert(index);
            }
        }
    }

    fn select_all(&mut self) {
        for &index in &self.filtered_subscribers {
            self.selected.insert(index);
        }
    }

    fn clear_selection(&mut self) {
        self.selected.clear();
    }

    /// Render the approval interface
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.size();
        match self.mode {
            ApprovalMode::List => self.render_list(frame),
            ApprovalMode::Help => self.render_help(frame, area),
            ApprovalMode::Confirm => self.render_confirm(frame, area),
        }
    }

    fn render_list(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(1),    // Table
                Constraint::Length(3), // Status bar
            ])
            .split(frame.size());

        // Header
        self.render_header(frame, chunks[0]);

        // Table
        self.render_table(frame, chunks[1]);

        // Status bar
        self.render_status_bar(frame, chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Title and filter
        let title_text = format!("Newsletter Subscribers - Filter: {}", self.filter);
        let title = Paragraph::new(title_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Blogr Newsletter Manager"),
            )
            .style(Style::default().fg(Color::Cyan));

        frame.render_widget(title, header_chunks[0]);

        // Statistics
        let stats = self.get_statistics_text();
        let stats_widget = Paragraph::new(stats)
            .block(Block::default().borders(Borders::ALL).title("Statistics"))
            .style(Style::default().fg(Color::Green));

        frame.render_widget(stats_widget, header_chunks[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_cells = ["", "Email", "Status", "Subscribed", "Notes"]
            .iter()
            .map(|h| ratatui::widgets::Cell::from(*h).style(Style::default().fg(Color::Yellow)));
        let header = Row::new(header_cells)
            .style(Style::default().bg(Color::DarkGray))
            .height(1);

        let rows: Vec<Row> = self
            .filtered_subscribers
            .iter()
            .map(|&subscriber_index| {
                let subscriber = &self.subscribers[subscriber_index];
                let is_selected = self.selected.contains(&subscriber_index);
                let selection_indicator = if is_selected { "●" } else { " " };

                let status_style = match subscriber.status {
                    SubscriberStatus::Pending => Style::default().fg(Color::Yellow),
                    SubscriberStatus::Approved => Style::default().fg(Color::Green),
                    SubscriberStatus::Declined => Style::default().fg(Color::Red),
                };

                let subscribed_date = subscriber
                    .subscribed_at
                    .format("%Y-%m-%d %H:%M")
                    .to_string();

                let notes = subscriber.notes.as_deref().unwrap_or("-");

                Row::new(vec![
                    ratatui::widgets::Cell::from(selection_indicator)
                        .style(Style::default().fg(Color::Cyan)),
                    ratatui::widgets::Cell::from(subscriber.email.clone()),
                    ratatui::widgets::Cell::from(subscriber.status.to_string()).style(status_style),
                    ratatui::widgets::Cell::from(subscribed_date),
                    ratatui::widgets::Cell::from(notes),
                ])
                .height(1)
            })
            .collect();

        let table = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(format!(
                "Subscribers ({}/{})",
                self.filtered_subscribers.len(),
                self.subscribers.len()
            )))
            .widths(&[
                Constraint::Length(2),  // Selection indicator
                Constraint::Min(25),    // Email
                Constraint::Length(10), // Status
                Constraint::Length(16), // Subscribed date
                Constraint::Min(10),    // Notes
            ])
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::REVERSED)
                    .fg(Color::White),
            )
            .highlight_symbol("► ");

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let status_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        // Status message or help
        let status_text = if let Some(ref message) = self.status_message {
            message.clone()
        } else {
            "Press 'h' for help, 'q' to quit".to_string()
        };

        let status = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .style(Style::default().fg(Color::White));

        frame.render_widget(status, status_chunks[0]);

        // Selection info
        let selection_info = if self.selected.is_empty() {
            "No items selected".to_string()
        } else {
            format!("{} item(s) selected", self.selected.len())
        };

        let selection = Paragraph::new(selection_info)
            .block(Block::default().borders(Borders::ALL).title("Selection"))
            .style(Style::default().fg(Color::Magenta));

        frame.render_widget(selection, status_chunks[1]);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let help_text = vec![
            Line::from("Newsletter Subscriber Management Help"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Navigation:",
                Style::default().fg(Color::Yellow).bold(),
            )]),
            Line::from("  ↑/k        - Move up"),
            Line::from("  ↓/j        - Move down"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Selection:",
                Style::default().fg(Color::Yellow).bold(),
            )]),
            Line::from("  Space      - Toggle selection of current item"),
            Line::from("  a          - Select all visible items"),
            Line::from("  n          - Clear all selections"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Actions:",
                Style::default().fg(Color::Yellow).bold(),
            )]),
            Line::from("  A          - Approve selected/current subscriber(s)"),
            Line::from("  D          - Decline selected/current subscriber(s)"),
            Line::from("  X          - Delete selected/current subscriber(s)"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Filters:",
                Style::default().fg(Color::Yellow).bold(),
            )]),
            Line::from("  1          - Show all subscribers"),
            Line::from("  2          - Show pending subscribers"),
            Line::from("  3          - Show approved subscribers"),
            Line::from("  4          - Show declined subscribers"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Other:",
                Style::default().fg(Color::Yellow).bold(),
            )]),
            Line::from("  r          - Refresh data"),
            Line::from("  h/F1       - Show this help"),
            Line::from("  q/Esc      - Quit"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Press any key to return",
                Style::default().fg(Color::Green).italic(),
            )]),
        ];

        let help = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .title_alignment(Alignment::Center),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        // Center the help dialog
        let popup_area = centered_rect(80, 80, area);
        frame.render_widget(Clear, popup_area);
        frame.render_widget(help, popup_area);
    }

    fn render_confirm(&self, frame: &mut Frame, area: Rect) {
        if let Some(ref action) = self.confirm_action {
            let (action_text, color) = match action {
                ConfirmAction::Approve => ("approve", Color::Green),
                ConfirmAction::Decline => ("decline", Color::Yellow),
                ConfirmAction::Delete => ("delete", Color::Red),
            };

            let count = self.selected.len();
            let confirm_text = vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    "Confirm Action",
                    Style::default().fg(Color::White).bold(),
                )]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("Are you sure you want to "),
                    Span::styled(action_text, Style::default().fg(color).bold()),
                    Span::raw(format!(" {} subscriber(s)?", count)),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    "Press 'y' to confirm, 'n' to cancel",
                    Style::default().fg(Color::Gray).italic(),
                )]),
                Line::from(""),
            ];

            let confirm = Paragraph::new(confirm_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Confirmation")
                        .title_alignment(Alignment::Center)
                        .border_style(Style::default().fg(color)),
                )
                .alignment(Alignment::Center);

            let popup_area = centered_rect(50, 25, area);
            frame.render_widget(Clear, popup_area);
            frame.render_widget(confirm, popup_area);
        }
    }

    fn get_statistics_text(&self) -> Text<'_> {
        let total = self.subscribers.len();
        let pending = self
            .subscribers
            .iter()
            .filter(|s| s.status == SubscriberStatus::Pending)
            .count();
        let approved = self
            .subscribers
            .iter()
            .filter(|s| s.status == SubscriberStatus::Approved)
            .count();

        Text::from(vec![
            Line::from(format!("Total: {}", total)),
            Line::from(vec![
                Span::styled("Pending: ", Style::default().fg(Color::Yellow)),
                Span::raw(pending.to_string()),
                Span::raw("  "),
                Span::styled("Approved: ", Style::default().fg(Color::Green)),
                Span::raw(approved.to_string()),
            ]),
        ])
    }
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
