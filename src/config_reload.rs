use serde::{Deserialize, Serialize};

/// Configuration for reloading quadit.
/// Used to serialize the contents of `config.yaml`
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigReload {
    /// The URL of the remote endpoint containing configuration changes.
    pub config_u_r_l: String,
    /// The schedule in cron format
    /// e.g.
    /// ```
    /// //               sec  min   hour   day of month   month   day of week   year
    /// let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
    /// // or run every minute
    /// let expression = "1/60 * * * * *";
    /// ```
    pub schedule: String,
}
