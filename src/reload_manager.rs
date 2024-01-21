use log::{error, info};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::config_reload::ConfigReload;
#[allow(dead_code)]
pub struct ReloadManager {
    scheduler: JobScheduler,
}

impl ReloadManager {
    pub async fn new() -> Result<ReloadManager, JobSchedulerError> {
        let sched = JobScheduler::new().await?;

        Ok(ReloadManager { scheduler: sched })
    }
    pub async fn from_config_reload(
        conf: ConfigReload,
    ) -> Result<ReloadManager, JobSchedulerError> {
        let reload_manager = ReloadManager::new().await?;

        reload_manager
            .scheduler
            .add(Job::new_async(conf.schedule.as_str(), |uuid, mut l| {
                Box::pin(async move {
                    info!("Added Reload Job");

                    // Query the next execution time for this job
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => info!("Next time for Reload Job is {:?}", ts),
                        _ => error!("Could not get next tick for Reload Job"),
                    }
                })
            })?)
            .await?;
        Ok(reload_manager)
    }

    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        info!("Starting schedule for reload config");
        self.scheduler.start().await
    }
}
