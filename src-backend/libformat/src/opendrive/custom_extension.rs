/// 自定义的扩展

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CleanArea {
    // attribute start
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@y")]
    pub y: f64,

    #[serde(rename = "polygon", skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Vec<[f64; 2]>>,
}
