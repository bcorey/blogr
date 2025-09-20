use crate::generator::assets::get_mime_type;
use crate::generator::SiteBuilder;
use crate::project::Project;
use crate::utils::Console;
use anyhow::{anyhow, Result};
use axum::{
    body::Body,
    extract::Path as AxumPath,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::path::PathBuf;
use tokio::fs;
use tower::ServiceBuilder;

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
        .route("/*path", get(serve_file))
        .with_state(output_dir.clone())
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
        // TODO: Open browser - could use `open` crate
    }

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn serve_index(
    axum::extract::State(output_dir): axum::extract::State<PathBuf>,
) -> impl IntoResponse {
    serve_file_from_path(output_dir.join("index.html")).await
}

async fn serve_file(
    AxumPath(path): AxumPath<String>,
    axum::extract::State(output_dir): axum::extract::State<PathBuf>,
) -> impl IntoResponse {
    let file_path = output_dir.join(&path);

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
        let html_path = output_dir.join(format!("{}.html", path));
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
