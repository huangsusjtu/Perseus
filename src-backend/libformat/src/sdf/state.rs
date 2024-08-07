use serde::{Deserialize, Serialize};

use crate::sdf::light::Light;
use crate::sdf::model::Model;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct State {
    pub world_name: String,
    pub iterations: u64, // default 0

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_time: Option<u64>, // default 0, nanoseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_time: Option<u64>, // default 0, nanoseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_time: Option<u64>, // default 0, nanoseconds

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertions: Option<Insertions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletions: Option<Deletions>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Insertions {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub model: Vec<Model>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub light: Vec<Light>,
}

/// Description: A list of names of deleted entities
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Deletions {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
}
