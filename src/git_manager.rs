use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::config_git::ConfigGit;

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
            git_manager.add_config(conf).await?;
        }

        Ok(git_manager)
    }

    pub async fn add_config(&mut self, conf: ConfigGit) -> Result<uuid::Uuid, JobSchedulerError> {
        self.scheduler
            .add(Job::new_async(conf.schedule.as_str(), |uuid, mut l| {
                Box::pin(async move {
                    println!("I run async every 7 seconds");

                    // Query the next execution time for this job
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
                        _ => println!("Could not get next tick for 7s job"),
                    }
                })
            })?)
            .await
    }
}
