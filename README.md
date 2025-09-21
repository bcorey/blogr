# Blogr

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Pages](https://img.shields.io/badge/deploy-GitHub%20Pages-blue.svg)](https://pages.github.com/)

A fast, lightweight static site generator built in Rust for creating and managing blogs. Write in Markdown, preview with a built-in terminal editor, and deploy to GitHub Pages with a single command.

## Features

**Content Creation**
- Write posts in Markdown with YAML frontmatter
- Built-in terminal editor with live preview
- Draft and published post management
- Tag-based organization
- Automatic slug generation

**Site Generation**
- Fast static site builds
- Minimal Retro theme included
- Syntax highlighting for code blocks
- RSS/Atom feeds
- SEO-friendly output

**Development**
- Live reload development server
- Interactive configuration editor
- Project validation and cleanup tools
- Comprehensive CLI commands

**Deployment**
- One-command GitHub Pages deployment
- Custom domain support with CNAME generation
- Automatic git branch management
- Deployment status checking

## Installation

**Requirements**
- Rust 1.70+
- Git (for deployment)
- GitHub account (for GitHub Pages deployment)

**Install from source:**
```bash
git clone https://github.com/bahdotsh/blogr.git
cd blogr
cargo install --path blogr-cli
```

**Install from crates.io:** (coming soon)
```bash
cargo install blogr-cli
```

## Quick Start

**1. Create a new blog**
```bash
blogr init my-blog
cd my-blog
```

**2. Set up GitHub token** (for deployment)
```bash
export GITHUB_TOKEN=your_github_token
```
Get a token at: https://github.com/settings/tokens (needs `repo` and `workflow` scopes)

**3. Create your first post**
```bash
blogr new "Hello World"
```

**4. Preview your blog**
```bash
blogr serve
# Opens http://localhost:3000
```

**5. Deploy to GitHub Pages**
```bash
blogr deploy
```

## Commands

**Project Management**
```bash
blogr init [NAME]           # Create new blog
blogr project info          # Show project details  
blogr project check         # Validate project
blogr project clean         # Clean build files
```

**Content Management**
```bash
blogr new "Post Title"      # Create new post
  --draft                   # Save as draft
  --tags "rust,web"         # Add tags

blogr list                  # List all posts
  --drafts                  # Show only drafts
  --tag rust                # Filter by tag

blogr edit my-post-slug     # Edit existing post
blogr delete my-post-slug   # Delete post
```

**Development**
```bash
blogr serve                 # Start dev server
  --port 8080               # Custom port
  --open                    # Open browser

blogr build                 # Build static site
  --drafts                  # Include drafts
```

**Deployment**
```bash
blogr deploy                # Deploy to GitHub Pages
  --message "Update"        # Custom commit message
```

**Configuration**
```bash
blogr config edit          # Interactive config editor
blogr config get blog.title # Get config value
blogr config set blog.title "My Blog" # Set config value

# Domain setup
blogr config domain set example.com     # Set custom domain
blogr config domain list                # List domains
```

## Terminal Editor

Blogr includes a built-in terminal editor for writing posts:

**Editor Controls**
- `i` - Insert mode (start typing)
- `Esc` - Normal mode  
- `p` - Preview mode
- `Tab` - Switch between editor and preview
- `s` - Save post
- `q` - Quit
- `h` - Show help

**Features**
- Live markdown preview
- Syntax highlighting  
- Split-pane view
- Auto-save

**Configuration Editor**
- `blogr config edit` - Interactive config editor
- `↑/↓` - Navigate settings
- `Enter` - Edit field
- `s` - Save changes

## Project Structure

```
my-blog/
├── blogr.toml              # Configuration
├── posts/                  # Markdown posts
│   ├── welcome.md          # Sample post
│   └── about.md            # About page
├── static/                 # Static files
│   ├── images/
│   ├── css/
│   └── js/
└── .github/workflows/      # GitHub Actions (auto-generated)
    └── deploy.yml
```

## Configuration

Edit `blogr.toml` to configure your blog:

```toml
[blog]
title = "My Blog"
author = "Your Name"
description = "My thoughts and ideas"
base_url = "https://yourusername.github.io/blog"

[theme]
name = "minimal-retro"

[github]
username = "yourusername"
repository = "blog"

[build]
output_dir = "dist"
```

**Custom domains:** Use `blogr config domain set yourdomain.com` to configure.

## Post Format

Posts use Markdown with YAML frontmatter:

```markdown
---
title: "My Blog Post"
date: "2024-01-15"
author: "Your Name"
description: "Brief description"
tags: ["rust", "programming"]
status: "published"
slug: "my-blog-post"
---

# My Blog Post

Your content goes here in **Markdown**.

```rust
fn main() {
    println!("Hello, world!");
}
```
```

## Themes

**Minimal Retro** (included)
- Clean, artistic design
- Retro color scheme
- Expandable post previews
- Mobile responsive
- Syntax highlighting

**Theme Commands**
```bash
blogr theme list        # Show available themes
blogr theme set minimal-retro  # Switch theme
```

**Custom Themes**
Themes are Rust modules in `blogr-themes/src/`. Each theme provides templates, CSS, and configuration options.

## Deployment

**GitHub Pages (recommended)**

1. Set up GitHub token:
   ```bash
   export GITHUB_TOKEN=your_token
   ```
   Get a token at https://github.com/settings/tokens (needs `repo` and `workflow` scopes)

2. Deploy:
   ```bash
   blogr deploy
   ```

Your blog will be available at `https://yourusername.github.io/repository`

**Custom Domains**

1. Configure domain:
   ```bash
   blogr config domain set yourdomain.com
   ```

2. Set up DNS records:
   - **A records** for apex domains: `185.199.108.153`, `185.199.109.153`, `185.199.110.153`, `185.199.111.153`
   - **CNAME record** for subdomains: `yourusername.github.io`

3. Deploy and configure in GitHub repository Settings → Pages → Custom domain

**Manual Deployment**
```bash
blogr build
# Copy contents of `dist/` folder to your web server
```

## Development

**Build from source:**
```bash
git clone https://github.com/bahdotsh/blogr.git
cd blogr
cargo build --release
```

**Run tests:**
```bash
cargo test
```

**Code structure:**
- `blogr-cli/` - Main CLI application
- `blogr-themes/` - Theme system

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Areas to help:**
- New themes
- Features and bug fixes
- Documentation
- Testing

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- [Issues](https://github.com/bahdotsh/blogr/issues)
- [Contributing](CONTRIBUTING.md)

---

Built with Rust 🦀
