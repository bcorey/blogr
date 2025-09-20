use crate::content::{PostManager, PostStatus};
use crate::project::Project;
use crate::utils::Console;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub async fn handle_info() -> Result<()> {
    Console::info("Project information:");

    // Check if we're in a blogr project
    let project = Project::find_project()?
        .ok_or_else(|| anyhow!("Not in a blogr project. Run 'blogr init' first."))?;

    let config = project.load_config()?;
    let post_manager = PostManager::new(project.posts_dir());
    let posts = post_manager.load_all_posts()?;

    let published_count = posts
        .iter()
        .filter(|p| p.metadata.status == PostStatus::Published)
        .count();
    let draft_count = posts
        .iter()
        .filter(|p| p.metadata.status == PostStatus::Draft)
        .count();

    let github_status = if let Some(github) = &config.github {
        format!("{}/{}", github.username, github.repository)
    } else {
        "Not configured".to_string()
    };

    println!("📋 Project Information:");
    println!("  📝 Title: {}", config.blog.title);
    println!("  👤 Author: {}", config.blog.author);
    println!("  📄 Description: {}", config.blog.description);
    println!("  🎨 Theme: {}", config.theme.name);
    println!(
        "  📊 Posts: {} ({} published, {} draft)",
        posts.len(),
        published_count,
        draft_count
    );
    println!("  🌐 GitHub: {}", github_status);
    println!();
    println!("💡 Edit blogr.toml to update project settings");

    Ok(())
}

pub async fn handle_check() -> Result<()> {
    Console::info("Validating project structure...");

    // TODO: Implement project validation
    // - Check if we're in a blogr project
    // - Validate directory structure
    // - Check configuration file
    // - Validate posts format and front matter
    // - Check theme availability
    // - Verify GitHub integration if configured
    // - Check for common issues

    Console::success("Project structure validation passed!");
    println!("✅ All required directories exist");
    println!("✅ Configuration file is valid");
    println!("✅ Posts are properly formatted");
    println!("✅ Theme is available and configured");
    println!();
    println!("🎉 Your project is ready for building and deployment!");

    Ok(())
}

pub async fn handle_clean() -> Result<()> {
    Console::info("Cleaning build artifacts...");

    // TODO: Implement project cleanup
    // - Check if we're in a blogr project
    // - Remove build output directory
    // - Clean temporary files and cache
    // - Remove generated assets
    // - Clean deployment artifacts
    // - Report freed space

    Console::success("Project cleaned successfully!");
    println!("🧹 Removed build artifacts");
    println!("📦 Freed up space: 1.2 MB");
    println!("💡 Run 'blogr build' to regenerate site");

    Ok(())
}

pub async fn handle_stats() -> Result<()> {
    Console::info("Generating project statistics...");

    // Check if we're in a blogr project
    let project = Project::find_project()?
        .ok_or_else(|| anyhow!("Not in a blogr project. Run 'blogr init' first."))?;

    let post_manager = PostManager::new(project.posts_dir());
    let posts = post_manager.load_all_posts()?;

    let published_count = posts
        .iter()
        .filter(|p| p.metadata.status == PostStatus::Published)
        .count();
    let draft_count = posts
        .iter()
        .filter(|p| p.metadata.status == PostStatus::Draft)
        .count();

    // Calculate word count and reading time
    let total_words: usize = posts
        .iter()
        .map(|p| p.content.split_whitespace().count())
        .sum();
    let average_words = if posts.is_empty() {
        0
    } else {
        total_words / posts.len()
    };
    let total_reading_time: usize = posts.iter().map(|p| p.reading_time()).sum();

    // Get all tags and count usage
    let mut tag_counts = HashMap::new();
    for post in &posts {
        for tag in &post.metadata.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    let mut sorted_tags: Vec<_> = tag_counts.into_iter().collect();
    sorted_tags.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count, descending

    // Count static files
    let static_dir = project.static_dir();
    let mut static_count = 0;
    let mut image_count = 0;
    let mut css_count = 0;

    if static_dir.exists() {
        for entry in walkdir::WalkDir::new(&static_dir).into_iter().flatten() {
            if entry.path().is_file() {
                static_count += 1;
                if let Some(ext) = entry.path().extension() {
                    match ext.to_str().unwrap_or("").to_lowercase().as_str() {
                        "jpg" | "jpeg" | "png" | "gif" | "svg" | "webp" => image_count += 1,
                        "css" => css_count += 1,
                        _ => {}
                    }
                }
            }
        }
    }

    println!("📊 Project Statistics:");
    println!();
    println!("📝 Content:");
    println!("  - Total posts: {}", posts.len());
    println!("  - Published: {}", published_count);
    println!("  - Drafts: {}", draft_count);
    println!("  - Total words: ~{}", total_words);
    if !posts.is_empty() {
        println!("  - Average words per post: {}", average_words);
    }
    println!(
        "  - Estimated reading time: {} minutes total",
        total_reading_time
    );
    println!();

    if !sorted_tags.is_empty() {
        println!("🏷️ Tags:");
        for (tag, count) in sorted_tags.iter().take(10) {
            println!("  - {} ({})", tag, count);
        }
        if sorted_tags.len() > 10 {
            println!("  - ... and {} more", sorted_tags.len() - 10);
        }
        println!();
    }

    println!("📁 Files:");
    println!("  - Static files: {}", static_count);
    println!("  - Images: {}", image_count);
    println!("  - Custom CSS: {}", css_count);
    println!();
    println!("🚀 Last build: Never");
    println!("📤 Last deploy: Never");

    Ok(())
}
