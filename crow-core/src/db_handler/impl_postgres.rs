use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::{Client, NoTls};
use std::sync::Arc;

use crate::db_handler::db_interface::JobDatabase;
use crate::tasks_handler::{JobAnalysis, JobStatus};


pub struct PostgresDbConn {
    pub client: Client,
}

// The `connect` method is not part of the `JobDatabase` trait because it is responsible
// for constructing a new `PostgresDbConn` instance (establishing the connection,
// spawning the background task, etc.). Traits define behavior on existing instances,
// whereas constructors describe how to create those instances. Placing `connect` on
// the `impl PostgresDbConn` block keeps the trait focused on instance methods and
// avoids forcing every implementor or trait object to implement a static creation
// routine that doesn’t conceptually belong to the trait’s interface.
impl PostgresDbConn {
    pub async fn connect(conn_str: &str) -> Result<Self> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Ok(Self { client })
    }

    pub async fn connect_to_database()-> Result<Arc<Self>> {
    
        let conn_str = "host=localhost user=postgres password=postgres dbname=crow";

        let conn = PostgresDbConn::connect(conn_str).await?;

        Ok(Arc::new(conn))

    }
}




// This implementation cleanly adapts the `JobDatabase` trait for PostgreSQL by switching the `id` column to `TEXT` 
// and handling UUIDs as strings, ensuring compatibility with `tokio-postgres` 0.7. The `initialize_database` 
// method safely creates the table if it doesn’t exist, while `connect_to_database` constructs the connection outside the trait
// (keeping instance setup separate). In `fetch_next_pending_job`, each row’s `id` is retrieved as a `String` 
//and used directly in the `JobAnalysis` struct, and `insert_new_job` immediately returns the stringified `id`.
// Finally, `update_job_status` accepts the `id` as a `String`, matching the schema. Overall, this approach leverages 
// simple text‐based UUID storage to avoid compatibility issues, but be mindful that parsing and formatting cost a 
//small overhead compared to native UUID support.  
#[async_trait]
impl JobDatabase for PostgresDbConn {


    async fn initialize_database (&self) -> Result<()> {

        self.client
            .execute(
                "CREATE TABLE IF NOT EXISTS jobs (
                id TEXT PRIMARY KEY,
                file_name TEXT NOT NULL,
                file_hash TEXT NOT NULL,
                submitted_at TIMESTAMPTZ NOT NULL,
                status TEXT NOT NULL
            )",
                &[],
            )
            .await?;
        Ok(())

    }




    async fn fetch_next_pending_job (&self)  -> Result<Vec<JobAnalysis>> {

        let rows = self
            .client
            .query(
                "SELECT id, file_name, file_hash, submitted_at, status FROM jobs WHERE status = $1",
                &[&"Pending"],
            )
            .await?;

        let mut jobs = Vec::new();

        for row in rows {
            let id: String = row.get("id");
            let file_name: String = row.get("file_name");
            let file_hash: String = row.get("file_hash");
            let submitted_at: DateTime<Utc> = row.get("submitted_at");
            let status_str: String = row.get("status");

            let status = match status_str.as_str() {
                "Pending" => JobStatus::Pending,
                "Running" => JobStatus::Running,
                "Done" => JobStatus::Done,
                "Error" => JobStatus::Error,
                _ => JobStatus::Error,
            };

            jobs.push(JobAnalysis {
                id,
                file_name,
                file_hash,
                submitted_at,
                status,
            });
        }

        Ok(jobs)

    }


   // async fn find_job_by_id         (&self, job_id: i64)                            -> Result<Option<JobAnalysis>>;


    async fn insert_new_job (&self, job: &JobAnalysis)  -> Result<String> {

        self.client.execute(
            "INSERT INTO jobs (id, file_name, file_hash, submitted_at, status) VALUES ($1, $2, $3, $4, $5)",
            &[&job.id, &job.file_name, &job.file_hash, &job.submitted_at, &format!("{:?}", job.status)],
        ).await?;

        Ok(job.id.to_string())

    }


 //   async fn find_job_by_hash       (&self, sha256_hash: &str)                      -> Result<Option<JobAnalysis>>;


    async fn update_job_status (&self, job_id: String, new_status: JobStatus) -> Result<()> {

        self.client
            .execute(
                "UPDATE jobs SET status = $1 WHERE id = $2",
                &[&format!("{:?}", new_status), &job_id],
            )
            .await?;
        Ok(())

    }


}