use crate::{config_quadit::ConfigQuadit, git_manager::GitManager, reload_manager::ReloadManager};
#[allow(dead_code)]
pub struct QuaditManager {
    git_manager: GitManager,
    config_manager: Option<ReloadManager>,
}

impl QuaditManager {
    pub async fn from_yaml(conf: String) -> Result<QuaditManager, anyhow::Error> {
        let quad = ConfigQuadit::from_yaml(conf)?;
        if quad.config_reload.is_some() {
            Ok(QuaditManager {
                git_manager: GitManager::from_target_configs(quad.target_configs).await?,
                config_manager: Some(
                    ReloadManager::from_config_reload(quad.config_reload.unwrap()).await?,
                ),
            })
        } else {
            Ok(QuaditManager {
                git_manager: GitManager::from_target_configs(quad.target_configs).await?,
                config_manager: None,
            })
        }
    }
}
