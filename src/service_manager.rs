use anyhow::Error;

use std::process::{Child, ExitStatus};
use std::time::Duration;
use std::{fmt, fs};
use tracing::instrument;

use crate::config_commands::ConfigCommands;
use crate::config_git::ConfigGit;
use crate::config_quadit::ConfigQuadit;
use crate::config_reload::ConfigReload;
use crate::file_manager::FileManager;
use crate::git_manager::GitManager;

const SYSTEMCTL_PATH: &str = "/usr/bin/systemctl";
const USER: bool = true;
pub struct ServiceManager {
    pub git_manager: GitManager,
    pub target_configs: Vec<ConfigGit>,
    pub config_reload: Option<ConfigReload>,
    pub systemd_commands: Option<Vec<ConfigCommands>>,
}

impl fmt::Debug for ServiceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Service Manager")
    }
}

impl ServiceManager {
    /// Returns an configured quadit manager.
    /// # Arguments
    ///
    /// * `conf` - A String slice that contains the complete `config.yaml`
    #[instrument]
    pub async fn configured() -> Result<ServiceManager, anyhow::Error> {
        if !FileManager::boot_url().is_empty() {
            FileManager::from_url(FileManager::boot_url()).await?;
        }
        let conf = FileManager::load_quadit_config()?;
        let quad = ConfigQuadit::from_yaml(conf)?;
        let tconfigs = quad.target_configs.clone();
        Ok(ServiceManager {
            git_manager: GitManager::from_target_configs(quad.target_configs).await?,
            target_configs: tconfigs,
            config_reload: quad.config_reload,
            systemd_commands: quad.systemd_commands,
        })
    }
    /// Creates an instance of the QuaditManager and starts it.
    #[instrument]
    pub async fn run(&mut self) -> Result<(), Error> {
        self.git_manager.start().await?;

        loop {
            let mut sleep: u64 = 100;
            if self.config_reload.is_some() {
                sleep = self.config_reload.as_ref().unwrap().interval;
            }
            std::thread::sleep(Duration::from_millis(sleep));

            if self.config_reload.is_some() {
                let changed = FileManager::from_url(
                    self.config_reload.as_ref().unwrap().config_u_r_l.as_str(),
                )
                .await?;
                if changed {
                    self.git_manager.stop().await?;
                    let configs = self.target_configs.clone();
                    self.git_manager = GitManager::from_target_configs(configs).await?;
                    self.git_manager.start().await?;
                }
            }
        }
    }

    #[instrument]
    pub async fn reload() -> Result<(), Error> {
        if FileManager::boot_url().is_empty() {
            FileManager::from_url(FileManager::boot_url()).await?;
        }
        // let serviceconf = FileManager::load_quadit_config()?;
        // let quadit = QuaditManager::from_yaml(serviceconf).await?;
        // quadit.start().await?;
        loop {
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// Restarts the systemd unit
    /// # Arguments
    ///
    /// * `unit` - A string slice with the unit name
    pub fn restart(unit: &str) -> std::io::Result<ExitStatus> {
        ServiceManager::systemctl(vec!["restart", unit])
    }

    pub fn remove(unit: &str, deployment_path: &str) -> std::io::Result<ExitStatus> {
        ServiceManager::systemctl(vec!["stop", unit])?;
        fs::remove_file(deployment_path)?;
        ServiceManager::daemon_reload()
    }
    /// Reloads the systemd daemon
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

// notes
// systemctl stop [servicename]
// systemctl disable [servicename]
// rm -fr /run/user/1000/systemd/generator/sleep.service
// rm -fr /run/user/1000/systemd/generator/default.target.wants/sleep.service
// rm -fr /run/user/1000/systemd/generator/multi-user.target.wants/sleep.service
// rm /etc/systemd/system/[servicename]
// rm /etc/systemd/system/[servicename] # and symlinks that might be related
// rm /usr/lib/systemd/system/[servicename]
// rm /usr/lib/systemd/system/[servicename] # and symlinks that might be related
// systemctl daemon-reload
// systemctl reset-failed
