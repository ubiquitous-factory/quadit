// The integration tests validate that the services can download a git repo
// and then copy that file into a known location.
// The status of the container is the responsibility of podman
// and so is outside of the scope of quadit.
#[cfg(test)]
mod test {

    // use claims::*;
    use quadit::{config_git::ConfigGit, git_manager::GitManager};

    use std::fs;
    use std::sync::mpsc::channel;
    use std::{env, path::PathBuf};

    use tokio_cron_scheduler::JobScheduler;
    use tracing::{info, Level};
    use tracing_subscriber::FmtSubscriber;
    // Needs multi_thread to test, otherwise it hangs on scheduler.add()
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]

    // #[tokio::test]
    async fn test_schedule() {
        let uloc = "/tmp/quadit_test/.config/containers/systemd";
        env::set_var("JOB_PATH", "/tmp/quadit_test");
        env::set_var("PODMAN_UNIT_PATH", uloc);

        fs::remove_dir_all(uloc).unwrap_or_default();

        fs::create_dir_all("/tmp/quadit_test/.config/containers/systemd").unwrap();

        info!("Create scheduler");
        let scheduler = JobScheduler::new().await.unwrap();
        info!("Add job");
        let conf = ConfigGit {
            url: "https://github.com/ubiquitous-factory/quadit".to_string(),
            target_path: "samples/helloworld".to_string(),
            branch: "main".to_string(),
            schedule: "1/2 * * * * *".to_string(),
        };
        let uid = scheduler
            .add(GitManager::create_job(conf).await.unwrap())
            .await
            .expect("Should be able to add a job");

        scheduler.start().await.unwrap();

        let timer = timer::Timer::new();
        let (tx, rx) = channel();
        // Timeouts in tests are never great but we need to be sure that we capture the
        // scheduler running continuously to make sure there are no overwrite problems.
        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(5), move || {
            // This closure is executed on the scheduler thread,
            // so we want to move it away asap.
            let _ignored = tx.send(()); // Avoid unwrapping here.
        });

        let _ = rx.recv();

        let mut pb = PathBuf::new();
        pb.push("/tmp/quadit_test/jobs");
        pb.push(uid.to_string());
        assert!(pb.exists());

        let mut p = PathBuf::new();
        p.push(uloc);
        assert!(p.exists());
    }
}
