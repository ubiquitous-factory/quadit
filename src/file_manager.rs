use std::{
    env::var,
    fs::{self, File},
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use tracing::{debug, error, info, instrument};
use url::Url;

const SUPPORTED_FILES: [&str; 5] = ["container", "volume", "pod", "network", "kube"];
/// Manages all the file system interactions
pub struct FileManager {}

impl FileManager {
    fn podman_unit_path() -> &'static str {
        static PODMAN_UNIT_PATH: OnceLock<String> = OnceLock::new();
        PODMAN_UNIT_PATH.get_or_init(|| {
            var("PODMAN_UNIT_PATH").unwrap_or(".config/containers/systemd".to_string())
        })
    }

    pub fn boot_url() -> &'static str {
        static BOOT_URL: OnceLock<String> = OnceLock::new();
        BOOT_URL.get_or_init(|| var("BOOT_URL").unwrap_or("".to_string()))
    }

    /// Bootstraps the reload manager from a URL.
    /// # Arguments
    ///
    /// * `url` - The location of the configuration
    #[instrument(level = "trace")]
    pub async fn from_url(url: &str) -> Result<bool, anyhow::Error> {
        // write the file to tmp.
        Url::parse(url)?;
        let resp = reqwest::get(url).await?;
        let body = resp.text().await?;
        fs::create_dir_all("/tmp/quadit")?;
        let tmp_loc = "/tmp/quadit/config.yaml.new";
        let mut out = File::create(tmp_loc)?;
        io::copy(&mut body.as_bytes(), &mut out)?;

        let conf_location = FileManager::resolve_quadit_config_location();
        let unm_conf_location = conf_location.clone();
        // check to see if the files are the same
        // if they are do nothing
        if !FileManager::are_identical(tmp_loc, &conf_location) {
            // check if there is already a file in .quadit
            // if there is then rename it bak
            info!("config files are different creating a backup");
            if FileManager::file_exists(&conf_location) {
                // move file to back
                let bak_name = format!("{}{}", conf_location, ".bak");
                fs::rename(conf_location, bak_name)?;
            }
            // copy the file from tmp to .quadit
            // the folder might not have been created yet so lets to that
            let quadit_home = FileManager::quadit_home();
            fs::create_dir_all(quadit_home)?;
            info!("Moving new config {} to {}", tmp_loc, unm_conf_location);
            fs::copy(tmp_loc, unm_conf_location)?;
            info!("Moved new config");
            return Ok(true);
        }

        // let reload_manager = ReloadManager::new().await?;

        Ok(false)
    }

    #[instrument(level = "trace")]
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
    /// gets the name of the job folder
    #[instrument(level = "trace")]
    pub fn job_folder() -> &'static str {
        static JOB_FOLDER: OnceLock<String> = OnceLock::new();
        JOB_FOLDER.get_or_init(|| var("JOB_FOLDER").unwrap_or("jobs".to_string()))
    }

    #[instrument(level = "trace")]
    pub fn is_local() -> &'static str {
        static LOCAL: OnceLock<String> = OnceLock::new();
        LOCAL.get_or_init(|| var("LOCAL").unwrap_or("no".to_string()))
    }

    /// Gets the root to the root of the job path folder.
    #[instrument(level = "trace")]
    pub fn job_path() -> &'static str {
        static JOB_PATH: OnceLock<String> = OnceLock::new();
        JOB_PATH.get_or_init(|| var("JOB_PATH").unwrap_or("".to_string()))
    }
    /// Simple wrapper around the `read_to_string`
    #[instrument(level = "trace")]
    pub fn readfile(file_path: String) -> Result<String, std::io::Error> {
        info!("Loading: {}", file_path);
        fs::read_to_string(file_path)
    }

    /// Generates the location of the quadit config.
    #[instrument(level = "trace")]
    pub fn resolve_quadit_config_location() -> String {
        let loc = format!("{}/{}", FileManager::quadit_home(), "config.yaml");
        info!("Using config location : {}", loc);
        loc
    }

    /// Test to see if the job folder exists.
    /// Used to validate if this is the first time the job has ran.
    /// # Arguments
    /// `uuid` - The uuid of the job - Usually `xxxxxxxx-xxxx-4xxx-Nxxx-xxxxxxxxxxxx`.
    #[instrument(level = "trace")]
    pub fn job_exists(uuid: uuid::Uuid) -> bool {
        let mut pb = PathBuf::new();
        pb.push(FileManager::job_folder());
        pb.push(uuid.to_string());
        pb.exists()
    }

    /// Gets the home location of the user currently running quadit
    pub fn quadit_home() -> String {
        if FileManager::is_local() == "yes" {
            let mut dir = match dirs::home_dir() {
                Some(s) => s,
                None => match std::env::current_dir() {
                    Ok(s) => s,
                    Err(e) => {
                        error!(
                            "couldn't find home or current directory \n {} \n going to try `./` ",
                            e
                        );
                        PathBuf::from("./")
                    }
                },
            };
            dir.push(".quadit");
            // dir.push("config.yaml");
            if !dir.exists() {
                error!(
                    "The file `{}` does not exist. See samples folder",
                    dir.to_string_lossy()
                );
                std::process::exit(1);
            };
            // TODO: OS specific but that's OK for linux
            dir.as_path().display().to_string()
        } else {
            "/opt/config".to_string()
        }
    }

    /// Loads the quadit config based on the resolved location.
    #[instrument(level = "trace")]
    pub fn load_quadit_config() -> Result<String, std::io::Error> {
        FileManager::readfile(FileManager::resolve_quadit_config_location())
    }

    /// converts the quadlet file name to a service name with some additional checks.
    #[instrument(level = "trace")]
    pub fn filename_to_unit_name(target_path: &str) -> Result<String, String> {
        let mut path = PathBuf::from(&target_path);
        if !SUPPORTED_FILES.contains(
            &path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
        ) {
            let msg = format!(
                "Target path MUST be a valid quadlet file. e.g. .container, .volume, .pod, .network, .kube.  Found: {}",
                target_path
            );
            return Err(msg);
        }
        path.set_extension("service");
        match path.file_name() {
            Some(s) => match s.to_os_string().into_string() {
                Ok(v) => Ok(v),
                Err(_) => Err("Failed to convert filename to string".to_string()),
            },
            None => Err("Service name not generated".to_string()),
        }
    }

    /// Test to see if a file exists in the container deployment location
    /// Currently `~/.config/containers/systemd/` but this may be expanded in later releases.
    /// # Arguments
    /// `job_path` - The path to the job - Usually `jobs/xxxxxxxx-xxxx-4xxx-Nxxx-xxxxxxxxxxxx`.
    /// `target_path` - The path of the file in the git repo  
    #[instrument(level = "trace")]
    pub fn container_file_deployed(job_path: &str, target_path: &str) -> bool {
        let mut definition_path = PathBuf::new();
        definition_path.push(job_path);
        definition_path.push(target_path);

        let path = Path::new(target_path);
        let mut config_path = FileManager::get_container_path();

        config_path.push(path.file_name().unwrap_or_default());

        debug!(
            "{} comparing {}",
            config_path.display(),
            definition_path.display()
        );
        FileManager::are_identical(
            config_path
                .as_os_str()
                .to_str()
                .unwrap_or("config_path_unknown"),
            definition_path
                .as_os_str()
                .to_str()
                .unwrap_or("definition_path_unknown"),
        )
    }

    /// Collects a directory contents as a vector of strings
    #[instrument(level = "trace")]
    pub fn get_files_in_directory(path: &str) -> Result<Vec<String>, anyhow::Error> {
        // Get a list of all entries in the folder
        let entries = fs::read_dir(path)?;

        // Extract the filenames from the directory entries and store them in a vector
        let file_names: Vec<String> = entries
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() {
                    path.file_name()?.to_str().map(|s| (s.to_owned()))
                } else {
                    None
                }
            })
            .collect();

        Ok(file_names)
    }
    /// Compare the bytes of two files incrementally to see if they differ.
    ///
    /// # Arguments
    /// `file_name1` - The first file to compare.
    /// `file_name2` - The second file to compare.    
    #[instrument(level = "trace")]
    pub fn are_identical(file_name1: &str, file_name2: &str) -> bool {
        if let Result::Ok(file1) = File::open(file_name1) {
            let mut reader1 = BufReader::new(file1);
            if let Result::Ok(file2) = File::open(file_name2) {
                let mut reader2 = BufReader::new(file2);
                let mut buf1 = [0; 10000];
                let mut buf2 = [0; 10000];
                // loop {
                while let Result::Ok(n1) = reader1.read(&mut buf1) {
                    if n1 > 0 {
                        if let Result::Ok(n2) = reader2.read(&mut buf2) {
                            if n1 == n2 && buf1 == buf2 {
                                continue;
                            }
                            return false;
                        }
                    } else {
                        break;
                    }
                }
                return true;
            };
        };
        false
    }
    /// Copy to the users ~/.config/containers/systemd/
    ///
    /// # Arguments
    /// `job_path` - The location that the repo has been copied to
    /// `target_path` - The location of the .container file in the repo
    #[instrument(level = "trace")]
    pub fn deploy_container_file(job_path: &str, target_path: &str) -> Result<String, String> {
        let path = Path::new(target_path);

        let mut definition_path = PathBuf::new();
        definition_path.push(job_path);
        definition_path.push(target_path);
        if !SUPPORTED_FILES.contains(
            &definition_path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
        ) {
            let msg = format!(
                "File MUST be a valid quadlet file. e.g. .container, .volume, .pod, .network, .kube.  Found: {}",
                target_path
            );
            return Err(msg);
        }
        let mut cont_path = FileManager::get_container_path();
        cont_path.push(path.file_name().unwrap_or_default());

        let cpath = cont_path.clone();
        let dpath = definition_path.clone();
        match fs::copy(definition_path, cont_path) {
            Ok(_) => {}
            Err(e) => {
                let msg = format!(
                    "Failed to copy {:?} to {:?}. {}",
                    dpath.to_str(),
                    cpath.to_str(),
                    e
                );
                error!("{}", msg);

                return Err(msg);
            }
        }

        Ok(cpath.as_path().display().to_string())
    }

    fn get_container_path() -> PathBuf {
        if FileManager::is_local() == "yes" {
            let mut config_path = match dirs::home_dir() {
                Some(p) => p,
                None => PathBuf::new(),
            };
            config_path.push(FileManager::podman_unit_path());
            config_path
        } else {
            let mut config_path = PathBuf::new();
            config_path.push("/opt/containers");
            config_path
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        path::PathBuf,
    };

    use super::FileManager;

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
    fn test_deploy_container_file() {
        let jobdir = "/tmp/test_deploy_container_file_job";
        let target_path = "test.container";
        fs::create_dir(jobdir).unwrap();

        let mut cont_path = FileManager::get_container_path();
        if !cont_path.exists() {
            let dir = cont_path.clone();
            fs::create_dir_all(dir).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }
        cont_path.push(target_path);

        let file_path: PathBuf = [jobdir, "test.container"].iter().collect();
        let rm_file_path: PathBuf = [jobdir, "test.container"].iter().collect();
        File::create(file_path).unwrap();

        let s = FileManager::deploy_container_file(jobdir, target_path).unwrap();
        println!("{}", cont_path.as_path().display());
        println!("{}", s);
        assert!(cont_path.exists());
        fs::remove_file(rm_file_path).unwrap();
        fs::remove_dir(jobdir).unwrap();
        fs::remove_file(cont_path).unwrap();
    }

    #[test]
    fn test_container_file_to_unit_name() {
        let original = "sample/test.container".to_string();
        let expected = "test.service".to_string();
        let resp = FileManager::filename_to_unit_name(&original);

        assert_eq!(expected, resp.unwrap());
    }
}
