use std::fs;

use log::info;

///
///
pub struct FileManager {}

impl FileManager {
    pub fn readfile(file_path: String) -> Result<String, std::io::Error> {
        info!("Loading: {}", file_path);
        fs::read_to_string(file_path)
    }

    pub fn resolve_quadit_config_location() -> String {
        let loc = format!("{}/{}", FileManager::quadit_home(), "config.yaml");
        info!("Using config location : {}", loc);
        loc
    }
    #[cfg(debug_assertions)]
    pub fn quadit_home() -> String {
        "samples".to_string()
    }

    #[cfg(not(debug_assertions))]
    pub fn quadit_home() {
        "/opt/mount".to_string()
    }
    pub fn load_quadit_config() -> Result<String, std::io::Error> {
        FileManager::readfile(FileManager::resolve_quadit_config_location())
    }
}
