use serde::{Deserialize, Serialize};

use crate::openscenario::common::{DateTime, Properties, UnsignedShort};

#[derive(Deserialize, Serialize, Debug)]
pub struct FileHeader {
    #[serde(rename = "License", skip_serializing_if = "Option::is_none")]
    pub license: Option<Vec<License>>,
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Properties>>,
    #[serde(rename = "@author")]
    pub author: String,
    #[serde(rename = "@date")]
    pub date: DateTime,
    #[serde(rename = "@description")]
    pub description: String,
    #[serde(rename = "@revMajor")]
    pub rev_major: UnsignedShort,
    #[serde(rename = "@revMinor")]
    pub rev_minor: UnsignedShort,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct License {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "@spdxId", skip_serializing_if = "Option::is_none")]
    pub spdx_id: Option<String>,
}
