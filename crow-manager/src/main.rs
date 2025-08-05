

//use anyhow::Result;
use std::{thread};
use crow_core::tasks_handler::{JobStatus, JobAnalysis};

//use crow_core::db_handler::{JobDatabase, PostgresDbConn};

use crow_core::db_handler::db_interface::JobDatabase;
use crow_core::db_handler::impl_postgres::PostgresDbConn;
use crow_core::db_handler::db_interface::{fetch_next_pending_job, update_job_status};


use crow_core::log_handler::logger;
use log::{info, error};


use tokio::time::{sleep, Duration};

///////////////// 
mod vm_handler;

use crate::vm_handler::impl_docker::DockerVm;
use crate::vm_handler::vm_interface::VirtualMachine;

use std::sync::Arc;



use tokio::{sync::mpsc, time::interval};
use crate::executor::analyze_job;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_log_handler("manager.log")?;
    log::info!("Manager Logger initialized");



    let vm: Arc<dyn VirtualMachine> = Arc::new(DockerVm::new("malware-sandbox".to_string()));

    vm.start().await?;

    // کانال برای پیام تموم شدن تحلیل job
    let (tx, mut rx) = mpsc::channel::<i64>(100); // i64 = job_id

    // لوپ برای بررسی دیتابیس
    let mut check_loop = interval(Duration::from_secs(5));

    loop {
        check_loop.tick().await;

        if let Some(job) = fetch_next_pending_job().await? {
            let job_id = job.id;
            let job_meta = job.meta.clone();

            // tx.clone چون ممکنه چند job همزمان در حال اجرا باشن
            let tx_clone = tx.clone();

            // تسک تحلیل جدا
            tokio::spawn(async move {
                if let Err(e) = analyze_job(job_meta).await {
                    eprintln!("خطا در تحلیل job {}: {:?}", job_id, e);
                }

                // پیام اتمام تحلیل به گیرنده اصلی
                let _ = tx_clone.send(job_id).await;
            });
        }

        // بررسی کانال برای jobهایی که تحلیل‌شون تموم شده
        while let Ok(job_id) = rx.try_recv() {
            update_job_status(job_id).await?;
            println!("[+] Job {} done.", job_id);
        }
    }

    Ok(())
}
