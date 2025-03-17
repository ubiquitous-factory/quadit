#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{self, File, OpenOptions},
        path::PathBuf,
    };

    use quadit::file_manager::FileManager;
    use tracing::error;

    #[test]
    fn test_are_identical() {
        let file_name1 = "Cargo.toml";
        let file_name2 = "Cargo.toml";

        assert!(FileManager::are_identical(file_name1, file_name2));
    }
    #[test]
    fn test_are_not_identical() {
        let file_name1 = "README.md";
        let file_name2 = "Cargo.toml";

        assert!(!FileManager::are_identical(file_name1, file_name2));
    }

    #[test]
    fn test_deploy_unit_file() {
        // Arrange
        let jobdir = "/tmp/test_deploy_container_file_job";
        let target_path = "test.container";
        fs::create_dir(jobdir).unwrap();

        let mut unit_path = FileManager::get_unit_path();
        if !unit_path.exists() {
            let dir = unit_path.clone();
            fs::create_dir_all(dir).unwrap_or_else(|why| {
                error!("! {:?}", why.kind());
            });
        }
        unit_path.push(target_path);

        let file_path: PathBuf = [jobdir, "test.container"].iter().collect();
        let rm_file_path: PathBuf = [jobdir, "test.container"].iter().collect();
        File::create(file_path).unwrap();

        // Act
        let s: String = FileManager::deploy_unit_file(jobdir, target_path).unwrap();
        println!("{}", unit_path.as_path().display());
        println!("{}", s);

        // Assert
        assert!(unit_path.exists());
        let deployed_unit_path: PathBuf = [s].iter().collect();
        assert!(deployed_unit_path.exists(), "Unit path wasn't deployed");

        // Tidy
        fs::remove_file(rm_file_path).unwrap();
        fs::remove_dir(jobdir).unwrap();
        fs::remove_file(unit_path).unwrap();
    }

    #[test]

    fn test_deploy_unit_folder_job() {
        // Arrange
        let jobdir = "/tmp/test_deploy_unit_folder_job";
        let target_path = "samples/helloworld";
        let full_job_folder = "/tmp/test_deploy_unit_folder_job/samples/helloworld";
        let file_path: PathBuf = [jobdir, "samples/helloworld", "test.container"]
            .iter()
            .collect();
        let rm_file_path: PathBuf = [jobdir, "samples/helloworld", "test.container"]
            .iter()
            .collect();

        fs::create_dir_all(full_job_folder).unwrap();
        File::create(&file_path).unwrap();
        let mut unit_path = FileManager::get_unit_path();
        if !unit_path.exists() {
            let dir = unit_path.clone();
            fs::create_dir_all(dir).unwrap_or_else(|why| {
                error!("! {:?}", why.kind());
            });
        }
        unit_path.push(target_path);

        println!(
            "Simulating downloaded file: {}",
            &file_path.as_path().display()
        );

        let _ = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path);

        // Act
        let s = FileManager::deploy_unit_file(jobdir, "samples/helloworld/test.container").unwrap();
        println!("{}", unit_path.as_path().display());
        println!("{}", s);

        // Assert
        assert!(unit_path.exists());
        assert_eq!(
            s,
            "/opt/containers/samples/helloworld/test.container".to_string(),
            "Deployed unit location is incorrect"
        );
        let deployed_unit_path: PathBuf = [s].iter().collect();
        assert!(deployed_unit_path.exists(), "Unit path wasn't deployed");
        // Clean Up
        fs::remove_file(&rm_file_path).unwrap();
        fs::remove_dir(full_job_folder).unwrap();
    }

    #[test]
    fn test_container_file_to_unit_name() {
        let original = "sample/test.container".to_string();
        let expected = "test.service".to_string();
        let resp = FileManager::filename_to_unit_name(&original);

        assert_eq!(expected, resp.unwrap());
    }

    #[test]
    fn test_default_podman_unit_location() {
        assert_eq!(
            FileManager::podman_unit_path(),
            ".config/containers/systemd",
            "Unexpected podman unit defaults"
        );
    }

    #[test]
    fn test_set_boot_url() {
        env::set_var("BOOT_URL", "iamset");
        assert_eq!(FileManager::boot_url(), "iamset", "Unexpected boot url");
    }
}
