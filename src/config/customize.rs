use nihility_common::{ClientType, ConnectionType};
use serde::{Deserialize, Serialize};

pub const SUBMODULE_NAME: &str = "submodule-template";
pub const CORE_PUBLIC_KEY_PATH: &str = "./auth/id_rsa.pub";
pub const DEFAULT_RECEIVER_SUBMODULE: &str = "test";
pub const DEFAULT_INSTRUCT: Vec<&str> = vec!["test1", "test2"];
pub const CONNECTION_TYPE: ConnectionType = ConnectionType::GrpcType;
pub const CLIENT_TYPE: ClientType = ClientType::NotReceiveType;
pub const CONN_CONFIG: &str = r#"{}"#;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct CustomizeConfig {}
