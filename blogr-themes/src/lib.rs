use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub mod brutja;
pub mod dark_minimal;
pub mod minimal_retro;
pub mod musashi;
pub mod obsidian;
pub mod slate_portfolio;
pub mod terminal_candy;
pub mod typewriter;

pub use brutja::BrutjaTheme;
pub use dark_minimal::DarkMinimalTheme;
pub use minimal_retro::MinimalRetroTheme;
pub use musashi::MusashiTheme;
pub use obsidian::ObsidianTheme;
pub use slate_portfolio::SlatePortfolioTheme;
pub use terminal_candy::TerminalCandyTheme;
pub use typewriter::TypewriterTheme;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SiteType {
    Blog,
    Personal,
}

impl fmt::Display for SiteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SiteType::Blog => write!(f, "blog"),
            SiteType::Personal => write!(f, "personal"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub config_schema: HashMap<String, ConfigOption>,
    /// Type of site this theme supports
    pub site_type: SiteType,
}

impl ThemeInfo {
    pub fn as_data_row(&self) -> [&String; 4] {
        [&self.name, &self.version, &self.author, &self.description]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOption {
    pub option: toml::Value,
    pub description: String,
}

pub trait Theme: Send + Sync {
    fn info(&self) -> ThemeInfo;
    fn templates(&self) -> ThemeTemplates;
    fn assets(&self) -> HashMap<String, Vec<u8>>;
    fn preview_tui_style(&self) -> ratatui::style::Style;
}

pub struct ThemeTemplates {
    templates: Vec<(&'static str, &'static str)>,
}

impl ThemeTemplates {
    // Base template must be first. This ensure it's registered first with Tera when we iterate through the templates.
    pub fn new(base_template_name: &'static str, base_template: &'static str) -> Self {
        Self {
            templates: vec![(base_template_name, base_template)],
        }
    }

    pub fn with_template(mut self, name: &'static str, template: &'static str) -> Self {
        self.templates.push((name, template));
        self
    }
}

impl IntoIterator for ThemeTemplates {
    type Item = (&'static str, &'static str);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.templates.into_iter()
    }
}

#[must_use]
pub fn get_all_themes() -> Vec<Box<dyn Theme>> {
    vec![
        Box::new(MinimalRetroTheme::new()),
        Box::new(ObsidianTheme::new()),
        Box::new(TerminalCandyTheme::new()),
        Box::new(DarkMinimalTheme::new()),
        Box::new(MusashiTheme::new()),
        Box::new(SlatePortfolioTheme::new()),
        Box::new(TypewriterTheme::new()),
        Box::new(BrutjaTheme::new()),
    ]
}

#[must_use]
pub fn get_theme(name: &str) -> Option<Box<dyn Theme>> {
    match name {
        "minimal-retro" => Some(Box::new(MinimalRetroTheme::new()) as Box<dyn Theme>),
        "obsidian" => Some(Box::new(ObsidianTheme::new()) as Box<dyn Theme>),
        "terminal-candy" => Some(Box::new(TerminalCandyTheme::new()) as Box<dyn Theme>),
        "dark-minimal" => Some(Box::new(DarkMinimalTheme::new()) as Box<dyn Theme>),
        "musashi" => Some(Box::new(MusashiTheme::new()) as Box<dyn Theme>),
        "slate-portfolio" => Some(Box::new(SlatePortfolioTheme::new()) as Box<dyn Theme>),
        "typewriter" => Some(Box::new(TypewriterTheme::new()) as Box<dyn Theme>),
        "brutja" => Some(Box::new(BrutjaTheme::new()) as Box<dyn Theme>),
        _ => None,
    }
}

#[must_use]
pub fn get_theme_by_name(name: &str) -> Option<Box<dyn Theme>> {
    get_theme(name)
}
