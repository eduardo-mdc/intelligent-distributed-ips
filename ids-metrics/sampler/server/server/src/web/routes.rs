use super::handlers::{
    dashboard_handler, health_handler, realtime_metrics_handler, summary_stats_handler,
    AppState,
};
use axum::{routing::get, Router};
use log::info;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

/// Start the Axum web server
pub async fn start_server(port: u16, state: AppState) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(dashboard_handler))
        .route("/api/metrics/realtime", get(realtime_metrics_handler))
        .route("/api/stats/summary", get(summary_stats_handler))
        .route("/health", get(health_handler))
        .nest_service("/static", ServeDir::new("server/static"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Web server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Handle graceful shutdown on Ctrl+C or SIGTERM
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received");
}
