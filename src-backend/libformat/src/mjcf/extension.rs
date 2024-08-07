use serde::{Deserialize, Serialize};

use crate::mjcf::util::Plugin;

#[derive(Deserialize, Serialize, Debug)]
pub struct Extension {
    #[serde(rename = "plugin", skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Vec<Plugin>>,
}

pub mod extension {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Instance {
        #[serde(rename = "@config")]
        pub config: Vec<Config>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Config {
        #[serde(rename = "@key")]
        pub key: String,
        #[serde(rename = "@value")]
        pub value: String,
    }
}
