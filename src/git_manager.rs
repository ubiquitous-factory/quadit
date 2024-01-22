use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock}
};

use log::{error, info, warn};

use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::{config_git::ConfigGit, file_manager::FileManager, service_manager::ServiceManager};

pub struct GitManager {
    scheduler: JobScheduler,
}

impl GitManager {
    pub async fn new() -> Result<GitManager, JobSchedulerError> {
        let sched = JobScheduler::new().await?;

        Ok(GitManager { scheduler: sched })
    }
    pub async fn from_target_configs(
        target_configs: Vec<ConfigGit>,
    ) -> Result<GitManager, JobSchedulerError> {
        let mut git_manager = GitManager::new().await?;

        for conf in target_configs {
            info!(
                "Starting config for {} with target_path: {} schedule: {}",
                conf.url, conf.target_path, conf.schedule
            );
            git_manager.add_config(conf).await?;
        }

        Ok(git_manager)
    }

    fn config_git_list() -> &'static Mutex<HashMap<uuid::Uuid, ConfigGit>> {
        static HASHMAP: OnceLock<Mutex<HashMap<uuid::Uuid, ConfigGit>>> = OnceLock::new();
        let hm: HashMap<uuid::Uuid, ConfigGit> = HashMap::new();
        HASHMAP.get_or_init(|| Mutex::new(hm))
    }
    pub async fn add_config(&mut self, conf: ConfigGit) -> Result<uuid::Uuid, JobSchedulerError> {
        let sched = conf.schedule.clone();

        self.scheduler
            .add(Job::new_async(sched.as_str(), move |uuid, mut l| {
                let this_conf = conf.clone();
                let mut hm = match GitManager::config_git_list().lock() {
                    Ok(g) => g,
                    Err(e) => {
                        error!("Failed to lock in scheduler {}: {}", uuid, e);
                        std::process::exit(1);
                    }
                };

                if hm.get(&uuid).is_none() {
                    info!(
                        "{} : Job created for {} branch: {}, path: {}",
                        uuid, this_conf.url, this_conf.branch, this_conf.target_path
                    );
                    let job_path = format!("jobs/{}", uuid); 
                    let gitsync = quaditsync::GitSync {
                        repo: this_conf.url.clone(),
                        dir: job_path.into(),
                        ..Default::default()
                    };

                    match gitsync.bootstrap() {
                        Ok(_) => {
                            hm.insert(uuid, this_conf);
                            None
                        }
                        Err(e) => {
                            error!(
                                "Failed to bootstrap for {} url: {} branch: {}, path: {} \n {:?}",
                                uuid, this_conf.url, this_conf.branch, this_conf.target_path, e
                            );
                            Some(e)
                        }
                    };
                }

                Box::pin(async move {
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => {
                            info!("{}: Getting GitConfig", uuid);
                            let internal_hm = match GitManager::config_git_list().lock() {
                                Ok(g) => g,
                                Err(e) => {
                                    error!("Failed to lock in pin {}: {}", uuid, e);
                                    std::process::exit(1);
                                }
                            };
                            // can force the get here as we have inserted above
                            let internal_gc = internal_hm.get(&uuid).unwrap();

                            info!(
                                "{}: Running sync for {} branch: {}, path: {}",
                                uuid, internal_gc.url, internal_gc.branch, internal_gc.target_path
                            );
                            let job_path = format!("jobs/{}", uuid); 
                            let first_run = FileManager::job_exists(uuid);

                            let quaditsync = quaditsync::GitSync {
                                repo: internal_gc.url.clone(),
                                dir: job_path.clone().into(),
                                ..Default::default()
                            };
                            // TODO: This currently relies on the quaditsync logging which is a bit meh.
                            let commitids = match quaditsync.sync() {
                                Ok(s) => s,
                                Err(e) => {
                                    error!(
                                        "Failed to quaditsync for {} url: {} branch: {}, path: {} \n {:?}",
                                        uuid, internal_gc.url, internal_gc.branch, internal_gc.target_path, e
                                    );
                                    return;
                                }
                            };
                            
                            // different commit ids so we are going to refresh the container file.
                            if !commitids.0.eq(&commitids.1) || first_run {
                                info!("{}, Updated {}, branch: {}, path: {} with {}",uuid, internal_gc.url, internal_gc.branch, internal_gc.target_path,commitids.1);
                                match FileManager::deploy_container_file(job_path, internal_gc.target_path.clone()) {
                                    Ok(s) => info!("{}: Deployed to {}", uuid, s),
                                    Err(e) => error!("Error deploying container file: {}", e)
                                }
                                
                                match ServiceManager::daemon_reload() {
                                    Ok(s) => info!("{}: Reloaded daemon with status: {}", uuid, s),
                                    Err(e) => error!("{}, Failed to reload daemon: {}", uuid, e)
                                };
                                let unit = match FileManager::container_file_to_unit_name(internal_gc.target_path.clone()) {
                                    Ok(s) => s,
                                    Err(e) => { error!("{}, Failed to get unit name: {}", uuid, e);
                                        return ;
                                    }
                                };

                                match ServiceManager::restart(&unit) {
                                    Ok(s) => info!("{}, Restarted {} with exit code:{}", uuid, unit, s),
                                    Err(e) => error!("{}: Failed to restart: {} {}", uuid,unit, e)
                                };  
                                
                                 
                                
                                // systemctl --user daemon-reload
                                // systemctl --user restart mysleep.service
                            } 

                            info!("Next scheduled run {:?}", ts);
                        }
                        _ => warn!("Could not get next tick for job"),
                    }
                })
            })?)
            .await
    }

    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        info!("Starting schedule for all git configs");
        self.scheduler.start().await
    }
}
