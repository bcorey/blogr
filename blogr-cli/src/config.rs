use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub blog: BlogConfig,
    pub theme: ThemeConfig,
    pub github: Option<GitHubConfig>,
    pub build: BuildConfig,
    pub dev: DevConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogConfig {
    pub title: String,
    pub author: String,
    pub description: String,
    pub base_url: String,
    pub language: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    #[serde(default)]
    pub config: HashMap<String, toml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub username: String,
    pub repository: String,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub output_dir: Option<String>,
    #[serde(default)]
    pub drafts: bool,
    #[serde(default)]
    pub future_posts: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub auto_reload: bool,
}

fn default_port() -> u16 {
    3000
}

impl Default for Config {
    fn default() -> Self {
        Self {
            blog: BlogConfig {
                title: "My Blog".to_string(),
                author: "Anonymous".to_string(),
                description: "A blog powered by Blogr".to_string(),
                base_url: "https://username.github.io/repository".to_string(),
                language: Some("en".to_string()),
                timezone: Some("UTC".to_string()),
            },
            theme: ThemeConfig {
                name: "minimal-retro".to_string(),
                config: HashMap::new(),
            },
            github: None,
            build: BuildConfig {
                output_dir: Some("dist".to_string()),
                drafts: false,
                future_posts: false,
            },
            dev: DevConfig {
                port: 3000,
                auto_reload: true,
            },
        }
    }
}

impl Config {
    /// Load configuration from blogr.toml file
    #[allow(dead_code)]
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        let config: Self =
            toml::from_str(&content).with_context(|| "Failed to parse configuration file")?;

        Ok(config)
    }

    /// Save configuration to blogr.toml file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content =
            toml::to_string_pretty(self).with_context(|| "Failed to serialize configuration")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;

        Ok(())
    }

    /// Create default configuration with custom values
    pub fn new_with_defaults(
        title: String,
        author: String,
        description: String,
        github_username: Option<String>,
        github_repo: Option<String>,
    ) -> Self {
        let mut config = Self::default();

        config.blog.title = title.clone();
        config.blog.author = author;
        config.blog.description = description;

        if let (Some(username), Some(repo)) = (github_username, github_repo) {
            config.blog.base_url = format!("https://{}.github.io/{}", username, repo);
            config.github = Some(GitHubConfig {
                username,
                repository: repo,
                branch: Some("main".to_string()),
            });
        }

        config
    }

    /// Get the project root directory (where blogr.toml is located)
    pub fn find_project_root() -> Result<Option<PathBuf>> {
        let mut current =
            std::env::current_dir().with_context(|| "Failed to get current directory")?;

        loop {
            let config_path = current.join("blogr.toml");
            if config_path.exists() {
                return Ok(Some(current));
            }

            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }

    /// Load configuration from the project root
    #[allow(dead_code)]
    pub fn load_from_project() -> Result<(Self, PathBuf)> {
        let project_root = Self::find_project_root()
            .with_context(|| "Failed to find project root")?
            .ok_or_else(|| {
                anyhow::anyhow!("No blogr project found. Run 'blogr init' to create a new project.")
            })?;

        let config_path = project_root.join("blogr.toml");
        let config = Self::load_from_file(&config_path)
            .with_context(|| "Failed to load project configuration")?;

        Ok((config, project_root))
    }

    /// Get the posts directory path
    pub fn posts_dir(&self, project_root: &Path) -> PathBuf {
        project_root.join("posts")
    }

    /// Get the themes directory path
    #[allow(dead_code)]
    pub fn themes_dir(&self, project_root: &Path) -> PathBuf {
        project_root.join("themes")
    }

    /// Get the output directory path
    #[allow(dead_code)]
    pub fn output_dir(&self, project_root: &Path) -> PathBuf {
        let output_dir = self.build.output_dir.as_deref().unwrap_or("dist");
        project_root.join(output_dir)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.blog.title.trim().is_empty() {
            anyhow::bail!("Blog title cannot be empty");
        }

        if self.blog.author.trim().is_empty() {
            anyhow::bail!("Blog author cannot be empty");
        }

        if self.blog.description.trim().is_empty() {
            anyhow::bail!("Blog description cannot be empty");
        }

        if !self.blog.base_url.starts_with("http://") && !self.blog.base_url.starts_with("https://")
        {
            anyhow::bail!("Base URL must start with http:// or https://");
        }

        if self.dev.port == 0 {
            anyhow::bail!("Development server port must be greater than 0");
        }

        Ok(())
    }

    /// Update theme configuration
    #[allow(dead_code)]
    pub fn set_theme(&mut self, theme_name: String, theme_config: HashMap<String, toml::Value>) {
        self.theme.name = theme_name;
        self.theme.config = theme_config;
    }

    /// Get theme configuration value
    #[allow(dead_code)]
    pub fn get_theme_config(&self, key: &str) -> Option<&toml::Value> {
        self.theme.config.get(key)
    }

    /// Set theme configuration value
    #[allow(dead_code)]
    pub fn set_theme_config(&mut self, key: String, value: toml::Value) {
        self.theme.config.insert(key, value);
    }
}

/// Environment variable utilities for configuration
pub struct EnvConfig;

impl EnvConfig {
    /// Get GitHub token from environment
    pub fn github_token() -> Option<String> {
        std::env::var("GITHUB_TOKEN")
            .ok()
            .or_else(|| std::env::var("GH_TOKEN").ok())
    }

    /// Get author name from git config or environment
    pub fn git_author_name() -> Option<String> {
        std::env::var("GIT_AUTHOR_NAME").ok().or_else(|| {
            std::process::Command::new("git")
                .args(["config", "user.name"])
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|s| s.trim().to_string())
        })
    }

    /// Get author email from git config or environment
    #[allow(dead_code)]
    pub fn git_author_email() -> Option<String> {
        std::env::var("GIT_AUTHOR_EMAIL").ok().or_else(|| {
            std::process::Command::new("git")
                .args(["config", "user.email"])
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|s| s.trim().to_string())
        })
    }

    /// Get GitHub username from environment or git config
    pub fn github_username() -> Option<String> {
        std::env::var("GITHUB_USERNAME")
            .ok()
            .or_else(|| std::env::var("GITHUB_USER").ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(config.blog.title, deserialized.blog.title);
        assert_eq!(config.theme.name, deserialized.theme.name);
    }

    #[test]
    fn test_config_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("blogr.toml");

        let config = Config::default();
        config.save_to_file(&config_path).unwrap();

        let loaded_config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.blog.title, loaded_config.blog.title);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        config.blog.title = "".to_string();
        assert!(config.validate().is_err());
    }
}
