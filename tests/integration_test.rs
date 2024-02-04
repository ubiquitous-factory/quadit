#[cfg(test)]
mod test {

    use anyhow::Ok;
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
        env::set_var("JOB_PATH", "/tmp");
        env::set_var("PODMAN_UNIT_PATH", uloc);

        fs::remove_dir_all(uloc).unwrap_or_default();

        fs::create_dir_all("/tmp/quadit_test/.config/containers/systemd").unwrap();

        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Setting default subscriber failed");

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
        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(5), move || {
            // This closure is executed on the scheduler thread,
            // so we want to move it away asap.

            let _ignored = tx.send(()); // Avoid unwrapping here.
        });

        let _ = rx.recv();

        let mut pb = PathBuf::new();
        pb.push("/tmp/jobs");
        pb.push(uid.to_string());
        assert!(pb.exists());

        let mut p = PathBuf::new();
        p.push(uloc);
        assert!(p.exists());
    }
}

// #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
// async fn test_functions_in_sequence() {
//     let subscriber = FmtSubscriber::builder()
//         .with_max_level(Level::TRACE)
//         .finish();
//     tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");
//     env::set_var("JOB_PATH", "/tmp");
//     env::set_var("PODMAN_UNIT_PATH", "/tmp/test/.config/containers/systemd");
//     let test_yaml = r#"
// configReload:
//   configURL: https://raw.githubusercontent.com/ubiquitous-factory/quadit/main/samples/config.yaml
//   schedule: 1/120 * * * * *
// targetConfigs:
// - url: https://github.com/ubiquitous-factory/quadit
//   targetPath: "samples/helloworld"
//   branch: "main"
//   schedule: 1/2 * * * * *
// "#;

//     let quadit = QuaditManager::from_yaml(test_yaml.to_string())
//         .await
//         .unwrap();

//     quadit.start().await.unwrap();
//     loop {
//         std::thread::sleep(Duration::from_millis(100));
//     }

//     // let qm = qm_result.unwrap();
//     // let st = qm.start().await;
//     // assert_ok!(st);

//     // let seconds = Duration::from_secs(10);
//     // let start = SystemTime::now();
//     // loop {
//     //     std::thread::sleep(Duration::new(2, 0));

//     //     //
//     // }
// }
