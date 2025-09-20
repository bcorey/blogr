# Blogr

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Pages](https://img.shields.io/badge/deploy-GitHub%20Pages-blue.svg)](https://pages.github.com/)

A fast, modern static site generator built in Rust, designed specifically for blogging with an integrated terminal user interface (TUI) and seamless GitHub Pages deployment.

## ✨ Features

### 🚀 Core Features
- **CLI Interface**: Complete command-line interface for all blog operations
- **TUI Editor**: Terminal-based markdown editor with live preview
- **Static Site Generation**: Convert markdown posts to themed HTML sites
- **GitHub Integration**: Automatic repository creation and GitHub Pages deployment
- **Theme System**: Extensible theming with community contribution support
- **Content Management**: Full CRUD operations for blog posts
- **RSS/Atom Feeds**: Automatic feed generation for content syndication

### 🎨 Theming
- **Minimal Retro Theme**: Beautiful, artistic theme featuring:
  - Click-to-expand posts on homepage
  - Artistic typography (Crimson Text, Space Mono)
  - Minimal design with no navigation clutter
  - Warm retro colors and elegant tag bubbles
  - Perfect markdown rendering with syntax highlighting
- **Template Engine**: Tera-based templating with inheritance
- **Asset Management**: Automatic CSS/JS optimization and copying
- **Live Preview**: Theme-aware preview in TUI editor

### 📝 Content Management
- **Markdown Support**: Full markdown with syntax highlighting
- **Frontmatter**: YAML frontmatter for post metadata
- **Draft System**: Draft/published status management
- **Tag System**: Organize posts with tags and generate tag pages
- **SEO Optimization**: Meta tags, sitemap, and feed generation

### 🌐 Deployment
- **GitHub Pages**: One-command deployment to GitHub Pages
- **Custom Domains**: Full domain and subdomain configuration with automatic CNAME file generation
- **Domain Management**: Primary domains, subdomains, and domain aliases support
- **HTTPS Enforcement**: Configurable HTTPS settings for custom domains
- **GitHub Actions**: Automated deployment workflows
- **RSS/Atom Feeds**: Automatic feed generation with domain-aware URLs
- **Status Validation**: Deployment status checking via GitHub API

## 📦 Installation

### Prerequisites
- Rust 1.70 or later
- Git (for version control and deployment)
- GitHub account (for deployment features)

### Install from Source

```bash
git clone https://github.com/bahdotsh/blogr.git
cd blogr
cargo install --path blogr-cli
```

### Install from Crates.io (Coming Soon)

```bash
cargo install blogr-cli
```

## 🚀 Quick Start

### 1. Initialize a New Blog

```bash
# Create a new blog project
blogr init my-blog
cd my-blog

# Or initialize in current directory
blogr init
```

### 2. Set Up GitHub Integration (Optional)

Set your GitHub token for deployment features:

```bash
export GITHUB_TOKEN=your_personal_access_token
```

Create a token at: https://github.com/settings/tokens
Required scopes: `repo`, `workflow`

### 3. Create Your First Post

```bash
# Create a new post
blogr new "My First Post"

# Create a draft post with tags
blogr new "Work in Progress" --draft --tags "rust,blogging"
```

### 4. Preview Your Blog

```bash
# Start development server
blogr serve

# Your blog will be available at http://localhost:3000
```

### 5. Deploy to GitHub Pages

```bash
# Build and deploy to GitHub Pages
blogr deploy

# Your blog will be live at https://username.github.io/repository
```

## 📚 CLI Commands

### Project Management

```bash
blogr init [NAME]              # Initialize new blog project
  --github-username <USER>     # Set GitHub username
  --github-repo <REPO>         # Set repository name
  --no-github                  # Skip GitHub integration

blogr project info            # Show comprehensive project information
blogr project stats           # Detailed analytics and statistics
blogr project check           # Validate project structure, posts, and configuration
blogr project clean           # Clean build artifacts and temporary files
```

### Content Management

```bash
blogr new <TITLE>              # Create new blog post
  --draft                      # Create as draft
  --tags "tag1,tag2"          # Add comma-separated tags
  --slug "custom-slug"        # Custom URL slug

blogr list                     # List all posts
  --drafts                     # Show only drafts
  --published                  # Show only published
  --tag <TAG>                  # Filter by tag
  --sort <date|title|slug>     # Sort order

blogr edit <SLUG>              # Edit existing post
blogr delete <SLUG>            # Delete a post
  --force                      # Skip confirmation
```

### Theme Management

```bash
blogr theme list               # List available themes with active status
blogr theme info <THEME>       # Show detailed theme information and config options
blogr theme set <THEME>        # Change active theme with automatic configuration
blogr theme preview <THEME>    # Preview theme with sample content
```

### Build & Deploy

```bash
blogr build                    # Build static site
  --output <DIR>               # Output directory
  --drafts                     # Include draft posts
  --future                     # Include future-dated posts

blogr serve                    # Development server
  --port <PORT>                # Port number (default: 3000)
  --host <HOST>                # Host address
  --drafts                     # Include drafts
  --open                       # Automatically open browser

blogr deploy                   # Deploy to GitHub Pages
  --branch <BRANCH>            # Deployment branch (default: gh-pages)
  --message <MESSAGE>          # Custom commit message
```

### Configuration Management

```bash
blogr config edit             # Open interactive configuration editor (TUI)
                              # Displays blog info, theme settings, GitHub integration
                              # Provides instructions for configuration modification

blogr config get <KEY>        # Get configuration value
                              # e.g., blogr config get blog.title
                              # e.g., blogr config get domains.primary

blogr config set <KEY> <VALUE> # Set configuration value
                              # e.g., blogr config set blog.title "My Blog"
                              # e.g., blogr config set blog.author "John Doe"
```

### Domain Configuration

```bash
blogr config domain set                    # Set domain interactively
blogr config domain set example.com        # Set primary domain
blogr config domain set blog.example.com   # Set subdomain (auto-detected)
  --subdomain <PREFIX>                      # Explicitly configure as subdomain
  --github-pages                           # Create CNAME file for GitHub Pages
  --enforce-https                          # Enforce HTTPS (default: true)

blogr config domain list                   # List all configured domains
blogr config domain clear                  # Clear all domain configuration
blogr config domain add-alias <DOMAIN>     # Add domain alias
blogr config domain remove-alias <DOMAIN>  # Remove domain alias
```

## 🖥️ TUI Editor

The terminal user interface provides a powerful markdown editor with live preview:

### Navigation
- `i` - Enter insert mode
- `Esc` - Return to normal mode
- `p` - Enter preview mode
- `Tab` - Switch between editor and preview panes
- `s` - Save post
- `q` - Quit (with save prompt)
- `h` or `F1` - Show help overlay

### Features
- **Live Preview**: See your rendered post in real-time with scroll indicator
- **Syntax Highlighting**: Markdown syntax highlighting in editor
- **Theme Integration**: Preview reflects your selected blog theme
- **Split Panes**: Side-by-side editor and preview with dynamic sizing
- **Modal Editing**: Vim-like modal editing for efficiency
- **Smart Navigation**: Height-aware page scrolling and cursor management
- **Auto-Save**: Automatic saving to filesystem with PostManager integration

## 🏗️ Project Structure

When you run `blogr init`, the following structure is created:

```
my-blog/
├── blogr.toml              # Configuration file
├── posts/                  # Blog posts directory
│   ├── welcome.md          # Sample welcome post
│   └── about.md            # About page
├── static/                 # Static assets
│   ├── images/             # Image files
│   ├── css/               # Custom CSS
│   └── js/                # Custom JavaScript
├── themes/                # Custom theme overrides
├── .github/               # GitHub Actions (if enabled)
│   └── workflows/
│       └── deploy.yml     # Auto-deployment workflow
├── .blogr/                # Internal build cache
├── .gitignore             # Git ignore rules
└── README.md              # Project documentation
```

## ⚙️ Configuration

The `blogr.toml` file contains all blog configuration:

```toml
[blog]
title = "My Programming Blog"
author = "John Doe"
description = "Thoughts on programming and software development"
base_url = "https://johndoe.github.io/blog"
language = "en"
timezone = "UTC"

# Optional domain configuration
[blog.domains]
primary = "myblog.com"                    # Primary custom domain
aliases = ["www.myblog.com", "blog.net"]  # Domain aliases
enforce_https = true                      # Enforce HTTPS
github_pages_domain = "myblog.com"        # Domain for CNAME file

# Optional subdomain configuration
[blog.domains.subdomain]
prefix = "blog"                           # Subdomain prefix
base_domain = "mycompany.com"             # Base domain

[theme]
name = "minimal-retro"
[theme.config]
primary_color = "#FF6B35"
secondary_color = "#2D1B0F"
show_reading_time = true
show_author = true

[github]
username = "johndoe"
repository = "blog"
branch = "main"

[build]
output_dir = "_site"
drafts = false
future_posts = false

[dev]
port = 3000
auto_reload = true
```

## 📝 Post Format

Blog posts use markdown with YAML frontmatter:

```yaml
+++
title = "My Blog Post"
date = "2024-01-15T10:30:00Z"
author = "John Doe"
description = "A brief description of the post"
tags = ["rust", "programming", "blogging"]
status = "published"  # or "draft"
slug = "my-blog-post"
featured = false
+++

# My Blog Post

Your content goes here in **Markdown** format.

## Subheading

- List item 1
- List item 2

```rust
fn main() {
    println!("Hello, world!");
}
```
```

## 🎨 Themes

### Built-in Themes

- **Minimal Retro**: A beautiful, artistic theme featuring:
  - **Expandable Posts**: Click-to-expand interface on the homepage
  - **Artistic Typography**: Crimson Text and Space Mono fonts
  - **Minimal Design**: No navigation clutter, focus on content
  - **Warm Retro Colors**: Burnt orange accents on cream background
  - **Tag Bubbles**: Elegant tag system with artistic styling
  - **Mobile Responsive**: Beautiful on all devices
  - **Markdown Excellence**: Perfect rendering of all markdown elements

- More themes coming soon!

### Theme Development

Themes are managed in the `blogr-themes` crate. Each theme implements the `Theme` trait:

```rust
pub trait Theme {
    fn info(&self) -> ThemeInfo;
    fn templates(&self) -> HashMap<String, &'static str>;
    fn assets(&self) -> HashMap<String, &'static [u8]>;
    fn preview_tui_style(&self) -> TuiThemeStyle;
}
```

### Contributing Themes

1. Fork the repository
2. Create your theme in `blogr-themes/src/your_theme/`
3. Follow the existing theme structure
4. Add your theme to the registry
5. Submit a pull request

## 🚀 Deployment

### GitHub Pages

Blogr provides seamless GitHub Pages deployment:

1. **Automatic Setup**: GitHub repository and workflow creation
2. **One-Command Deploy**: `blogr deploy` handles everything
3. **Custom Domains**: Automatic CNAME file generation
4. **Status Validation**: Deployment status checking
5. **Branch Management**: Orphan branch creation for gh-pages

### GitHub Actions

When GitHub integration is enabled, Blogr creates a workflow that:

- Builds your site on every push to main
- Deploys automatically to GitHub Pages
- Runs tests on pull requests
- Caches dependencies for faster builds

### Manual Deployment

You can also deploy manually to any hosting provider:

```bash
blogr build
# Copy _site/ directory to your hosting provider
```

## 🔧 Development

### Building from Source

```bash
git clone https://github.com/bahdotsh/blogr.git
cd blogr
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
cargo clippy
cargo fmt
```

### Project Structure

```
blogr/
├── Cargo.toml              # Workspace configuration
├── blogr-cli/              # Main CLI application
│   ├── src/
│   │   ├── main.rs         # CLI entry point
│   │   ├── cli/            # Command implementations
│   │   ├── tui/            # Terminal user interface
│   │   ├── generator/      # Static site generation
│   │   ├── config/         # Configuration management
│   │   └── content/        # Content management
│   └── templates/          # Project templates
├── blogr-themes/           # Themes crate
│   └── src/
│       ├── lib.rs          # Theme registry
│       └── minimal_retro/  # Built-in theme
└── README.md               # This file
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas for Contribution

- **Themes**: Create new themes for different styles
- **Features**: Add new functionality
- **Documentation**: Improve docs and examples
- **Testing**: Add tests and improve coverage
- **Performance**: Optimize build times and memory usage

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Documentation](https://docs.rs/blogr-cli)
- [Issue Tracker](https://github.com/bahdotsh/blogr/issues)
- [Discussions](https://github.com/bahdotsh/blogr/discussions)
- [Changelog](CHANGELOG.md)

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Terminal UI powered by [Ratatui](https://ratatui.rs/)
- Markdown processing with [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)
- Syntax highlighting via [syntect](https://github.com/trishume/syntect)
- Template engine using [Tera](https://tera.netlify.app/)
- Git operations with [git2](https://github.com/rust-lang/git2-rs)

## 🎉 Recent Updates

### v0.3.0 - Domain Configuration & Management
- 🌐 **Domain Configuration**: Full support for custom domains and subdomains
- 🏷️ **Domain Management**: Add, remove, and list domain aliases with CLI commands
- 🔒 **HTTPS Enforcement**: Configurable HTTPS settings for custom domains
- 📝 **CNAME Generation**: Automatic CNAME file creation for GitHub Pages
- 🔗 **URL Integration**: Domain-aware RSS/Atom feeds and site generation
- ⚙️ **Interactive Setup**: Smart domain detection and user-friendly configuration
- 🎯 **GitHub Pages**: Seamless integration with GitHub Pages custom domains

### v0.2.0 - Enhanced Functionality
- ✅ **Complete TODO Resolution**: All planned features implemented
- 🎨 **Enhanced Theme System**: Full theme management with preview capabilities
- 🔧 **Project Validation**: Comprehensive project structure and content validation
- 🧹 **Smart Cleanup**: Intelligent build artifact and temporary file cleanup
- 💾 **Reliable Saving**: PostManager integration for filesystem persistence
- 📏 **Dynamic UI**: Height-aware scrolling and responsive TUI components
- 🌐 **Browser Integration**: Automatic browser opening for development server
- ⚙️ **Configuration Management**: Interactive configuration display and guidance

### Code Quality Improvements
- 🦀 **Rust Best Practices**: Clippy-clean codebase following Rust conventions
- 🎯 **Zero Warnings**: All compiler and linter warnings resolved
- 📝 **Consistent Formatting**: rustfmt applied across entire codebase
- 🔒 **Type Safety**: Enhanced error handling and type safety throughout

---

**Made with ❤️ and Rust**
