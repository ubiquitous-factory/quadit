use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::{error, info, warn};

use crate::{config_reload::ConfigReload, file_manager::FileManager};

/// Manages the reloading of the configuration from a URL endpoint in the `config.yaml`
pub struct ReloadManager {
    /// The scheduler
    scheduler: JobScheduler,
    uuid: Option<uuid::Uuid>,
}

impl ReloadManager {
    /// Returns a new ReloadManager with the scheduler initialised.
    pub async fn new() -> Result<ReloadManager, JobSchedulerError> {
        let sched = JobScheduler::new().await?;

        Ok(ReloadManager {
            scheduler: sched,
            uuid: None,
        })
    }

    /// Returns a ReloadManager with the ConfigReload loaded as a job.
    /// # Arguments
    ///
    /// * `conf` - A populated `ConfigReload` object
    pub async fn from_config_reload(
        conf: ConfigReload,
    ) -> Result<ReloadManager, JobSchedulerError> {
        let mut reload_manager = ReloadManager::new().await?;

        reload_manager
            .scheduler
            .add(Job::new_async(
                conf.schedule.as_str(),
                move |uuid, mut l| {
                    reload_manager.uuid = Some(uuid);
                    Box::pin(async move {
                        info!("{}: Reload job started", uuid);
                        // Query the next execution time for this job
                        let next_tick = l.next_tick_for_job(uuid).await;
                        match next_tick {
                            Ok(Some(ts)) => {
                                let is_updated =
                                    match FileManager::from_url(FileManager::boot_url()).await {
                                        Ok(b) => b,
                                        Err(e) => {
                                            error!(
                                                "{}: failed to get url {} {}",
                                                uuid,
                                                FileManager::boot_url(),
                                                e
                                            );
                                            false
                                        }
                                    };

                                if is_updated {
                                    // WHO SAYS REWIND ALREADY?
                                }
                                info!("{}: Next reload execution {:?}", uuid, ts)
                            }
                            _ => error!("Could not get next tick for Reload Job"),
                        }
                    })
                },
            )?)
            .await?;
        Ok(reload_manager)
    }

    /// Starts the `ReloadManager scheduler`
    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        info!("Starting schedule for reload config");
        self.scheduler.start().await
    }

    /// Removes the job and stops the `ReloadManager scheduler`
    pub async fn stop(&mut self) -> Result<(), JobSchedulerError> {
        info!("Stopping schedule for reload config");
        // If job doesn't exist
        let uuid = match &self.uuid {
            Some(u) => u,
            None => {
                self.scheduler.shutdown().await?;
                return Ok(());
            }
        };
        self.scheduler.remove(uuid).await?;
        Ok(())
    }
}
