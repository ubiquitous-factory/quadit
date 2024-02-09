use serde::{Deserialize, Serialize};

/// Configuration for reloading quadit.
/// Used to serialize the contents of `config.yaml`
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigReload {
    /// The URL of the remote endpoint containing configuration changes.
    pub config_u_r_l: String,
    /// The interval to poll in milliseconds
    pub interval: u64,
}
