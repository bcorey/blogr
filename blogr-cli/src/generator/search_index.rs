use crate::content::Post;
use crate::generator::markdown;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Search document schema for the index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    /// Unique identifier (post slug)
    pub id: String,
    /// URL path relative to site root
    pub url: String,
    /// Post title
    pub title: String,
    /// Post tags
    pub tags: Vec<String>,
    /// Post date in ISO 8601 format
    pub date: String,
    /// Post description
    pub description: String,
    /// Plain text content (truncated if needed)
    pub content: String,
    /// Pre-built excerpt for search results
    pub excerpt: String,
}

/// Search index configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Whether search is enabled
    pub enabled: bool,
    /// Fields to include in search
    pub fields: Vec<String>,
    /// Paths to exclude from search
    pub exclude: Vec<String>,
    /// Maximum content length in characters
    pub max_content_chars: usize,
    /// Excerpt length in words
    pub excerpt_words: usize,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            fields: vec![
                "title".to_string(),
                "tags".to_string(),
                "content".to_string(),
            ],
            exclude: vec!["drafts/".to_string()],
            max_content_chars: 2000,
            excerpt_words: 30,
        }
    }
}

/// Search index generator
pub struct SearchIndexer {
    config: SearchConfig,
}

impl SearchIndexer {
    /// Create a new search indexer with default configuration
    pub fn new() -> Self {
        Self {
            config: SearchConfig::default(),
        }
    }

    /// Create a new search indexer with custom configuration
    #[allow(dead_code)]
    pub fn with_config(config: SearchConfig) -> Self {
        Self { config }
    }

    /// Generate search index from posts
    pub fn generate_index(&self, posts: &[Post], output_dir: &Path) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        println!("🔍 Generating search index...");

        // Convert posts to search documents
        let mut documents = Vec::new();
        for post in posts {
            if self.should_include_post(post) {
                let document = self.post_to_search_document(post)?;
                documents.push(document);
            }
        }

        // Serialize to minified JSON
        let json_content = serde_json::to_string(&documents)
            .map_err(|e| anyhow::anyhow!("Failed to serialize search index: {}", e))?;

        // Write to output directory
        let index_file = output_dir.join("search_index.json");
        fs::write(&index_file, json_content)
            .map_err(|e| anyhow::anyhow!("Failed to write search index: {}", e))?;

        println!("✅ Search index generated: {} documents", documents.len());
        Ok(())
    }

    /// Check if a post should be included in the search index
    fn should_include_post(&self, post: &Post) -> bool {
        // Check if post is excluded by path patterns
        let post_path = post.file_path.to_string_lossy();
        for exclude_pattern in &self.config.exclude {
            if post_path.contains(exclude_pattern) {
                return false;
            }
        }

        // Only include published posts
        post.metadata.status == crate::content::PostStatus::Published
    }

    /// Convert a post to a search document
    fn post_to_search_document(&self, post: &Post) -> Result<SearchDocument> {
        // Convert markdown to plain text
        let plain_text = markdown::markdown_to_text(&post.content);

        // Truncate content if needed
        let content = if plain_text.len() > self.config.max_content_chars {
            let truncated = &plain_text[..self.config.max_content_chars];
            // Find the last complete word
            if let Some(last_space) = truncated.rfind(' ') {
                format!("{}...", &truncated[..last_space])
            } else {
                format!("{}...", truncated)
            }
        } else {
            plain_text
        };

        // Generate excerpt
        let excerpt = markdown::extract_excerpt(&post.content, self.config.excerpt_words);

        // Generate URL path
        let url = format!("/posts/{}.html", post.metadata.slug);

        Ok(SearchDocument {
            id: post.metadata.slug.clone(),
            url,
            title: post.metadata.title.clone(),
            tags: post.metadata.tags.clone(),
            date: post.metadata.date.format("%Y-%m-%d").to_string(),
            description: post.metadata.description.clone(),
            content,
            excerpt,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{Post, PostMetadata, PostStatus};
    use chrono::Utc;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_post() -> Post {
        Post {
            metadata: PostMetadata {
                title: "Test Post".to_string(),
                date: Utc::now(),
                author: "Test Author".to_string(),
                description: "A test post".to_string(),
                tags: vec!["test".to_string(), "example".to_string()],
                status: PostStatus::Published,
                slug: "test-post".to_string(),
                featured: false,
            },
            content: "# Test Post\n\nThis is a test post with some content.".to_string(),
            file_path: PathBuf::from("test-post.md"),
        }
    }

    #[test]
    fn test_search_document_creation() {
        let indexer = SearchIndexer::new();
        let post = create_test_post();
        let document = indexer.post_to_search_document(&post).unwrap();

        assert_eq!(document.id, "test-post");
        assert_eq!(document.title, "Test Post");
        assert_eq!(document.url, "/posts/test-post.html");
        assert_eq!(document.tags, vec!["test", "example"]);
        assert!(document.content.contains("This is a test post"));
    }

    #[test]
    fn test_index_generation() {
        let temp_dir = TempDir::new().unwrap();
        let indexer = SearchIndexer::new();
        let post = create_test_post();

        indexer.generate_index(&[post], temp_dir.path()).unwrap();

        let index_file = temp_dir.path().join("search_index.json");
        assert!(index_file.exists());

        let content = fs::read_to_string(&index_file).unwrap();
        let documents: Vec<SearchDocument> = serde_json::from_str(&content).unwrap();
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].id, "test-post");
    }

    #[test]
    fn test_draft_exclusion() {
        let mut post = create_test_post();
        post.metadata.status = PostStatus::Draft;

        let indexer = SearchIndexer::new();
        let temp_dir = TempDir::new().unwrap();

        indexer.generate_index(&[post], temp_dir.path()).unwrap();

        let index_file = temp_dir.path().join("search_index.json");
        let content = fs::read_to_string(&index_file).unwrap();
        let documents: Vec<SearchDocument> = serde_json::from_str(&content).unwrap();
        assert_eq!(documents.len(), 0);
    }
}
