use crate::config::Config;
use crate::project::Project;
use crate::tui::theme::TuiTheme;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub type AppResult<T> = anyhow::Result<T>;

/// Configuration field types
#[derive(Debug, Clone)]
pub enum ConfigField {
    BlogTitle,
    BlogAuthor,
    BlogDescription,
    BlogBaseUrl,
    BlogLanguage,
    BlogTimezone,
    ThemeName,
    DomainPrimary,
    DomainEnforceHttps,
    BuildOutputDir,
    BuildDrafts,
    BuildFuturePosts,
    DevPort,
    DevAutoReload,
}

impl ConfigField {
    pub fn display_name(&self) -> &'static str {
        match self {
            ConfigField::BlogTitle => "Blog Title",
            ConfigField::BlogAuthor => "Blog Author",
            ConfigField::BlogDescription => "Blog Description",
            ConfigField::BlogBaseUrl => "Base URL",
            ConfigField::BlogLanguage => "Language",
            ConfigField::BlogTimezone => "Timezone",
            ConfigField::ThemeName => "Theme Name",
            ConfigField::DomainPrimary => "Primary Domain",
            ConfigField::DomainEnforceHttps => "Enforce HTTPS",
            ConfigField::BuildOutputDir => "Output Directory",
            ConfigField::BuildDrafts => "Include Drafts",
            ConfigField::BuildFuturePosts => "Include Future Posts",
            ConfigField::DevPort => "Development Port",
            ConfigField::DevAutoReload => "Auto Reload",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            ConfigField::BlogTitle
            | ConfigField::BlogAuthor
            | ConfigField::BlogDescription
            | ConfigField::BlogBaseUrl
            | ConfigField::BlogLanguage
            | ConfigField::BlogTimezone => "Blog Settings",
            ConfigField::ThemeName => "Theme Settings",
            ConfigField::DomainPrimary | ConfigField::DomainEnforceHttps => "Domain Settings",
            ConfigField::BuildOutputDir
            | ConfigField::BuildDrafts
            | ConfigField::BuildFuturePosts => "Build Settings",
            ConfigField::DevPort | ConfigField::DevAutoReload => "Development Settings",
        }
    }

    pub fn get_value(&self, config: &Config) -> String {
        match self {
            ConfigField::BlogTitle => config.blog.title.clone(),
            ConfigField::BlogAuthor => config.blog.author.clone(),
            ConfigField::BlogDescription => config.blog.description.clone(),
            ConfigField::BlogBaseUrl => config.blog.base_url.clone(),
            ConfigField::BlogLanguage => config.blog.language.as_deref().unwrap_or("").to_string(),
            ConfigField::BlogTimezone => config.blog.timezone.as_deref().unwrap_or("").to_string(),
            ConfigField::ThemeName => config.theme.name.clone(),
            ConfigField::DomainPrimary => {
                if let Some(domains) = &config.blog.domains {
                    domains.primary.as_deref().unwrap_or("").to_string()
                } else {
                    "".to_string()
                }
            }
            ConfigField::DomainEnforceHttps => {
                if let Some(domains) = &config.blog.domains {
                    domains.enforce_https.to_string()
                } else {
                    "true".to_string()
                }
            }
            ConfigField::BuildOutputDir => config
                .build
                .output_dir
                .as_deref()
                .unwrap_or("dist")
                .to_string(),
            ConfigField::BuildDrafts => config.build.drafts.to_string(),
            ConfigField::BuildFuturePosts => config.build.future_posts.to_string(),
            ConfigField::DevPort => config.dev.port.to_string(),
            ConfigField::DevAutoReload => config.dev.auto_reload.to_string(),
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(
            self,
            ConfigField::DomainEnforceHttps
                | ConfigField::BuildDrafts
                | ConfigField::BuildFuturePosts
                | ConfigField::DevAutoReload
        )
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, ConfigField::DevPort)
    }
}

/// Configuration editor mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigMode {
    Browse,
    Edit,
    Help,
}

/// Configuration editor application
pub struct ConfigApp {
    /// Current running state
    pub running: bool,
    /// Current mode
    pub mode: ConfigMode,
    /// Current configuration
    pub config: Config,
    /// Project reference
    pub project: Project,
    /// Current theme
    pub theme: TuiTheme,
    /// Available configuration fields
    pub fields: Vec<ConfigField>,
    /// Current field selection
    pub selected_field: usize,
    /// List state for field selection
    pub list_state: ListState,
    /// Current edit buffer
    pub edit_buffer: String,
    /// Status message
    pub status_message: String,
    /// Whether config has been modified
    pub modified: bool,
    /// Show help overlay
    pub show_help: bool,
}

impl ConfigApp {
    /// Create a new configuration app
    pub fn new(config: Config, project: Project, theme: TuiTheme) -> Self {
        let fields = vec![
            ConfigField::BlogTitle,
            ConfigField::BlogAuthor,
            ConfigField::BlogDescription,
            ConfigField::BlogBaseUrl,
            ConfigField::BlogLanguage,
            ConfigField::BlogTimezone,
            ConfigField::ThemeName,
            ConfigField::DomainPrimary,
            ConfigField::DomainEnforceHttps,
            ConfigField::BuildOutputDir,
            ConfigField::BuildDrafts,
            ConfigField::BuildFuturePosts,
            ConfigField::DevPort,
            ConfigField::DevAutoReload,
        ];

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            running: true,
            mode: ConfigMode::Browse,
            config,
            project,
            theme,
            fields,
            selected_field: 0,
            list_state,
            edit_buffer: String::new(),
            status_message: "Navigate with ↑/↓, Enter to edit, 'q' to quit, 's' to save"
                .to_string(),
            modified: false,
            show_help: false,
        }
    }

    /// Handle key events
    pub fn handle_key_event(&mut self, key: KeyEvent) -> AppResult<()> {
        match self.mode {
            ConfigMode::Browse => self.handle_browse_mode(key),
            ConfigMode::Edit => self.handle_edit_mode(key),
            ConfigMode::Help => self.handle_help_mode(key),
        }
    }

    fn handle_browse_mode(&mut self, key: KeyEvent) -> AppResult<()> {
        match key.code {
            KeyCode::Char('q') => {
                if self.modified {
                    self.status_message =
                        "Configuration modified! Press 's' to save or 'Q' to quit without saving"
                            .to_string();
                } else {
                    self.running = false;
                }
            }
            KeyCode::Char('Q') => {
                self.running = false;
            }
            KeyCode::Char('s') => {
                self.save_config()?;
            }
            KeyCode::Char('h') | KeyCode::F(1) => {
                self.show_help = true;
                self.mode = ConfigMode::Help;
            }
            KeyCode::Up => {
                if self.selected_field > 0 {
                    self.selected_field -= 1;
                    self.list_state.select(Some(self.selected_field));
                }
            }
            KeyCode::Down => {
                if self.selected_field < self.fields.len() - 1 {
                    self.selected_field += 1;
                    self.list_state.select(Some(self.selected_field));
                }
            }
            KeyCode::Enter => {
                self.enter_edit_mode();
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_edit_mode(&mut self, key: KeyEvent) -> AppResult<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = ConfigMode::Browse;
                self.edit_buffer.clear();
                self.status_message = "Edit cancelled".to_string();
            }
            KeyCode::Enter => {
                self.apply_edit()?;
            }
            KeyCode::Backspace => {
                self.edit_buffer.pop();
            }
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    if c == 'c' {
                        self.mode = ConfigMode::Browse;
                        self.edit_buffer.clear();
                        self.status_message = "Edit cancelled".to_string();
                    }
                } else {
                    self.edit_buffer.push(c);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_help_mode(&mut self, key: KeyEvent) -> AppResult<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Char('h') | KeyCode::F(1) => {
                self.show_help = false;
                self.mode = ConfigMode::Browse;
            }
            _ => {}
        }
        Ok(())
    }

    fn enter_edit_mode(&mut self) {
        if let Some(field) = self.fields.get(self.selected_field) {
            self.edit_buffer = field.get_value(&self.config);
            self.mode = ConfigMode::Edit;
            self.status_message = format!(
                "Editing {}: Press Enter to save, Esc to cancel",
                field.display_name()
            );
        }
    }

    fn apply_edit(&mut self) -> AppResult<()> {
        if let Some(field) = self.fields.get(self.selected_field) {
            let value = self.edit_buffer.trim().to_string();

            // Apply the change to the configuration
            match field {
                ConfigField::BlogTitle => {
                    if !value.is_empty() {
                        self.config.blog.title = value;
                        self.modified = true;
                    }
                }
                ConfigField::BlogAuthor => {
                    if !value.is_empty() {
                        self.config.blog.author = value;
                        self.modified = true;
                    }
                }
                ConfigField::BlogDescription => {
                    if !value.is_empty() {
                        self.config.blog.description = value;
                        self.modified = true;
                    }
                }
                ConfigField::BlogBaseUrl => {
                    if !value.is_empty() {
                        self.config.blog.base_url = value;
                        self.modified = true;
                    }
                }
                ConfigField::BlogLanguage => {
                    self.config.blog.language = if value.is_empty() { None } else { Some(value) };
                    self.modified = true;
                }
                ConfigField::BlogTimezone => {
                    self.config.blog.timezone = if value.is_empty() { None } else { Some(value) };
                    self.modified = true;
                }
                ConfigField::ThemeName => {
                    if !value.is_empty() {
                        self.config.theme.name = value;
                        self.modified = true;
                    }
                }
                ConfigField::DomainPrimary => {
                    if self.config.blog.domains.is_none() {
                        self.config.blog.domains = Some(crate::config::DomainConfig {
                            primary: None,
                            aliases: Vec::new(),
                            subdomain: None,
                            enforce_https: true,
                            github_pages_domain: None,
                        });
                    }
                    if let Some(domains) = &mut self.config.blog.domains {
                        domains.primary = if value.is_empty() {
                            None
                        } else {
                            Some(value.clone())
                        };
                        domains.github_pages_domain =
                            if value.is_empty() { None } else { Some(value) };
                        self.modified = true;
                    }
                }
                ConfigField::DomainEnforceHttps => {
                    let enforce_https = value.to_lowercase() == "true";
                    if self.config.blog.domains.is_none() {
                        self.config.blog.domains = Some(crate::config::DomainConfig {
                            primary: None,
                            aliases: Vec::new(),
                            subdomain: None,
                            enforce_https,
                            github_pages_domain: None,
                        });
                    }
                    if let Some(domains) = &mut self.config.blog.domains {
                        domains.enforce_https = enforce_https;
                        self.modified = true;
                    }
                }
                ConfigField::BuildOutputDir => {
                    self.config.build.output_dir =
                        if value.is_empty() { None } else { Some(value) };
                    self.modified = true;
                }
                ConfigField::BuildDrafts => {
                    self.config.build.drafts = value.to_lowercase() == "true";
                    self.modified = true;
                }
                ConfigField::BuildFuturePosts => {
                    self.config.build.future_posts = value.to_lowercase() == "true";
                    self.modified = true;
                }
                ConfigField::DevPort => {
                    if let Ok(port) = value.parse::<u16>() {
                        if port > 0 {
                            self.config.dev.port = port;
                            self.modified = true;
                        }
                    }
                }
                ConfigField::DevAutoReload => {
                    self.config.dev.auto_reload = value.to_lowercase() == "true";
                    self.modified = true;
                }
            }

            self.mode = ConfigMode::Browse;
            self.edit_buffer.clear();
            self.status_message = format!("{} updated", field.display_name());
        }
        Ok(())
    }

    fn save_config(&mut self) -> AppResult<()> {
        let config_path = self.project.root.join("blogr.toml");
        self.config.save_to_file(&config_path)?;
        self.modified = false;
        self.status_message = "Configuration saved successfully!".to_string();
        Ok(())
    }

    /// Render the application
    pub fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main content
                Constraint::Length(3), // Status bar
            ])
            .split(frame.size());

        self.render_header(frame, chunks[0]);
        self.render_main_content(frame, chunks[1]);
        self.render_status_bar(frame, chunks[2]);

        if self.show_help {
            self.render_help_overlay(frame);
        }
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = if self.modified {
            "Blogr Configuration Editor *"
        } else {
            "Blogr Configuration Editor"
        };

        let header = Paragraph::new(title)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_style())
                    .title("Configuration")
                    .title_style(self.theme.title_style()),
            )
            .style(self.theme.text_style());

        frame.render_widget(header, area);
    }

    fn render_main_content(&mut self, frame: &mut Frame, area: Rect) {
        match self.mode {
            ConfigMode::Browse => self.render_browse_mode(frame, area),
            ConfigMode::Edit => self.render_edit_mode(frame, area),
            ConfigMode::Help => {} // Help is rendered as overlay
        }
    }

    fn render_browse_mode(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Render field list
        self.render_field_list(frame, chunks[0]);

        // Render field details
        self.render_field_details(frame, chunks[1]);
    }

    fn render_field_list(&mut self, frame: &mut Frame, area: Rect) {
        let mut current_category = "";
        let mut items = Vec::new();

        for field in &self.fields {
            let category = field.category();
            if category != current_category {
                if !current_category.is_empty() {
                    items.push(ListItem::new(""));
                }
                items.push(ListItem::new(Line::from(Span::styled(
                    category,
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(self.theme.primary_color),
                ))));
                current_category = category;
            }

            let value = field.get_value(&self.config);
            let display_value = if value.len() > 20 {
                format!("{}...", &value[..17])
            } else {
                value
            };

            let line = Line::from(vec![
                Span::styled(
                    format!("  {}: ", field.display_name()),
                    Style::default().fg(self.theme.text_color),
                ),
                Span::styled(display_value, Style::default().fg(Color::Gray)),
            ]);

            let item = ListItem::new(line);
            items.push(item);
        }

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Configuration Fields")
                    .border_style(self.theme.focused_border_style()),
            )
            .highlight_style(
                Style::default()
                    .bg(self.theme.primary_color)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_field_details(&self, frame: &mut Frame, area: Rect) {
        if let Some(field) = self.fields.get(self.selected_field) {
            let value = field.get_value(&self.config);
            let effective_url = if matches!(field, ConfigField::BlogBaseUrl) {
                format!("\nEffective URL: {}", self.config.get_effective_base_url())
            } else {
                String::new()
            };

            let content = format!(
                "Field: {}\nCategory: {}\nCurrent Value: {}{}\n\nPress Enter to edit this field",
                field.display_name(),
                field.category(),
                value,
                effective_url
            );

            let details = Paragraph::new(content)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Field Details")
                        .border_style(self.theme.border_style()),
                )
                .wrap(Wrap { trim: true })
                .style(self.theme.text_style());

            frame.render_widget(details, area);
        }
    }

    fn render_edit_mode(&self, frame: &mut Frame, area: Rect) {
        if let Some(field) = self.fields.get(self.selected_field) {
            let edit_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(5), Constraint::Min(0)])
                .split(area);

            let input_text = if field.is_boolean() {
                format!("{} (true/false)", self.edit_buffer)
            } else if field.is_numeric() {
                format!("{} (number)", self.edit_buffer)
            } else {
                self.edit_buffer.clone()
            };

            let input = Paragraph::new(input_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Editing: {}", field.display_name()))
                        .border_style(self.theme.focused_border_style()),
                )
                .style(self.theme.text_style());

            frame.render_widget(input, edit_area[0]);

            let help_text = if field.is_boolean() {
                "Enter 'true' or 'false'"
            } else if field.is_numeric() {
                "Enter a valid number"
            } else {
                "Enter the new value"
            };

            let help = Paragraph::new(format!(
                "{}\n\nPress Enter to save, Esc to cancel",
                help_text
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .border_style(self.theme.border_style()),
            )
            .wrap(Wrap { trim: true })
            .style(self.theme.text_style());

            frame.render_widget(help, edit_area[1]);
        }
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let status = Paragraph::new(self.status_message.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_style()),
            )
            .style(self.theme.text_style());

        frame.render_widget(status, area);
    }

    fn render_help_overlay(&self, frame: &mut Frame) {
        let area = frame.size();
        let popup_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(area)[1];

        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(popup_area)[1];

        frame.render_widget(Clear, popup_area);

        let help_text = vec![
            "Blogr Configuration Editor Help",
            "",
            "Navigation:",
            "  ↑/↓       - Navigate fields",
            "  Enter     - Edit selected field",
            "  Esc       - Cancel edit",
            "",
            "Actions:",
            "  s         - Save configuration",
            "  q         - Quit (with save prompt)",
            "  Q         - Quit without saving",
            "  h/F1      - Toggle this help",
            "",
            "Field Types:",
            "  Text      - Enter any text",
            "  Boolean   - Enter 'true' or 'false'",
            "  Number    - Enter a valid number",
            "",
            "Press any key to close this help",
        ];

        let help = Paragraph::new(help_text.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .border_style(self.theme.focused_border_style()),
            )
            .wrap(Wrap { trim: true })
            .style(self.theme.text_style());

        frame.render_widget(help, popup_area);
    }

    /// Handle tick event
    pub fn tick(&self) {}
}
