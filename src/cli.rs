use clap::{Parser, Subcommand};

use crate::service_manager::ServiceManager;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
pub struct QuaditCli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// does testing things
    Run {},
}

impl QuaditCli {
    pub async fn run() -> Result<(), anyhow::Error> {
        let cli = QuaditCli::parse();
        match cli.debug {
            0 => println!("Debug mode is off"),
            1 => println!("Debug mode is kind of on"),
            2 => println!("Debug mode is on"),
            _ => println!("Don't be crazy"),
        }

        match &cli.command {
            Commands::Run {} => {
                ServiceManager::run().await?;
            }
        }
        Ok(())
    }
}
