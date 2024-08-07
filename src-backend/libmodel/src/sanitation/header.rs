use serde::{Deserialize, Serialize};

/// 场景的头部
#[derive(Debug, Clone,Deserialize, Serialize)]
pub struct Header {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@map")]
    pub map_name: String,
    #[serde(rename = "@date")]
    pub date: String, // chrono::prelude::Local::now().to_string()
}

impl Default for Header {
    fn default() -> Self {
        Header {
            name: "default".to_string(),
            map_name: "base_map".to_string(),
            date: chrono::prelude::Local::now().to_string(),
        }
    }
}

pub enum Standard {}