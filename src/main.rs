use std::time::Duration;

use anyhow::Result;

use clap::Parser;
use quadit::{cli::QuaditCli, file_manager::FileManager, quadit_manager::QuaditManager};

#[tokio::main]
async fn main() -> Result<()> {
    if dotenvy::dotenv().is_ok() {
        println!("Using .env")
    }

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

    let cli = QuaditCli::parse();
    //println!("{:?}", args.manifest_path);
    // if args.version {}
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
    let serviceconf = FileManager::load_quadit_config()?;
    let quadit = QuaditManager::from_yaml(serviceconf).await?;
    quadit.start().await?;
    tokio::time::sleep(Duration::from_secs(100)).await;
    Ok(())
}
