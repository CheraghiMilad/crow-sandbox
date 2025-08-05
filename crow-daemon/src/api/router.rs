use anyhow::Result;
use axum::Router;
use std::fs;
use tokio::net::TcpListener;

use crate::api::upload;

const UPLOAD_DIR: &str = "uploads";
const LISTEN_ADDR: &str = "127.0.0.1:9090";

fn init_upload_dir() -> Result<()> {
    fs::create_dir_all(UPLOAD_DIR)?;
    Ok(())
}

fn build_router() -> Router {
    Router::new().nest("/api", upload::routes())
}

async fn build_listener() -> Result<TcpListener> {
    let listener = TcpListener::bind(LISTEN_ADDR).await?;
    Ok(listener)
}


pub async fn init_app() -> Result<(TcpListener, Router)> {
    init_upload_dir()?;
    let app = build_router();
    let listener = build_listener().await?;
    Ok((listener, app))
}
