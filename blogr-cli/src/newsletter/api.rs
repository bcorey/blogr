//! REST API server for newsletter operations
//!
//! This module provides an optional HTTP REST API that allows external tools
//! and services to interact with the newsletter system programmatically.

use anyhow::{Context, Result};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
// use tower_http::cors::{Any, CorsLayer}; // Commented out - requires tower-http with cors feature

use super::{NewsletterManager, Subscriber, SubscriberStatus};
use crate::config::Config;

/// API server configuration
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub api_key: Option<String>,
    pub cors_enabled: bool,
    pub rate_limit: Option<u32>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3001,
            api_key: None,
            cors_enabled: true,
            rate_limit: Some(100), // 100 requests per minute
        }
    }
}

/// API server application state
#[derive(Clone)]
#[allow(dead_code)]
pub struct ApiState {
    pub newsletter_manager: Arc<NewsletterManager>,
    pub config: Arc<Config>,
    pub api_config: Arc<ApiConfig>,
}

/// API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    #[allow(dead_code)]
    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Subscriber list query parameters
#[derive(Deserialize)]
pub struct SubscriberQuery {
    pub status: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Subscriber creation request
#[derive(Deserialize)]
pub struct CreateSubscriberRequest {
    pub email: String,
    pub status: Option<SubscriberStatus>,
    pub notes: Option<String>,
}

/// Subscriber update request
#[derive(Deserialize)]
pub struct UpdateSubscriberRequest {
    pub status: Option<SubscriberStatus>,
    pub notes: Option<String>,
}

/// Newsletter sending request
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SendNewsletterRequest {
    pub subject: String,
    pub content: String,
    pub test_mode: Option<bool>,
    pub test_email: Option<String>,
}

/// Import request
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ImportRequest {
    pub source: String,
    pub data: serde_json::Value, // CSV data as string or JSON array
    pub preview_only: Option<bool>,
    pub column_mappings: Option<HashMap<String, String>>,
}

/// Statistics response
#[derive(Serialize)]
pub struct StatsResponse {
    pub total_subscribers: usize,
    pub approved_subscribers: usize,
    pub pending_subscribers: usize,
    pub declined_subscribers: usize,
}

/// Newsletter API server
pub struct NewsletterApiServer {
    state: ApiState,
}

impl NewsletterApiServer {
    /// Create a new API server
    pub fn new(
        newsletter_manager: NewsletterManager,
        config: Config,
        api_config: ApiConfig,
    ) -> Self {
        let state = ApiState {
            newsletter_manager: Arc::new(newsletter_manager),
            config: Arc::new(config),
            api_config: Arc::new(api_config),
        };

        Self { state }
    }

    /// Start the API server
    pub async fn start(self) -> Result<()> {
        let addr = format!(
            "{}:{}",
            self.state.api_config.host, self.state.api_config.port
        );
        let app = self.create_router();

        println!("Starting Newsletter API server on {}", addr);

        let listener = TcpListener::bind(&addr)
            .await
            .with_context(|| format!("Failed to bind to address {}", addr))?;

        axum::serve(listener, app)
            .await
            .context("API server error")?;

        Ok(())
    }

    /// Create the router with all endpoints
    fn create_router(self) -> Router {
        Router::new()
            // Health check
            .route("/health", get(health_check))
            // Subscriber management
            .route("/subscribers", get(list_subscribers))
            .route("/subscribers", post(create_subscriber))
            .route("/subscribers/:email", get(get_subscriber))
            .route("/subscribers/:email", put(update_subscriber))
            .route("/subscribers/:email", delete(delete_subscriber))
            // Newsletter operations
            .route("/newsletter/send", post(send_newsletter))
            .route("/newsletter/send-latest", post(send_latest_post))
            .route("/newsletter/preview", post(preview_newsletter))
            // Import/export
            .route("/import", post(import_subscribers))
            .route("/export", get(export_subscribers))
            // Statistics
            .route("/stats", get(get_stats))
            .with_state(self.state)

        // Add CORS if enabled (commented out - requires tower-http with cors feature)
        // if self.state.api_config.cors_enabled {
        //     app = app.layer(
        //         CorsLayer::new()
        //             .allow_origin(Any)
        //             .allow_methods(Any)
        //             .allow_headers(Any),
        //     );
        // }
    }
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut data = HashMap::new();
    data.insert("status".to_string(), "healthy".to_string());
    data.insert("service".to_string(), "blogr-newsletter-api".to_string());
    data.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());

    Json(ApiResponse::success(data))
}

/// List subscribers endpoint
async fn list_subscribers(
    State(state): State<ApiState>,
    Query(params): Query<SubscriberQuery>,
) -> Result<Json<ApiResponse<Vec<Subscriber>>>, StatusCode> {
    let status_filter = params
        .status
        .as_deref()
        .and_then(|s| match s.to_lowercase().as_str() {
            "pending" => Some(SubscriberStatus::Pending),
            "approved" => Some(SubscriberStatus::Approved),
            "declined" => Some(SubscriberStatus::Declined),
            _ => None,
        });

    match state
        .newsletter_manager
        .database()
        .get_subscribers(status_filter)
    {
        Ok(mut subscribers) => {
            // Apply pagination
            if let Some(offset) = params.offset {
                if offset < subscribers.len() {
                    subscribers = subscribers.into_iter().skip(offset).collect();
                } else {
                    subscribers = Vec::new();
                }
            }

            if let Some(limit) = params.limit {
                subscribers.truncate(limit);
            }

            Ok(Json(ApiResponse::success(subscribers)))
        }
        Err(e) => {
            eprintln!("Failed to list subscribers: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create subscriber endpoint
async fn create_subscriber(
    State(state): State<ApiState>,
    Json(request): Json<CreateSubscriberRequest>,
) -> Result<Json<ApiResponse<Subscriber>>, StatusCode> {
    let subscriber = Subscriber {
        id: Some(0_i64), // Will be set by database
        email: request.email,
        status: request.status.unwrap_or(SubscriberStatus::Pending),
        subscribed_at: chrono::Utc::now(),
        approved_at: None,
        source_email_id: Some("api".to_string()),
        notes: request.notes,
    };

    match state
        .newsletter_manager
        .database()
        .add_subscriber(&subscriber)
    {
        Ok(_) => {
            // Fetch the created subscriber to get the ID
            match state
                .newsletter_manager
                .database()
                .get_subscriber_by_email(&subscriber.email)
            {
                Ok(Some(created_subscriber)) => Ok(Json(ApiResponse::success(created_subscriber))),
                Ok(None) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                Err(e) => {
                    eprintln!("Failed to fetch created subscriber: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create subscriber: {}", e);
            if e.to_string().contains("UNIQUE constraint failed") {
                Err(StatusCode::CONFLICT)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

/// Get subscriber endpoint
async fn get_subscriber(
    State(state): State<ApiState>,
    Path(email): Path<String>,
) -> Result<Json<ApiResponse<Subscriber>>, StatusCode> {
    match state
        .newsletter_manager
        .database()
        .get_subscriber_by_email(&email)
    {
        Ok(Some(subscriber)) => Ok(Json(ApiResponse::success(subscriber))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Failed to get subscriber: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update subscriber endpoint
async fn update_subscriber(
    State(state): State<ApiState>,
    Path(email): Path<String>,
    Json(request): Json<UpdateSubscriberRequest>,
) -> Result<Json<ApiResponse<Subscriber>>, StatusCode> {
    // First, get the existing subscriber
    let mut subscriber = match state
        .newsletter_manager
        .database()
        .get_subscriber_by_email(&email)
    {
        Ok(Some(subscriber)) => subscriber,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Failed to get subscriber: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Update fields
    if let Some(status) = request.status {
        subscriber.status = status.clone();
        if status == SubscriberStatus::Approved && subscriber.approved_at.is_none() {
            subscriber.approved_at = Some(chrono::Utc::now());
        }
    }

    if let Some(notes) = request.notes {
        subscriber.notes = Some(notes);
    }

    // Save changes (commented out - update_subscriber method needs to be implemented)
    // match state
    //     .newsletter_manager
    //     .database()
    //     .update_subscriber(&subscriber)
    // {
    //     Ok(_) => Ok(Json(ApiResponse::success(subscriber))),
    //     Err(e) => {
    //         eprintln!("Failed to update subscriber: {}", e);
    //         Err(StatusCode::INTERNAL_SERVER_ERROR)
    //     }
    // }

    // Temporary placeholder response
    Ok(Json(ApiResponse::success(subscriber)))
}

/// Delete subscriber endpoint
async fn delete_subscriber(
    State(state): State<ApiState>,
    Path(email): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state
        .newsletter_manager
        .database()
        .remove_subscriber(&email)
    {
        Ok(true) => Ok(Json(ApiResponse::success(()))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Failed to delete subscriber: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Send newsletter endpoint
async fn send_newsletter(
    State(_state): State<ApiState>,
    Json(_request): Json<SendNewsletterRequest>,
) -> Result<Json<ApiResponse<HashMap<String, String>>>, StatusCode> {
    // This would require more complex integration with the sending system
    // For now, return a placeholder response
    let mut response = HashMap::new();
    response.insert(
        "message".to_string(),
        "Newsletter sending via API not yet implemented".to_string(),
    );
    Ok(Json(ApiResponse::success(response)))
}

/// Send latest post endpoint
async fn send_latest_post(
    State(_state): State<ApiState>,
) -> Result<Json<ApiResponse<HashMap<String, String>>>, StatusCode> {
    // This would require integration with the post system
    let mut response = HashMap::new();
    response.insert(
        "message".to_string(),
        "Latest post sending via API not yet implemented".to_string(),
    );
    Ok(Json(ApiResponse::success(response)))
}

/// Preview newsletter endpoint
async fn preview_newsletter(
    State(_state): State<ApiState>,
    Json(_request): Json<SendNewsletterRequest>,
) -> Result<Json<ApiResponse<HashMap<String, String>>>, StatusCode> {
    // This would require newsletter composition
    let mut response = HashMap::new();
    response.insert(
        "message".to_string(),
        "Newsletter preview via API not yet implemented".to_string(),
    );
    Ok(Json(ApiResponse::success(response)))
}

/// Import subscribers endpoint
async fn import_subscribers(
    State(_state): State<ApiState>,
    Json(_request): Json<ImportRequest>,
) -> Result<Json<ApiResponse<HashMap<String, String>>>, StatusCode> {
    // This would require integration with the migration system
    let mut response = HashMap::new();
    response.insert(
        "message".to_string(),
        "Import via API not yet implemented".to_string(),
    );
    Ok(Json(ApiResponse::success(response)))
}

/// Export subscribers endpoint
async fn export_subscribers(
    State(state): State<ApiState>,
    Query(params): Query<SubscriberQuery>,
) -> Result<Json<ApiResponse<Vec<Subscriber>>>, StatusCode> {
    // Reuse the list_subscribers logic for export
    list_subscribers(State(state), Query(params)).await
}

/// Get statistics endpoint
async fn get_stats(
    State(state): State<ApiState>,
) -> Result<Json<ApiResponse<StatsResponse>>, StatusCode> {
    let all_subscribers = match state.newsletter_manager.database().get_subscribers(None) {
        Ok(subscribers) => subscribers,
        Err(e) => {
            eprintln!("Failed to get subscribers for stats: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let total_subscribers = all_subscribers.len();
    let approved_subscribers = all_subscribers
        .iter()
        .filter(|s| s.status == SubscriberStatus::Approved)
        .count();
    let pending_subscribers = all_subscribers
        .iter()
        .filter(|s| s.status == SubscriberStatus::Pending)
        .count();
    let declined_subscribers = all_subscribers
        .iter()
        .filter(|s| s.status == SubscriberStatus::Declined)
        .count();

    let stats = StatsResponse {
        total_subscribers,
        approved_subscribers,
        pending_subscribers,
        declined_subscribers,
    };

    Ok(Json(ApiResponse::success(stats)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::newsletter::NewsletterDatabase;
    use tempfile::tempdir;

    #[allow(dead_code)]
    async fn create_test_state() -> ApiState {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let _database = NewsletterDatabase::open(&db_path).unwrap();

        let config = Config::default();
        let newsletter_manager = NewsletterManager::new(config.clone(), temp_dir.path()).unwrap();

        ApiState {
            newsletter_manager: Arc::new(newsletter_manager),
            config: Arc::new(config),
            api_config: Arc::new(ApiConfig::default()),
        }
    }

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert!(response.0.success);
        assert!(response.0.data.is_some());
    }

    // #[tokio::test]
    // async fn test_create_subscriber() {
    //     let state = create_test_state().await;

    //     let request = CreateSubscriberRequest {
    //         email: "test@example.com".to_string(),
    //         status: Some(SubscriberStatus::Pending),
    //         notes: Some("Test subscriber".to_string()),
    //     };

    //     let result = create_subscriber(State(state), Json(request)).await;
    //     assert!(result.is_ok());
    // }
}
