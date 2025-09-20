use crate::tui::app::AppMode;
use ratatui::style::{Color, Modifier, Style};

/// TUI theme configuration
#[derive(Debug, Clone)]
pub struct TuiTheme {
    /// Primary color from blog theme
    pub primary_color: Color,
    /// Secondary color from blog theme
    pub secondary_color: Color,
    /// Background color from blog theme
    #[allow(dead_code)]
    pub background_color: Color,
    /// Text color
    pub text_color: Color,
    /// Border color
    pub border_color: Color,
    /// Focused border color
    pub focused_border_color: Color,
    /// Cursor color
    pub cursor_color: Color,
}

impl TuiTheme {
    /// Create a new theme from blog theme configuration
    pub fn from_blog_theme(
        primary_color: &str,
        secondary_color: &str,
        background_color: &str,
    ) -> Self {
        Self {
            primary_color: Self::parse_color(primary_color).unwrap_or(Color::Blue),
            secondary_color: Self::parse_color(secondary_color).unwrap_or(Color::Cyan),
            background_color: Self::parse_color(background_color).unwrap_or(Color::Black),
            text_color: Color::White,
            border_color: Color::Gray,
            focused_border_color: Self::parse_color(primary_color).unwrap_or(Color::Blue),
            cursor_color: Color::White,
        }
    }

    /// Create the default minimal retro theme
    pub fn minimal_retro() -> Self {
        Self {
            primary_color: Color::Rgb(255, 107, 53),   // #FF6B35
            secondary_color: Color::Rgb(247, 147, 30), // #F7931E
            background_color: Color::Rgb(45, 27, 15),  // #2D1B0F
            text_color: Color::White,
            border_color: Color::DarkGray,
            focused_border_color: Color::Rgb(255, 107, 53),
            cursor_color: Color::Rgb(247, 147, 30),
        }
    }

    /// Parse a color string (hex format)
    fn parse_color(color_str: &str) -> Option<Color> {
        if let Some(hex) = color_str.strip_prefix('#') {
            if hex.len() == 6 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex[0..2], 16),
                    u8::from_str_radix(&hex[2..4], 16),
                    u8::from_str_radix(&hex[4..6], 16),
                ) {
                    return Some(Color::Rgb(r, g, b));
                }
            }
        }
        None
    }

    /// Get the mode-specific color
    pub fn mode_color(&self, mode: AppMode) -> Color {
        match mode {
            AppMode::Normal => Color::Blue,
            AppMode::Insert => Color::Green,
            AppMode::Preview => Color::Magenta,
            AppMode::Help => Color::Yellow,
        }
    }

    /// Basic text style
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text_color)
    }

    /// Title style
    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.primary_color)
            .add_modifier(Modifier::BOLD)
    }

    /// Border style
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border_color)
    }

    /// Focused border style
    pub fn focused_border_style(&self) -> Style {
        Style::default().fg(self.focused_border_color)
    }

    /// Cursor color
    pub fn cursor_color(&self) -> Color {
        self.cursor_color
    }

    // Markdown-specific styles
    pub fn markdown_header_style(&self) -> Style {
        Style::default()
            .fg(self.primary_color)
            .add_modifier(Modifier::BOLD)
    }

    pub fn markdown_h1_style(&self) -> Style {
        Style::default()
            .fg(self.primary_color)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    }

    pub fn markdown_h2_style(&self) -> Style {
        Style::default()
            .fg(self.primary_color)
            .add_modifier(Modifier::BOLD)
    }

    pub fn markdown_h3_style(&self) -> Style {
        Style::default()
            .fg(self.secondary_color)
            .add_modifier(Modifier::BOLD)
    }

    pub fn markdown_bold_style(&self) -> Style {
        Style::default()
            .fg(self.text_color)
            .add_modifier(Modifier::BOLD)
    }

    pub fn markdown_italic_style(&self) -> Style {
        Style::default()
            .fg(self.text_color)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn markdown_code_style(&self) -> Style {
        Style::default().fg(Color::Green).bg(Color::DarkGray)
    }

    pub fn markdown_code_block_style(&self) -> Style {
        Style::default().fg(Color::Green).bg(Color::DarkGray)
    }

    pub fn markdown_link_style(&self) -> Style {
        Style::default()
            .fg(self.secondary_color)
            .add_modifier(Modifier::UNDERLINED)
    }

    pub fn markdown_list_style(&self) -> Style {
        Style::default().fg(self.primary_color)
    }

    pub fn markdown_blockquote_style(&self) -> Style {
        Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn markdown_rule_style(&self) -> Style {
        Style::default().fg(self.border_color)
    }

    pub fn markdown_html_style(&self) -> Style {
        Style::default().fg(Color::Yellow)
    }
}
