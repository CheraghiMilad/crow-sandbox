use anyhow::Result;
use async_trait::async_trait;
use bollard::Docker;
use bollard::container::{StartContainerOptions, StopContainerOptions, RestartContainerOptions};
use bollard::exec::{CreateExecOptions, StartExecResults};

use crate::vm_handler::vm_interface::VirtualMachine;

pub struct DockerVm {
    pub container_id: String,
    docker: Docker,
}

impl DockerVm {
    pub fn new(container_id: String) -> Self {
        let docker = Docker::connect_with_local_defaults().expect("[-] Failed to connect to Docker");
        Self { container_id, docker }
    }
}

#[async_trait]
impl VirtualMachine for DockerVm {
    async fn start(&self) -> Result<()> {
        self.docker
            .start_container(&self.container_id, None::<StartContainerOptions<String>>)
            .await?;
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        self.docker
            .stop_container(&self.container_id, Some(StopContainerOptions { t: 5 }))
            .await?;
        Ok(())
    }

    async fn restart(&self) -> Result<()> {
        self.docker
            .restart_container(&self.container_id, Some(RestartContainerOptions { t: 5 }))
            .await?;
        Ok(())
    }

}
