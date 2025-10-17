use crate::config::Config;
use crate::project::Project;
use crate::tui::theme::TuiTheme;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use strum::{EnumIter, IntoEnumIterator, VariantArray};

pub type AppResult<T> = anyhow::Result<T>;

#[derive(PartialEq, Eq)]
enum HighLevelListItem {
    Field(ConfigField),
    Section(ConfigSection),
    BlankLine,
}

struct HighLevelConfigList(Vec<HighLevelListItem>);
impl HighLevelConfigList {
    fn new() -> Self {
        let inner = ConfigSection::iter()
            .map(|section| (section, section.get_fields()))
            .map(|(section, fields)| {
                let mut list_section = vec![
                    HighLevelListItem::BlankLine,
                    HighLevelListItem::Section(section),
                ];
                list_section.append(
                    &mut fields
                        .iter()
                        .map(|field| HighLevelListItem::Field(*field))
                        .collect(),
                );
                list_section
            })
            .fold(Vec::new(), |mut acc, mut list_section| {
                acc.append(&mut list_section);
                acc
            });

        Self(inner)
    }

    fn index_of(&self, field: &ConfigField) -> Option<usize> {
        self.0.iter().position(|item| match item {
            HighLevelListItem::Field(i) => i == field,
            _ => false,
        })
    }
}

/// Configuration field types
#[derive(Debug, Clone, Copy, EnumIter, VariantArray, PartialEq, Eq)]
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

impl std::fmt::Display for ConfigField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::BlogTitle => "Blog Title",
            Self::BlogAuthor => "Blog Author",
            Self::BlogDescription => "Blog Description",
            Self::BlogBaseUrl => "Base URL",
            Self::BlogLanguage => "Language",
            Self::BlogTimezone => "Timezone",
            Self::ThemeName => "Theme Name",
            Self::DomainPrimary => "Primary Domain",
            Self::DomainEnforceHttps => "Enforce HTTPS",
            Self::BuildOutputDir => "Output Directory",
            Self::BuildDrafts => "Include Drafts",
            Self::BuildFuturePosts => "Include Future Posts",
            Self::DevPort => "Development Port",
            Self::DevAutoReload => "Auto Reload",
        };
        write!(f, "{name}")
    }
}

impl ConfigField {
    pub fn category(&self) -> ConfigSection {
        match self {
            Self::BlogTitle
            | Self::BlogAuthor
            | Self::BlogDescription
            | Self::BlogBaseUrl
            | Self::BlogLanguage
            | Self::BlogTimezone => ConfigSection::Blog,
            Self::ThemeName => ConfigSection::Theme,
            Self::DomainPrimary | ConfigField::DomainEnforceHttps => ConfigSection::Domain,
            Self::BuildOutputDir | Self::BuildDrafts | Self::BuildFuturePosts => {
                ConfigSection::Build
            }
            Self::DevPort | ConfigField::DevAutoReload => ConfigSection::Development,
        }
    }

    pub fn get_value(&self, config: &Config) -> String {
        match self {
            Self::BlogTitle => config.blog.title.clone(),
            Self::BlogAuthor => config.blog.author.clone(),
            Self::BlogDescription => config.blog.description.clone(),
            Self::BlogBaseUrl => config.blog.base_url.clone(),
            Self::BlogLanguage => config.blog.language.as_deref().unwrap_or("").to_string(),
            Self::BlogTimezone => config.blog.timezone.as_deref().unwrap_or("").to_string(),
            Self::ThemeName => config.theme.name.clone(),
            Self::DomainPrimary => {
                if let Some(domains) = &config.blog.domains {
                    domains.primary.as_deref().unwrap_or("").to_string()
                } else {
                    "".to_string()
                }
            }
            Self::DomainEnforceHttps => {
                if let Some(domains) = &config.blog.domains {
                    domains.enforce_https.to_string()
                } else {
                    "true".to_string()
                }
            }
            Self::BuildOutputDir => config
                .build
                .output_dir
                .as_deref()
                .unwrap_or("dist")
                .to_string(),
            Self::BuildDrafts => config.build.drafts.to_string(),
            Self::BuildFuturePosts => config.build.future_posts.to_string(),
            Self::DevPort => config.dev.port.to_string(),
            Self::DevAutoReload => config.dev.auto_reload.to_string(),
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(
            self,
            Self::DomainEnforceHttps
                | Self::BuildDrafts
                | Self::BuildFuturePosts
                | Self::DevAutoReload
        )
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Self::DevPort)
    }
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Hash)]
pub enum ConfigSection {
    Blog,
    Theme,
    Domain,
    Build,
    Development,
}

impl std::fmt::Display for ConfigSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Blog => "Blog Settings",
            Self::Theme => "Theme Settings",
            Self::Domain => "Domain Settings",
            Self::Build => "Build Settings",
            Self::Development => "Development Settings",
        };
        write!(f, "{name}")
    }
}

impl ConfigSection {
    fn get_fields(&self) -> Vec<ConfigField> {
        ConfigField::iter()
            .filter(|field| field.category() == *self)
            .collect()
    }
}

/// public API of the application
pub struct ConfigApp {
    state: ConfigAppState,
    theme: TuiTheme,
}

impl ConfigApp {
    pub fn new(config: Config, project: Project, theme: TuiTheme) -> Self {
        let state = ConfigAppState::new(config, project);
        Self { state, theme }
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

        self.state.render_header(frame, chunks[0], &self.theme);
        self.state
            .render_main_content(frame, chunks[1], &self.theme);
        self.state.render_status_bar(frame, chunks[2], &self.theme);
    }

    pub fn handle_key_event(mut self, key: KeyEvent) -> AppResult<Self> {
        self.state = self.state.handle_key_event(key)?;
        Ok(self)
    }

    /// Handle tick event
    pub fn tick(&self) {}

    pub fn is_stopped(&self) -> bool {
        matches!(self.state, ConfigAppState::Shutdown(_))
    }
}

/// Configuration editor mode.
/// The larger variants are boxed at Clippy's suggestion.
enum ConfigAppState {
    Browse(Box<Browse>),
    Edit(Box<Edit>),
    Help(Box<Help>),
    Shutdown(Shutdown),
}

impl ConfigAppState {
    pub fn new(config: Config, project: Project) -> Self {
        Self::Browse(Box::new(Browse::new(config, project)))
    }

    fn render_main_content(&mut self, frame: &mut Frame, area: Rect, theme: &TuiTheme) {
        match self {
            Self::Browse(app) => app.render_browse_mode(frame, area, theme),
            Self::Edit(app) => app.render_edit_mode(frame, area, theme),
            Self::Help(app) => app.render_help_overlay(frame, theme), // Help is rendered as overlay
            Self::Shutdown(_) => {}
        }
    }

    fn render_header(&self, frame: &mut Frame, area: Rect, theme: &TuiTheme) {
        let title = "Blogr Configuration Editor";

        let header = Paragraph::new(title)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme.border_style())
                    .title("Configuration")
                    .title_style(theme.title_style()),
            )
            .style(theme.text_style());

        frame.render_widget(header, area);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect, theme: &TuiTheme) {
        let status = Paragraph::new(self.get_status())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme.border_style()),
            )
            .style(theme.text_style());

        frame.render_widget(status, area);
    }

    fn handle_key_event(self, key: KeyEvent) -> AppResult<Self> {
        match self {
            Self::Browse(app) => Ok(app.handle_key_event(key)),
            Self::Edit(app) => app.handle_key_event(key),
            Self::Help(app) => Ok(app.handle_key_event(key)),
            Self::Shutdown(app) => Ok(app.into()),
        }
    }

    fn get_status(&self) -> String {
        match self {
            Self::Browse(app) => app.status_message.clone(),
            Self::Edit(app) => app.browse_data.status_message.clone(),
            Self::Help(app) => app.browse_data.status_message.clone(),
            Self::Shutdown(_) => "Shutting down".to_string(),
        }
    }
}

impl From<Browse> for ConfigAppState {
    fn from(value: Browse) -> Self {
        ConfigAppState::Browse(Box::new(value))
    }
}

impl From<Edit> for ConfigAppState {
    fn from(value: Edit) -> Self {
        ConfigAppState::Edit(Box::new(value))
    }
}

impl From<Help> for ConfigAppState {
    fn from(value: Help) -> Self {
        ConfigAppState::Help(Box::new(value))
    }
}

impl From<Shutdown> for ConfigAppState {
    fn from(value: Shutdown) -> Self {
        ConfigAppState::Shutdown(value)
    }
}

struct Browse {
    config: Config,
    project: Project,
    /// Current field selection
    selected_field: ConfigField,
    /// The index of the field in the ConfigField enum variant array
    config_index: usize,
    /// Lays out the ConfigField into sections with headers and blank lines.
    /// Maps the ConfigField index into the actual list layout so ListState can render the selection.
    list_layout: HighLevelConfigList,
    /// List state for field selection
    list_state: ListState,
    status_message: String,
}

impl Browse {
    fn new(config: Config, project: Project) -> Self {
        let mut list_state = ListState::default();
        let list_layout = HighLevelConfigList::new();
        let selected_field = ConfigField::BlogTitle;
        list_state.select(list_layout.index_of(&selected_field));
        Self {
            config,
            project,
            selected_field,
            config_index: 0, // must match the index of selected_field in ConfigField::VARIANTS
            list_layout,
            list_state,
            status_message: "Navigate with ↑/↓, Enter to edit, 'q' to quit".to_string(),
        }
    }

    pub fn handle_key_event(self, key: KeyEvent) -> ConfigAppState {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.enter_shutdown_mode().into(),
            KeyCode::Char('h') | KeyCode::F(1) => self.enter_help_mode().into(),
            KeyCode::Up => self.key_up().into(),
            KeyCode::Down => self.key_down().into(),
            KeyCode::Enter => self.enter_edit_mode().into(),
            _ => self.into(),
        }
    }

    fn key_up(mut self) -> Self {
        if self.config_index == 0 {
            return self;
        }
        let prev = ConfigField::VARIANTS.get(self.config_index - 1);
        if let Some(prev) = prev {
            self.selected_field = *prev;
            self.config_index -= 1;
            self.list_state.select(self.list_layout.index_of(prev));
        }
        self
    }

    fn key_down(mut self) -> Self {
        let next = ConfigField::VARIANTS.get(self.config_index + 1);
        if let Some(next) = next {
            self.selected_field = *next;
            self.config_index += 1;
            self.list_state.select(self.list_layout.index_of(next));
        }
        self
    }

    fn render_browse_mode(&mut self, frame: &mut Frame, area: Rect, theme: &TuiTheme) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Render field list
        self.render_field_list(frame, chunks[0], theme);

        // Render field details
        self.render_field_details(frame, chunks[1], theme);
    }

    fn render_field_list(&mut self, frame: &mut Frame, area: Rect, theme: &TuiTheme) {
        let items = self
            .list_layout
            .0
            .iter()
            .map(|item| match item {
                HighLevelListItem::BlankLine => ListItem::new(""),
                HighLevelListItem::Field(field) => {
                    let value = field.get_value(&self.config);
                    let display_value = if value.len() > 20 {
                        format!("{}...", &value[..17])
                    } else {
                        value
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("  {}: ", field),
                            Style::default().fg(theme.text_color),
                        ),
                        Span::styled(display_value, Style::default().fg(theme.text_color)),
                    ]))
                }
                HighLevelListItem::Section(section) => ListItem::new(Line::from(Span::styled(
                    section.to_string(),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(theme.primary_color),
                ))),
            })
            .collect::<Vec<ListItem>>();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Configuration Fields")
                    .border_style(theme.focused_border_style()),
            )
            .highlight_style(
                Style::default()
                    .bg(theme.primary_color)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_field_details(&self, frame: &mut Frame, area: Rect, theme: &TuiTheme) {
        let value = self.selected_field.get_value(&self.config);
        let effective_url = if matches!(self.selected_field, ConfigField::BlogBaseUrl) {
            format!("\nEffective URL: {}", self.config.get_effective_base_url())
        } else {
            String::new()
        };

        let content = format!(
            "Field: {}\nCategory: {}\nCurrent Value: {}{}\n\nPress Enter to edit this field",
            self.selected_field,
            self.selected_field.category(),
            value,
            effective_url
        );

        let details = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Field Details")
                    .border_style(theme.border_style()),
            )
            .wrap(Wrap { trim: true })
            .style(theme.text_style());

        frame.render_widget(details, area);
    }

    fn enter_edit_mode(mut self) -> Edit {
        let edit_buffer = self.selected_field.get_value(&self.config);
        self.status_message = format!(
            "Editing {}: Press Enter to save, Esc to cancel",
            self.selected_field
        );
        Edit {
            new_config: self.config.clone(),
            browse_data: self,
            edit_buffer,
        }
    }

    fn enter_help_mode(self) -> Help {
        Help { browse_data: self }
    }

    fn enter_shutdown_mode(self) -> Shutdown {
        Shutdown
    }
}

struct Edit {
    browse_data: Browse,
    edit_buffer: String,
    new_config: Config,
}

struct Help {
    browse_data: Browse,
}

struct Shutdown;

impl Edit {
    pub fn handle_key_event(mut self, key: KeyEvent) -> AppResult<ConfigAppState> {
        match key.code {
            KeyCode::Esc => {
                let mut browse_data = self.enter_browse_mode();
                browse_data.status_message = "Edit cancelled".to_string();
                Ok(browse_data.into())
            }
            KeyCode::Enter => {
                let browse_data = self.apply_edit()?;
                Ok(browse_data.into())
            }
            KeyCode::Backspace => {
                self.edit_buffer.pop();
                Ok(self.into())
            }
            KeyCode::Char(c) => {
                self.edit_buffer.push(c);
                Ok(self.into())
            }
            _ => Ok(self.into()),
        }
    }

    fn render_edit_mode(&self, frame: &mut Frame, area: Rect, theme: &TuiTheme) {
        let edit_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(0)])
            .split(area);

        let input_text = if self.browse_data.selected_field.is_boolean() {
            format!("{} (true/false)", self.edit_buffer)
        } else if self.browse_data.selected_field.is_numeric() {
            format!("{} (number)", self.edit_buffer)
        } else {
            self.edit_buffer.clone()
        };

        let input = Paragraph::new(input_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Editing: {}", self.browse_data.selected_field))
                    .border_style(theme.focused_border_style()),
            )
            .style(theme.text_style());

        frame.render_widget(input, edit_area[0]);

        let help_text = if self.browse_data.selected_field.is_boolean() {
            "Enter 'true' or 'false'"
        } else if self.browse_data.selected_field.is_numeric() {
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
                .border_style(theme.border_style()),
        )
        .wrap(Wrap { trim: true })
        .style(theme.text_style());

        frame.render_widget(help, edit_area[1]);
    }

    fn apply_edit(mut self) -> AppResult<Browse> {
        let value = self.edit_buffer.trim().to_string();

        // Apply the change to the configuration
        match self.browse_data.selected_field {
            ConfigField::BlogTitle => {
                if !value.is_empty() {
                    self.new_config.blog.title = value;
                }
            }
            ConfigField::BlogAuthor => {
                if !value.is_empty() {
                    self.new_config.blog.author = value;
                }
            }
            ConfigField::BlogDescription => {
                if !value.is_empty() {
                    self.new_config.blog.description = value;
                }
            }
            ConfigField::BlogBaseUrl => {
                if !value.is_empty() {
                    self.new_config.blog.base_url = value;
                }
            }
            ConfigField::BlogLanguage => {
                self.new_config.blog.language = if value.is_empty() { None } else { Some(value) };
            }
            ConfigField::BlogTimezone => {
                self.new_config.blog.timezone = if value.is_empty() { None } else { Some(value) };
            }
            ConfigField::ThemeName => {
                if !value.is_empty() {
                    self.new_config.theme.name = value;
                }
            }
            ConfigField::DomainPrimary => {
                if self.new_config.blog.domains.is_none() {
                    self.new_config.blog.domains = Some(crate::config::DomainConfig {
                        primary: None,
                        aliases: Vec::new(),
                        subdomain: None,
                        enforce_https: true,
                        github_pages_domain: None,
                    });
                }
                if let Some(domains) = &mut self.new_config.blog.domains {
                    domains.primary = if value.is_empty() {
                        None
                    } else {
                        Some(value.clone())
                    };
                    domains.github_pages_domain = if value.is_empty() { None } else { Some(value) };
                }
            }
            ConfigField::DomainEnforceHttps => {
                let enforce_https = value.to_lowercase() == "true";
                if self.new_config.blog.domains.is_none() {
                    self.new_config.blog.domains = Some(crate::config::DomainConfig {
                        primary: None,
                        aliases: Vec::new(),
                        subdomain: None,
                        enforce_https,
                        github_pages_domain: None,
                    });
                }
                if let Some(domains) = &mut self.new_config.blog.domains {
                    domains.enforce_https = enforce_https;
                }
            }
            ConfigField::BuildOutputDir => {
                self.new_config.build.output_dir =
                    if value.is_empty() { None } else { Some(value) };
            }
            ConfigField::BuildDrafts => {
                self.new_config.build.drafts = value.to_lowercase() == "true";
            }
            ConfigField::BuildFuturePosts => {
                self.new_config.build.future_posts = value.to_lowercase() == "true";
            }
            ConfigField::DevPort => {
                if let Ok(port) = value.parse::<u16>() {
                    if port > 0 {
                        self.new_config.dev.port = port;
                    }
                }
            }
            ConfigField::DevAutoReload => {
                self.new_config.dev.auto_reload = value.to_lowercase() == "true";
            }
        }

        let config_path = self.browse_data.project.root.join("blogr.toml");
        self.new_config.save_to_file(&config_path)?;
        self.browse_data.config = self.new_config.clone();
        self.browse_data.status_message = "Configuration saved successfully!".to_string();
        Ok(self.enter_browse_mode())
    }

    fn enter_browse_mode(self) -> Browse {
        self.browse_data
    }
}

impl Help {
    pub fn handle_key_event(self, key: KeyEvent) -> ConfigAppState {
        match key.code {
            KeyCode::Esc | KeyCode::Char('h') | KeyCode::F(1) => self.enter_browse_mode().into(),
            _ => self.into(),
        }
    }

    fn render_help_overlay(&self, frame: &mut Frame, theme: &TuiTheme) {
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
            "  q         - Quit",
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
                    .border_style(theme.focused_border_style()),
            )
            .wrap(Wrap { trim: true })
            .style(theme.text_style());

        frame.render_widget(help, popup_area);
    }

    fn enter_browse_mode(self) -> Browse {
        self.browse_data
    }
}
