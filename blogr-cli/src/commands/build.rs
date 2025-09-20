use crate::generator::SiteBuilder;
use crate::project::Project;
use crate::utils::Console;
use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub async fn handle_build(output: Option<PathBuf>, drafts: bool, future: bool) -> Result<()> {
    Console::info("Building static site...");

    // Check if we're in a blogr project
    let project = Project::find_project()?
        .ok_or_else(|| anyhow!("Not in a blogr project. Run 'blogr init' first."))?;

    // Create site builder
    let site_builder = SiteBuilder::new(project, output, drafts, future)?;

    // Build the site
    site_builder.build()?;

    Console::success("Site built successfully!");
    println!(
        "📦 Built site saved to: {}",
        site_builder.output_dir().display()
    );
    println!("🌐 Ready for deployment");

    // Show what was included
    if drafts {
        println!("📝 Draft posts included in build");
    }
    if future {
        println!("🔮 Future-dated posts included in build");
    }

    Ok(())
}
