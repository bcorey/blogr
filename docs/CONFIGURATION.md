# Blogr Configuration

This document covers all configuration options for Blogr.

## Basic Configuration

Edit `blogr.toml` to configure your site. Use `blogr config edit` for an interactive editor.

### Basic Settings
```toml
[blog]
title = "My Blog"
author = "Your Name"
description = "My thoughts and ideas"
base_url = "https://yourusername.github.io/blog"

[theme]
name = "minimal-retro"  # or "dark-minimal" for personal sites

[github]
username = "yourusername"
repository = "blog"

[site]
site_type = "blog"  # or "personal" for portfolio sites
```

## Theme Configuration

### Dark Minimal Theme
```toml
[theme]
name = "dark-minimal"

[theme.config]
# Colors
primary_color = "#00ff88"           # Neon green accent
secondary_color = "#ff00ff"         # Magenta accent
accent_color = "#00d4ff"            # Cyan accent
background_color = "#0a0a0a"        # Pure dark background
text_color = "#e0e0e0"              # Soft white text

# Typography
font_family = "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif"

# Features
enable_animations = true            # Smooth animations
show_social_icons = true            # Display social media links

# Status Bar (customizable availability indicator)
show_status_bar = true              # Show/hide status bar
status_text = "Available for opportunities"  # Custom text
status_color = "#00ff88"            # Dot color (any hex code)
```

**Status Bar Examples:**
```toml
# Available for work
status_text = "Available for hire"
status_color = "#00ff88"  # Green

# Currently busy
status_text = "Currently working on exciting projects"
status_color = "#00d4ff"  # Cyan

# Not available
status_text = "Not accepting new projects"
status_color = "#ff4444"  # Red

# Custom message
status_text = "Building cool stuff"
status_color = "#ff00ff"  # Magenta

# Hide status bar
show_status_bar = false
```

### Typewriter Theme
```toml
[theme]
name = "typewriter"

[theme.config]
# Colors
paper_color = "#f4f1e8"           # Vintage cream paper
ink_color = "#2b2b2b"              # Dark charcoal ink
accent_color = "#8b4513"           # Vintage brown accent

# Typography
font_family = "'Courier Prime', 'Courier New', monospace"

# Visual Effects
show_paper_texture = true          # Subtle paper texture overlay
typing_animation = true            # Typewriter typing animation
show_date_stamp = true             # Vintage date stamp
cursor_blink = true                # Blinking cursor effect
```

## Search Configuration

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

## Newsletter Configuration

### Basic Newsletter Settings
```toml
[newsletter]
# Enable/disable newsletter functionality
enabled = true

# Email address for newsletter subscriptions
subscribe_email = "subscribe@yourdomain.com"

# Name displayed in newsletter emails
sender_name = "Your Blog Name"

# Subject line for confirmation emails
confirmation_subject = "Welcome to Your Blog Newsletter"
```

### IMAP Configuration
```toml
[newsletter.imap]
server = "imap.gmail.com"     # IMAP server address
port = 993                    # IMAP port (993 for SSL, 143 for non-SSL)
username = "your-email@domain.com"  # IMAP username
use_tls = true               # Enable TLS/SSL encryption
```

### SMTP Configuration
```toml
[newsletter.smtp]
server = "smtp.gmail.com"     # SMTP server address
port = 587                    # SMTP port (587 for TLS, 465 for SSL, 25 for non-encrypted)
username = "your-email@domain.com"  # SMTP username
use_tls = true               # Enable TLS/SSL encryption
```

### Plugin Configuration
```toml
[newsletter.plugins.analytics]
enabled = true
api_key = "your-analytics-api-key"
endpoint = "https://api.analytics.com"

[newsletter.plugins.webhook]
enabled = true
url = "https://yoursite.com/webhook"
events = ["subscriber_approved", "newsletter_sent"]
```

## Custom Domains

Use `blogr config domain set yourdomain.com` to set up custom domains.

### DNS Configuration
- **A records** for apex domains: `185.199.108.153`, `185.199.109.153`, `185.199.110.153`, `185.199.111.153`
- **CNAME record** for subdomains: `yourusername.github.io`

### GitHub Pages Setup
1. Configure domain: `blogr config domain set yourdomain.com`
2. Deploy: `blogr deploy`
3. Configure in GitHub repository Settings → Pages → Custom domain
