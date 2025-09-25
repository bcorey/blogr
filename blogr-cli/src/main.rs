use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod config;
mod content;
mod generator;
mod newsletter;
mod project;
mod tui;
mod tui_launcher;
mod utils;

use commands::*;

#[derive(Parser)]
#[command(name = "blogr")]
#[command(about = "A CLI static site generator for blogs")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new blog project
    Init {
        /// Project name
        name: Option<String>,
        /// Project directory (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// GitHub username for repository creation
        #[arg(long)]
        github_username: Option<String>,
        /// GitHub repository name (defaults to project name)
        #[arg(long)]
        github_repo: Option<String>,
        /// Skip GitHub repository creation
        #[arg(long)]
        no_github: bool,
    },
    /// Create a new blog post
    New {
        /// Post title
        title: String,
        /// Post template to use
        #[arg(short, long, default_value = "post")]
        template: String,
        /// Set post as draft
        #[arg(short, long)]
        draft: bool,
        /// Custom slug for the post URL
        #[arg(short, long)]
        slug: Option<String>,
        /// Post tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
        /// Open in TUI editor instead of external editor
        #[arg(long)]
        tui: bool,
    },
    /// List all blog posts
    List {
        /// Show only draft posts
        #[arg(long)]
        drafts: bool,
        /// Show only published posts
        #[arg(long)]
        published: bool,
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
        /// Sort order (date, title, slug)
        #[arg(short, long, default_value = "date")]
        sort: String,
    },
    /// Edit an existing blog post
    Edit {
        /// Post slug to edit
        slug: String,
        /// Open in TUI editor instead of external editor
        #[arg(long)]
        tui: bool,
    },
    /// Delete a blog post
    Delete {
        /// Post slug to delete
        slug: String,
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
    /// Build the static site
    Build {
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Include draft posts
        #[arg(long)]
        drafts: bool,
        /// Include future-dated posts
        #[arg(long)]
        future: bool,
    },
    /// Start development server with live reload
    Serve {
        /// Port to serve on
        #[arg(short, long, default_value = "3000")]
        port: u16,
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Include draft posts
        #[arg(long)]
        drafts: bool,
        /// Open browser automatically
        #[arg(long)]
        open: bool,
    },
    /// Deploy the site to GitHub Pages
    Deploy {
        /// Deploy branch (default: gh-pages)
        #[arg(short, long, default_value = "gh-pages")]
        branch: String,
        /// Deployment message
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Theme management commands
    Theme {
        #[command(subcommand)]
        action: ThemeAction,
    },
    /// Project management commands
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },
    /// Configuration management commands
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Newsletter management commands
    Newsletter {
        #[command(subcommand)]
        action: NewsletterAction,
    },
}

#[derive(Subcommand)]
enum ThemeAction {
    /// List available themes
    List,
    /// Show theme details
    Info {
        /// Theme name
        name: String,
    },
    /// Set active theme
    Set {
        /// Theme name
        name: String,
    },
    /// Preview theme in TUI
    Preview {
        /// Theme name
        name: String,
    },
}

#[derive(Subcommand)]
enum ProjectAction {
    /// Show project information
    Info,
    /// Validate project structure
    Check,
    /// Clean build artifacts
    Clean,
    /// Show project statistics
    Stats,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Open interactive configuration editor (TUI)
    Edit,
    /// Get configuration value
    Get {
        /// Configuration key (e.g., blog.title, domains.primary)
        key: String,
    },
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Domain configuration commands
    Domain {
        #[command(subcommand)]
        action: DomainAction,
    },
}

#[derive(Subcommand)]
enum DomainAction {
    /// Set primary domain or subdomain
    Set {
        /// Domain name (e.g., example.com or blog.example.com)
        domain: Option<String>,
        /// Subdomain prefix (for subdomain configuration)
        #[arg(long)]
        subdomain: Option<String>,
        /// Enforce HTTPS
        #[arg(long, default_value = "true")]
        enforce_https: bool,
        /// Create CNAME file for GitHub Pages
        #[arg(long)]
        github_pages: bool,
    },
    /// List all configured domains
    List,
    /// Clear all domain configuration
    Clear,
    /// Add domain alias
    AddAlias {
        /// Alias domain name
        alias: String,
    },
    /// Remove domain alias
    RemoveAlias {
        /// Alias domain name to remove
        alias: String,
    },
}

#[derive(Subcommand)]
enum NewsletterAction {
    /// Fetch subscribers from email inbox
    FetchSubscribers {
        /// Interactive IMAP server configuration
        #[arg(long)]
        interactive: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok(); // Load .env file if it exists

    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            name,
            path,
            github_username,
            github_repo,
            no_github,
        } => init::handle_init(name, path, github_username, github_repo, no_github).await,
        Commands::New {
            title,
            template,
            draft,
            slug,
            tags,
            tui,
        } => new::handle_new(title, template, draft, slug, tags, tui).await,
        Commands::List {
            drafts,
            published,
            tag,
            sort,
        } => list::handle_list(drafts, published, tag, sort).await,
        Commands::Edit { slug, tui } => edit::handle_edit(slug, tui).await,
        Commands::Delete { slug, force } => delete::handle_delete(slug, force).await,
        Commands::Build {
            output,
            drafts,
            future,
        } => build::handle_build(output, drafts, future).await,
        Commands::Serve {
            port,
            host,
            drafts,
            open,
        } => serve::handle_serve(port, host, drafts, open).await,
        Commands::Deploy { branch, message } => deploy::handle_deploy(branch, message).await,
        Commands::Theme { action } => match action {
            ThemeAction::List => theme::handle_list().await,
            ThemeAction::Info { name } => theme::handle_info(name).await,
            ThemeAction::Set { name } => theme::handle_set(name).await,
            ThemeAction::Preview { name } => theme::handle_preview(name).await,
        },
        Commands::Project { action } => match action {
            ProjectAction::Info => project_cmd::handle_info().await,
            ProjectAction::Check => project_cmd::handle_check().await,
            ProjectAction::Clean => project_cmd::handle_clean().await,
            ProjectAction::Stats => project_cmd::handle_stats().await,
        },
        Commands::Config { action } => match action {
            ConfigAction::Edit => {
                use crate::project::Project;
                use crate::tui_launcher;

                let project = Project::find_project()?.ok_or_else(|| {
                    anyhow::anyhow!("Not in a blogr project. Run 'blogr init' first.")
                })?;

                tui_launcher::launch_config_editor(&project).await
            }
            ConfigAction::Get { key } => commands::config::handle_get(key).await,
            ConfigAction::Set { key, value } => commands::config::handle_set(key, value).await,
            ConfigAction::Domain { action } => {
                let domain_action = match action {
                    DomainAction::Set {
                        domain,
                        subdomain,
                        enforce_https,
                        github_pages,
                    } => commands::config::DomainAction::Set {
                        domain,
                        subdomain,
                        enforce_https,
                        github_pages,
                    },
                    DomainAction::List => commands::config::DomainAction::List,
                    DomainAction::Clear => commands::config::DomainAction::Clear,
                    DomainAction::AddAlias { alias } => {
                        commands::config::DomainAction::AddAlias { alias }
                    }
                    DomainAction::RemoveAlias { alias } => {
                        commands::config::DomainAction::RemoveAlias { alias }
                    }
                };
                commands::config::handle_domain(domain_action).await
            }
        },
        Commands::Newsletter { action } => match action {
            NewsletterAction::FetchSubscribers { interactive } => {
                commands::newsletter::handle_fetch_subscribers(interactive).await
            }
        },
    }
}
