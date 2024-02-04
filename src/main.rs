use std::{env, str::FromStr};

use anyhow::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[cfg(feature = "cli")]
use quadit::cli::QuaditCli;

#[cfg(not(feature = "cli"))]
use quadit::service_manager::ServiceManager;

#[tokio::main]
async fn main() -> Result<()> {
    if dotenvy::dotenv().is_ok() {
        println!("Using .env")
    }

    let log_level = Level::from_str(
        env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .as_str(),
    )
    .unwrap_or(Level::INFO);
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    // more comments
    #[cfg(feature = "cli")]
    QuaditCli::run().await?;

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
