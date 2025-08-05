use anyhow::Result;
use async_trait::async_trait;

// A generic interface for virtualization systems like Docker or VMware.
#[async_trait]
pub trait VirtualMachine: Send + Sync {
    // Start the virtual machine with necessary context
    async fn start(&self) -> Result<()>;

    // Stop the virtual machine gracefully
    async fn stop(&self) -> Result<()>;

    // Restart the virtual machine
    async fn restart(&self) -> Result<()>;

}

// Push a file into the virtual machine
// async fn push_file(&self, local_path: &str, dest_path: &str) -> Result<()>;
// Retrieve analysis results from the virtual machine
// async fn get_results(&self, result_dir: &str) -> Result<()>;
// async fn take_snapshot(&self) -> Result<()>;
