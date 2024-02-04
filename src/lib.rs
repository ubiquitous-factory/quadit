pub mod cli;
pub mod config_commands;
pub mod config_git;
pub mod config_quadit;
pub mod config_reload;
pub mod file_manager;
pub mod git_manager;
pub mod quadit_manager;
pub mod reload_manager;
pub mod service_manager;

// use chrono::prelude::*;
// use colored::Colorize;
// // use env_logger::fmt::Formatter;
// // use log::Record;
// use std::io::Write;
// Logging formatter function
// pub fn log_formatter(
//     buf: &mut Formatter,
//     record: &Record,
// ) -> std::result::Result<(), std::io::Error> {
//     let utc: DateTime<Utc> = Utc::now();
//     let prefix = match record.level() {
//         log::Level::Error => format!("{} {} ", "ERROR".red(), utc),
//         log::Level::Warn => format!("{} {} ", "WARN".yellow(), utc),
//         _ => format!("{} {} ", "INFO".green(), utc),
//     };

//     writeln!(buf, "{}{}", prefix, record.args())
// }
