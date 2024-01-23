use serde::{Deserialize, Serialize};

use crate::config_git::ConfigGit;
use crate::config_reload::ConfigReload;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigQuadit {
    pub target_configs: Vec<ConfigGit>,
    pub config_reload: Option<ConfigReload>,
}

impl ConfigQuadit {
    pub fn from_yaml(yaml: String) -> Result<ConfigQuadit, serde_yaml::Error> {
        serde_yaml::from_str::<ConfigQuadit>(&yaml)
    }
}

#[cfg(test)]
mod tests {
    use crate::config_quadit::ConfigQuadit;

    #[test]
    fn test_quaditconfig_from_string() {
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
        let deser: ConfigQuadit = ConfigQuadit::from_yaml(test_yaml.to_string()).unwrap();
        println!("{:#?}", deser);
        assert_eq!(
            deser.target_configs[0].url,
            "https://github.com/ubiquitous-factory/quadit".to_string()
        );
    }
    //     #[test]
    //     fn test_quaditconfig_from_string_2() {
    //         let test_yaml = r#"
    // configReload:
    //   configURL: https://raw.githubusercontent.com/ubiquitous-factory/ai-remote-edge/main/deploy/config.yaml
    //   schedule: "*/2 * * * *"
    // targetConfigs:
    // - url: "https://github.com/ubiquitous-factory/quadit"
    //   targetPath: "samples/sleep.container"
    //   branch: "main"
    //   schedule: "*/1 * * * *"
    //   start: true
    // "#;
    //         let deser: ConfigQuadit = ConfigQuadit::from_yaml(test_yaml.to_string()).unwrap();
    //         println!("{:#?}", deser);
    //         assert_eq!(
    //             deser.target_configs[0].url,
    //             "https://github.com/ubiquitous-factory/quadit".to_string()
    //         );
    //         assert_eq!(deser.target_configs[0].start, true);
    //     }
}
