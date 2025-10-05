use crate::{Theme, ThemeInfo, ThemeTemplates};
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

    fn templates(&self) -> ThemeTemplates {
        ThemeTemplates::new("base_html", include_str!("templates/base.html"))
            .with_template("post_card.html", include_str!("templates/post_card.html"))
            .with_template("index.html", include_str!("templates/index.html"))
            .with_template("post.html", include_str!("templates/post.html"))
            .with_template("archive.html", include_str!("templates/archive.html"))
            .with_template("tag.html", include_str!("templates/tag.html"))
            .with_template("tags.html", include_str!("templates/tags.html"))
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
