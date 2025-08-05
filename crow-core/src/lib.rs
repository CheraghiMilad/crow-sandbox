pub mod log_handler;
pub mod tasks_handler_mod;

//pub use tasks_handler::{JobAnalysis, JobStatus};
use std::sync::Arc;

pub mod db_handler;
pub mod tasks_handler;


use anyhow::Result;

use db_handler::impl_postgres::PostgresDbConn;

use db_handler::db_interface::JobDatabase;


pub async fn initialize_database_main_thread() -> Result<Arc<dyn JobDatabase>> {
    let conn = PostgresDbConn::connect_to_database().await?;
    conn.initialize_database().await?;
    Ok(conn)
}