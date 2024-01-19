use std::fs;

pub struct FileManager {}

impl FileManager {
    pub fn readconfig(file_path: String) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }
}
