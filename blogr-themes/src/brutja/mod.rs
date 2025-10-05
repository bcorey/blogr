use crate::{Theme, ThemeInfo};
use ratatui::style::Style;
use std::collections::HashMap;

pub struct BrutjaTheme;

impl BrutjaTheme {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Theme for BrutjaTheme {
    fn info(&self) -> ThemeInfo {
        let schema = HashMap::new();

        ThemeInfo {
            name: "Brutja".to_string(),
            version: "1.0.0".to_string(),
            author: "Benjamin Corey".to_string(),
            description: "Somewhat brutalist theme".to_string(),
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
            "post_card.html".to_string(),
            include_str!("templates/post_card.html").to_string(),
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
        let mut assets = HashMap::new();

        // Bundle a default Obsidian-compatible CSS
        assets.insert(
            "css/brutja-default.css".to_string(),
            include_bytes!("assets/brutja-default.css").to_vec(),
        );

        assets
    }

    fn preview_tui_style(&self) -> Style {
        use ratatui::style::Color;
        Style::default()
            .fg(Color::Rgb(167, 139, 250)) // Obsidian purple accent
            .bg(Color::Rgb(32, 32, 32)) // Dark background similar to Obsidian
    }
}

impl Default for BrutjaTheme {
    fn default() -> Self {
        Self::new()
    }
}
