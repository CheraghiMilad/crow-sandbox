use anyhow::Result;
use std::path::PathBuf;
use clap::Parser;
use tokio::fs;
use sha2::{Digest, Sha256};

use crow_core::db_handler::impl_postgres::PostgresDbConn;
use crow_core::db_handler::db_interface::JobDatabase;
use crow_core::tasks_handler::{JobAnalysis, JobStatus};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, DateTime};

use crow_core::initialize_database_main_thread;

// Logger setup
use crow_core::log_handler::logger;
use log::{error, info};

/// Command-line argument structure using `clap`
#[derive(Parser)]
#[command(name = "crow-cli")]
#[command(about = "CLI for interacting with the malware analysis sandbox")]
struct Cli {
    /// Path to the file to be uploaded to the database
    #[arg(short, long)]
    file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    log::info!("[+] Crow-cli Running");

    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize database connection
    let db: Arc<dyn JobDatabase>;
    match initialize_database_main_thread().await {
        Ok(connect) => {
            log::info!("[+] Database connected successfully from crow-cli.");
            db = connect;
        }
        Err(e) => {
            log::error!("[-] Failed to connect to database from crow-cli: {}", e);
            std::process::exit(1);
        }
    }

    // Read file content asynchronously
    let content = fs::read(&cli.file).await?;

    // Calculate the SHA256 hash of the file
    let file_hash = {
        let mut hasher = Sha256::new();
        hasher.update(&content);
        format!("{:x}", hasher.finalize())
    };

    // Create a job record to insert into the database
    let job = JobAnalysis {
        id: Uuid::new_v4().to_string(),
        file_name: cli.file.to_string_lossy().to_string(),
        file_hash,
        submitted_at: Utc::now(),
        status: JobStatus::Pending,
    };

    // Insert the job record into the database
    db.insert_new_job(&job).await?;

    // Display the submitted job ID with a green checkmark
    println!("[+] Job submitted with ID: {}", job.id);

    Ok(())
}
