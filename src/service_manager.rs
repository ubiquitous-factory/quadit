use std::time::Duration;

use anyhow::Error;

use crate::{file_manager::FileManager, quadit_manager::QuaditManager};

pub struct ServiceManager {}

impl ServiceManager {
    pub async fn run() -> Result<(), Error> {
        let serviceconf = FileManager::load_quadit_config()?;
        let quadit = QuaditManager::from_yaml(serviceconf).await?;
        quadit.start().await?;
        tokio::time::sleep(Duration::from_secs(100)).await;
        Ok(())
    }
}
