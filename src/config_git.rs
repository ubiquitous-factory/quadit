use serde::{Deserialize, Serialize};

/// Configuration for the git repository.
/// Used to serialize the contents of `config.yaml`
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigGit {
    /// The url of the remote repo
    pub url: String,
    /// The path from the top of the cloned repo to scan for
    /// `.container`, `.volume`, `.network`, `.pod` and `.kube` files
    /// e.g. ./
    pub target_path: String,
    /// The remote branch to check out
    pub branch: String,
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
