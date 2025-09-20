use crate::content::PostManager;
use crate::generator::assets::get_mime_type;
use crate::generator::SiteBuilder;
use crate::project::Project;
use crate::utils::Console;
use anyhow::{anyhow, Result};
use axum::{
    body::Body,
    extract::{Path as AxumPath, Query},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tower::ServiceBuilder;

#[derive(Clone)]
struct AppState {
    output_dir: PathBuf,
    project: Project,
    include_drafts: bool,
}

#[derive(Deserialize)]
struct PostsQuery {
    page: Option<usize>,
    limit: Option<usize>,
}

#[derive(Serialize)]
struct PostsResponse {
    posts: Vec<serde_json::Value>,
    has_more: bool,
    total: usize,
    page: usize,
    limit: usize,
}

pub async fn handle_serve(port: u16, host: String, drafts: bool, open: bool) -> Result<()> {
    Console::info(&format!("Starting development server on {}:{}", host, port));

    // Check if we're in a blogr project
    let project = Project::find_project()?
        .ok_or_else(|| anyhow!("Not in a blogr project. Run 'blogr init' first."))?;

    // Build site initially
    Console::info("Building site...");

    // Load config to get the correct output directory
    let config = project.load_config()?;
    let output_dir = config
        .build
        .output_dir
        .as_ref()
        .map(|p| project.root.join(p))
        .unwrap_or_else(|| project.root.join("_site"));

    let site_builder = SiteBuilder::new(project.clone(), Some(output_dir.clone()), drafts, false)?;
    site_builder.build()?;

    // Create router
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/posts", get(serve_posts_api))
        .route("/*path", get(serve_file))
        .with_state(AppState {
            output_dir: output_dir.clone(),
            project: project.clone(),
            include_drafts: drafts,
        })
        .layer(ServiceBuilder::new());

    Console::success(&format!(
        "Development server running at http://{}:{}",
        host, port
    ));
    println!("📝 Site built and ready");
    println!("🌐 Server serving from: {}", output_dir.display());
    if drafts {
        println!("📝 Including draft posts");
    }
    println!("Press Ctrl+C to stop");

    // Open browser if requested
    if open {
        let url = format!("http://{}:{}", host, port);
        Console::info(&format!("Opening browser to {}", url));
        if let Err(e) = ::open::that(&url) {
            Console::warn(&format!("Failed to open browser: {}", e));
        }
    }

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn serve_index(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    serve_file_from_path(state.output_dir.join("index.html")).await
}

async fn serve_file(
    AxumPath(path): AxumPath<String>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let file_path = state.output_dir.join(&path);

    // If it's a directory, try to serve index.html
    if file_path.is_dir() {
        let index_path = file_path.join("index.html");
        if index_path.exists() {
            return serve_file_from_path(index_path).await;
        }
    }

    // Try to serve the file directly
    if file_path.exists() && file_path.is_file() {
        return serve_file_from_path(file_path).await;
    }

    // If it's an HTML request without .html extension, try adding it
    if !path.ends_with(".html") && !path.contains('.') {
        let html_path = state.output_dir.join(format!("{}.html", path));
        if html_path.exists() {
            return serve_file_from_path(html_path).await;
        }
    }

    // File not found
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("File not found"))
        .unwrap()
}

async fn serve_posts_api(
    Query(params): Query<PostsQuery>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // Load all posts
    let post_manager = PostManager::new(state.project.posts_dir());
    let posts = match post_manager.load_all_posts() {
        Ok(mut posts) => {
            // Filter drafts if not including them
            if !state.include_drafts {
                posts.retain(|p| p.metadata.status == crate::content::PostStatus::Published);
            }
            posts
        }
        Err(e) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(format!(
                    r#"{{"error": "Failed to load posts: {}"}}"#,
                    e
                )))
                .unwrap();
        }
    };

    let total = posts.len();
    let start = (page.saturating_sub(1)) * limit;
    let end = (start + limit).min(total);

    let page_posts: Vec<_> = posts.iter().skip(start).take(limit).collect();
    let mut posts_with_content = Vec::new();

    for post in &page_posts {
        // Convert markdown to HTML for each post
        let html_content = match crate::generator::markdown::render_markdown(&post.content) {
            Ok(content) => content,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to render markdown for post {}: {}",
                    post.metadata.slug, e
                );
                post.content.clone() // Fallback to raw content
            }
        };

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

    let response = PostsResponse {
        posts: posts_with_content,
        has_more: end < total,
        total,
        page,
        limit,
    };

    Json(response).into_response()
}

async fn serve_file_from_path(path: PathBuf) -> Response {
    match fs::read(&path).await {
        Ok(content) => {
            let mime_type = get_mime_type(&path);
            Response::builder()
                .header(header::CONTENT_TYPE, mime_type)
                .body(Body::from(content))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Error reading file"))
            .unwrap(),
    }
}
