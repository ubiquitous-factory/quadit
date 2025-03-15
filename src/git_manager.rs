// use std::{
//     collections::HashMap,
//     sync::{Mutex, OnceLock},
// };

use std::{
    collections::HashMap,
    fmt,
    fs::metadata,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};
use tracing::{error, info, instrument, warn};

use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::{config_git::ConfigGit, file_manager::FileManager, service_manager::ServiceManager};

/// Manages the check out and syncing of each `GitConfig` using async tasks.
pub struct GitManager {
    /// The scheduler responsible for executing a job per entry in `config.yaml`
    scheduler: JobScheduler,
}

impl fmt::Debug for GitManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Git Manager")
    }
}

impl GitManager {
    /// Returns a new GitManager with the scheduler initialised.
    pub async fn new() -> Result<GitManager, JobSchedulerError> {
        let sched = JobScheduler::new().await?;

        Ok(GitManager { scheduler: sched })
    }

    /// Returns a GitManager with the targetConfigs loaded as jobs.
    /// # Arguments
    ///
    /// * `target_configs` - A vector of `ConfigGit` objects
    #[instrument(level = "trace")]
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

    /// A shared object between the git configurations and the jobs.
    /// Used for accessing data across async calls.
    /// Beware three arrow dragons!!
    #[instrument(level = "trace")]
    fn config_git_list() -> &'static Mutex<HashMap<uuid::Uuid, ConfigGit>> {
        static HASHMAP: OnceLock<Mutex<HashMap<uuid::Uuid, ConfigGit>>> = OnceLock::new();
        let hm: HashMap<uuid::Uuid, ConfigGit> = HashMap::new();
        HASHMAP.get_or_init(|| Mutex::new(hm))
    }

    /// Returns the uuid of the configuration and sets up the call back for the Job to run on each tick.
    /// # Arguments
    ///
    /// * `conf` - A`ConfigGit` object
    #[instrument(level = "trace")]
    pub async fn add_config(&mut self, conf: ConfigGit) -> Result<uuid::Uuid, JobSchedulerError> {
        self.scheduler
            .add(GitManager::create_job(conf).await?)
            .await
    }

    #[instrument(level = "trace")]
    pub async fn create_job(conf: ConfigGit) -> Result<Job, JobSchedulerError> {
        let sched = conf.schedule.clone();
        Job::new_async(sched.as_str(), move |uuid, mut l| {
            let this_conf = conf.clone();
            let mut hm = match GitManager::config_git_list().lock() {
                Ok(g) => g,
                Err(e) => {
                    error!("Failed to lock in scheduler {}: {}", uuid, e);
                    std::process::exit(1);
                }
            };

            if hm.get(&uuid).is_none() {
                let mut job_path = PathBuf::new();
                job_path.push(FileManager::job_path());
                job_path.push(FileManager::job_folder());
                job_path.push(uuid.to_string());
                // let job_path = format!("{}jobs/{}", FileManager::job_path(), uuid);
                info!(
                    "{}: Job creating for {} branch: {}, path: {} in dir {}",
                    uuid,
                    this_conf.url,
                    this_conf.branch,
                    this_conf.target_path,
                    job_path.as_path().display()
                );

                let gitsync = quaditsync::GitSync {
                    repo: this_conf.url.clone(),
                    dir: job_path,
                    ..Default::default()
                };

                match gitsync.bootstrap() {
                    Ok(_) => {
                        hm.insert(uuid, this_conf);
                        info!(
                            "{}: Successfully created job. Cloned {} to {}",
                            uuid,
                            gitsync.repo,
                            gitsync.dir.as_path().display().to_string()
                        );
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
                        let mut internal_job_path = PathBuf::new();
                        internal_job_path.push(FileManager::job_path());
                        internal_job_path.push(FileManager::job_folder());
                        internal_job_path.push(uuid.to_string());
                        //let first_job = FileManager::job_exists(uuid);
                        let tpath = internal_job_path.clone();
                        let quaditsync = quaditsync::GitSync {
                            repo: internal_gc.url.clone(),
                            dir: internal_job_path,
                            ..Default::default()
                        };

                        let commitids = match quaditsync.sync() {
                            Ok(s) => {
                                info!(
                                    "{}: Sync complete. original oid: {}, new oid: {}",
                                    uuid, s.0, s.1
                                );
                                s
                            }
                            Err(e) => {
                                error!(
                                    "Failed to quaditsync for {} url: {} branch: {}, path: {} \n {:?}",
                                    uuid, internal_gc.url, internal_gc.branch, internal_gc.target_path, e
                                );
                                return;
                            }
                        };

                        info!(
                            "{}: Updated {}, branch: {}, path: {} with {}",
                            uuid,
                            internal_gc.url,
                            internal_gc.branch,
                            internal_gc.target_path,
                            commitids.1
                        );
                        if GitManager::process_repo(
                            tpath.to_str().unwrap_or_default(),
                            &internal_gc.target_path,
                            uuid,
                        ) {
                            info!(
                                "{}: Completed processing of {} for {}",
                                uuid,
                                &internal_gc.target_path,
                                tpath.to_str().unwrap_or_default()
                            );
                        } else {
                            error!(
                                "{}: Failed processing of {}",
                                uuid, &internal_gc.target_path
                            );
                        }

                        info!("{}: Next git run {:?}", uuid, ts);
                    }
                    _ => error!("Could not get next tick for job"),
                }
            })
        })
    }

    /// Processes the repo based on the target path supplied.
    /// If it's a directory it iterates through the top level of the directory
    /// Multi level structures should be implemented as different targets in the `config.yaml`
    /// # Arguments
    /// `job_path` - The path to the job - Usually `jobs/xxxxxxxx-xxxx-4xxx-Nxxx-xxxxxxxxxxxx`.
    /// `config_target_path` - The targetPath specified in the `config.yaml`
    #[instrument(level = "trace")]
    fn process_repo(job_path: &str, config_target_path: &str, uuid: uuid::Uuid) -> bool {
        let mut full_job_path = PathBuf::new();
        full_job_path.push(job_path);
        full_job_path.push(config_target_path);

        let md = match metadata(&full_job_path) {
            Ok(m) => m,
            Err(e) => {
                error!(
                    "{}: Error getting metadata {} {:?}",
                    uuid, e, &full_job_path
                );
                return false;
            }
        };
        info!(
            "{}: Processing target: {} is file: {}",
            uuid,
            &full_job_path.display(),
            md.is_file()
        );
        if md.is_file() && !FileManager::is_unit_file_deployed(job_path, config_target_path) {
            // Iteratively loop through the job directory and only deploy the files that are different.
            match FileManager::deploy_unit_file(job_path, config_target_path) {
                Ok(s) => info!("{}: Deployed to {}", uuid, s),
                Err(e) => {
                    error!("{}: Error deploying container file: {}", uuid, e);
                    return false;
                }
            }

            match ServiceManager::daemon_reload() {
                Ok(s) => info!("{}: Reloaded daemon with status: {}", uuid, s),
                Err(e) => {
                    error!("{}, Failed to reload daemon: {}", uuid, e);
                    return false;
                }
            };
            let unit = match FileManager::filename_to_unit_name(config_target_path) {
                Ok(s) => s,
                Err(e) => {
                    error!("{}, Failed to get unit name: {}", uuid, e);
                    return false;
                }
            };

            match ServiceManager::restart(&unit) {
                Ok(s) => info!("{}, Restarted {} with exit code:{}", uuid, unit, s),
                Err(e) => error!("{}: Failed to restart: {} {}", uuid, unit, e),
            };
        } else if md.is_dir() {
            info!(
                "{}: Processing Directory {}",
                uuid,
                &full_job_path.as_path().display()
            );

            match FileManager::get_files_in_directory(full_job_path.to_str().unwrap_or_default()) {
                Ok(file_names) => {
                    for file_name in file_names {
                        let file_path =
                            format!("{}/{}", full_job_path.as_path().display(), file_name);
                        GitManager::process_repo("", &file_path, uuid);
                    }
                }
                Err(e) => error!("{}: Error: {}", uuid, e),
            }
        }
        true
    }
    /// Starts the `GitManager scheduler`
    #[instrument(level = "trace")]
    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        info!("Starting schedule for all git configs");
        self.scheduler.start().await
    }

    pub async fn stop(&mut self) -> Result<(), JobSchedulerError> {
        let hm;
        {
            let internal_hm = match GitManager::config_git_list().lock() {
                Ok(g) => g,
                Err(e) => {
                    error!("Failed to lock: {}", e);
                    std::process::exit(1);
                }
            };
            hm = internal_hm.clone();
        }
        for key in hm.keys() {
            self.scheduler.remove(key).await?;
            let mut internal_hm = match GitManager::config_git_list().lock() {
                Ok(g) => g,
                Err(e) => {
                    error!("Failed to lock: {}", e);
                    std::process::exit(1);
                }
            };
            internal_hm.remove(key);
        }

        self.scheduler.shutdown().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::GitManager;
    use crate::config_git::ConfigGit;
    use claims::*;

    #[tokio::test]
    pub async fn test_creation() {
        let cg = ConfigGit {
            url: "https://github.com/ubiquitous-factory/quadit".to_string(),
            target_path: "samples/helloworld".to_string(),
            branch: "main".to_string(),
            schedule: "1/1 * * * * *".to_string(),
        };

        let configs = vec![cg];
        let gm = GitManager::from_target_configs(configs).await.unwrap();
        let st = gm.start().await;
        assert_ok!(st);
    }
}
