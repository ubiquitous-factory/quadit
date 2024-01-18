mod config_reload;
mod git_config;
mod quadit_config;

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
