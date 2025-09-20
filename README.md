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
blogr config edit             # Open interactive TUI configuration editor
                              # Full-screen interface with live editing
                              # Navigate with ↑/↓, Enter to edit, 's' to save

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

## 🖥️ TUI Interfaces

Blogr provides multiple terminal user interfaces for different tasks:

### 📝 Post Editor

The markdown editor TUI provides a powerful editing experience with live preview:

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

### ⚙️ Configuration Editor

The interactive configuration editor TUI allows you to modify all blog settings:

#### Navigation
- `↑/↓` - Navigate between configuration fields
- `Enter` - Edit selected field
- `Esc` - Cancel edit
- `s` - Save configuration
- `q` - Quit (with save prompt)
- `h` or `F1` - Show help overlay

#### Features
- **Categorized Fields**: Settings organized by Blog, Theme, Domain, Build, and Development
- **Live Validation**: Type checking for boolean and numeric fields
- **Visual Feedback**: Current values displayed with modification indicators
- **Smart Editing**: Context-aware input validation and help text
- **Theme Integration**: Uses your blog's color scheme for consistent experience

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

### Understanding Base URL

The **Base URL** is the complete web address (URL) where your blog will be publicly accessible on the internet. Think of it as your blog's "home address" on the web.

#### What is a Base URL?

A base URL consists of:
- **Protocol**: `https://` (or `http://`, though HTTPS is recommended)
- **Domain**: The website address (e.g., `myblog.com`, `username.github.io`)
- **Path** (optional): Additional path segments (e.g., `/blog`, `/my-project`)

#### Why is it Important?

Blogr uses your base URL to generate:
- **Absolute links** in RSS/Atom feeds (so feed readers can find your posts)
- **Canonical URLs** for SEO (helps search engines understand your content's primary location)
- **Social media sharing links** (when people share your posts)
- **Sitemap entries** (for search engine indexing)
- **Internal navigation** (ensuring all links work correctly)

#### Common Examples:

**GitHub Pages (default):**
```
https://username.github.io/repository-name
```
- `username`: Your GitHub username
- `repository-name`: Your blog's repository name

**Custom domain:**
```
https://myblog.com
```
- Simple, professional-looking address

**Subdomain:**
```
https://blog.mycompany.com
```
- Great for company blogs or when you want to separate your blog from your main site

**With path:**
```
https://mywebsite.com/blog
```
- When your blog is part of a larger website

#### Configuration Notes:

- When you configure custom domains in Blogr, the system automatically uses your domain configuration instead of the base URL for generating links
- Always include the protocol (`https://` or `http://`)
- Don't include a trailing slash (`/`) at the end
- Make sure the URL matches exactly where your blog will be accessible

```toml
[blog]
title = "My Programming Blog"
author = "John Doe"
description = "Thoughts on programming and software development"
base_url = "https://johndoe.github.io/blog"  # Your blog's web address
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

Blogr provides comprehensive deployment solutions with seamless GitHub Pages integration and custom domain support.

### Quick Deployment

Deploy your blog to GitHub Pages with a single command:

```bash
blogr deploy
```

This command will:
- Build your static site
- Create/update the `gh-pages` branch
- Push to GitHub
- Create CNAME file for custom domains
- Validate deployment status

### GitHub Pages Setup

#### Prerequisites

1. **GitHub Token**: Set up a personal access token for deployment
   ```bash
   export GITHUB_TOKEN=your_personal_access_token
   ```
   
   Create a token at: https://github.com/settings/tokens
   
   **Required scopes**: `pages`, `workflow`, `actions` with read and write permissions

2. **Git Repository**: Initialize your blog as a Git repository
   ```bash
   git init
   git remote add origin git@github.com:username/repository-name.git
   ```

#### Basic GitHub Pages Deployment

For a basic GitHub Pages deployment (username.github.io/repository):

1. **Initialize with GitHub integration**:
   ```bash
   blogr init my-blog --github-username yourusername --github-repo my-blog
   ```

2. **Configure your blog**:
   ```bash
   cd my-blog
   blogr config set blog.base_url "https://yourusername.github.io/my-blog"
   ```

3. **Deploy**:
   ```bash
   blogr deploy
   ```

Your blog will be available at `https://yourusername.github.io/my-blog`

### Custom Domain Configuration

#### Setting Up Custom Domains

Blogr supports both primary domains and subdomains with automatic CNAME file generation.

##### Option 1: Using CLI Commands

**For a primary domain** (e.g., `myblog.com`):
```bash
blogr config domain set myblog.com
```

**For a subdomain** (e.g., `blog.example.com`):
```bash
blogr config domain set blog.example.com
```

**For explicit subdomain configuration**:
```bash
blogr config domain set blog.example.com --subdomain blog
```

##### Option 2: Manual Configuration

Edit your `blogr.toml` file:

```toml
[blog]
base_url = "https://blog.example.com"

[blog.domains]
# For primary domains
primary = "myblog.com"                    # Primary custom domain
github_pages_domain = "myblog.com"        # Domain for CNAME file

# OR for subdomains
[blog.domains.subdomain]
prefix = "blog"                           # Subdomain prefix
base_domain = "example.com"               # Base domain
github_pages_domain = "blog.example.com"  # Domain for CNAME file

# Optional settings
[blog.domains]
aliases = ["www.myblog.com"]              # Domain aliases
enforce_https = true                      # Enforce HTTPS (default)
```

#### DNS Configuration

Configure your DNS provider to point to GitHub Pages:

**For apex domains** (e.g., `myblog.com`):
```
Type: A
Name: @
Value: 185.199.108.153
       185.199.109.153
       185.199.110.153
       185.199.111.153
```

**For subdomains** (e.g., `blog.example.com`):
```
Type: CNAME
Name: blog
Value: yourusername.github.io
```

**For www subdomain**:
```
Type: CNAME
Name: www
Value: yourusername.github.io
```

#### GitHub Pages Repository Settings

**Critical Step**: After deploying with a custom domain, you must configure GitHub Pages in your repository settings:

1. **Go to your GitHub repository**: `https://github.com/yourusername/your-repo`
2. **Click "Settings"** tab
3. **Scroll to "Pages"** section in the left sidebar
4. **In "Custom domain" field**, enter your domain (e.g., `blog.example.com`)
5. **Click "Save"**
6. **Wait for DNS verification** (GitHub will verify your DNS configuration)

> **Important**: The CNAME file created by blogr is not sufficient alone. You must also configure the custom domain in GitHub's repository settings interface.

#### Deployment Process for Custom Domains

1. **Configure your domain**:
   ```bash
   blogr config domain set blog.example.com
   ```

2. **Deploy your site**:
   ```bash
   blogr deploy
   ```

3. **Configure GitHub Pages settings** (in GitHub web interface):
   - Repository Settings → Pages → Custom domain: `blog.example.com`

4. **Verify DNS configuration** (wait for propagation):
   ```bash
   dig blog.example.com
   ```

Your blog will be available at `https://blog.example.com`

### Advanced Deployment Options

#### Custom Deployment Branch

Deploy to a different branch:
```bash
blogr deploy --branch production
```

#### Custom Commit Message

Add a custom commit message:
```bash
blogr deploy --message "Deploy version 1.2.0"
```

#### Including Drafts and Future Posts

```bash
blogr build --drafts --future
blogr deploy
```

### GitHub Actions Automation

When GitHub integration is enabled, Blogr creates a workflow that:

- **Builds automatically**: On every push to main branch
- **Deploys automatically**: To GitHub Pages
- **Runs tests**: On pull requests
- **Caches dependencies**: For faster builds
- **Handles secrets**: Uses repository secrets for tokens

The workflow file is created at `.github/workflows/deploy.yml`:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ main ]
  workflow_dispatch:

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Build and Deploy
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      run: |
        cargo install --path blogr-cli
        blogr deploy
```

### Deployment Status Monitoring

Check your deployment status:
```bash
blogr deploy
# Automatically checks GitHub Pages status after deployment
```

Manual status check:
```bash
# Check if GitHub Pages is properly configured
curl -H "Authorization: Bearer $GITHUB_TOKEN" \
     https://api.github.com/repos/username/repo/pages
```

### Manual Deployment

For other hosting providers, build and copy the static files:

```bash
# Build static site
blogr build

# Output directory contains your site
ls _site/
# Copy _site/ directory to your hosting provider
```

### Deployment Troubleshooting

#### Common Issues

**1. Site shows 404 or wrong URL**
- **Problem**: GitHub Pages not configured for custom domain
- **Solution**: Configure custom domain in repository Settings → Pages

**2. Styles and CSS not loading (blank/unstyled site)**
- **Problem**: Asset URLs being HTML-encoded in templates
- **Solution**: This is fixed in the latest version. Ensure you're using relative paths for assets
- **Technical**: Template functions now use the `| safe` filter to prevent HTML escaping

**3. CNAME file missing**
- **Problem**: Custom domain not properly configured in blogr.toml
- **Solution**: Use `blogr config domain set yourdomain.com`

**4. DNS not resolving**
- **Problem**: DNS records not configured or propagation delay
- **Solution**: Check DNS configuration and wait for propagation (up to 24 hours)

**5. HTTPS not working**
- **Problem**: GitHub Pages HTTPS not enabled or DNS issues
- **Solution**: Ensure DNS is correct and enable HTTPS in GitHub Pages settings

**6. Deploy fails with authentication error**
- **Problem**: GitHub token missing or insufficient permissions
- **Solution**: Set `GITHUB_TOKEN` with `pages`, `workflow`, `actions` permissions

#### Debug Commands

```bash
# Check current configuration
blogr config get blog.base_url
blogr config domain list

# Validate project structure
blogr project check

# Test DNS resolution
dig yourdomain.com
nslookup yourdomain.com

# Check GitHub Pages API
curl -H "Authorization: Bearer $GITHUB_TOKEN" \
     https://api.github.com/repos/username/repo/pages
```

#### Getting Help

If you encounter deployment issues:

1. **Check the logs**: Deployment output shows detailed error messages
2. **Validate configuration**: Use `blogr project check`
3. **Verify DNS**: Use online DNS checkers
4. **GitHub Status**: Check GitHub Pages status at https://www.githubstatus.com/
5. **Community Support**: Open an issue at https://github.com/bahdotsh/blogr/issues

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

### v0.3.1 - Interactive Configuration Editor
- 🖥️ **Interactive TUI Config**: Full-screen configuration editor with live editing
- 📝 **Intuitive Navigation**: Arrow key navigation with Enter to edit
- 🎨 **Categorized Settings**: Fields organized by Blog, Theme, Domain, Build, Dev
- ✅ **Live Validation**: Smart input validation for different field types
- 💾 **Safe Editing**: Save prompts and modification indicators
- 📖 **Built-in Help**: Comprehensive help system with F1 key
- 🌈 **Theme Integration**: Editor uses your blog's color scheme

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
