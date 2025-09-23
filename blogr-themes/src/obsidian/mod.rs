use crate::{ConfigOption, Theme, ThemeInfo};
use ratatui::style::Style;
use std::collections::HashMap;

pub struct ObsidianTheme;

impl ObsidianTheme {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Theme for ObsidianTheme {
    fn info(&self) -> ThemeInfo {
        let mut schema = HashMap::new();

        schema.insert(
            "obsidian_css".to_string(),
            ConfigOption {
                option_type: "string".to_string(),
                default: "static/obsidian.css".to_string(),
                description: "Path to Obsidian CSS (served from /static/)".to_string(),
            },
        );

        schema.insert(
            "color_mode".to_string(),
            ConfigOption {
                option_type: "string".to_string(),
                default: "auto".to_string(),
                description: "Dark/light mode handling (auto | dark | light)".to_string(),
            },
        );

        ThemeInfo {
            name: "Obsidian".to_string(),
            version: "1.0.0".to_string(),
            author: "Blogr Team".to_string(),
            description: "Adopts Obsidian community themes to style Blogr content".to_string(),
            config_schema: schema,
        }
    }

    fn templates(&self) -> HashMap<String, String> {
        let mut templates = HashMap::new();

        templates.insert(
            "base.html".to_string(),
            include_str!("templates/base.html").to_string(),
        );
        templates.insert(
            "index.html".to_string(),
            include_str!("templates/index.html").to_string(),
        );
        templates.insert(
            "post.html".to_string(),
            include_str!("templates/post.html").to_string(),
        );
        templates.insert(
            "archive.html".to_string(),
            include_str!("templates/archive.html").to_string(),
        );
        templates.insert(
            "tag.html".to_string(),
            include_str!("templates/tag.html").to_string(),
        );
        templates.insert(
            "tags.html".to_string(),
            include_str!("templates/tags.html").to_string(),
        );

        templates
    }

    fn assets(&self) -> HashMap<String, Vec<u8>> {
        HashMap::new()
    }

    fn preview_tui_style(&self) -> Style {
        Style::default()
    }
}

impl Default for ObsidianTheme {
    fn default() -> Self {
        Self::new()
    }
}
