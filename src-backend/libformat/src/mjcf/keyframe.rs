use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct KeyFrame {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<Key>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Key {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@time", skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    #[serde(rename = "@qpos", skip_serializing_if = "Option::is_none")]
    pub q_pos: Option<Vec<f64>>,
    #[serde(rename = "@qpos", skip_serializing_if = "Option::is_none")]
    pub q_vel: Option<Vec<f64>>,
    #[serde(rename = "@act", skip_serializing_if = "Option::is_none")]
    pub act: Option<Vec<f64>>,
    #[serde(rename = "@ctrl", skip_serializing_if = "Option::is_none")]
    pub ctrl: Option<Vec<f64>>,
    #[serde(rename = "@mpos", skip_serializing_if = "Option::is_none")]
    pub m_pos: Option<Vec<f64>>,
    #[serde(rename = "@mquat", skip_serializing_if = "Option::is_none")]
    pub m_quat: Option<Vec<f64>>,
}
