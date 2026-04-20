use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use bytes::Bytes;
use serde_json::json;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use common::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    common::init_tracing();

    let config = AppConfig::default();
    let app = build_router();

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!(%addr, "web server listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn build_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/photo/info", post(photo_info))
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

async fn root() -> &'static str {
    "rust-devin web service"
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

/// Accept a raw image payload and return its dimensions + detected format.
async fn photo_info(body: Bytes) -> impl IntoResponse {
    match photo::decode(&body) {
        Ok((_image, info)) => (StatusCode::OK, Json(json!(info))).into_response(),
        Err(err) => {
            tracing::warn!(?err, "failed to decode photo");
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response()
        }
    }
}
