use std::time::Duration;

use anyhow::Error;

use crate::{file_manager::FileManager, quadit_manager::QuaditManager};

pub struct ServiceManager {}

impl ServiceManager {
    pub async fn run() -> Result<(), Error> {
        let serviceconf = FileManager::load_quadit_config()?;
        let quadit = QuaditManager::from_yaml(serviceconf).await?;
        quadit.start().await?;
        loop {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}
