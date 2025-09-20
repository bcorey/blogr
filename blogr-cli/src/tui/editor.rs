use crate::tui::theme::TuiTheme;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

/// Text editor component
pub struct Editor {
    /// The content being edited
    content: Vec<String>,
    /// Current cursor position (line, column)
    cursor: (usize, usize),
    /// Current scroll offset
    scroll: (usize, usize),
    /// Whether the editor is in insert mode
    insert_mode: bool,
}

impl Editor {
    /// Create a new editor with the given content
    pub fn new(content: String) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let lines = if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        };

        Self {
            content: lines,
            cursor: (0, 0),
            scroll: (0, 0),
            insert_mode: false,
        }
    }

    /// Get the current content as a string
    pub fn get_content(&self) -> String {
        self.content.join("\n")
    }

    /// Handle a key event and return true if the content was modified
    pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char(c) => {
                self.insert_char(c);
                true
            }
            KeyCode::Enter => {
                self.insert_newline();
                true
            }
            KeyCode::Backspace => {
                self.delete_char_before_cursor();
                true
            }
            KeyCode::Delete => {
                self.delete_char_at_cursor();
                true
            }
            KeyCode::Left => {
                self.move_cursor_left();
                false
            }
            KeyCode::Right => {
                self.move_cursor_right();
                false
            }
            KeyCode::Up => {
                self.move_cursor_up();
                false
            }
            KeyCode::Down => {
                self.move_cursor_down();
                false
            }
            KeyCode::Home => {
                self.move_cursor_to_line_start();
                false
            }
            KeyCode::End => {
                self.move_cursor_to_line_end();
                false
            }
            KeyCode::PageUp => {
                self.page_up();
                false
            }
            KeyCode::PageDown => {
                self.page_down();
                false
            }
            KeyCode::Tab => {
                self.insert_char(' ');
                self.insert_char(' ');
                self.insert_char(' ');
                self.insert_char(' ');
                true
            }
            _ => false,
        }
    }

    /// Insert a character at the cursor position
    fn insert_char(&mut self, c: char) {
        let (line, col) = self.cursor;
        if line < self.content.len() {
            self.content[line].insert(col, c);
            self.cursor.1 += 1;
        }
    }

    /// Insert a newline at the cursor position
    fn insert_newline(&mut self) {
        let (line, col) = self.cursor;
        if line < self.content.len() {
            let current_line = self.content[line].clone();
            let (left, right) = current_line.split_at(col);
            self.content[line] = left.to_string();
            self.content.insert(line + 1, right.to_string());
            self.cursor = (line + 1, 0);
        }
    }

    /// Delete the character before the cursor
    fn delete_char_before_cursor(&mut self) {
        let (line, col) = self.cursor;
        if col > 0 {
            self.content[line].remove(col - 1);
            self.cursor.1 -= 1;
        } else if line > 0 {
            // Join with previous line
            let current_line = self.content.remove(line);
            let prev_line_len = self.content[line - 1].len();
            self.content[line - 1].push_str(&current_line);
            self.cursor = (line - 1, prev_line_len);
        }
    }

    /// Delete the character at the cursor
    fn delete_char_at_cursor(&mut self) {
        let (line, col) = self.cursor;
        if line < self.content.len() {
            if col < self.content[line].len() {
                self.content[line].remove(col);
            } else if line + 1 < self.content.len() {
                // Join with next line
                let next_line = self.content.remove(line + 1);
                self.content[line].push_str(&next_line);
            }
        }
    }

    /// Move cursor left
    fn move_cursor_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.content[self.cursor.0].len();
        }
    }

    /// Move cursor right
    fn move_cursor_right(&mut self) {
        let (line, col) = self.cursor;
        if line < self.content.len() {
            if col < self.content[line].len() {
                self.cursor.1 += 1;
            } else if line + 1 < self.content.len() {
                self.cursor = (line + 1, 0);
            }
        }
    }

    /// Move cursor up
    fn move_cursor_up(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            let line_len = self.content[self.cursor.0].len();
            if self.cursor.1 > line_len {
                self.cursor.1 = line_len;
            }
        }
    }

    /// Move cursor down
    fn move_cursor_down(&mut self) {
        if self.cursor.0 + 1 < self.content.len() {
            self.cursor.0 += 1;
            let line_len = self.content[self.cursor.0].len();
            if self.cursor.1 > line_len {
                self.cursor.1 = line_len;
            }
        }
    }

    /// Move cursor to start of line
    fn move_cursor_to_line_start(&mut self) {
        self.cursor.1 = 0;
    }

    /// Move cursor to end of line
    fn move_cursor_to_line_end(&mut self) {
        let line = self.cursor.0;
        if line < self.content.len() {
            self.cursor.1 = self.content[line].len();
        }
    }

    /// Page up
    fn page_up(&mut self) {
        let page_size = 10; // TODO: Calculate based on widget height
        if self.cursor.0 >= page_size {
            self.cursor.0 -= page_size;
        } else {
            self.cursor.0 = 0;
        }
        self.move_cursor_to_line_end();
    }

    /// Page down
    fn page_down(&mut self) {
        let page_size = 10; // TODO: Calculate based on widget height
        if self.cursor.0 + page_size < self.content.len() {
            self.cursor.0 += page_size;
        } else {
            self.cursor.0 = self.content.len().saturating_sub(1);
        }
        self.move_cursor_to_line_end();
    }

    /// Render the editor
    pub fn render(&self, frame: &mut Frame, area: Rect, block: Block, theme: &TuiTheme) {
        let inner_area = block.inner(area);

        // Create lines with syntax highlighting
        let mut lines = Vec::new();
        let visible_start = self.scroll.0;
        let visible_end = (visible_start + inner_area.height as usize).min(self.content.len());

        for (line_idx, line_content) in self
            .content
            .iter()
            .enumerate()
            .skip(visible_start)
            .take(visible_end - visible_start)
        {
            let line_number = line_idx + 1;
            let is_cursor_line = line_idx == self.cursor.0;

            // Create line with basic syntax highlighting
            let spans = self.highlight_line(line_content, theme);

            // Add line number
            let mut line_spans = vec![Span::styled(
                format!("{:4} ", line_number),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(if is_cursor_line {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            )];
            line_spans.extend(spans);

            lines.push(Line::from(line_spans));
        }

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .style(theme.text_style());

        frame.render_widget(paragraph, area);

        // Render cursor if in insert mode
        if self.insert_mode {
            self.render_cursor(frame, inner_area, theme);
        }
    }

    /// Simple syntax highlighting for markdown
    fn highlight_line(&self, line: &str, theme: &TuiTheme) -> Vec<Span<'_>> {
        let mut spans = Vec::new();
        let mut chars = line.chars().peekable();
        let mut current_span = String::new();
        let mut in_code = false;
        let mut in_bold = false;
        let mut in_italic = false;

        while let Some(c) = chars.next() {
            match c {
                '#' if current_span.is_empty() => {
                    // Header
                    let mut header_level = 1;
                    while let Some('#') = chars.peek() {
                        chars.next();
                        header_level += 1;
                    }

                    // Consume the rest of the line as header
                    let header_text: String = chars.collect();
                    spans.push(Span::styled(
                        format!("{}{}", "#".repeat(header_level), header_text),
                        theme.markdown_header_style(),
                    ));
                    break;
                }
                '`' => {
                    if !current_span.is_empty() {
                        spans.push(Span::styled(current_span.clone(), theme.text_style()));
                        current_span.clear();
                    }

                    in_code = !in_code;
                    current_span.push(c);

                    if !in_code {
                        spans.push(Span::styled(
                            current_span.clone(),
                            theme.markdown_code_style(),
                        ));
                        current_span.clear();
                    }
                }
                '*' if chars.peek() == Some(&'*') => {
                    // Bold
                    if !current_span.is_empty() {
                        spans.push(Span::styled(current_span.clone(), theme.text_style()));
                        current_span.clear();
                    }

                    chars.next(); // consume second *
                    in_bold = !in_bold;
                    current_span.push_str("**");
                }
                '*' => {
                    // Italic
                    if !current_span.is_empty() {
                        spans.push(Span::styled(current_span.clone(), theme.text_style()));
                        current_span.clear();
                    }

                    in_italic = !in_italic;
                    current_span.push(c);
                }
                _ => {
                    current_span.push(c);
                }
            }
        }

        if !current_span.is_empty() {
            let style = if in_code {
                theme.markdown_code_style()
            } else if in_bold {
                theme.markdown_bold_style()
            } else if in_italic {
                theme.markdown_italic_style()
            } else {
                theme.text_style()
            };
            spans.push(Span::styled(current_span, style));
        }

        if spans.is_empty() {
            spans.push(Span::styled("", theme.text_style()));
        }

        spans
    }

    /// Render the cursor
    fn render_cursor(&self, frame: &mut Frame, area: Rect, theme: &TuiTheme) {
        // Calculate cursor position on screen
        let cursor_line = self.cursor.0.saturating_sub(self.scroll.0);
        let cursor_col = self.cursor.1.saturating_sub(self.scroll.1) + 5; // Account for line numbers

        if cursor_line < area.height as usize && cursor_col < area.width as usize {
            let cursor_area = Rect {
                x: area.x + cursor_col as u16,
                y: area.y + cursor_line as u16,
                width: 1,
                height: 1,
            };

            let cursor = Paragraph::new(" ").style(Style::default().bg(theme.cursor_color()));

            frame.render_widget(cursor, cursor_area);
        }
    }
}
