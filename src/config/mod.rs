use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use figment::Figment;
use figment::providers::{Format, Json, Serialized, Toml, Yaml};
use nihility_common::{ConnParams, GrpcServerConfig, LogConfig, SubmoduleInfo};
use serde::{Deserialize, Serialize};

use crate::config::customize::{CLIENT_TYPE, CONN_CONFIG, CONNECTION_TYPE, CORE_PUBLIC_KEY_PATH, CustomizeConfig, DEFAULT_INSTRUCT, DEFAULT_RECEIVER_SUBMODULE, SUBMODULE_NAME};

mod customize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubmoduleConfig {
    pub submodule_name: String,
    pub core_public_key_path: String,
    pub default_receiver_submodule: String,
    pub info: SubmoduleInfo,
    pub log_config: SubmoduleLogConfig,
    pub is_instruct_server_enable: bool,
    pub is_manipulate_server_enable: bool,
    // 这个字段可以修改类型，但是必须和当前使用的Server匹配，字段名称不建议修改
    pub server_config: GrpcServerConfig,
    pub customize_config: CustomizeConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubmoduleLogConfig {
    pub logs: Vec<LogConfig>,
}

impl Default for SubmoduleLogConfig {
    fn default() -> Self {
        SubmoduleLogConfig {
            logs: vec![LogConfig::default()]
        }
    }
}

impl Default for SubmoduleConfig {
    fn default() -> Self {
        SubmoduleConfig {
            submodule_name: SUBMODULE_NAME.to_string(),
            core_public_key_path: CORE_PUBLIC_KEY_PATH.to_string(),
            default_receiver_submodule: DEFAULT_RECEIVER_SUBMODULE.to_string(),
            info: SubmoduleInfo {
                default_instruct: DEFAULT_INSTRUCT.iter().map(|x| x.to_string()).collect(),
                conn_params: ConnParams {
                    connection_type: CONNECTION_TYPE,
                    client_type: CLIENT_TYPE,
                    conn_config: serde_json::from_str(CONN_CONFIG).expect("CONN_CONFIG Format Error!"),
                },
            },
            ..Default::default()
        }
    }
}

const JSON_CONFIG_FILE_NAME: &str = "config.json";
const TOML_CONFIG_FILE_NAME: &str = "config.toml";
const YAML_CONFIG_FILE_NAME: &str = "config.yaml";

impl SubmoduleConfig {
    pub fn init() -> Result<Self> {
        let config = SubmoduleConfig::default();
        if Path::try_exists(TOML_CONFIG_FILE_NAME.as_ref())? {
            Ok(Figment::merge(
                Figment::from(Serialized::defaults(config)),
                Toml::file(TOML_CONFIG_FILE_NAME),
            )
                .extract()?)
        } else if Path::try_exists(YAML_CONFIG_FILE_NAME.as_ref())? {
            Ok(Figment::from(Serialized::defaults(config))
                .merge(Yaml::file(YAML_CONFIG_FILE_NAME))
                .extract()?)
        } else if Path::try_exists(JSON_CONFIG_FILE_NAME.as_ref())? {
            Ok(Figment::from(Serialized::defaults(config))
                .merge(Json::file(JSON_CONFIG_FILE_NAME))
                .extract()?)
        } else {
            let mut config_file: File = File::create(JSON_CONFIG_FILE_NAME)?;
            config_file.write_all(serde_json::to_string_pretty(&config)?.as_bytes())?;
            config_file.flush()?;
            Ok(config)
        }
    }
}