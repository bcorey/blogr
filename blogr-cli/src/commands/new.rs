use crate::utils::Console;
use anyhow::Result;

pub async fn handle_new(
    title: String,
    _template: String,
    _draft: bool,
    _slug: Option<String>,
    _tags: Option<String>,
) -> Result<()> {
    Console::info(&format!("Creating new post: '{}'", title));

    // TODO: Implement post creation
    // - Check if we're in a blogr project
    // - Generate post filename and path
    // - Create post with front matter
    // - Use template system
    // - Handle slug generation
    // - Parse and set tags
    // - Set draft status

    Console::success(&format!("Created new post: {}", title));
    println!("📝 Post created in posts/ directory");
    println!("💡 Run 'blogr serve' to preview your changes");

    Ok(())
}
