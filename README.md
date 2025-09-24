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
- Multiple themes: Minimal Retro and Obsidian (supports community themes)
- Full-text search with MiniSearch integration
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

**Install from crates.io:**
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

**4. Choose a theme (optional)**
```bash
# Use default Minimal Retro theme, or switch to Obsidian
blogr theme set obsidian              # For Obsidian community themes
curl -o static/obsidian.css https://raw.githubusercontent.com/kepano/obsidian-minimal/HEAD/obsidian.css
```

**5. Preview your blog**
```bash
blogr serve
# Opens http://localhost:3000
```

**6. Deploy to GitHub Pages**
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

[search]
enabled = true
fields = ["title", "tags", "content"]
exclude = ["drafts/"]
max_content_chars = 2000
excerpt_words = 30
minify = true
lazy_load = true
remove_stopwords = false

# Optional: Custom field boost weights for search scoring
[search.field_boosts]
title = 5.0
tags = 3.0
content = 1.0
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

## Search

Blogr includes a powerful client-side full-text search feature powered by MiniSearch. Search is enabled by default and works entirely in the browser without requiring a server.

### Features

- **Full-text search**: Search across post titles, tags, and content
- **Real-time results**: Instant search results as you type
- **Keyboard navigation**: Use `/` to focus search, arrow keys to navigate, Enter to open
- **Smart excerpts**: Highlighted search terms in result snippets
- **Lazy loading**: Search index and library loaded only when needed (configurable)
- **Responsive design**: Works seamlessly on desktop and mobile

### Usage

Once your site is built and deployed, readers can:

1. **Quick search**: Use the search bar in the site header
2. **Keyboard shortcut**: Press `/` to focus the search input
3. **Navigate results**: Use arrow keys to navigate, Enter to open a result
4. **Tag search**: Click on tags in search results to search by that tag
5. **Clear search**: Press Escape to clear and hide results

### Configuration Options

The search feature can be customized in your `blogr.toml`:

```toml
[search]
# Enable or disable search entirely
enabled = true

# Fields to include in search index
fields = ["title", "tags", "content"]

# Paths to exclude from indexing (relative to posts directory)
exclude = ["drafts/", "private/"]

# Maximum characters to include from post content
max_content_chars = 2000

# Number of words to include in search result excerpts
excerpt_words = 30

# Whether to minify the search index JSON (recommended for production)
minify = true

# Whether to lazy-load search assets (improves initial page load)
lazy_load = true

# Whether to remove common English stopwords (experimental)
remove_stopwords = false

# Custom field boost weights for search scoring
[search.field_boosts]
title = 5.0    # Matches in titles are weighted 5x
tags = 3.0     # Matches in tags are weighted 3x  
content = 1.0  # Matches in content have base weight
```

### Performance

- **Index size**: Typically 10-50KB for small to medium blogs
- **Load time**: < 100ms for initial search setup with lazy loading
- **Search speed**: < 10ms for typical queries on 100+ posts
- **Browser support**: Works in all modern browsers (IE11+)

### Customization

Search UI can be customized by modifying theme templates:

- Search input styling via CSS classes `.search-input`, `.search-results`
- Result item layout in theme templates
- Search behavior by modifying the generated `search.js`

### Troubleshooting

**Search not working?**
- Ensure `search.enabled = true` in your config
- Check that `search_index.json` exists in your `dist/` folder after build
- Verify JavaScript is enabled in the browser

**Large index files?**
- Reduce `max_content_chars` to limit content per post
- Enable `minify = true` to compress the JSON
- Consider adding more paths to `exclude` for draft content

## Themes

**Minimal Retro** (default)
- Clean, artistic design
- Retro color scheme
- Expandable post previews
- Mobile responsive
- Syntax highlighting

**Obsidian** (community themes support)
- Uses authentic Obsidian workspace structure
- Compatible with any Obsidian community theme CSS
- Familiar note-taking interface
- Callouts, backlinks, and embedded content
- Dark/light mode with system detection

**Theme Commands**
```bash
blogr theme list              # Show available themes
blogr theme set minimal-retro # Switch to Minimal Retro theme
blogr theme set obsidian      # Switch to Obsidian theme
blogr theme info obsidian     # Show theme configuration options
```

**Available Themes:**

- **Minimal Retro** - Clean, artistic design with retro aesthetics
- **Obsidian** - Adopts Obsidian community themes for familiar note-taking styling

**Obsidian Theme Setup**

The Obsidian theme allows you to use any Obsidian community theme CSS with your blog:

1. **Switch to Obsidian theme:**
   ```bash
   blogr theme set obsidian
   ```

2. **Add an Obsidian community theme:**
   ```bash
   # Download a popular theme (example: Minimal by @kepano)
   curl -o static/obsidian.css https://raw.githubusercontent.com/kepano/obsidian-minimal/HEAD/obsidian.css
   
   # Or use any other Obsidian theme CSS file
   # Save it as static/obsidian.css in your blog directory
   ```

3. **Configure theme options (optional):**
   ```bash
   blogr config edit
   ```
   ```toml
   [theme.config]
   obsidian_css = "static/obsidian.css"  # Path to your Obsidian CSS
   color_mode = "auto"                   # auto | dark | light
   ```

4. **Build and deploy:**
   ```bash
   blogr build
   blogr deploy
   ```

**Popular Obsidian Themes:**
- **Minimal by @kepano**: `https://raw.githubusercontent.com/kepano/obsidian-minimal/HEAD/obsidian.css`
- **California Coast**: Available in Obsidian community themes
- **Atom**: Available in Obsidian community themes
- **Shimmering Focus**: Available in Obsidian community themes

**Obsidian Theme Features:**
- ✅ Full Obsidian workspace HTML structure
- ✅ Supports any Obsidian community theme CSS
- ✅ Automatic dark/light mode detection
- ✅ Fallback styles when CSS fails to load
- ✅ Obsidian-style callouts, tags, and embeds
- ✅ Compatible with existing Blogr functionality

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
