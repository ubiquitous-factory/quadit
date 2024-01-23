use std::{
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use log::{error, info};

///
///
pub struct FileManager {}

impl FileManager {
    /// Simple wrapper around the `read_to_string`
    pub fn readfile(file_path: String) -> Result<String, std::io::Error> {
        info!("Loading: {}", file_path);
        fs::read_to_string(file_path)
    }

    /// Generates the location of the quadit config.
    pub fn resolve_quadit_config_location() -> String {
        let loc = format!("{}/{}", FileManager::quadit_home(), "config.yaml");
        info!("Using config location : {}", loc);
        loc
    }

    pub fn job_exists(uuid: uuid::Uuid) -> bool {
        let mut pb = PathBuf::new();
        pb.push("jobs");
        pb.push(uuid.to_string());
        pb.exists()
    }

    pub fn quadit_home() -> String {
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

        // TODO: OS specific
        dir.as_path().display().to_string()
    }

    pub fn load_quadit_config() -> Result<String, std::io::Error> {
        FileManager::readfile(FileManager::resolve_quadit_config_location())
    }

    /// converts the .container file name to a service name with some additional checks.
    pub fn container_file_to_unit_name(target_path: String) -> Result<String, String> {
        let path = Path::new(&target_path);

        if path.extension() != Some(OsStr::new("container")) {
            error!(
                "Target path MUST be a .container file. Found: {}",
                target_path
            );
            return Err("File Extension unknown".to_string());
        }
        let file_name = path.file_name().unwrap_or_default();
        let retval = match OsString::from(file_name).into_string() {
            Ok(s) => s.replace(".container", ".service"),
            Err(e) => {
                let msg = format!("Problem converting name {:?}", e);
                error!("{}", msg);
                return Err(msg);
            }
        };

        Ok(retval)
    }

    pub fn container_file_deployed(job_path: String, target_path: String) -> bool {
        let mut definition_path = PathBuf::new();
        definition_path.push(job_path);
        definition_path.push(target_path.clone());

        let path = Path::new(target_path.as_str());
        let mut config_path = match dirs::home_dir() {
            Some(p) => p,
            None => PathBuf::new(),
        };
        config_path.push(".config/containers/systemd");
        config_path.push(path.file_name().unwrap_or_default());

        FileManager::are_identical(
            config_path.display().to_string(),
            definition_path.display().to_string(),
        )
    }

    pub fn are_identical(file_name1: String, file_name2: String) -> bool {
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
    pub fn deploy_container_file(job_path: String, target_path: String) -> Result<String, String> {
        if !target_path.ends_with(".container") {
            error!(
                "Target path MUST be a .container file. Found: {}",
                target_path
            );
            return Err("UNKNOWN_FILE".to_string());
        }
        let tpath = target_path.clone();
        let path = Path::new(tpath.as_str());

        let mut definition_path = PathBuf::new();
        definition_path.push(job_path);
        definition_path.push(target_path);

        let mut config_path = match dirs::home_dir() {
            Some(p) => p,
            None => PathBuf::new(),
        };
        config_path.push(".config/containers/systemd");
        config_path.push(path.file_name().unwrap_or_default());

        let cpath = config_path.clone();
        let dpath = definition_path.clone();
        match fs::copy(definition_path, config_path) {
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
        // let retval = match OsString::from(cpath).into_string() {
        //     Ok(s) => s.replace(".container", ".service"),
        //     Err(e) => {
        //         let msg = format!("Problem converting name {:?}", e);
        //         error!("{}", msg);
        //         return Err(msg);
        //     }
        // };

        // Ok(retval)
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
        let file_name1 = String::from("Cargo.toml");
        let file_name2 = String::from("Cargo.toml");

        assert!(FileManager::are_identical(file_name1, file_name2));
    }
    #[test]
    fn test_are_not_identical() {
        let file_name1 = String::from("README.md");
        let file_name2 = String::from("Cargo.toml");

        assert!(!FileManager::are_identical(file_name1, file_name2));
    }

    #[test]
    fn test_deploy_container_file() {
        let jobdir = "test_deploy_container_file_job";
        let target_path = "test.container";
        fs::create_dir(jobdir).unwrap();

        let mut config_path = match dirs::home_dir() {
            Some(p) => p,
            None => PathBuf::new(),
        };
        config_path.push(".config/containers/systemd");
        if !config_path.exists() {
            let dir = config_path.clone();
            fs::create_dir_all(dir).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }
        config_path.push(target_path);

        let file_path: PathBuf = [jobdir, "test.container"].iter().collect();
        let rm_file_path: PathBuf = [jobdir, "test.container"].iter().collect();
        File::create(file_path).unwrap();

        let s = FileManager::deploy_container_file(jobdir.to_string(), target_path.to_string())
            .unwrap();
        println!("{}", config_path.as_path().display());
        println!("{}", s);
        assert!(config_path.exists());
        fs::remove_file(rm_file_path).unwrap();
        fs::remove_dir(jobdir).unwrap();
        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_container_file_to_unit_name() {
        let original = "test.container".to_string();
        let expected = "test.service".to_string();
        let resp = FileManager::container_file_to_unit_name(original);
        assert_eq!(expected, resp.unwrap());
    }
}
