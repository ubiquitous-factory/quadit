use std::{env, str::FromStr};

use anyhow::Result;
use tracing::{warn, Level};
use tracing_subscriber::FmtSubscriber;

use quadit::service_manager::ServiceManager;

#[tokio::main]
async fn main() -> Result<()> {
    if users::get_current_uid() == 0 {
        warn!(
            r#"
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
quadit running with root permissions
This isn't currently supported and
May lead to security problems and 
undefined behaviour
See https://github.com/ubiquitous-factory/quadit/issues/76
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
"#
        );
    }

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

    svc().await?;

    Ok(())
}

async fn svc() -> Result<(), anyhow::Error> {
    if env::args().collect::<Vec<_>>().len() != 1 {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("quadit Server Edition v{}", VERSION);
        println!("This process accepts no arguments.");
        println!("See documentation https://github.com/ubiquitous-factory/quadit/");
    } else {
        let mut svc = ServiceManager::configured().await?;
        svc.run().await?;
    }
    Ok(())
}
