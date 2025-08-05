use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JobStatus {
    Pending,
    Running,
    Done,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobAnalysis {
    pub id: String,
    pub file_name: String,
    pub file_hash: String,
    pub submitted_at: DateTime<Utc>,
    pub status: JobStatus,
}

impl JobAnalysis {
    pub fn new(file_name: String, file_hash: String) -> Self {
        JobAnalysis {
            id: Uuid::new_v4().to_string(),
            file_name,
            file_hash,
            submitted_at: Utc::now(),
            status: JobStatus::Pending,
        }
    }
}
