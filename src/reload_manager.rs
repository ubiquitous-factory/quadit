use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::{error, info, warn};

use crate::config_reload::ConfigReload;

/// Manages the reloading of the configuration from a URL endpoint in the `config.yaml`
pub struct ReloadManager {
    /// The scheduler
    scheduler: JobScheduler,
}

impl ReloadManager {
    /// Returns a new ReloadManager with the scheduler initialised.
    pub async fn new() -> Result<ReloadManager, JobSchedulerError> {
        let sched = JobScheduler::new().await?;

        Ok(ReloadManager { scheduler: sched })
    }

    /// Returns a ReloadManager with the ConfigReload loaded as a job.
    /// # Arguments
    ///
    /// * `conf` - A populated `ConfigReload` object
    pub async fn from_config_reload(
        conf: ConfigReload,
    ) -> Result<ReloadManager, JobSchedulerError> {
        let reload_manager = ReloadManager::new().await?;

        reload_manager
            .scheduler
            .add(Job::new_async(conf.schedule.as_str(), |uuid, mut l| {
                Box::pin(async move {
                    info!("{}: Reload job started", uuid);
                    warn!("{}: RELOAD NOT IMPLEMENTED", uuid);
                    // Query the next execution time for this job
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => info!("{}: Next reload execution {:?}", uuid, ts),
                        _ => error!("Could not get next tick for Reload Job"),
                    }
                })
            })?)
            .await?;
        Ok(reload_manager)
    }

    /// Starts the `ReloadManager scheduler`
    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        info!("Starting schedule for reload config");
        self.scheduler.start().await
    }
}
