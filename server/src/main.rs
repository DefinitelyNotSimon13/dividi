use std::sync::Arc;

use anyhow::Context;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use axum::{Router, routing::post};
use handler::files::upload_file;
use handler::files::{delete_file, download_file, get_files};
use state::AppState;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handler;
mod middleware;
mod state;
mod uploaded_file;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    trace!("initializing app state...");

    let app_state = Arc::new(AppState::new().await);

    let assets_path = "../public";

    let port: u16 = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(8000);

    trace!("initializing file router...");
    let file_router = Router::new()
        .route("/", get(get_files))
        .route("/", post(upload_file))
        .route("/{file_name}", delete(delete_file))
        .route("/{file_name}/download", get(download_file));

    info!("initializing v1 router...");
    let v1_router = Router::new()
        .route("/", get(health_check))
        .nest("/files", file_router);

    info!("initializing app router...");
    let app = Router::new()
        .nest("/v1", v1_router)
        .layer(request_tracing!())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .context("error while creating listener")?;

    info!("router initialized, listening on port {port}");

    axum::serve(listener, app)
        .await
        .context("error while starting server")?;

    info!("server shutting down...");

    Ok(())
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "healthy")
}
