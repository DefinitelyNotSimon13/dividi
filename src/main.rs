use std::sync::Arc;

use anyhow::Context;
use axum::routing::{delete, get};
use axum::{Router, routing::post};
use handler::files::{delete_file, download_file, get_files};
use handler::{files::upload_file, home::home};
use state::AppState;
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handler;
mod middleware;
mod state;
mod templates;
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

    info!("Initializing router...");

    let app_state = Arc::new(AppState::new().await);

    let assets_path = std::env::current_dir()?;

    let port: u16 = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(8000);

    let api_router = Router::new()
        .route("/files", get(get_files))
        .route("/files", post(upload_file))
        .route("/files/{file_name}", delete(delete_file));

    let app = Router::new()
        .nest("/api", api_router)
        .route("/", get(home))
        .route("/download/{file_name}", get(download_file))
        .route_service("/favicon.ico", ServeFile::new("public/favicon.ico"))
        .nest_service(
            "/public",
            ServeDir::new(format!("{}/public", assets_path.to_str().unwrap())),
        )
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
