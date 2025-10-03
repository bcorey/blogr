use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod minimal_retro;
pub mod obsidian;
pub mod terminal_candy;

pub use minimal_retro::MinimalRetroTheme;
pub use obsidian::ObsidianTheme;
pub use terminal_candy::TerminalCandyTheme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub config_schema: HashMap<String, ConfigOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOption {
    pub option_type: String,
    pub default: String,
    pub description: String,
}

pub trait Theme: Send + Sync {
    fn info(&self) -> ThemeInfo;
    fn templates(&self) -> HashMap<String, String>;
    fn assets(&self) -> HashMap<String, Vec<u8>>;
    fn preview_tui_style(&self) -> ratatui::style::Style;
}

#[must_use]
pub fn get_all_themes() -> HashMap<String, Box<dyn Theme>> {
    let mut themes: HashMap<String, Box<dyn Theme>> = HashMap::new();

    let minimal_retro = MinimalRetroTheme::new();
    themes.insert("minimal-retro".to_string(), Box::new(minimal_retro));

    let obsidian = ObsidianTheme::new();
    themes.insert("obsidian".to_string(), Box::new(obsidian));

    let terminal_candy = TerminalCandyTheme::new();
    themes.insert("terminal-candy".to_string(), Box::new(terminal_candy));

    themes
}

#[must_use]
pub fn get_theme(name: &str) -> Option<Box<dyn Theme>> {
    match name {
        "minimal-retro" => Some(Box::new(MinimalRetroTheme::new()) as Box<dyn Theme>),
        "obsidian" => Some(Box::new(ObsidianTheme::new()) as Box<dyn Theme>),
        "terminal-candy" => Some(Box::new(TerminalCandyTheme::new()) as Box<dyn Theme>),
        _ => None,
    }
}

#[must_use]
pub fn get_theme_by_name(name: &str) -> Option<Box<dyn Theme>> {
    get_theme(name)
}
