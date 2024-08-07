use serde::{Deserialize, Serialize};

use crate::sdf::Color;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Scene {
    pub ambient: Color,    // Description: Color of the ambient light.
    pub background: Color, // Description: Color of the background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky: Option<Sky>, // Description: Properties for the sky
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadows: Option<bool>, // Description: Enable/disable shadows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog: Option<Fog>, // Description: Controls fog
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<bool>, // Description: Enable/disable the grid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_visual: Option<bool>,
    /* Description: Show/hide world origin
                               * indicator */
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Sky {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>, // Time of day [0..24], default 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunrise: Option<f64>, // Time of day [0..24], default 6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunset: Option<f64>, // Time of day [0..24], default 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clouds: Option<Cloud>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cubemap_uri: Option<String>,
    /* The URI to a cubemap texture for a
                                         * skybox. A .dds file
                                         * is typically used for the cubemap. */
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Cloud {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>, // default 0.6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<f64>, // default 0.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity: Option<f64>, // default 0.5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_size: Option<f64>, // default 0.5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient: Option<Color>,
    /* default  0.800000012 0.800000012
                                    * 0.800000012 1 */
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Fog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>, // default 1 1 1 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>, // default 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>, // default 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>, // default 1
}
