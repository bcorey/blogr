use crate::tui::markdown::MarkdownRenderer;
use crate::tui::theme::TuiTheme;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

/// Preview component for rendering markdown
pub struct Preview {
    /// Rendered content lines
    content: Vec<Line<'static>>,
    /// Scroll position
    scroll: usize,
    /// Markdown renderer
    renderer: MarkdownRenderer,
}

impl Preview {
    /// Create a new preview
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            scroll: 0,
            renderer: MarkdownRenderer::new(),
        }
    }

    /// Update the content to preview
    pub fn update_content(&mut self, markdown: String, theme: &TuiTheme) {
        self.content = self.renderer.render_markdown(&markdown, theme);
        // Reset scroll when content changes
        self.scroll = 0;
    }

    /// Handle key events for scrolling
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                if self.scroll > 0 {
                    self.scroll -= 1;
                }
            }
            KeyCode::Down => {
                if self.scroll + 1 < self.content.len() {
                    self.scroll += 1;
                }
            }
            KeyCode::PageUp => {
                let page_size = 10; // TODO: Calculate based on widget height
                if self.scroll >= page_size {
                    self.scroll -= page_size;
                } else {
                    self.scroll = 0;
                }
            }
            KeyCode::PageDown => {
                let page_size = 10; // TODO: Calculate based on widget height
                if self.scroll + page_size < self.content.len() {
                    self.scroll += page_size;
                } else {
                    self.scroll = self.content.len().saturating_sub(1);
                }
            }
            KeyCode::Home => {
                self.scroll = 0;
            }
            KeyCode::End => {
                self.scroll = self.content.len().saturating_sub(1);
            }
            _ => {}
        }
    }

    /// Render the preview
    pub fn render(&self, frame: &mut Frame, area: Rect, block: Block, theme: &TuiTheme) {
        let inner_area = block.inner(area);

        // Calculate visible content
        let visible_start = self.scroll;
        let visible_end = (visible_start + inner_area.height as usize).min(self.content.len());

        let visible_content: Vec<Line> = self
            .content
            .iter()
            .skip(visible_start)
            .take(visible_end - visible_start)
            .cloned()
            .collect();

        let paragraph = Paragraph::new(visible_content)
            .block(block)
            .wrap(Wrap { trim: false })
            .style(theme.text_style());

        frame.render_widget(paragraph, area);

        // TODO: Add scroll indicator
    }

    /// Get the current scroll position
    #[allow(dead_code)]
    pub fn scroll_position(&self) -> usize {
        self.scroll
    }

    /// Get the total number of lines
    #[allow(dead_code)]
    pub fn total_lines(&self) -> usize {
        self.content.len()
    }
}
