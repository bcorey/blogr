# Blogr Themes

Blogr comes with multiple built-in themes, each designed for different purposes.

## Blog Themes

### Minimal Retro (default for blogs)
- Clean, artistic design
- Retro color scheme
- Expandable post previews
- Mobile responsive
- Syntax highlighting

### Obsidian (community themes support)
- Uses authentic Obsidian workspace structure
- Compatible with any Obsidian community theme CSS
- Familiar note-taking interface
- Callouts, backlinks, and embedded content
- Dark/light mode with system detection

### Terminal Candy (quirky personal sites)
- Terminal-inspired design with pastel colors
- Glitch effects and playful animations
- ASCII art decorations
- Typewriter animations
- Perfect for creative personal websites

## Personal Website Themes

### Dark Minimal (default for personal sites)
- Dark minimalist-maximalist aesthetic
- Cyberpunk/brutalist design
- Neon accent colors (green, magenta, cyan)
- Animated grid background
- Auto-glitching gradient text
- Geometric shapes and clip-paths
- Dramatic shadows and hover effects
- Customizable status bar
- Perfect for portfolios and personal brands

### Musashi (dynamic personal sites)
- Modern dynamic content loading
- Smooth animations and transitions
- Clean typography
- Responsive design
- Perfect for personal websites and project showcases

### Slate Portfolio (professional portfolios)
- Modern glassmorphic design
- Frosted glass effects
- Elegant transitions
- Professional layout
- Perfect for freelancers and professionals

### Typewriter (NEW in v0.4.0 - vintage personal sites)
- Vintage typewriter-inspired aesthetics
- Cream paper background with subtle texture
- Monospace Courier font family
- Typewriter typing animation for title
- Blinking cursor effect
- Vintage date stamp
- Typewriter-style line separators
- Perfect for writers, bloggers, and literary portfolios

## Obsidian Theme Setup

The Obsidian theme allows you to use any Obsidian community theme CSS with your blog:

**1. Switch to Obsidian theme:**
```bash
blogr theme set obsidian
```

**2. Add an Obsidian community theme:**
```bash
# Download a popular theme (example: Minimal by @kepano)
curl -o static/obsidian.css https://raw.githubusercontent.com/kepano/obsidian-minimal/HEAD/obsidian.css

# Or use any other Obsidian theme CSS file
# Save it as static/obsidian.css in your blog directory
```

**3. Configure theme options (optional):**
```bash
blogr config edit
```
```toml
[theme.config]
obsidian_css = "static/obsidian.css"  # Path to your Obsidian CSS
color_mode = "auto"                   # auto | dark | light
```

**4. Build and deploy:**
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

## Custom Themes

Themes are Rust modules in `blogr-themes/src/`. Each theme provides templates, CSS, and configuration options.

## Available Themes Summary

**Blog Themes:**
- **Minimal Retro** - Clean, artistic design with retro aesthetics
- **Obsidian** - Adopts Obsidian community themes for familiar note-taking styling
- **Terminal Candy** - Quirky terminal-inspired theme with pastel colors

**Personal Website Themes:**
- **Dark Minimal** - Dark minimalist-maximalist with cyberpunk aesthetics
- **Musashi** - Dynamic modern theme with smooth animations
- **Slate Portfolio** - Glassmorphic professional portfolio theme
- **Typewriter** - Vintage typewriter aesthetics with nostalgic charm
