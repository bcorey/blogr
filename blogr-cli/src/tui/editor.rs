use crate::tui::theme::TuiTheme;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
        }
    }

    /// Get the current content as a string
    pub fn get_content(&self) -> String {
        self.content.join("\n")
    }

    /// Handle a key event and return true if the content was modified
    pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        // Handle key combinations with modifiers first
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('k') => {
                    self.delete_current_line();
                    return true;
                }
                KeyCode::Char('d') => {
                    self.delete_word();
                    return true;
                }
                KeyCode::Char('u') => {
                    self.delete_to_line_start();
                    return true;
                }
                KeyCode::Char('a') => {
                    self.move_cursor_to_line_start();
                    return false;
                }
                KeyCode::Char('e') => {
                    self.move_cursor_to_line_end();
                    return false;
                }
                _ => {}
            }
        }

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
                self.update_scroll();
                false
            }
            KeyCode::Right => {
                self.move_cursor_right();
                self.update_scroll();
                false
            }
            KeyCode::Up => {
                self.move_cursor_up();
                self.update_scroll();
                false
            }
            KeyCode::Down => {
                self.move_cursor_down();
                self.update_scroll();
                false
            }
            KeyCode::Home => {
                self.move_cursor_to_line_start();
                self.update_scroll();
                false
            }
            KeyCode::End => {
                self.move_cursor_to_line_end();
                self.update_scroll();
                false
            }
            KeyCode::PageUp => {
                self.page_up();
                self.update_scroll();
                false
            }
            KeyCode::PageDown => {
                self.page_down();
                self.update_scroll();
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

    /// Delete the current line
    fn delete_current_line(&mut self) {
        let line = self.cursor.0;
        if line < self.content.len() {
            if self.content.len() > 1 {
                self.content.remove(line);
                // Move cursor to the start of the next line, or previous line if at end
                if line >= self.content.len() {
                    self.cursor.0 = self.content.len().saturating_sub(1);
                }
                self.cursor.1 = 0;
            } else {
                // If it's the only line, just clear it
                self.content[0].clear();
                self.cursor = (0, 0);
            }
        }
    }

    /// Delete from cursor to end of current word
    fn delete_word(&mut self) {
        let (line, col) = self.cursor;
        if line < self.content.len() {
            let line_content = &mut self.content[line];
            if col < line_content.len() {
                let remaining = &line_content[col..];
                // Find the end of the current word
                let mut end_pos = col;
                let chars: Vec<char> = remaining.chars().collect();

                // Skip whitespace first
                while end_pos - col < chars.len() && chars[end_pos - col].is_whitespace() {
                    end_pos += 1;
                }

                // Then skip non-whitespace characters
                while end_pos - col < chars.len() && !chars[end_pos - col].is_whitespace() {
                    end_pos += 1;
                }

                // Remove the characters
                line_content.replace_range(col..end_pos, "");
            }
        }
    }

    /// Delete from cursor to start of line
    fn delete_to_line_start(&mut self) {
        let (line, col) = self.cursor;
        if line < self.content.len() && col > 0 {
            let line_content = &mut self.content[line];
            line_content.replace_range(0..col, "");
            self.cursor.1 = 0;
        }
    }

    /// Update scroll position to keep cursor visible
    fn update_scroll(&mut self) {
        // This will be enhanced when we have the actual viewport dimensions
        // For now, basic vertical scrolling
        let cursor_line = self.cursor.0;
        let visible_lines = 20; // Approximate, will be calculated from actual area

        // Scroll down if cursor is below visible area
        if cursor_line >= self.scroll.0 + visible_lines {
            self.scroll.0 = cursor_line.saturating_sub(visible_lines - 1);
        }

        // Scroll up if cursor is above visible area
        if cursor_line < self.scroll.0 {
            self.scroll.0 = cursor_line;
        }
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
            let spans = self.highlight_line_with_cursor(line_content, line_idx, theme);

            // Add line number with highlighting for cursor line
            let mut line_spans = vec![Span::styled(
                format!("{:4} ", line_number),
                Style::default()
                    .fg(if is_cursor_line {
                        theme.primary_color
                    } else {
                        Color::DarkGray
                    })
                    .add_modifier(if is_cursor_line {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            )];
            line_spans.extend(spans);

            lines.push(Line::from(line_spans));
        }

        // Add empty lines if content is shorter than visible area
        while lines.len() < inner_area.height as usize {
            lines.push(Line::from(vec![Span::styled(
                "   ~ ",
                Style::default().fg(Color::DarkGray),
            )]));
        }

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .style(theme.text_style());

        frame.render_widget(paragraph, area);
    }

    /// Highlight line with cursor position awareness
    fn highlight_line_with_cursor(
        &self,
        line: &str,
        line_idx: usize,
        theme: &TuiTheme,
    ) -> Vec<Span<'_>> {
        let is_cursor_line = line_idx == self.cursor.0;

        if is_cursor_line {
            // For cursor line, highlight the character at cursor position
            let cursor_col = self.cursor.1;
            let line_chars: Vec<char> = line.chars().collect();

            let mut result = Vec::new();

            // Add each character individually, highlighting the cursor position
            for (i, &ch) in line_chars.iter().enumerate() {
                if i == cursor_col {
                    // This is the cursor position - highlight with theme colors
                    result.push(Span::styled(
                        ch.to_string(),
                        Style::default()
                            .bg(theme.cursor_color())
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    // Regular character
                    result.push(Span::styled(ch.to_string(), theme.text_style()));
                }
            }

            // If cursor is at end of line, add a space cursor
            if cursor_col >= line_chars.len() {
                result.push(Span::styled(
                    " ",
                    Style::default()
                        .bg(theme.cursor_color())
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                ));
            }

            // If line is empty, just show cursor
            if result.is_empty() {
                result.push(Span::styled(
                    " ",
                    Style::default()
                        .bg(theme.cursor_color())
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                ));
            }

            result
        } else {
            // For non-cursor lines, use regular highlighting
            let mut spans = self.highlight_line(line, theme);

            // If line is empty, add a space for proper rendering
            if line.is_empty() {
                spans = vec![Span::styled(" ", theme.text_style())];
            }

            spans
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
}
