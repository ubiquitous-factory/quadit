use anyhow::Result;

#[cfg(feature = "cli")]
use quadit::cli::QuaditCli;

#[cfg(not(feature = "cli"))]
use quadit::service_manager::ServiceManager;

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

    #[cfg(feature = "cli")]
    let _ = QuaditCli::run().await;

    #[cfg(not(feature = "cli"))]
    svc().await?;

    Ok(())
}

#[cfg(not(feature = "cli"))]
async fn svc() -> Result<(), anyhow::Error> {
    use std::env;

    if env::args()
        .filter(|a| a == &"--version".to_string() || a == &"--help".to_string())
        .collect::<Vec<_>>()
        .len()
        > 0
    {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("quadit Server Edition v{}", VERSION);
        println!("This process accepts no arguments.");
        println!("See documentation https://github.com/ubiquitous-factory/quadit/");
    } else {
        ServiceManager::run().await?;
    }
    Ok(())
}
