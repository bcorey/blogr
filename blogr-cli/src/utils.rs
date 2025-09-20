use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

/// Utility functions for the CLI
pub struct Utils;

impl Utils {
    /// Convert a title to a URL-friendly slug
    pub fn slugify(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Calculate estimated reading time based on word count
    pub fn calculate_reading_time(content: &str) -> usize {
        const WORDS_PER_MINUTE: usize = 200;
        let word_count = content.split_whitespace().count();
        let minutes = (word_count + WORDS_PER_MINUTE - 1) / WORDS_PER_MINUTE; // Ceiling division
        minutes.max(1) // Minimum 1 minute
    }

    /// Extract excerpt from content
    pub fn extract_excerpt(content: &str, max_words: usize) -> String {
        let words: Vec<&str> = content.split_whitespace().take(max_words).collect();
        let mut excerpt = words.join(" ");

        if content.split_whitespace().count() > max_words {
            excerpt.push_str("...");
        }

        excerpt
    }

    /// Ensure directory exists, create if it doesn't
    pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        if !path.exists() {
            fs::create_dir_all(path)
                .with_context(|| format!("Failed to create directory: {}", path.display()))?;
        }
        Ok(())
    }

    /// Copy file with proper error handling
    pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
        let from = from.as_ref();
        let to = to.as_ref();

        if let Some(parent) = to.parent() {
            Self::ensure_dir_exists(parent)?;
        }

        fs::copy(from, to)
            .with_context(|| format!("Failed to copy {} to {}", from.display(), to.display()))?;
        Ok(())
    }

    /// Write content to file with proper error handling
    pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            Self::ensure_dir_exists(parent)?;
        }

        fs::write(path, content)
            .with_context(|| format!("Failed to write file: {}", path.display()))?;
        Ok(())
    }

    /// Read file with proper error handling
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
        let path = path.as_ref();
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path.display()))
    }

    /// Get file extension from path
    pub fn get_file_extension<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }

    /// Check if a string is a valid URL
    pub fn is_valid_url(url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }

    /// Format file size in human readable format
    pub fn format_file_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }

    /// Format timestamp for display
    pub fn format_timestamp(timestamp: DateTime<Utc>) -> String {
        timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }

    /// Validate GitHub repository name
    pub fn is_valid_github_repo_name(name: &str) -> bool {
        if name.is_empty() || name.len() > 100 {
            return false;
        }

        // GitHub repository names can contain alphanumeric characters, hyphens, underscores, and periods
        // They cannot start or end with hyphens
        if name.starts_with('-') || name.ends_with('-') {
            return false;
        }

        name.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    }

    /// Validate GitHub username
    pub fn is_valid_github_username(username: &str) -> bool {
        if username.is_empty() || username.len() > 39 {
            return false;
        }

        // GitHub usernames can contain alphanumeric characters and hyphens
        // They cannot start or end with hyphens
        // They cannot contain consecutive hyphens
        if username.starts_with('-') || username.ends_with('-') || username.contains("--") {
            return false;
        }

        username.chars().all(|c| c.is_alphanumeric() || c == '-')
    }

    /// Generate a unique filename if one already exists
    pub fn unique_filename<P: AsRef<Path>>(path: P) -> String {
        let path = path.as_ref();
        if !path.exists() {
            return path.to_string_lossy().to_string();
        }

        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let extension = path
            .extension()
            .map(|ext| format!(".{}", ext.to_string_lossy()))
            .unwrap_or_default();
        let parent = path.parent().unwrap_or(Path::new(""));

        for i in 1..1000 {
            let new_name = format!("{}-{}{}", stem, i, extension);
            let new_path = parent.join(&new_name);
            if !new_path.exists() {
                return new_path.to_string_lossy().to_string();
            }
        }

        // Fallback with timestamp if we can't find a unique name
        let timestamp = Utc::now().timestamp();
        let new_name = format!("{}-{}{}", stem, timestamp, extension);
        parent.join(new_name).to_string_lossy().to_string()
    }

    /// Parse comma-separated tags
    pub fn parse_tags(tags_str: &str) -> Vec<String> {
        tags_str
            .split(',')
            .map(|tag| tag.trim().to_string())
            .filter(|tag| !tag.is_empty())
            .collect()
    }

    /// Open URL in default browser
    pub fn open_browser(url: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(url)
                .spawn()
                .with_context(|| "Failed to open browser on macOS")?;
        }

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(&["/C", "start", url])
                .spawn()
                .with_context(|| "Failed to open browser on Windows")?;
        }

        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(url)
                .spawn()
                .with_context(|| "Failed to open browser on Linux")?;
        }

        Ok(())
    }

    /// Check if git is available
    pub fn is_git_available() -> bool {
        std::process::Command::new("git")
            .arg("--version")
            .output()
            .is_ok()
    }

    /// Initialize git repository
    pub fn init_git_repo<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();

        if !Self::is_git_available() {
            anyhow::bail!(
                "Git is not available. Please install git to use version control features."
            );
        }

        std::process::Command::new("git")
            .args(&["init"])
            .current_dir(path)
            .output()
            .with_context(|| "Failed to initialize git repository")?;

        Ok(())
    }

    /// Add all files to git staging area
    pub fn git_add_all<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();

        std::process::Command::new("git")
            .args(&["add", "."])
            .current_dir(path)
            .output()
            .with_context(|| "Failed to add files to git")?;

        Ok(())
    }

    /// Create initial git commit
    pub fn git_initial_commit<P: AsRef<Path>>(path: P, message: &str) -> Result<()> {
        let path = path.as_ref();

        std::process::Command::new("git")
            .args(&["commit", "-m", message])
            .current_dir(path)
            .output()
            .with_context(|| "Failed to create initial git commit")?;

        Ok(())
    }

    /// Set git remote origin
    pub fn git_set_remote<P: AsRef<Path>>(path: P, remote_url: &str) -> Result<()> {
        let path = path.as_ref();

        std::process::Command::new("git")
            .args(&["remote", "add", "origin", remote_url])
            .current_dir(path)
            .output()
            .with_context(|| "Failed to set git remote")?;

        Ok(())
    }

    /// Push to git remote
    pub fn git_push<P: AsRef<Path>>(path: P, branch: &str) -> Result<()> {
        let path = path.as_ref();

        std::process::Command::new("git")
            .args(&["push", "-u", "origin", branch])
            .current_dir(path)
            .output()
            .with_context(|| "Failed to push to git remote")?;

        Ok(())
    }
}

/// Console output utilities
pub struct Console;

impl Console {
    /// Print success message
    pub fn success(message: &str) {
        println!("✅ {}", message);
    }

    /// Print error message
    pub fn error(message: &str) {
        eprintln!("❌ {}", message);
    }

    /// Print warning message
    pub fn warn(message: &str) {
        println!("⚠️  {}", message);
    }

    /// Print info message
    pub fn info(message: &str) {
        println!("ℹ️  {}", message);
    }

    /// Print step message
    pub fn step(step: u8, total: u8, message: &str) {
        println!("[{}/{}] {}", step, total, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(Utils::slugify("Hello World"), "hello-world");
        assert_eq!(
            Utils::slugify("My Awesome Blog Post!"),
            "my-awesome-blog-post"
        );
        assert_eq!(Utils::slugify("123 Test"), "123-test");
        assert_eq!(Utils::slugify("---test---"), "test");
    }

    #[test]
    fn test_reading_time() {
        let short_text = "Hello world";
        assert_eq!(Utils::calculate_reading_time(short_text), 1);

        let long_text = (0..300).map(|_| "word").collect::<Vec<_>>().join(" ");
        assert!(Utils::calculate_reading_time(&long_text) > 1);
    }

    #[test]
    fn test_extract_excerpt() {
        let text = "This is a long text with many words that should be truncated properly";
        let excerpt = Utils::extract_excerpt(text, 5);
        assert_eq!(excerpt, "This is a long text...");
    }

    #[test]
    fn test_github_validation() {
        assert!(Utils::is_valid_github_username("valid-username"));
        assert!(Utils::is_valid_github_username("user123"));
        assert!(!Utils::is_valid_github_username("-invalid"));
        assert!(!Utils::is_valid_github_username("invalid-"));
        assert!(!Utils::is_valid_github_username("invalid--double"));

        assert!(Utils::is_valid_github_repo_name("valid-repo"));
        assert!(Utils::is_valid_github_repo_name("repo.name"));
        assert!(Utils::is_valid_github_repo_name("repo_name"));
        assert!(!Utils::is_valid_github_repo_name("-invalid"));
        assert!(!Utils::is_valid_github_repo_name("invalid-"));
    }

    #[test]
    fn test_parse_tags() {
        let tags = Utils::parse_tags("rust, programming, web");
        assert_eq!(tags, vec!["rust", "programming", "web"]);

        let tags_with_spaces = Utils::parse_tags("  tag1  ,  tag2  , tag3  ");
        assert_eq!(tags_with_spaces, vec!["tag1", "tag2", "tag3"]);
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(Utils::format_file_size(512), "512 B");
        assert_eq!(Utils::format_file_size(1024), "1.0 KB");
        assert_eq!(Utils::format_file_size(1536), "1.5 KB");
        assert_eq!(Utils::format_file_size(1048576), "1.0 MB");
    }
}
