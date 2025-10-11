# Blogr Commands

This document provides a comprehensive reference for all Blogr CLI commands.

## Project Management

### Create a new blog or personal website
```bash
blogr init my-blog                    # Create new blog
blogr init --personal my-portfolio    # Create personal website
blogr init --github-username USER --github-repo REPO  # Set GitHub details
```

### Project information
```bash
blogr project info                    # Show project details
blogr project check                   # Validate project
blogr project clean                   # Clean build files
```

## Content Management

### Create and manage posts
```bash
blogr new "My Post Title"             # Create new post
blogr new "Draft Post" --draft        # Create draft post
blogr new "Tagged Post" --tags "rust,web"  # Create post with tags
```

### List and edit posts
```bash
blogr list                            # List all posts
blogr list --drafts                   # Show only drafts
blogr list --tag rust                 # Filter by tag
blogr edit my-post-slug               # Edit existing post
blogr delete my-post-slug             # Delete post
```

## Development

### Development server
```bash
blogr serve                           # Start dev server (localhost:3000)
blogr serve --port 8080              # Custom port
blogr serve --open                    # Open browser automatically
```

### Build static site
```bash
blogr build                           # Build static site
blogr build --drafts                  # Include drafts in build
```

## Deployment

### Deploy to GitHub Pages
```bash
blogr deploy                          # Deploy to GitHub Pages
blogr deploy --message "Update"       # Custom commit message
```

## Configuration

### Interactive configuration
```bash
blogr config edit                     # Interactive config editor
blogr config get blog.title           # Get config value
blogr config set blog.title "My Blog" # Set config value
```

### Domain setup
```bash
blogr config domain set example.com   # Set custom domain
blogr config domain list              # List domains
```

## Newsletter Commands

### Subscriber Management
```bash
# Fetch new subscribers from email inbox
blogr newsletter fetch-subscribers
blogr newsletter fetch-subscribers --interactive  # Configure IMAP interactively

# Launch approval UI to manage subscriber requests
blogr newsletter approve

# List all subscribers
blogr newsletter list
blogr newsletter list --status approved    # Filter by status
blogr newsletter list --status pending

# Remove a subscriber
blogr newsletter remove user@example.com
blogr newsletter remove user@example.com --force  # Skip confirmation

# Export subscribers
blogr newsletter export --format csv --output subscribers.csv
blogr newsletter export --format json --status approved
```

### Newsletter Creation & Sending
```bash
# Send newsletter with latest blog post
blogr newsletter send-latest
blogr newsletter send-latest --interactive  # Interactive confirmation

# Send custom newsletter
blogr newsletter send-custom "Weekly Update" "# This Week\n\nHere's what's new..."
blogr newsletter send-custom "Weekly Update" "content" --interactive

# Preview newsletters without sending
blogr newsletter draft-latest                    # Preview latest post
blogr newsletter draft-custom "Subject" "Content"  # Preview custom content

# Send test email
blogr newsletter test user@example.com
blogr newsletter test user@example.com --interactive
```

### Import & Export
```bash
# Import from popular services
blogr newsletter import --source mailchimp subscribers.csv
blogr newsletter import --source convertkit subscribers.json
blogr newsletter import --source substack subscribers.csv
blogr newsletter import --source beehiiv subscribers.csv
blogr newsletter import --source generic subscribers.csv

# Preview imports before applying
blogr newsletter import --source mailchimp --preview subscribers.csv
blogr newsletter import --source generic --preview --email-column "email" --name-column "name" subscribers.csv

# Custom column mapping for generic CSV
blogr newsletter import --source generic \
  --email-column "email_address" \
  --name-column "full_name" \
  --status-column "subscription_status" \
  subscribers.csv
```

### Plugin System
```bash
# List available plugins
blogr newsletter plugin list

# Get plugin information
blogr newsletter plugin info analytics-plugin

# Enable/disable plugins
blogr newsletter plugin enable webhook-plugin
blogr newsletter plugin disable analytics-plugin

# Run custom plugin commands
blogr newsletter plugin run sync-external
blogr newsletter plugin run generate-report pdf
```

### API Server
```bash
# Start API server for external integrations
blogr newsletter api-server

# Custom configuration
blogr newsletter api-server --port 8080 --host 0.0.0.0
blogr newsletter api-server --api-key your-secret-key
blogr newsletter api-server --port 3001 --no-cors
```

## Theme Commands

### Theme management
```bash
# List all available themes
blogr theme list

# Switch to a specific theme
blogr theme set minimal-retro     # Blog theme
blogr theme set obsidian          # Blog theme
blogr theme set terminal-candy    # Blog theme
blogr theme set dark-minimal      # Personal theme
blogr theme set musashi          # Personal theme
blogr theme set slate-portfolio  # Personal theme
blogr theme set typewriter       # Personal theme

# Get theme information
blogr theme info typewriter      # Show theme configuration options
```
