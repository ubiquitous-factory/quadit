use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigGit {
    pub url: String,
    pub target_path: String,
    pub branch: String,
    pub schedule: String,
    pub start: bool,
    #[serde(flatten)]
    pub extras: Option<HashMap<String, String>>,
}
