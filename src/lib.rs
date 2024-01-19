pub mod cli;
pub mod config_git;
pub mod config_quadit;
pub mod config_reload;
pub mod file_manager;
pub mod git_manager;
pub mod quadit_manager;
pub mod reload_manager;

use env_logger::fmt::Formatter;
use log::Record;
use std::io::Write;

/// Logging formatter function
pub fn log_formatter(
    buf: &mut Formatter,
    record: &Record,
) -> std::result::Result<(), std::io::Error> {
    let prefix = match record.level() {
        log::Level::Error => format!("{} ", "!!"),
        log::Level::Warn => format!("{} ", "?!"),
        _ => "".to_string(),
    };

    writeln!(buf, "{}{}", prefix, record.args())
}
