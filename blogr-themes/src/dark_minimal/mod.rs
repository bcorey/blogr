use crate::{ConfigOption, Theme, ThemeInfo};
use ratatui::style::{Color, Style};
use std::collections::HashMap;

pub struct DarkMinimalTheme;

impl DarkMinimalTheme {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Theme for DarkMinimalTheme {
    fn info(&self) -> ThemeInfo {
        let mut config_schema = HashMap::new();

        config_schema.insert(
            "primary_color".to_string(),
            ConfigOption {
                option_type: "string".to_string(),
                default: "#00ff88".to_string(),
                description: "Primary accent color (neon green)".to_string(),
            },
        );

        config_schema.insert(
            "background_color".to_string(),
            ConfigOption {
                option_type: "string".to_string(),
                default: "#0a0a0a".to_string(),
                description: "Background color (pure dark)".to_string(),
            },
        );

        config_schema.insert(
            "text_color".to_string(),
            ConfigOption {
                option_type: "string".to_string(),
                default: "#e0e0e0".to_string(),
                description: "Main text color (soft white)".to_string(),
            },
        );

        config_schema.insert(
            "secondary_text_color".to_string(),
            ConfigOption {
                option_type: "string".to_string(),
                default: "#888888".to_string(),
                description: "Secondary text color (gray)".to_string(),
            },
        );

        config_schema.insert(
            "font_family".to_string(),
            ConfigOption {
                option_type: "string".to_string(),
                default:
                    "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif"
                        .to_string(),
                description: "Font family".to_string(),
            },
        );

        config_schema.insert(
            "enable_animations".to_string(),
            ConfigOption {
                option_type: "boolean".to_string(),
                default: "true".to_string(),
                description: "Enable smooth animations".to_string(),
            },
        );

        config_schema.insert(
            "show_social_icons".to_string(),
            ConfigOption {
                option_type: "boolean".to_string(),
                default: "true".to_string(),
                description: "Show social media icons".to_string(),
            },
        );

        config_schema.insert(
            "show_status_bar".to_string(),
            ConfigOption {
                option_type: "boolean".to_string(),
                default: "true".to_string(),
                description: "Show availability status bar".to_string(),
            },
        );

        config_schema.insert(
            "status_text".to_string(),
            ConfigOption {
                option_type: "string".to_string(),
                default: "Available for opportunities".to_string(),
                description: "Custom text for status bar".to_string(),
            },
        );

        config_schema.insert(
            "status_color".to_string(),
            ConfigOption {
                option_type: "string".to_string(),
                default: "#00ff88".to_string(),
                description: "Status dot color (hex code)".to_string(),
            },
        );

        ThemeInfo {
            name: "Dark Minimal".to_string(),
            version: "1.0.0".to_string(),
            author: "Blogr Team".to_string(),
            description: "A dark, minimal theme for personal websites with quirky interactions and clean aesthetics.".to_string(),
            config_schema,
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

        templates
    }

    fn assets(&self) -> HashMap<String, Vec<u8>> {
        let mut assets = HashMap::new();

        assets.insert(
            "css/style.css".to_string(),
            include_bytes!("assets/style.css").to_vec(),
        );

        assets
    }

    fn preview_tui_style(&self) -> Style {
        Style::default()
            .fg(Color::Rgb(0, 255, 136)) // Neon green
            .bg(Color::Rgb(10, 10, 10)) // Pure dark
    }
}

impl Default for DarkMinimalTheme {
    fn default() -> Self {
        Self::new()
    }
}
