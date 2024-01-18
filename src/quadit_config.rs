use serde::{Deserialize, Serialize};

use crate::config_reload::ConfigReload;
use crate::git_config::GitConfig;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuaditConfig {
    pub target_configs: Vec<GitConfig>,
    pub config_reload: Option<ConfigReload>,
}

impl QuaditConfig {
    // fn load() -> Result<GitConfig, Error> {

    // }
}

#[cfg(test)]
mod tests {
    use crate::quadit_config::QuaditConfig;

    #[test]
    fn serialize_quaditconfig() {
        let test_yaml = r#"
configReload:
  configURL: https://raw.githubusercontent.com/ubiquitous-factory/ai-remote-edge/main/deploy/config.yaml
  schedule: "*/2 * * * *"
targetConfigs:
- url: "https://github.com/ubiquitous-factory/quadit"
  targetPath: "samples/sleep.container"
  branch: "main"
  schedule: "*/1 * * * *"
  start: false
"#;

        let deser: QuaditConfig = serde_yaml::from_str(test_yaml).unwrap();
        println!("{:#?}", deser);
        assert_eq!(
            deser.target_configs[0].url,
            "https://github.com/ubiquitous-factory/quadit".to_string()
        );
    }
}
