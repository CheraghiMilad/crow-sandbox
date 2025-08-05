// logger
use crow_core::log_handler::logger;
use log::{error, info};

// database
use anyhow::Result;

use crow_core::db_handler::db_interface::{self, JobDatabase};

use crow_core::tasks_handler_mod::checks::FileInfo;
use crow_core::tasks_handler_mod::traits::FileChecks;

// api web
mod api;

use crate::api::router::init_app;
use axum::Server;

use std::net::TcpListener as StdTcpListener;
use tokio::net::TcpListener as TokioTcpListener;

//use axum::Server;
use std::sync::Arc;


use crow_core::initialize_database_main_thread;


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    logger::init_log_handler("daemon.log")?;
    log::info!("[+] Daemon Logger initialized");

    let db: Arc<dyn JobDatabase> = initialize_database_main_thread().await?;
    
    log::info!("Database Connected");

    let file = FileInfo {
        path: "/usr/bin/ls".into(),
    };


    let (tokio_listener, app) = init_app().await?;
    info!("API initialized");

    let std_listener: StdTcpListener = tokio_listener.into_std()?;

    let addr = std_listener.local_addr()?;
    println!("🚀 Server running at http://{}", addr);

    Server::from_tcp(std_listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
