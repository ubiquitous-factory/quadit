use serde::{Deserialize, Serialize};

/// Configuration for executing systemd commands.
/// Used to serialize the contents of `config.yaml`
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigCommands {
    /// The name of the service e.g. sleep
    pub service_name: String,
    /// The required action
    pub action: Actions,
}

/// Enum of the supported systemd commands.
/// Used to serialize the contents of `config.yaml`
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Actions {
    /// Starts the systemd service
    start,
    /// Stops the systemd service
    stop,
    /// restarts the systemd service
    restart,
}
