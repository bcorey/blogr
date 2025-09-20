use crate::content::PostManager;
use crate::project::Project;
use crate::utils::Console;
use anyhow::{anyhow, Result};
use std::io::{self, Write};

pub async fn handle_delete(slug: String, force: bool) -> Result<()> {
    // Check if we're in a blogr project
    let project = Project::find_project()?
        .ok_or_else(|| anyhow!("Not in a blogr project. Run 'blogr init' first."))?;

    let post_manager = PostManager::new(project.posts_dir());

    // Find the post by slug to get details
    let post = post_manager
        .find_by_slug(&slug)?
        .ok_or_else(|| anyhow!("Post with slug '{}' not found", slug))?;

    // Show post details
    Console::info(&format!("Post to delete: '{}'", post.metadata.title));
    println!("📝 Slug: {}", post.metadata.slug);
    println!(
        "📅 Date: {}",
        post.metadata.date.format("%Y-%m-%d %H:%M UTC")
    );
    println!("👤 Author: {}", post.metadata.author);
    println!("📊 Status: {:?}", post.metadata.status);
    if !post.metadata.tags.is_empty() {
        println!("🏷️  Tags: {}", post.metadata.tags.join(", "));
    }
    println!(
        "📄 File: {}",
        project.posts_dir().join(post.filename()).display()
    );

    // Confirmation (unless forced)
    if !force {
        println!();
        print!(
            "🗑️  Are you sure you want to delete this post? This action cannot be undone. (y/N): "
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            Console::info("Delete operation cancelled");
            return Ok(());
        }
    }

    // Delete the post
    match post_manager.delete_post(&slug)? {
        true => {
            Console::success(&format!(
                "Successfully deleted post '{}'",
                post.metadata.title
            ));
            println!(
                "🗑️  File removed: {}",
                project.posts_dir().join(post.filename()).display()
            );
            println!();
            println!("💡 Next steps:");
            println!("  • List remaining posts: blogr list");
            println!("  • Create a new post: blogr new \"Title\"");
            println!("  • Rebuild site: blogr build");
        }
        false => {
            Console::error(&format!("Failed to delete post '{}'", slug));
            return Err(anyhow!("Post deletion failed"));
        }
    }

    Ok(())
}
