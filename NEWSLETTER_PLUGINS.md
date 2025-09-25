# Newsletter Plugin Development Guide

This document provides a comprehensive guide for developing plugins for the Blogr Newsletter system. The plugin system allows developers to extend newsletter functionality with custom features, integrations, and workflows.

## Table of Contents

1. [Plugin Architecture Overview](#plugin-architecture-overview)
2. [Getting Started](#getting-started)
3. [Plugin Trait Implementation](#plugin-trait-implementation)
4. [Plugin Hooks](#plugin-hooks)
5. [Configuration Management](#configuration-management)
6. [Custom Commands](#custom-commands)
7. [Custom Templates](#custom-templates)
8. [Plugin Context](#plugin-context)
9. [Error Handling](#error-handling)
10. [Testing Plugins](#testing-plugins)
11. [Plugin Examples](#plugin-examples)
12. [Best Practices](#best-practices)
13. [Distribution](#distribution)

## Plugin Architecture Overview

The Blogr Newsletter plugin system is built around the `NewsletterPlugin` trait, which provides a standardized interface for extending newsletter functionality. Plugins can:

- Hook into various points in the newsletter workflow
- Provide custom CLI commands
- Render custom email templates
- Access and modify subscriber data
- Integrate with external services

### Core Components

- **PluginManager**: Manages plugin lifecycle and execution
- **NewsletterPlugin**: Main trait that plugins must implement
- **PluginContext**: Provides access to system state and data
- **PluginHook**: Defines integration points in the newsletter workflow
- **PluginResult**: Standard return type for plugin operations

## Getting Started

### Prerequisites

- Rust development environment
- Access to the Blogr Newsletter codebase
- Basic understanding of the newsletter system

### Creating Your First Plugin

1. **Create a new Rust library**:
```bash
cargo new my-newsletter-plugin --lib
cd my-newsletter-plugin
```

2. **Add dependencies to `Cargo.toml`**:
```toml
[dependencies]
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
# Add blogr-cli as a dependency (adjust path as needed)
blogr-cli = { path = "../blogr/blogr-cli" }
```

3. **Implement the plugin**:
```rust
// src/lib.rs
use anyhow::Result;
use blogr_cli::newsletter::{
    NewsletterPlugin, PluginMetadata, PluginConfig, PluginHook, 
    PluginContext, PluginResult, Newsletter
};
use std::collections::HashMap;

pub struct MyNewsletterPlugin {
    metadata: PluginMetadata,
    config: Option<PluginConfig>,
}

impl MyNewsletterPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "my-newsletter-plugin".to_string(),
                version: "1.0.0".to_string(),
                author: "Your Name".to_string(),
                description: "A sample newsletter plugin".to_string(),
                homepage: Some("https://github.com/yourname/my-newsletter-plugin".to_string()),
                repository: Some("https://github.com/yourname/my-newsletter-plugin".to_string()),
                license: Some("MIT".to_string()),
                keywords: vec!["newsletter".to_string(), "email".to_string()],
                dependencies: Vec::new(),
                min_blogr_version: Some("0.2.0".to_string()),
            },
            config: None,
        }
    }
}

impl NewsletterPlugin for MyNewsletterPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn initialize(&mut self, config: &PluginConfig) -> Result<()> {
        self.config = Some(config.clone());
        println!("MyNewsletterPlugin initialized!");
        Ok(())
    }

    fn handles_hook(&self, hook: &PluginHook) -> bool {
        matches!(hook, PluginHook::PreSend | PluginHook::PostSend)
    }

    fn execute_hook(&self, context: &PluginContext) -> Result<PluginResult> {
        match context.hook {
            PluginHook::PreSend => {
                println!("About to send newsletter!");
                Ok(PluginResult {
                    success: true,
                    message: Some("Pre-send hook executed".to_string()),
                    data: HashMap::new(),
                    modified_newsletter: None,
                    modified_subscribers: None,
                })
            }
            PluginHook::PostSend => {
                println!("Newsletter sent!");
                Ok(PluginResult {
                    success: true,
                    message: Some("Post-send hook executed".to_string()),
                    data: HashMap::new(),
                    modified_newsletter: None,
                    modified_subscribers: None,
                })
            }
            _ => Ok(PluginResult {
                success: false,
                message: Some("Hook not supported".to_string()),
                data: HashMap::new(),
                modified_newsletter: None,
                modified_subscribers: None,
            })
        }
    }
}
```

## Plugin Trait Implementation

### Required Methods

#### `metadata() -> &PluginMetadata`
Returns plugin metadata including name, version, author, and description.

#### `initialize(&mut self, config: &PluginConfig) -> Result<()>`
Called when the plugin is loaded. Use this to:
- Parse plugin configuration
- Initialize external connections
- Validate dependencies
- Set up internal state

#### `handles_hook(&self, hook: &PluginHook) -> bool`
Returns whether the plugin handles a specific hook. This is called before `execute_hook` to determine if the plugin should be invoked.

#### `execute_hook(&self, context: &PluginContext) -> Result<PluginResult>`
Main plugin execution method. Called when a supported hook is triggered.

### Optional Methods

#### `custom_commands(&self) -> Vec<String>`
Returns a list of custom CLI commands provided by the plugin.

#### `execute_command(&self, command: &str, args: &[String], context: &PluginContext) -> Result<PluginResult>`
Executes a custom CLI command.

#### `custom_templates(&self) -> Vec<String>`
Returns a list of custom email templates provided by the plugin.

#### `render_template(&self, template: &str, newsletter: &Newsletter, context: &PluginContext) -> Result<Newsletter>`
Renders a custom email template.

## Plugin Hooks

Plugins can hook into various points in the newsletter workflow:

### Available Hooks

- **PreFetch**: Before fetching subscribers from IMAP
- **PostFetch**: After fetching subscribers from IMAP
- **PreApprove**: Before approving subscribers
- **PostApprove**: After approving subscribers
- **PreCompose**: Before composing newsletter
- **PostCompose**: After composing newsletter
- **PreSend**: Before sending newsletter
- **PostSend**: After sending newsletter
- **CustomCommand**: For custom CLI commands
- **CustomTemplate**: For custom email templates

### Hook Usage Examples

```rust
fn execute_hook(&self, context: &PluginContext) -> Result<PluginResult> {
    match context.hook {
        PluginHook::PreSend => {
            // Modify newsletter before sending
            if let Some(newsletter_data) = context.data.get("newsletter") {
                // Process newsletter data
                let mut modified_newsletter = // ... modify newsletter
                return Ok(PluginResult {
                    success: true,
                    message: Some("Newsletter modified".to_string()),
                    data: HashMap::new(),
                    modified_newsletter: Some(modified_newsletter),
                    modified_subscribers: None,
                });
            }
        }
        PluginHook::PostFetch => {
            // Process newly fetched subscribers
            if let Some(subscribers_data) = context.data.get("subscribers") {
                // Filter or modify subscribers
                let modified_subscribers = // ... process subscribers
                return Ok(PluginResult {
                    success: true,
                    message: Some("Subscribers processed".to_string()),
                    data: HashMap::new(),
                    modified_newsletter: None,
                    modified_subscribers: Some(modified_subscribers),
                });
            }
        }
        _ => {
            Ok(PluginResult {
                success: false,
                message: Some("Hook not supported".to_string()),
                data: HashMap::new(),
                modified_newsletter: None,
                modified_subscribers: None,
            })
        }
    }
}
```

## Configuration Management

### Plugin Configuration in `blogr.toml`

```toml
[newsletter.plugins.my-plugin]
enabled = true
api_key = "your-api-key"
endpoint = "https://api.example.com"
custom_setting = "value"
```

### Accessing Configuration

```rust
fn initialize(&mut self, config: &PluginConfig) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    // Access custom configuration
    if let Some(api_key) = config.config.get("api_key") {
        if let Some(key_str) = api_key.as_str() {
            self.api_key = Some(key_str.to_string());
        }
    }

    // Validate required configuration
    if self.api_key.is_none() {
        return Err(anyhow::anyhow!("API key is required"));
    }

    Ok(())
}
```

## Custom Commands

Plugins can provide custom CLI commands that integrate seamlessly with the Blogr CLI.

### Implementing Custom Commands

```rust
impl NewsletterPlugin for MyPlugin {
    fn custom_commands(&self) -> Vec<String> {
        vec![
            "sync-external".to_string(),
            "generate-report".to_string(),
        ]
    }

    fn execute_command(&self, command: &str, args: &[String], context: &PluginContext) -> Result<PluginResult> {
        match command {
            "sync-external" => {
                println!("Syncing with external service...");
                // Implementation here
                Ok(PluginResult {
                    success: true,
                    message: Some("Sync completed".to_string()),
                    data: HashMap::new(),
                    modified_newsletter: None,
                    modified_subscribers: None,
                })
            }
            "generate-report" => {
                let format = args.get(0).unwrap_or(&"text".to_string());
                println!("Generating report in {} format...", format);
                // Implementation here
                Ok(PluginResult {
                    success: true,
                    message: Some(format!("Report generated in {} format", format)),
                    data: HashMap::new(),
                    modified_newsletter: None,
                    modified_subscribers: None,
                })
            }
            _ => Err(anyhow::anyhow!("Unknown command: {}", command))
        }
    }
}
```

### Usage

```bash
# Run custom plugin commands
blogr newsletter plugin run sync-external
blogr newsletter plugin run generate-report pdf
```

## Custom Templates

Plugins can provide custom email templates for different newsletter styles.

### Implementing Custom Templates

```rust
impl NewsletterPlugin for MyPlugin {
    fn custom_templates(&self) -> Vec<String> {
        vec![
            "corporate".to_string(),
            "newsletter-digest".to_string(),
        ]
    }

    fn render_template(&self, template: &str, newsletter: &Newsletter, context: &PluginContext) -> Result<Newsletter> {
        match template {
            "corporate" => {
                let mut modified_newsletter = newsletter.clone();
                
                // Modify the newsletter HTML with corporate styling
                modified_newsletter.html_content = format!(
                    r#"
                    <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
                        <header style="background-color: #003366; color: white; padding: 20px;">
                            <h1>{}</h1>
                        </header>
                        <main style="padding: 20px;">
                            {}
                        </main>
                        <footer style="background-color: #f0f0f0; padding: 10px; text-align: center;">
                            <p>© 2024 Your Company</p>
                        </footer>
                    </div>
                    "#,
                    newsletter.subject,
                    newsletter.html_content
                );

                Ok(modified_newsletter)
            }
            "newsletter-digest" => {
                // Implementation for digest template
                let mut modified_newsletter = newsletter.clone();
                // ... template modifications
                Ok(modified_newsletter)
            }
            _ => Err(anyhow::anyhow!("Unknown template: {}", template))
        }
    }
}
```

## Plugin Context

The `PluginContext` provides access to system state and data:

```rust
pub struct PluginContext {
    pub config: Arc<Config>,           // Blogr configuration
    pub database: Arc<NewsletterDatabase>, // Newsletter database
    pub project_root: PathBuf,         // Project root directory
    pub hook: PluginHook,              // Current hook being executed
    pub data: HashMap<String, serde_json::Value>, // Hook-specific data
}
```

### Accessing Context Data

```rust
fn execute_hook(&self, context: &PluginContext) -> Result<PluginResult> {
    // Access configuration
    let blog_title = &context.config.blog.title;
    
    // Access database
    let subscribers = context.database.get_subscribers(None)?;
    
    // Access hook-specific data
    if let Some(newsletter_data) = context.data.get("newsletter") {
        // Process newsletter data
    }
    
    // Access project information
    let project_path = &context.project_root;
    
    Ok(PluginResult {
        success: true,
        message: Some("Context accessed successfully".to_string()),
        data: HashMap::new(),
        modified_newsletter: None,
        modified_subscribers: None,
    })
}
```

## Error Handling

### Best Practices

1. **Use `anyhow::Result`** for all fallible operations
2. **Provide meaningful error messages** with context
3. **Gracefully handle missing dependencies**
4. **Validate configuration early** in the `initialize` method

```rust
fn initialize(&mut self, config: &PluginConfig) -> Result<()> {
    // Validate configuration
    let api_key = config.config.get("api_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing required 'api_key' configuration"))?;

    // Test external connections
    self.test_connection(api_key)
        .with_context(|| "Failed to connect to external service")?;

    Ok(())
}

fn test_connection(&self, api_key: &str) -> Result<()> {
    // Implementation for testing external service connection
    Ok(())
}
```

## Testing Plugins

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use blogr_cli::newsletter::{create_plugin_context, NewsletterDatabase};
    use std::sync::Arc;
    use tempfile::tempdir;

    #[test]
    fn test_plugin_initialization() {
        let mut plugin = MyNewsletterPlugin::new();
        
        let config = PluginConfig {
            enabled: true,
            config: {
                let mut map = HashMap::new();
                map.insert("api_key".to_string(), serde_json::Value::String("test-key".to_string()));
                map
            },
        };

        assert!(plugin.initialize(&config).is_ok());
    }

    #[test]
    fn test_hook_execution() {
        let plugin = MyNewsletterPlugin::new();
        
        // Create test context
        let temp_dir = tempdir().unwrap();
        let config = Arc::new(Config::default());
        let database = Arc::new(NewsletterDatabase::in_memory().unwrap());
        
        let context = create_plugin_context(
            config,
            database,
            temp_dir.path().to_path_buf(),
            PluginHook::PreSend,
            HashMap::new(),
        );

        let result = plugin.execute_hook(&context).unwrap();
        assert!(result.success);
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_plugin_with_real_newsletter() {
        // Create a real newsletter system for testing
        // This would require more setup but provides comprehensive testing
    }
}
```

## Plugin Examples

### 1. Analytics Plugin

```rust
pub struct AnalyticsPlugin {
    metadata: PluginMetadata,
    tracking_endpoint: Option<String>,
}

impl NewsletterPlugin for AnalyticsPlugin {
    fn execute_hook(&self, context: &PluginContext) -> Result<PluginResult> {
        match context.hook {
            PluginHook::PostSend => {
                // Track newsletter send event
                self.track_event("newsletter_sent", &context.data)?;
                Ok(PluginResult {
                    success: true,
                    message: Some("Analytics event tracked".to_string()),
                    data: HashMap::new(),
                    modified_newsletter: None,
                    modified_subscribers: None,
                })
            }
            _ => Ok(PluginResult { /* ... */ })
        }
    }
}
```

### 2. Webhook Plugin

```rust
pub struct WebhookPlugin {
    metadata: PluginMetadata,
    webhook_url: Option<String>,
}

impl NewsletterPlugin for WebhookPlugin {
    fn execute_hook(&self, context: &PluginContext) -> Result<PluginResult> {
        match context.hook {
            PluginHook::PostApprove => {
                // Send webhook notification for new subscribers
                if let Some(url) = &self.webhook_url {
                    self.send_webhook(url, &context.data)?;
                }
                Ok(PluginResult { /* ... */ })
            }
            _ => Ok(PluginResult { /* ... */ })
        }
    }
}
```

### 3. Template Enhancement Plugin

```rust
pub struct TemplateEnhancementPlugin {
    metadata: PluginMetadata,
}

impl NewsletterPlugin for TemplateEnhancementPlugin {
    fn custom_templates(&self) -> Vec<String> {
        vec![
            "modern-minimal".to_string(),
            "newsletter-magazine".to_string(),
        ]
    }

    fn render_template(&self, template: &str, newsletter: &Newsletter, context: &PluginContext) -> Result<Newsletter> {
        match template {
            "modern-minimal" => {
                let mut enhanced = newsletter.clone();
                enhanced.html_content = self.apply_modern_minimal_styling(&newsletter.html_content);
                Ok(enhanced)
            }
            _ => Err(anyhow::anyhow!("Template not found"))
        }
    }
}
```

## Best Practices

### 1. Plugin Design

- **Single Responsibility**: Each plugin should have a clear, focused purpose
- **Minimal Dependencies**: Keep external dependencies to a minimum
- **Graceful Degradation**: Handle missing dependencies gracefully
- **Configuration Validation**: Validate all configuration early

### 2. Performance

- **Async Operations**: Use async/await for I/O operations
- **Caching**: Cache expensive operations when appropriate
- **Resource Management**: Clean up resources properly
- **Error Recovery**: Don't let plugin errors crash the system

### 3. Security

- **Input Validation**: Validate all external input
- **Secure Defaults**: Use secure defaults for configuration
- **Credential Management**: Handle API keys and credentials securely
- **Rate Limiting**: Implement rate limiting for external API calls

### 4. User Experience

- **Clear Error Messages**: Provide helpful error messages
- **Progress Indicators**: Show progress for long-running operations
- **Documentation**: Include comprehensive documentation
- **Examples**: Provide usage examples

## Distribution

### 1. Packaging

Create a proper Rust crate structure:

```
my-newsletter-plugin/
├── Cargo.toml
├── README.md
├── LICENSE
├── src/
│   ├── lib.rs
│   └── plugin.rs
├── examples/
│   └── basic_usage.rs
└── tests/
    └── integration_tests.rs
```

### 2. Documentation

- Include comprehensive README
- Document all configuration options
- Provide usage examples
- Include troubleshooting guide

### 3. Publishing

```bash
# Publish to crates.io
cargo publish

# Or distribute via Git
git tag v1.0.0
git push origin v1.0.0
```

### 4. Registration

Currently, plugins must be registered programmatically. In the future, we plan to add:

- Dynamic plugin loading
- Plugin marketplace
- Automatic plugin discovery

## Conclusion

The Blogr Newsletter plugin system provides a powerful way to extend newsletter functionality. By following this guide and best practices, you can create robust, reliable plugins that enhance the newsletter experience for users.

For more examples and updates, check the [Blogr repository](https://github.com/your-org/blogr) and join our community discussions.
