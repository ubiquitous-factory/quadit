use anyhow::Result;
use log::{error, info, warn};

fn main() -> Result<()> {
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

    warn!("oh oh");
    info!("just so you know");
    error!("broken");
    Ok(())
}
