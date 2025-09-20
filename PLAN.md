# Blogr - CLI Static Site Generator

## Project Overview

Blogr is a terminal-based static site generator built in Rust, designed specifically for blogging. It provides a CLI interface with an integrated TUI editor for writing and managing blog posts, with automatic deployment to GitHub Pages. Themes are managed as a separate crate within the project workspace, allowing community contributions via PR and instant availability upon application updates.

## Core Features

### 🎯 Primary Functionality
- **CLI Interface**: Command-line interface for all blog operations
- **TUI Editor**: Terminal-based markdown editor with live preview
- **Static Site Generation**: Convert markdown posts to themed HTML sites
- **GitHub Integration**: Automatic initialization and deployment to GitHub Pages
- **Theme System**: Extensible theming with visual preview in TUI
- **Content Management**: Full CRUD operations for blog posts

### 🛠 CLI Commands
```bash
blogr init [project-name]     # Initialize new blog project + GitHub repo
blogr new [title]             # Create new blog post (opens TUI)
blogr edit <post-id>          # Edit existing post (opens TUI)
blogr list                    # List all posts with status
blogr delete <post-id>        # Delete a post
blogr publish                 # Generate site and deploy to GitHub Pages
blogr serve                   # Local development server
blogr config                  # Show/edit configuration (opens TUI)
```

### 🚨 Project Detection & Auto-Initialization
All commands except `init` automatically detect if they're being run in a blogr project:
- **Detection**: Looks for `blogr.toml` in current directory or parent directories
- **Auto-prompt**: If not found, offers interactive initialization
- **Smart defaults**: Suggests project name based on current directory
- **GitHub integration**: Validates token during auto-initialization

## Technical Architecture

### 🏗 Project Structure (Cargo Workspace)
```
blogr/
├── Cargo.toml               # Workspace root configuration
├── crates/
│   ├── blogr-cli/           # Main CLI application crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs      # CLI entry point
│   │       ├── cli/         # CLI command implementations
│   │       │   ├── mod.rs
│   │       │   ├── init.rs  # Project initialization + GitHub
│   │       │   ├── new.rs   # Create new post (launch TUI)
│   │       │   ├── edit.rs  # Edit existing post (launch TUI)
│   │       │   ├── publish.rs # Generate site and deploy
│   │       │   ├── delete.rs # Delete post operations
│   │       │   ├── list.rs  # List posts with metadata
│   │       │   └── serve.rs # Local development server
│   │       ├── tui/         # Terminal User Interface
│   │       │   ├── mod.rs
│   │       │   ├── app.rs   # Main TUI application state
│   │       │   ├── editor.rs # Markdown editor component
│   │       │   ├── preview.rs # Live HTML preview pane
│   │       │   ├── config.rs # Configuration TUI (theme selection)
│   │       │   └── components/ # Reusable TUI components
│   │       ├── generator/   # Static site generation
│   │       │   ├── mod.rs
│   │       │   ├── site.rs  # Site builder and HTML generation
│   │       │   ├── markdown.rs # Markdown processing
│   │       │   └── assets.rs # Asset management
│   │       ├── git/         # Git and GitHub integration
│   │       │   ├── mod.rs
│   │       │   ├── github.rs # GitHub API operations
│   │       │   └── operations.rs # Git operations
│   │       ├── config/      # Configuration management
│   │       │   ├── mod.rs
│   │       │   ├── settings.rs # Project configuration
│   │       │   └── validation.rs # Config validation
│   │       └── content/     # Content management
│   │           ├── mod.rs
│   │           ├── post.rs  # Blog post data structures
│   │           └── metadata.rs # Post metadata handling
│   └── blogr-themes/        # Themes crate (community contributions)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs       # Theme registry and exports
│           ├── minimal/     # Blueprint theme (reference implementation)
│           │   ├── mod.rs
│           │   ├── templates.rs # Template definitions
│           │   └── assets.rs # CSS/JS assets
│           ├── classic/     # Additional built-in theme
│           ├── dark/        # Dark theme
│           └── registry.rs  # Theme registration system
└── templates/               # Project initialization templates
    ├── gitignore
    ├── github_workflow.yml
    └── readme_template.md
```

### 🔧 Technology Stack

**Workspace Root (`Cargo.toml`):**
```toml
[workspace]
members = ["crates/blogr-cli", "crates/blogr-themes"]

[workspace.dependencies]
# Shared dependencies across crates
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
anyhow = "1.0"
thiserror = "1.0"
```

**Main CLI Crate (`crates/blogr-cli/Cargo.toml`):**
```toml
[dependencies]
# Internal dependencies
blogr-themes = { path = "../blogr-themes" }

# CLI Framework
clap = { version = "4.0", features = ["derive"] }

# Terminal UI
ratatui = "0.24"
crossterm = "0.27"

# Markdown Processing
pulldown-cmark = "0.9"
syntect = "5.1"              # Syntax highlighting

# Templating
tera = "1.19"

# Workspace shared dependencies
serde = { workspace = true }
toml = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }

# Git Operations
git2 = "0.18"

# HTTP Client for GitHub API
reqwest = { version = "0.11", features = ["json"] }

# Environment variable handling
dotenvy = "0.15"

# Async Runtime
tokio = { version = "1.0", features = ["full"] }

# Date/Time
chrono = { version = "0.4", features = ["serde"] }

# File Operations
walkdir = "2.4"

# UUID Generation
uuid = { version = "1.6", features = ["v4"] }

# Development Server
axum = "0.7"                 # For local dev server
tower = "0.4"
```

**Themes Crate (`crates/blogr-themes/Cargo.toml`):**
```toml
[dependencies]
# Workspace shared dependencies
serde = { workspace = true }
toml = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }

# Theme-specific dependencies
include_dir = "0.7"          # Embed theme assets at compile time
```

## Data Structures

### 📄 Blog Post Format
```markdown
+++
title = "My First Blog Post"
date = "2024-01-15T10:30:00Z"
author = "Author Name"
description = "A brief description of the post"
tags = ["rust", "cli", "blogging"]
status = "draft"              # draft, published
slug = "my-first-post"
featured = false
+++

# My First Blog Post

This is the content of my blog post written in **Markdown**.

## Subheading

- List item 1
- List item 2

```code
fn main() {
    println!("Hello, world!");
}
```

### ⚙️ Project Configuration (`blogr.toml`)
```toml
[blog]
title = "My Programming Blog"
author = "John Doe"
description = "Thoughts on programming, Rust, and software engineering"
base_url = "https://johndoe.github.io/blog"
language = "en"
timezone = "UTC"

[theme]
name = "minimal"
[theme.config]
primary_color = "#007acc"
secondary_color = "#333333"
font_family = "Inter, sans-serif"
show_reading_time = true
show_author = true

[github]
username = "johndoe"
repository = "blog"
branch = "gh-pages"
# GitHub Personal Access Token should be set as environment variable: GITHUB_TOKEN
# Never store the token in this config file for security reasons

[build]
output_dir = "_site"
drafts = false              # Include drafts in build
future_posts = false        # Include future-dated posts

[dev]
port = 3000
auto_reload = true
```

### 🎨 Theme Structure (in `blogr-themes` crate)

**Theme Registration (`src/lib.rs`):**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod minimal;
pub mod classic;
pub mod dark;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub config_schema: HashMap<String, ConfigOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOption {
    pub option_type: String,
    pub default: String,
    pub description: String,
}

pub trait Theme {
    fn info(&self) -> ThemeInfo;
    fn templates(&self) -> HashMap<String, &'static str>;
    fn assets(&self) -> HashMap<String, &'static [u8]>;
    fn preview_tui_style(&self) -> TuiThemeStyle; // For TUI theming
}

pub fn get_all_themes() -> Vec<Box<dyn Theme>> {
    vec![
        Box::new(minimal::MinimalTheme::new()),
        Box::new(classic::ClassicTheme::new()),
        Box::new(dark::DarkTheme::new()),
    ]
}
```

**Theme Implementation Example (`src/minimal/mod.rs`):**
```rust
use super::{Theme, ThemeInfo, ConfigOption, TuiThemeStyle};
use include_dir::{include_dir, Dir};
use std::collections::HashMap;

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/themes/minimal/templates");
static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/themes/minimal/assets");

pub struct MinimalTheme;

impl MinimalTheme {
    pub fn new() -> Self {
        Self
    }
}

impl Theme for MinimalTheme {
    fn info(&self) -> ThemeInfo {
        ThemeInfo {
            name: "minimal".to_string(),
            version: "1.0.0".to_string(),
            author: "Blogr Team".to_string(),
            description: "A clean, minimal theme - perfect blueprint for contributions".to_string(),
            config_schema: HashMap::from([
                ("primary_color".to_string(), ConfigOption {
                    option_type: "color".to_string(),
                    default: "#007acc".to_string(),
                    description: "Primary accent color".to_string(),
                }),
                ("show_reading_time".to_string(), ConfigOption {
                    option_type: "boolean".to_string(),
                    default: "true".to_string(),
                    description: "Show estimated reading time".to_string(),
                }),
            ]),
        }
    }

    // Implementation continues...
}
```

## Implementation Phases

### 🚀 Phase 1: Core Infrastructure & Workspace Setup (Week 1-2)
- [ ] Set up Cargo workspace with `blogr-cli` and `blogr-themes` crates
- [ ] Create blueprint theme (`minimal`) in themes crate
- [ ] Implement basic CLI command structure with `clap`
- [ ] Create project initialization (`blogr init`)
- [ ] Project detection system (find `blogr.toml` in directory tree)
- [ ] Auto-initialization prompting for commands run outside projects
- [ ] Basic configuration management
- [ ] File system operations for posts
- [ ] Git repository initialization
- [ ] GitHub repository creation via API (with secure token handling)
- [ ] Theme registry system in themes crate
- [ ] Environment variable validation and security checks

**Deliverables:**
- Working Cargo workspace with both crates
- Minimal theme as blueprint for community contributions
- Working `blogr init` command that creates a project and GitHub repo
- Theme registration and loading system
- Basic project structure with configuration files
- Git integration for initial commit and push

### 📝 Phase 2: Content Management (Week 3-4)
- [ ] Post data structures and metadata parsing
- [ ] CRUD operations for blog posts
- [ ] Post listing and filtering
- [ ] Slug generation and validation
- [ ] Draft/published status management

**Deliverables:**
- `blogr new`, `blogr list`, `blogr delete` commands
- Post metadata parsing and validation
- File-based post storage system

### 🖥 Phase 3: TUI Development with Theme Integration (Week 5-7)
- [ ] Basic TUI framework setup with `ratatui`
- [ ] Markdown editor with syntax highlighting
- [ ] Multi-pane layout (editor + preview)
- [ ] Configuration TUI for theme selection
- [ ] Theme-aware UI styling (TUI reflects selected blog theme colors)
- [ ] Live theme preview in TUI
- [ ] Keyboard shortcuts and navigation
- [ ] Save/cancel operations in TUI

**Deliverables:**
- Functional TUI editor launched by `blogr new` and `blogr edit`
- Configuration TUI for theme selection (`blogr config`)
- Live markdown preview with selected theme
- TUI styling that matches blog theme
- Intuitive keyboard-driven interface

### 🏗 Phase 4: Static Site Generation (Week 8-10)
- [ ] Template engine setup with `tera`
- [ ] HTML generation from markdown
- [ ] CSS and JavaScript asset handling
- [ ] Index page generation (list of posts)
- [ ] Archive and tag pages
- [ ] RSS/Atom feed generation
- [ ] SEO optimization (meta tags, sitemap)

**Deliverables:**
- Working `blogr publish` command
- GitHub Pages-compatible site generation
- SEO-optimized HTML output

### 🎨 Phase 5: Advanced Theme System (Week 11-12)
- [ ] Complete theme loading and validation
- [ ] Template inheritance system
- [ ] Theme configuration handling with live updates
- [ ] Additional built-in themes (classic, dark, modern)
- [ ] Advanced theme preview in TUI
- [ ] Theme contribution documentation and guidelines
- [ ] Automated theme validation for PR reviews

**Deliverables:**
- At least 4 built-in themes showcasing different styles
- Comprehensive theme development documentation
- Theme contribution guidelines for community PRs
- Robust theme validation system

### ⚡ Phase 6: Advanced Features (Week 13-14)
- [ ] Local development server (`blogr serve`)
- [ ] Auto-reload on file changes
- [ ] Enhanced TUI features (split panes, tabs)
- [ ] Image handling and optimization
- [ ] Comment system integration (optional)
- [ ] Analytics integration (optional)

**Deliverables:**
- Complete feature set as specified
- Performance optimization
- Comprehensive documentation

### 🧪 Phase 7: Testing & Polish (Week 15-16)
- [ ] Unit tests for all core functionality
- [ ] Integration tests for CLI commands
- [ ] Error handling and user feedback
- [ ] Documentation and examples
- [ ] Performance benchmarks
- [ ] Cross-platform compatibility testing

**Deliverables:**
- Production-ready release (v1.0.0)
- Complete documentation
- Installation instructions

## User Workflows

### 🆕 New User Workflow
1. Install blogr: `cargo install blogr-cli`
2. Set GitHub token: `export GITHUB_TOKEN=your_token_here` 
   - Create token at: https://github.com/settings/tokens
   - Required scopes: `repo`, `workflow`
   - Token persists in shell session/environment
3. Initialize project: `blogr init my-blog`
   - Validates GitHub token before proceeding
   - Creates GitHub repository using API
   - Sets up initial project structure
4. Create first post: `blogr new "Hello World"`
5. Write content in TUI editor with live preview
6. Select theme in configuration TUI if desired
7. Publish to GitHub Pages: `blogr publish`

### ✍️ Regular Writing Workflow
1. Create new post: `blogr new "My New Post"`
2. Write content in TUI with live theme-aware preview
3. Save draft and continue later: `blogr edit my-new-post`
4. When ready, publish: `blogr publish`
5. View live site at GitHub Pages URL

### 🎨 Theme Selection Workflow
1. Open configuration: `blogr config`
2. Navigate to themes section in TUI
3. Browse available themes with live preview
4. Select theme and customize options
5. Save configuration and see changes in editor preview
6. Publish with new theme: `blogr publish`

### 👥 Theme Contribution Workflow
1. Fork blogr repository
2. Follow theme blueprint in `crates/blogr-themes/src/minimal/`
3. Create new theme module following the `Theme` trait
4. Add theme to registry in `lib.rs`
5. Test theme with existing blog posts
6. Submit PR with theme implementation
7. Theme becomes available in next release

## Success Metrics

### 📊 Technical Goals
- [ ] Sub-second site generation for typical blogs (< 50 posts)
- [ ] Memory usage under 50MB during normal operation
- [ ] Cross-platform support (Windows, macOS, Linux)
- [ ] Zero-config deployment to GitHub Pages
- [ ] Intuitive TUI with responsive design

### 👥 User Experience Goals
- [ ] Complete workflow from init to publish in under 5 minutes
- [ ] No external dependencies required (single binary)
- [ ] Clear error messages and helpful suggestions
- [ ] Comprehensive documentation with examples
- [ ] Active community and theme ecosystem

## Future Enhancements

### 🔮 Post-1.0 Features
- [ ] Theme marketplace with community voting/ratings
- [ ] Hot theme reloading during development
- [ ] Theme inheritance system (extend existing themes)
- [ ] Plugin system for extended functionality
- [ ] Multiple deployment targets (Netlify, Vercel, etc.)
- [ ] Content import from other platforms (Medium, Dev.to)
- [ ] Collaborative editing features
- [ ] Advanced SEO tools and analytics
- [ ] Mobile-responsive theme editor
- [ ] Automated social media integration
- [ ] Multi-language support
- [ ] Theme analytics (usage statistics)

### 🔧 Technical Improvements
- [ ] WebAssembly compilation for browser usage
- [ ] Performance optimizations for large sites
- [ ] Advanced caching mechanisms
- [ ] Real-time collaboration features
- [ ] Advanced theme development tools
- [ ] Plugin marketplace integration

## Security Considerations

### 🔒 GitHub Token Handling
- **Environment Variable Only**: Token stored as `GITHUB_TOKEN` env var
- **No File Storage**: Never stored in config files or project directories
- **Validation**: Token validity checked before GitHub operations
- **Scope Requirements**: Requires `repo` and `workflow` scopes for full functionality
- **Error Handling**: Clear messages when token is missing/invalid
- **Documentation**: Clear setup instructions for token creation

### 🛡️ Security Best Practices
- **Token Validation**: Verify token has required scopes on first use
- **Secure Communication**: All GitHub API calls use HTTPS
- **No Token Logging**: Ensure token never appears in logs or error messages
- **Environment Detection**: Check for token in multiple env var formats:
  - `GITHUB_TOKEN` (primary)
  - `GH_TOKEN` (GitHub CLI compatibility)
- **User Guidance**: Provide clear instructions for token setup and troubleshooting

### 🔧 Implementation Details
```rust
// Example token handling in code
fn get_github_token() -> anyhow::Result<String> {
    std::env::var("GITHUB_TOKEN")
        .or_else(|_| std::env::var("GH_TOKEN"))
        .map_err(|_| anyhow::anyhow!(
            "GitHub token not found. Please set GITHUB_TOKEN environment variable.\n\
             Create token at: https://github.com/settings/tokens\n\
             Required scopes: repo, workflow"
        ))
}

fn validate_token(token: &str) -> anyhow::Result<()> {
    // Validate token has required scopes via GitHub API
    // Return helpful error messages for common issues
}

// Project detection and auto-initialization
fn ensure_blogr_project() -> anyhow::Result<PathBuf> {
    if let Some(project_root) = find_project_root()? {
        Ok(project_root)
    } else {
        prompt_auto_initialization()
    }
}

fn find_project_root() -> anyhow::Result<Option<PathBuf>> {
    let mut current = std::env::current_dir()?;
    loop {
        if current.join("blogr.toml").exists() {
            return Ok(Some(current));
        }
        if !current.pop() {
            break;
        }
    }
    Ok(None)
}

fn prompt_auto_initialization() -> anyhow::Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let suggested_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-blog");

    println!("❌ Not in a blogr project directory.");
    println!();
    print!("Would you like to initialize a new blog project here? (y/n): ");
    
    // Handle user input and proceed with initialization
    // Return project root path after successful init
}
```

---

## Error Handling & Edge Cases

### 🔍 Project Detection Logic
- **Search Pattern**: Look for `blogr.toml` starting from current directory, walking up parent directories
- **Git Integration**: Detect if already in a Git repository and handle accordingly
- **Corrupted Config**: Validate and repair corrupted `blogr.toml` files when possible

### 🤝 Auto-Initialization Scenarios

**Scenario 1: `blogr new` in empty directory**
```
❌ Not in a blogr project directory.

Would you like to initialize a new blog project here? (y/n): y
Project name [my-folder]: My Programming Blog
Setting up GitHub repository...
✅ Initialized blog project 'My Programming Blog'

Now creating your first post...
```

**Scenario 2: `blogr new` in existing Git repository**
```
❌ Not in a blogr project directory.
ℹ️  Detected existing Git repository.

Initialize blogr in existing repository? (y/n): y
This will add blogr.toml and posts/ directory.
Continue? (y/n): y
✅ Blogr initialized in existing repository.

Now creating your first post...
```

**Scenario 3: Missing GitHub token during auto-init**
```
❌ Not in a blogr project directory.

Would you like to initialize a new blog project here? (y/n): y
❌ GitHub token not found. Please set GITHUB_TOKEN environment variable.

Options:
1. Set token now and continue: export GITHUB_TOKEN=your_token
2. Initialize locally only (skip GitHub): blogr init --local
3. Cancel and set up token later: Ctrl+C
```

**Scenario 4: `blogr edit` in blogr subdirectory**
```
✅ Found blogr project at: /home/user/my-blog/
Available posts:
1. hello-world
2. rust-tips
3. tui-development

Select post to edit [1-3]: 2
Opening 'rust-tips' in editor...
```

### 🛠 Command-Specific Handling

| Command | Outside Project | Inside Project | In Subdirectory |
|---------|----------------|----------------|-----------------|
| `init` | ✅ Works | ⚠️ Warns about existing project | ⚠️ Warns, suggests location |
| `new` | 🔄 Auto-init prompt | ✅ Works | ✅ Works (finds project root) |
| `edit` | 🔄 Auto-init prompt | ✅ Works | ✅ Works (finds project root) |
| `list` | 🔄 Auto-init prompt | ✅ Works | ✅ Works (finds project root) |
| `publish` | ❌ Error (needs posts) | ✅ Works | ✅ Works (finds project root) |
| `delete` | ❌ Error (needs posts) | ✅ Works | ✅ Works (finds project root) |
| `serve` | ❌ Error (needs site) | ✅ Works | ✅ Works (finds project root) |
| `config` | 🔄 Auto-init prompt | ✅ Works | ✅ Works (finds project root) |

---

**Project Timeline:** 16 weeks
**Target Release:** v1.0.0
**License:** MIT
**Repository:** https://github.com/username/blogr