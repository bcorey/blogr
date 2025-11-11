# Newsletter System

Blogr includes a comprehensive email newsletter system that allows you to collect subscribers, manage them through an approval interface, and send newsletters based on your blog posts or custom content.

## Features

### Subscriber Management
- Email subscription collection via IMAP
- Interactive approval interface for subscriber requests
- Export subscribers to CSV/JSON formats
- Import from popular services (Mailchimp, ConvertKit, Substack, Beehiiv)
- REST API for external integrations

### Newsletter Composition
- Automatic newsletters from latest blog posts
- Custom newsletter creation with Markdown content
- Template-based email rendering
- HTML and text versions generated automatically
- Preview newsletters before sending

### Sending & Delivery
- SMTP integration for reliable email delivery
- Rate limiting to prevent spam issues
- Test email functionality
- Batch sending with progress tracking

### Plugin System
- Extensible plugin architecture
- Custom templates and workflows
- Third-party service integrations
- Hook-based event system

## Setup

### 1. Enable Newsletter in Configuration

Add to your `blogr.toml`:
```toml
[newsletter]
enabled = true
subscribe_email = "subscribe@yourdomain.com"
sender_name = "Your Blog Name"
confirmation_subject = "Welcome to Your Blog Newsletter"

# IMAP Configuration for fetching subscribers
[newsletter.imap]
server = "imap.gmail.com"
port = 993
username = "subscribe@yourdomain.com"
use_tls = true

# SMTP Configuration for sending emails
[newsletter.smtp]
server = "smtp.gmail.com"
port = 587
username = "subscribe@yourdomain.com"
use_tls = true
```

### 2. Set Environment Variables

```bash
# IMAP password for fetching subscription emails
export NEWSLETTER_IMAP_PASSWORD="your-imap-password"

# SMTP password for sending emails
export NEWSLETTER_SMTP_PASSWORD="your-smtp-password"
```

### 3. Create Email Account

Set up a dedicated email address for newsletter subscriptions:
- Gmail: Create app-specific password for IMAP/SMTP access
- Other providers: Ensure IMAP/SMTP access is enabled

## Email Provider Setup

### Gmail
1. Enable 2-factor authentication on your Google account
2. Generate an app-specific password:
   - Go to Google Account settings → Security → App passwords
   - Generate password for "Mail" application
3. Use app-specific password for both IMAP and SMTP

### Outlook/Hotmail
1. Enable 2-factor authentication
2. Generate app password in security settings
3. Use app password for authentication

### Other Providers
- Ensure IMAP and SMTP access is enabled
- Use provider-specific server settings
- Some providers may require app-specific passwords

## API Integration

The newsletter system includes a REST API for external integrations:

**Start API Server:**
```bash
blogr newsletter api-server --port 3001 --api-key your-secret-key
```

**Available Endpoints:**
- `GET /health` - Health check
- `GET /subscribers` - List subscribers
- `POST /subscribers` - Create subscriber
- `GET /subscribers/:email` - Get specific subscriber
- `PUT /subscribers/:email` - Update subscriber
- `DELETE /subscribers/:email` - Remove subscriber
- `GET /stats` - Get newsletter statistics
- `GET /export` - Export subscribers

**Example Usage:**
```bash
# List approved subscribers
curl -H "Authorization: Bearer your-secret-key" \
     "http://localhost:3001/subscribers?status=approved"

# Add new subscriber
curl -X POST -H "Authorization: Bearer your-secret-key" \
     -H "Content-Type: application/json" \
     -d '{"email":"user@example.com","status":"pending"}' \
     http://localhost:3001/subscribers
```

## Plugin Development

Create custom plugins to extend newsletter functionality:

```rust
use blogr_cli::newsletter::{NewsletterPlugin, PluginMetadata, PluginHook};

pub struct MyPlugin {
    metadata: PluginMetadata,
}

impl NewsletterPlugin for MyPlugin {
    fn handles_hook(&self, hook: &PluginHook) -> bool {
        matches!(hook, PluginHook::PreSend | PluginHook::PostSend)
    }

    fn execute_hook(&self, context: &PluginContext) -> Result<PluginResult> {
        // Custom plugin logic
        Ok(PluginResult::success("Plugin executed"))
    }
}
```

**Available Hooks:**
- `PreFetch` - Before fetching subscribers
- `PostFetch` - After fetching subscribers
- `PreApprove` - Before approving subscribers
- `PostApprove` - After approving subscribers
- `PreCompose` - Before composing newsletter
- `PostCompose` - After composing newsletter
- `PreSend` - Before sending newsletter
- `PostSend` - After sending newsletter

## Troubleshooting

**Common Issues:**

1. **IMAP Connection Failed**
   - Verify server settings and credentials
   - Enable "Less secure app access" or use app-specific passwords
   - Check firewall/network connectivity

2. **SMTP Sending Failed**
   - Verify SMTP server settings
   - Check authentication credentials
   - Ensure port is not blocked by firewall

3. **No Subscribers Found**
   - Run `blogr newsletter fetch-subscribers` first
   - Check that subscription emails are being received
   - Verify IMAP folder contains emails

4. **Newsletter Not Rendering**
   - Check that blog posts exist and are published
   - Verify theme templates are working
   - Test with `blogr newsletter draft-latest`

**Additional Resources:**
- For detailed API documentation, see [NEWSLETTER_API.md](NEWSLETTER_API.md)
- For plugin development guide, see [NEWSLETTER_PLUGINS.md](NEWSLETTER_PLUGINS.md)
