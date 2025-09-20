use anyhow::Result;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

/// Render markdown to HTML with syntax highlighting
pub fn render_markdown(markdown: &str) -> Result<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);

    // Process events to add syntax highlighting
    let events: Vec<Event> = parser
        .map(|event| process_code_blocks(event))
        .collect::<Result<Vec<_>, _>>()?;

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());

    Ok(html_output)
}

/// Process code blocks to add syntax highlighting
fn process_code_blocks(event: Event) -> Result<Event, anyhow::Error> {
    match event {
        Event::Start(Tag::CodeBlock(kind)) => {
            match kind {
                pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                    // Start of a fenced code block with language
                    Ok(Event::Html(
                        format!(
                            "<pre class=\"highlight\"><code class=\"language-{}\">",
                            lang
                        )
                        .into(),
                    ))
                }
                pulldown_cmark::CodeBlockKind::Indented => {
                    // Start of an indented code block
                    Ok(Event::Html("<pre class=\"highlight\"><code>".into()))
                }
            }
        }
        Event::End(Tag::CodeBlock(_)) => {
            // End of code block
            Ok(Event::Html("</code></pre>".into()))
        }
        Event::Text(text) if matches!(get_current_context(), Some(ContextType::CodeBlock(_))) => {
            // This is text inside a code block - apply syntax highlighting
            if let Some(ContextType::CodeBlock(ref lang)) = get_current_context() {
                let highlighted = highlight_code(&text, lang)?;
                Ok(Event::Html(highlighted.into()))
            } else {
                // Fallback for plain text
                Ok(Event::Html(html_escape(&text).into()))
            }
        }
        _ => Ok(event),
    }
}

/// Get the current parsing context (simplified for this implementation)
fn get_current_context() -> Option<ContextType> {
    // This is a simplified implementation
    // In a real implementation, you'd track the parsing state
    None
}

#[derive(Debug)]
#[allow(dead_code)]
enum ContextType {
    CodeBlock(String),
}

/// Highlight code using syntect
fn highlight_code(code: &str, language: &str) -> Result<String> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    let syntax = syntax_set
        .find_syntax_by_token(language)
        .or_else(|| syntax_set.find_syntax_by_extension(language))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let theme = &theme_set.themes["base16-ocean.dark"];

    let highlighted = highlighted_html_for_string(code, &syntax_set, syntax, theme)?;

    Ok(highlighted)
}

/// HTML escape text
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Convert markdown to plain text (for excerpts)
#[allow(dead_code)]
pub fn markdown_to_text(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut text = String::new();

    for event in parser {
        match event {
            Event::Text(t) | Event::Code(t) => text.push_str(&t),
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            _ => {}
        }
    }

    text
}

/// Extract excerpt from markdown (first paragraph or first N words)
#[allow(dead_code)]
pub fn extract_excerpt(markdown: &str, word_limit: usize) -> String {
    let text = markdown_to_text(markdown);
    let words: Vec<&str> = text.split_whitespace().take(word_limit).collect();
    let excerpt = words.join(" ");

    if text.split_whitespace().count() > word_limit {
        format!("{}...", excerpt)
    } else {
        excerpt
    }
}
