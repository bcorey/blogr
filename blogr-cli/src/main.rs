use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod config;
mod project;
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
        } => new::handle_new(title, template, draft, slug, tags).await,
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
    }
}
