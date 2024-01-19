use std::time::Duration;

use anyhow::Result;
use log::info;
use quadit::{file_manager::FileManager, quadit_manager::QuaditManager};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    env_logger::builder()
        .format(quadit::log_formatter)
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .format_timestamp(None)
        .format_target(false)
        .format_module_path(false)
        .format_level(false)
        .target(env_logger::Target::Stdout)
        .init();

    let service_conf_location = "/opt/mount/config.yaml".to_string();
    info!("loading configuration from {}", service_conf_location);
    let serviceconf = FileManager::readconfig(service_conf_location)?;
    QuaditManager::from_yaml(serviceconf).await?;
    tokio::time::sleep(Duration::from_secs(100)).await;
    Ok(())
}
