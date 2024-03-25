use nihility_common::SubmoduleInfo;
use serde::{Deserialize, Serialize};
use crate::config::customize::CustomizeConfig;

mod customize;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct SubmoduleConfig {
    pub submodule_name: String,
    pub core_public_key_path: String,
    pub default_receiver_submodule: String,
    pub info: SubmoduleInfo,
    pub log_config: SubmoduleLogConfig,
    pub instruct_server_config: InstructServerConfig,
    pub manipulate_server_config: ManipulateServerConfig,
    pub customize_config: CustomizeConfig,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct InstructServerConfig {

}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ManipulateServerConfig {

}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct SubmoduleLogConfig {

}