use anyhow::Error;
use std::process::{Child, ExitStatus};
use std::time::Duration;

use crate::{file_manager::FileManager, quadit_manager::QuaditManager};

const SYSTEMCTL_PATH: &str = "/usr/bin/systemctl";
const USER: bool = true;
pub struct ServiceManager {}

impl ServiceManager {
    // env::set_var("SYSTEMCTL_PATH", "VALUE");

    pub async fn run() -> Result<(), Error> {
        let serviceconf = FileManager::load_quadit_config()?;
        let quadit = QuaditManager::from_yaml(serviceconf).await?;
        quadit.start().await?;
        loop {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    pub fn restart(unit: &str) -> std::io::Result<ExitStatus> {
        ServiceManager::systemctl(vec!["restart", unit])
    }

    pub fn daemon_reload() -> std::io::Result<ExitStatus> {
        ServiceManager::systemctl(vec!["daemon-reload"])
    }

    fn spawn_child(args: Vec<&str>) -> std::io::Result<Child> {
        std::process::Command::new(std::env::var("SYSTEMCTL_PATH").unwrap_or(SYSTEMCTL_PATH.into()))
            .args(args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
    }

    /// Invokes `systemctl $args` silently
    fn systemctl(mut args: Vec<&str>) -> std::io::Result<ExitStatus> {
        // USER is always true for now but might support root services later
        if USER {
            let mut first_arg = vec!["--user"];
            first_arg.append(&mut args);
            ServiceManager::spawn_child(first_arg)?.wait()
        } else {
            ServiceManager::spawn_child(args)?.wait()
        }
    }
}
