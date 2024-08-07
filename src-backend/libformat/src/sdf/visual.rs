/// align to doc: http://sdformat.org/spec?ver=1.11&elem=visual
/// finished
use serde::{Deserialize, Serialize};

use crate::sdf::geometry::Geometry;
use crate::sdf::material::Material;
use crate::sdf::util::{Plugin, Pose};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Visual {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_shadows: Option<bool>, // default true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub laser_retro: Option<f64>, // default 0.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency: Option<f64>, // default 0.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_flags: Option<u32>, // default 4294967295

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<visual::Meta>, //
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>, // 0 0 0 0 0 0

    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<Material>,

    pub geometry: Geometry,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugin: Vec<Plugin>,
}

pub mod visual {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Meta {
        pub layer: Option<i32>, // default 0
    }
}
