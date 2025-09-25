//! Plugin system for newsletter extensions
//!
//! This module provides a plugin architecture that allows third-party extensions
//! to integrate with the newsletter system. Plugins can extend functionality for:
//! - Custom email templates
//! - Additional sending providers
//! - Enhanced analytics
//! - Integration with external services

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use super::{Newsletter, NewsletterDatabase, Subscriber};
use crate::config::Config;

/// Plugin metadata and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub keywords: Vec<String>,
    pub dependencies: Vec<String>,
    pub min_blogr_version: Option<String>,
}

/// Plugin configuration stored in blogr.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub config: HashMap<String, serde_json::Value>,
}

/// Plugin hook types that can be implemented
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum PluginHook {
    /// Called before fetching subscribers
    PreFetch,
    /// Called after fetching subscribers
    PostFetch,
    /// Called before approving subscribers
    PreApprove,
    /// Called after approving subscribers
    PostApprove,
    /// Called before composing newsletter
    PreCompose,
    /// Called after composing newsletter
    PostCompose,
    /// Called before sending newsletter
    PreSend,
    /// Called after sending newsletter
    PostSend,
    /// Called for custom CLI commands
    CustomCommand,
    /// Called for custom email templates
    CustomTemplate,
}

/// Context passed to plugin hooks
#[derive(Debug)]
#[allow(dead_code)]
pub struct PluginContext {
    pub config: Arc<Config>,
    pub database: Arc<NewsletterDatabase>,
    pub project_root: PathBuf,
    pub hook: PluginHook,
    pub data: HashMap<String, serde_json::Value>,
}

/// Result returned by plugin hooks
#[derive(Debug)]
#[allow(dead_code)]
pub struct PluginResult {
    pub success: bool,
    pub message: Option<String>,
    pub data: HashMap<String, serde_json::Value>,
    pub modified_newsletter: Option<Newsletter>,
    pub modified_subscribers: Option<Vec<Subscriber>>,
}

/// Trait that all plugins must implement
#[allow(dead_code)]
pub trait NewsletterPlugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;

    /// Initialize the plugin with configuration
    fn initialize(&mut self, config: &PluginConfig) -> Result<()>;

    /// Check if this plugin handles the given hook
    fn handles_hook(&self, hook: &PluginHook) -> bool;

    /// Execute the plugin hook
    fn execute_hook(&self, context: &PluginContext) -> Result<PluginResult>;

    /// Get available custom commands provided by this plugin
    fn custom_commands(&self) -> Vec<String> {
        Vec::new()
    }

    /// Execute a custom command
    fn execute_command(
        &self,
        _command: &str,
        _args: &[String],
        _context: &PluginContext,
    ) -> Result<PluginResult> {
        Err(anyhow::anyhow!("Command '{}' not implemented", _command))
    }

    /// Get available custom templates provided by this plugin
    fn custom_templates(&self) -> Vec<String> {
        Vec::new()
    }

    /// Render a custom template
    fn render_template(
        &self,
        template: &str,
        _newsletter: &Newsletter,
        _context: &PluginContext,
    ) -> Result<Newsletter> {
        Err(anyhow::anyhow!("Template '{}' not implemented", template))
    }
}

/// Plugin manager for loading and executing plugins
#[allow(dead_code)]
pub struct PluginManager {
    plugins: Vec<Box<dyn NewsletterPlugin>>,
    plugin_configs: HashMap<String, PluginConfig>,
    project_root: PathBuf,
}

#[allow(dead_code)]
impl PluginManager {
    /// Create a new plugin manager
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            plugins: Vec::new(),
            plugin_configs: HashMap::new(),
            project_root,
        }
    }

    /// Load plugin configuration from blogr.toml
    pub fn load_plugin_configs(&mut self, config: &Config) -> Result<()> {
        // For now, plugins are configured in the newsletter section
        // In the future, we might add a dedicated [plugins] section
        if let Some(ref plugins_config) = config.newsletter.plugins {
            for (name, plugin_config) in plugins_config {
                self.plugin_configs
                    .insert(name.clone(), plugin_config.clone());
            }
        }
        Ok(())
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, mut plugin: Box<dyn NewsletterPlugin>) -> Result<()> {
        let plugin_name = plugin.metadata().name.clone();

        // Initialize plugin with its configuration
        if let Some(config) = self.plugin_configs.get(&plugin_name) {
            plugin
                .initialize(config)
                .with_context(|| format!("Failed to initialize plugin '{}'", plugin_name))?;
        } else {
            // Initialize with default configuration
            let default_config = PluginConfig {
                enabled: false,
                config: HashMap::new(),
            };
            plugin
                .initialize(&default_config)
                .with_context(|| format!("Failed to initialize plugin '{}'", plugin_name))?;
        }

        self.plugins.push(plugin);
        Ok(())
    }

    /// Execute all plugins for a specific hook
    pub fn execute_hook(
        &self,
        hook: PluginHook,
        context: &PluginContext,
    ) -> Result<Vec<PluginResult>> {
        let mut results = Vec::new();

        for plugin in &self.plugins {
            let plugin_name = &plugin.metadata().name;

            // Check if plugin is enabled
            if let Some(config) = self.plugin_configs.get(plugin_name) {
                if !config.enabled {
                    continue;
                }
            } else {
                continue; // Skip disabled plugins
            }

            // Check if plugin handles this hook
            if !plugin.handles_hook(&hook) {
                continue;
            }

            match plugin.execute_hook(context) {
                Ok(result) => results.push(result),
                Err(e) => {
                    eprintln!(
                        "Warning: Plugin '{}' failed to execute hook {:?}: {}",
                        plugin_name, hook, e
                    );
                    results.push(PluginResult {
                        success: false,
                        message: Some(format!("Plugin error: {}", e)),
                        data: HashMap::new(),
                        modified_newsletter: None,
                        modified_subscribers: None,
                    });
                }
            }
        }

        Ok(results)
    }

    /// Get all custom commands from all plugins
    pub fn get_custom_commands(&self) -> HashMap<String, String> {
        let mut commands = HashMap::new();

        for plugin in &self.plugins {
            let plugin_name = &plugin.metadata().name;

            // Check if plugin is enabled
            if let Some(config) = self.plugin_configs.get(plugin_name) {
                if !config.enabled {
                    continue;
                }
            } else {
                continue;
            }

            for command in plugin.custom_commands() {
                commands.insert(command, plugin_name.clone());
            }
        }

        commands
    }

    /// Execute a custom command
    pub fn execute_custom_command(
        &self,
        command: &str,
        args: &[String],
        context: &PluginContext,
    ) -> Result<PluginResult> {
        for plugin in &self.plugins {
            let plugin_name = &plugin.metadata().name;

            // Check if plugin is enabled
            if let Some(config) = self.plugin_configs.get(plugin_name) {
                if !config.enabled {
                    continue;
                }
            } else {
                continue;
            }

            if plugin.custom_commands().contains(&command.to_string()) {
                return plugin.execute_command(command, args, context);
            }
        }

        Err(anyhow::anyhow!("Custom command '{}' not found", command))
    }

    /// Get all custom templates from all plugins
    pub fn get_custom_templates(&self) -> HashMap<String, String> {
        let mut templates = HashMap::new();

        for plugin in &self.plugins {
            let plugin_name = &plugin.metadata().name;

            // Check if plugin is enabled
            if let Some(config) = self.plugin_configs.get(plugin_name) {
                if !config.enabled {
                    continue;
                }
            } else {
                continue;
            }

            for template in plugin.custom_templates() {
                templates.insert(template, plugin_name.clone());
            }
        }

        templates
    }

    /// Render a custom template
    pub fn render_custom_template(
        &self,
        template: &str,
        newsletter: &Newsletter,
        context: &PluginContext,
    ) -> Result<Newsletter> {
        for plugin in &self.plugins {
            let plugin_name = &plugin.metadata().name;

            // Check if plugin is enabled
            if let Some(config) = self.plugin_configs.get(plugin_name) {
                if !config.enabled {
                    continue;
                }
            } else {
                continue;
            }

            if plugin.custom_templates().contains(&template.to_string()) {
                return plugin.render_template(template, newsletter, context);
            }
        }

        Err(anyhow::anyhow!("Custom template '{}' not found", template))
    }

    /// List all loaded plugins
    pub fn list_plugins(&self) -> Vec<&PluginMetadata> {
        self.plugins.iter().map(|p| p.metadata()).collect()
    }

    /// Get plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&dyn NewsletterPlugin> {
        self.plugins
            .iter()
            .find(|p| p.metadata().name == name)
            .map(|p| p.as_ref())
    }
}

/// Helper function to create a plugin context
pub fn create_plugin_context(
    config: Arc<Config>,
    database: Arc<NewsletterDatabase>,
    project_root: PathBuf,
    hook: PluginHook,
    data: HashMap<String, serde_json::Value>,
) -> PluginContext {
    PluginContext {
        config,
        database,
        project_root,
        hook,
        data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        metadata: PluginMetadata,
        initialized: bool,
    }

    impl TestPlugin {
        fn new() -> Self {
            Self {
                metadata: PluginMetadata {
                    name: "test-plugin".to_string(),
                    version: "1.0.0".to_string(),
                    author: "Test Author".to_string(),
                    description: "A test plugin".to_string(),
                    homepage: None,
                    repository: None,
                    license: Some("MIT".to_string()),
                    keywords: vec!["test".to_string()],
                    dependencies: Vec::new(),
                    min_blogr_version: None,
                },
                initialized: false,
            }
        }
    }

    impl NewsletterPlugin for TestPlugin {
        fn metadata(&self) -> &PluginMetadata {
            &self.metadata
        }

        fn initialize(&mut self, _config: &PluginConfig) -> Result<()> {
            self.initialized = true;
            Ok(())
        }

        fn handles_hook(&self, hook: &PluginHook) -> bool {
            matches!(hook, PluginHook::PreSend | PluginHook::PostSend)
        }

        fn execute_hook(&self, _context: &PluginContext) -> Result<PluginResult> {
            Ok(PluginResult {
                success: true,
                message: Some("Test plugin executed".to_string()),
                data: HashMap::new(),
                modified_newsletter: None,
                modified_subscribers: None,
            })
        }
    }

    #[test]
    fn test_plugin_registration() {
        let mut manager = PluginManager::new(PathBuf::from("/tmp"));
        let plugin = Box::new(TestPlugin::new());

        assert!(manager.register_plugin(plugin).is_ok());
        assert_eq!(manager.plugins.len(), 1);
    }

    #[test]
    fn test_plugin_hook_execution() {
        // This would need mock implementations for full testing
        // For now, just test that the structure compiles
        let manager = PluginManager::new(PathBuf::from("/tmp"));
        assert_eq!(manager.plugins.len(), 0);
    }
}
