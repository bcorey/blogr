use crate::content::{Post, PostManager};
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

    // Create post manager
    let post_manager = PostManager::new(project.posts_dir());

    // Create app
    let mut app = App::new(post, tui_theme, post_manager);

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
            Event::Redraw => {}
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
pub async fn launch_config_editor(project: &Project) -> Result<()> {
    // Load current configuration
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

    // Create configuration app
    let mut config_app = crate::tui::config_app::ConfigApp::new(config, project.clone(), tui_theme);

    // Main event loop
    let result = loop {
        // Draw the interface
        tui.draw_config(&mut config_app)?;

        // Handle events
        match tui.events.next()? {
            Event::Tick => {
                config_app.tick();
            }
            Event::Key(key_event) => {
                config_app.handle_key_event(key_event)?;
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::Redraw => {}
        }

        // Check if we should quit
        if !config_app.running {
            break Ok(());
        }
    };

    // Cleanup
    tui.exit()?;

    result
}
