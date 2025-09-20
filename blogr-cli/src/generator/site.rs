use crate::config::Config;
use crate::content::{Post, PostManager, PostStatus};
use crate::project::Project;
use anyhow::{anyhow, Result};
use blogr_themes::{get_theme_by_name, Theme};
use chrono::{Datelike, Utc};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

/// Static site generator
pub struct SiteBuilder {
    /// Project reference
    project: Project,
    /// Build configuration
    config: Config,
    /// Template engine
    tera: Tera,
    /// Theme instance
    theme: Box<dyn Theme>,
    /// Output directory
    output_dir: PathBuf,
    /// Include drafts in build
    include_drafts: bool,
    /// Include future posts in build
    include_future: bool,
}

impl SiteBuilder {
    /// Create a new site builder
    pub fn new(
        project: Project,
        output_dir: Option<PathBuf>,
        include_drafts: bool,
        include_future: bool,
    ) -> Result<Self> {
        let config = project.load_config()?;

        // Get theme
        let theme_name = &config.theme.name;
        let theme = get_theme_by_name(theme_name)
            .ok_or_else(|| anyhow!("Theme '{}' not found", theme_name))?;

        // Set up template engine (create empty Tera instance)
        let mut tera = Tera::default();

        // Register theme templates - base template first
        let templates = theme.templates();

        // Register base template first if it exists
        if let Some(base_template) = templates.get("base.html") {
            tera.add_raw_template("base.html", base_template)
                .map_err(|e| anyhow!("Failed to register base template: {}", e))?;
        }

        // Register all other templates
        for (name, template) in &templates {
            if name != "base.html" {
                tera.add_raw_template(name, template)
                    .map_err(|e| anyhow!("Failed to register template '{}': {}", name, e))?;
            }
        }

        let output_dir = output_dir.unwrap_or_else(|| {
            config
                .build
                .output_dir
                .as_ref()
                .map(|p| project.root.join(p))
                .unwrap_or_else(|| project.root.join("_site"))
        });

        Ok(Self {
            project,
            config,
            tera,
            theme,
            output_dir,
            include_drafts,
            include_future,
        })
    }

    /// Build the entire site
    pub fn build(&self) -> Result<()> {
        println!("🚀 Building site with theme '{}'", self.config.theme.name);

        // Clean output directory
        self.clean_output_dir()?;

        // Load all posts
        let post_manager = PostManager::new(self.project.posts_dir());
        let mut all_posts = post_manager.load_all_posts()?;

        // Filter posts based on build options
        all_posts.retain(|post| self.should_include_post(post));

        // Sort posts by date (newest first)
        all_posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));

        println!("📝 Processing {} posts", all_posts.len());

        // Generate individual post pages
        self.generate_post_pages(&all_posts)?;

        // Generate index page
        self.generate_index_page(&all_posts)?;

        // Generate archive pages
        self.generate_archive_pages(&all_posts)?;

        // Generate tag pages
        self.generate_tag_pages(&all_posts)?;

        // Generate RSS feed
        self.generate_rss_feed(&all_posts)?;

        // Copy theme assets
        self.copy_theme_assets()?;

        // Copy project static assets
        self.copy_static_assets()?;

        // Generate CNAME file if domain configuration exists
        self.generate_cname_file()?;

        println!(
            "✅ Site built successfully to: {}",
            self.output_dir.display()
        );
        Ok(())
    }

    /// Check if a post should be included in the build
    fn should_include_post(&self, post: &Post) -> bool {
        // Check draft status
        if post.metadata.status == PostStatus::Draft && !self.include_drafts {
            return false;
        }

        // Check future posts
        if !self.include_future {
            let now = Utc::now();
            if post.metadata.date > now {
                return false;
            }
        }

        true
    }

    /// Clean the output directory
    fn clean_output_dir(&self) -> Result<()> {
        if self.output_dir.exists() {
            fs::remove_dir_all(&self.output_dir)
                .map_err(|e| anyhow!("Failed to clean output directory: {}", e))?;
        }
        fs::create_dir_all(&self.output_dir)
            .map_err(|e| anyhow!("Failed to create output directory: {}", e))?;
        Ok(())
    }

    /// Generate individual post pages
    fn generate_post_pages(&self, posts: &[Post]) -> Result<()> {
        for post in posts {
            let mut context = Context::new();

            // Add site config
            context.insert("site", &self.config);

            // Add post data
            context.insert("post", post);

            // Convert markdown to HTML
            let html_content = crate::generator::markdown::render_markdown(&post.content)?;
            context.insert("content", &html_content);

            // Calculate reading time (average 200 words per minute)
            let word_count = post.content.split_whitespace().count();
            let reading_time = (word_count / 200).max(1);
            context.insert("reading_time", &reading_time);

            // Render template
            let html = self
                .tera
                .render("post.html", &context)
                .map_err(|e| anyhow!("Failed to render post template: {}", e))?;

            // Write to file
            let post_dir = self.output_dir.join("posts");
            fs::create_dir_all(&post_dir)?;

            let post_file = post_dir.join(format!("{}.html", post.metadata.slug));
            fs::write(&post_file, html).map_err(|e| anyhow!("Failed to write post file: {}", e))?;
        }
        Ok(())
    }

    /// Generate index page
    fn generate_index_page(&self, posts: &[Post]) -> Result<()> {
        let mut context = Context::new();

        // Add site config
        context.insert("site", &self.config);

        // Prepare posts with rendered content for index
        let recent_posts: Vec<_> = posts.iter().take(10).collect();
        let mut posts_with_content = Vec::new();

        for post in &recent_posts {
            // Convert markdown to HTML for each post
            let html_content = crate::generator::markdown::render_markdown(&post.content)?;

            // Calculate reading time (average 200 words per minute)
            let word_count = post.content.split_whitespace().count();
            let reading_time = (word_count / 200).max(1);

            // Create a struct that includes both post data and rendered content
            let post_data = serde_json::json!({
                "metadata": post.metadata,
                "content": html_content,
                "reading_time": reading_time
            });

            posts_with_content.push(post_data);
        }

        context.insert("posts", &posts_with_content);

        // Add pagination info
        context.insert("has_more", &(posts.len() > 10));
        context.insert("total_posts", &posts.len());

        // Render template
        let html = self
            .tera
            .render("index.html", &context)
            .map_err(|e| anyhow!("Failed to render index template: {}", e))?;

        // Write to file
        let index_file = self.output_dir.join("index.html");
        fs::write(&index_file, html).map_err(|e| anyhow!("Failed to write index file: {}", e))?;

        Ok(())
    }

    /// Generate archive pages
    fn generate_archive_pages(&self, posts: &[Post]) -> Result<()> {
        let mut context = Context::new();

        // Add site config
        context.insert("site", &self.config);

        // Prepare posts with rendered content
        let mut posts_with_content = Vec::new();
        for post in posts {
            // Convert markdown to HTML for each post
            let html_content = crate::generator::markdown::render_markdown(&post.content)?;

            // Calculate reading time (average 200 words per minute)
            let word_count = post.content.split_whitespace().count();
            let reading_time = (word_count / 200).max(1);

            let post_data = serde_json::json!({
                "metadata": post.metadata,
                "content": html_content,
                "reading_time": reading_time
            });

            posts_with_content.push(post_data);
        }

        context.insert("posts", &posts_with_content);

        // Group posts by year
        let mut posts_by_year: HashMap<i32, Vec<serde_json::Value>> = HashMap::new();
        for post_data in &posts_with_content {
            let year = post_data["metadata"]["date"]
                .as_str()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.year())
                .unwrap_or(2024); // fallback year
            posts_by_year
                .entry(year)
                .or_default()
                .push(post_data.clone());
        }
        context.insert("posts_by_year", &posts_by_year);

        // Render template
        let html = self
            .tera
            .render("archive.html", &context)
            .map_err(|e| anyhow!("Failed to render archive template: {}", e))?;

        // Write to file
        let archive_file = self.output_dir.join("archive.html");
        fs::write(&archive_file, html)
            .map_err(|e| anyhow!("Failed to write archive file: {}", e))?;

        Ok(())
    }

    /// Generate tag pages
    fn generate_tag_pages(&self, posts: &[Post]) -> Result<()> {
        // Group posts by tag
        let mut posts_by_tag: HashMap<String, Vec<&Post>> = HashMap::new();
        for post in posts {
            for tag in &post.metadata.tags {
                posts_by_tag.entry(tag.clone()).or_default().push(post);
            }
        }

        // Create tags directory
        let tags_dir = self.output_dir.join("tags");
        fs::create_dir_all(&tags_dir)?;

        // Generate individual tag pages
        for (tag, tag_posts) in &posts_by_tag {
            let mut context = Context::new();

            // Add site config
            context.insert("site", &self.config);

            // Add tag info
            context.insert("tag", tag);

            // Prepare posts with rendered content for this tag
            let mut posts_with_content = Vec::new();
            for post in tag_posts {
                // Convert markdown to HTML for each post
                let html_content = crate::generator::markdown::render_markdown(&post.content)?;

                // Calculate reading time (average 200 words per minute)
                let word_count = post.content.split_whitespace().count();
                let reading_time = (word_count / 200).max(1);

                let post_data = serde_json::json!({
                    "metadata": post.metadata,
                    "content": html_content,
                    "reading_time": reading_time
                });

                posts_with_content.push(post_data);
            }

            context.insert("posts", &posts_with_content);

            // Render template
            let html = self
                .tera
                .render("tag.html", &context)
                .map_err(|e| anyhow!("Failed to render tag template for '{}': {}", tag, e))?;

            // Write to file
            let tag_file = tags_dir.join(format!("{}.html", tag));
            fs::write(&tag_file, html)
                .map_err(|e| anyhow!("Failed to write tag file for '{}': {}", tag, e))?;
        }

        // Generate tags index
        let mut context = Context::new();
        context.insert("site", &self.config);

        // Create tag info with post counts
        let tag_info: Vec<(String, usize)> = posts_by_tag
            .iter()
            .map(|(tag, posts)| (tag.clone(), posts.len()))
            .collect();
        context.insert("tags", &tag_info);

        let html = self
            .tera
            .render("tags.html", &context)
            .map_err(|e| anyhow!("Failed to render tags index template: {}", e))?;

        let tags_index = self.output_dir.join("tags").join("index.html");
        fs::write(&tags_index, html).map_err(|e| anyhow!("Failed to write tags index: {}", e))?;

        Ok(())
    }

    /// Generate RSS feed
    fn generate_rss_feed(&self, posts: &[Post]) -> Result<()> {
        // Get effective base URL for all feed URLs
        let effective_base_url = self.config.get_effective_base_url();

        // Take only the most recent 20 posts for the RSS feed
        let recent_posts: Vec<&Post> = posts.iter().take(20).collect();

        let mut rss_items = Vec::new();

        for post in recent_posts {
            // Convert markdown to HTML for RSS content
            let html_content = crate::generator::markdown::render_markdown(&post.content)?;

            // Create RSS item
            let post_url = format!(
                "{}/posts/{}.html",
                effective_base_url.trim_end_matches('/'),
                post.metadata.slug
            );

            let rss_item = format!(
                r#"    <item>
      <title><![CDATA[{}]]></title>
      <link>{}</link>
      <guid>{}</guid>
      <description><![CDATA[{}]]></description>
      <pubDate>{}</pubDate>
      <author>{}</author>
    </item>"#,
                post.metadata.title,
                post_url,
                post_url,
                html_content,
                post.metadata.date.format("%a, %d %b %Y %H:%M:%S %z"),
                self.config.blog.author
            );

            rss_items.push(rss_item);
        }

        // Generate RSS XML
        let rss_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title><![CDATA[{}]]></title>
    <link>{}</link>
    <atom:link href="{}/rss.xml" rel="self" type="application/rss+xml" />
    <description><![CDATA[{}]]></description>
    <language>{}</language>
    <lastBuildDate>{}</lastBuildDate>
    <generator>Blogr Static Site Generator</generator>
{}
  </channel>
</rss>"#,
            self.config.blog.title,
            effective_base_url,
            effective_base_url.trim_end_matches('/'),
            self.config.blog.description,
            self.config.blog.language.as_deref().unwrap_or("en"),
            Utc::now().format("%a, %d %b %Y %H:%M:%S %z"),
            rss_items.join("\n")
        );

        // Write RSS feed
        let rss_file = self.output_dir.join("rss.xml");
        fs::write(&rss_file, rss_content)
            .map_err(|e| anyhow!("Failed to write RSS feed: {}", e))?;

        // Also generate Atom feed
        self.generate_atom_feed(posts)?;

        Ok(())
    }

    /// Generate Atom feed
    fn generate_atom_feed(&self, posts: &[Post]) -> Result<()> {
        // Get effective base URL for all feed URLs
        let effective_base_url = self.config.get_effective_base_url();

        // Take only the most recent 20 posts for the Atom feed
        let recent_posts: Vec<&Post> = posts.iter().take(20).collect();

        let mut atom_entries = Vec::new();

        for post in recent_posts {
            // Convert markdown to HTML for Atom content
            let html_content = crate::generator::markdown::render_markdown(&post.content)?;

            // Create Atom entry
            let post_url = format!(
                "{}/posts/{}.html",
                effective_base_url.trim_end_matches('/'),
                post.metadata.slug
            );

            let atom_entry = format!(
                r#"  <entry>
    <title><![CDATA[{}]]></title>
    <link href="{}"/>
    <id>{}</id>
    <updated>{}</updated>
    <summary><![CDATA[{}]]></summary>
    <content type="html"><![CDATA[{}]]></content>
    <author>
      <name>{}</name>
    </author>
  </entry>"#,
                post.metadata.title,
                post_url,
                post_url,
                post.metadata.date.format("%Y-%m-%dT%H:%M:%S%z"),
                &post.metadata.description,
                html_content,
                self.config.blog.author
            );

            atom_entries.push(atom_entry);
        }

        // Generate Atom XML
        let atom_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title><![CDATA[{}]]></title>
  <link href="{}"/>
  <link href="{}/atom.xml" rel="self"/>
  <id>{}</id>
  <updated>{}</updated>
  <subtitle><![CDATA[{}]]></subtitle>
  <generator>Blogr Static Site Generator</generator>
{}
</feed>"#,
            self.config.blog.title,
            effective_base_url,
            effective_base_url.trim_end_matches('/'),
            effective_base_url,
            Utc::now().format("%Y-%m-%dT%H:%M:%S%z"),
            self.config.blog.description,
            atom_entries.join("\n")
        );

        // Write Atom feed
        let atom_file = self.output_dir.join("atom.xml");
        fs::write(&atom_file, atom_content)
            .map_err(|e| anyhow!("Failed to write Atom feed: {}", e))?;

        Ok(())
    }

    /// Copy theme assets
    fn copy_theme_assets(&self) -> Result<()> {
        let assets = self.theme.assets();
        for (path, content) in &assets {
            // Place assets directly in output directory (e.g., css/style.css -> /css/style.css)
            let asset_path = self.output_dir.join(path);

            // Create parent directories if needed
            if let Some(parent) = asset_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&asset_path, content)
                .map_err(|e| anyhow!("Failed to write asset '{}': {}", path, e))?;
        }

        Ok(())
    }

    /// Copy project static assets
    fn copy_static_assets(&self) -> Result<()> {
        let static_dir = self.project.root.join("static");
        if !static_dir.exists() {
            return Ok(());
        }

        let output_static = self.output_dir.join("static");
        crate::generator::assets::copy_dir_recursive(&static_dir, &output_static)
            .map_err(|e| anyhow!("Failed to copy static assets: {}", e))?;

        Ok(())
    }

    /// Get the output directory
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }

    /// Generate CNAME file for custom domains (GitHub Pages)
    fn generate_cname_file(&self) -> Result<()> {
        if let Some(domains) = &self.config.blog.domains {
            if let Some(github_domain) = &domains.github_pages_domain {
                let cname_file = self.output_dir.join("CNAME");
                fs::write(&cname_file, format!("{}\n", github_domain))
                    .map_err(|e| anyhow!("Failed to write CNAME file: {}", e))?;

                println!("📝 Generated CNAME file for: {}", github_domain);
            }
        }
        Ok(())
    }
}
