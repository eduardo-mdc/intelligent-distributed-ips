use crate::metrics::MetricsAggregator;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type AppState = Arc<RwLock<MetricsAggregator>>;

/// Serve the dashboard HTML
pub async fn dashboard_handler() -> impl IntoResponse {
    Html(include_str!("../../static/index.html"))
}

/// Get real-time metrics
pub async fn realtime_metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    let aggregator = state.read().await;
    Json(aggregator.get_realtime_metrics())
}

/// Get summary statistics
pub async fn summary_stats_handler(State(state): State<AppState>) -> impl IntoResponse {
    let aggregator = state.read().await;
    Json(aggregator.get_summary())
}

/// Health check endpoint
pub async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
