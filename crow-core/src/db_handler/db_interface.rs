
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::tasks_handler::JobAnalysis;
use crate::tasks_handler::JobStatus;

/// The JobDatabase trait defines an asynchronous abstraction for interacting with the database in the malware
/// analysis system. It provides essential operations such as initializing the database, inserting new jobs, 
/// updating job statuses, and fetching jobs marked as "Pending." Designed with Send and Sync bounds, it 
/// supports safe usage in multi-threaded contexts and allows flexible backend implementations like PostgreSQL 
/// without requiring changes to the core application logic.
#[async_trait]
pub trait JobDatabase: Send + Sync {
    /// Create the `jobs` table if it does not already exist.
    pub async fn initialize_database(&self) -> Result<()>;  

    /// Retrieve all jobs whose status is "Pending".
    async fn fetch_next_pending_job (&self) -> Result<Vec<JobAnalysis>>;

    /// Insert a new job record and return its `id` as a `String`.
    async fn insert_new_job (&self, job: &JobAnalysis) -> Result<String>;

    /// Update the `status` field of the job identified by `job_id`.
    async fn update_job_status (&self, job_id: String, new_status: JobStatus) -> Result<()>;


//    async fn find_job_by_id         (&self, job_id: i64)                            -> Result<Option<JobAnalysis>>;
//    async fn find_job_by_hash       (&self, sha256_hash: &str)                      -> Result<Option<JobAnalysis>>;

}


