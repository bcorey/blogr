use crate::content::Post;
use crate::project::Project;
use crate::tui::theme::TuiTheme;
use crate::tui::{self, App, Event};
use anyhow::Result;

/// Launch the TUI editor for a post
pub async fn launch_editor(post: Post, project: &Project) -> Result<Post> {
    // Load theme configuration
    let config = project.load_config()?;

    // Create TUI theme from blog theme
    let tui_theme = if let Some(theme_config) = config.theme.config.get("primary_color") {
        let primary = theme_config.as_str().unwrap_or("#FF6B35");
        let secondary = config
            .theme
            .config
            .get("secondary_color")
            .and_then(|v| v.as_str())
            .unwrap_or("#F7931E");
        let background = config
            .theme
            .config
            .get("background_color")
            .and_then(|v| v.as_str())
            .unwrap_or("#2D1B0F");

        TuiTheme::from_blog_theme(primary, secondary, background)
    } else {
        TuiTheme::minimal_retro()
    };

    // Initialize TUI
    let mut tui = tui::init()?;
    tui.init()?;

    // Create app
    let mut app = App::new(post, tui_theme);

    // Main event loop
    let result = loop {
        // Draw the interface
        tui.draw(&mut app)?;

        // Handle events
        match tui.events.next()? {
            Event::Tick => {
                app.tick();
            }
            Event::Key(key_event) => {
                app.handle_key_event(key_event)?;
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }

        // Check if we should quit
        if !app.running {
            break Ok(app.post);
        }
    };

    // Cleanup
    tui.exit()?;

    result
}

/// Launch the configuration TUI
pub async fn launch_config_editor(_project: &Project) -> Result<()> {
    // TODO: Implement configuration TUI
    // This would allow users to:
    // - Select themes
    // - Configure theme colors
    // - Set blog metadata
    // - Configure GitHub settings

    println!("Configuration TUI not yet implemented. Coming in a future update!");
    println!("For now, edit blogr.toml directly.");

    Ok(())
}
