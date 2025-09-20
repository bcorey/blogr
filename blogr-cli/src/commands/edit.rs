use crate::content::PostManager;
use crate::project::Project;
use crate::utils::Console;
use anyhow::{anyhow, Result};
use std::process::Command;

pub async fn handle_edit(slug: String, use_tui: bool) -> Result<()> {
    Console::info(&format!("Opening post '{}' for editing", slug));

    // Check if we're in a blogr project
    let project = Project::find_project()?
        .ok_or_else(|| anyhow!("Not in a blogr project. Run 'blogr init' first."))?;

    let post_manager = PostManager::new(project.posts_dir());

    // Find the post by slug
    let post = post_manager
        .find_by_slug(&slug)?
        .ok_or_else(|| anyhow!("Post with slug '{}' not found", slug))?;

    if use_tui {
        // Use TUI editor
        Console::info("Launching TUI editor...");

        use crate::tui_launcher;

        let edited_post = tui_launcher::launch_editor(post, &project).await?;

        // Save the edited post
        let post_manager = PostManager::new(project.posts_dir());
        let final_file_path = post_manager.save_post(&edited_post)?;

        Console::success("Post edited and saved!");
        println!("📝 Post saved to: {}", final_file_path.display());
        println!("💡 Next steps:");
        println!("  • Preview changes: blogr serve");
        println!("  • List all posts: blogr list");
    } else {
        // Use external editor
        let file_path = project.posts_dir().join(post.filename());

        // Try to determine the best editor to use
        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| {
                // Try to find a suitable editor
                if Command::new("code").arg("--version").output().is_ok() {
                    "code".to_string()
                } else if Command::new("vim").arg("--version").output().is_ok() {
                    "vim".to_string()
                } else if Command::new("nano").arg("--version").output().is_ok() {
                    "nano".to_string()
                } else {
                    "vi".to_string() // Last resort, should be available on most systems
                }
            });

        Console::info(&format!("Opening with editor: {}", editor));
        println!("📝 File: {}", file_path.display());

        // Open the file in the editor
        let status = Command::new(&editor)
            .arg(&file_path)
            .status()
            .map_err(|e| anyhow!("Failed to open editor '{}': {}", editor, e))?;

        if status.success() {
            Console::success("Post editing completed");
            println!("💡 Next steps:");
            println!("  • Preview changes: blogr serve");
            println!("  • List all posts: blogr list");

            // Check if the post was modified by trying to reload it
            match post_manager.find_by_slug(&slug) {
                Ok(Some(updated_post)) => {
                    println!("  • Current status: {:?}", updated_post.metadata.status);
                    if !updated_post.metadata.tags.is_empty() {
                        println!("  • Tags: {}", updated_post.metadata.tags.join(", "));
                    }
                }
                Ok(None) => {
                    Console::warn("Post seems to have been deleted during editing");
                }
                Err(e) => {
                    Console::warn(&format!("Error reloading post: {}", e));
                }
            }
        } else {
            Console::error("Editor exited with non-zero status");
            return Err(anyhow!("Editor failed"));
        }
    }

    Ok(())
}
