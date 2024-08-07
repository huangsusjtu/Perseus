/// align to doc: http://sdformat.org/spec?ver=1.11&elem=world
/// finished
use serde::{Deserialize, Serialize};

use crate::sdf::actor::Actor;
use crate::sdf::joint::Joint;
use crate::sdf::light::Light;
use crate::sdf::material::Material;
use crate::sdf::model::Model;
use crate::sdf::physics::Physics;
use crate::sdf::scene::Scene;
use crate::sdf::state::State;
use crate::sdf::util::{Frame, Include, Plugin, Pose, Vector3};

/// Description: The world element encapsulates an entire world description
/// including: models, scene, physics, and plugins.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct World {
    // attribute start
    #[serde(rename = "@name")]
    pub name: String,
    // attribute end

    // element start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind: Option<Wind>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<Include>,
    pub gravity: Vector3<f64>,
    pub magnetic_field: Vector3<f64>,
    pub atmosphere: AtmoSphere,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui: Option<Gui>,
    pub physics: Physics,
    pub scene: Scene,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lights: Vec<Light>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub frames: Vec<Frame>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub joints: Vec<Joint>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub models: Vec<Model>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actors: Vec<Actor>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<Plugin>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roads: Vec<Road>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spherical_coodinates: Option<SphericalCoordinates>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<State>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub populations: Vec<Population>,
    // element end
}

/// Global audio properties.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Audio {
    pub device: String,
}

/// The wind tag specifies the type and properties of the wind.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Wind {
    pub liner_velocity: Vec<f64>, //  Linear velocity of the wind. vec3
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct AtmoSphere {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_gradient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure: Option<f64>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Gui {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullscreen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<Camera>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<Plugin>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Camera {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_controller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_visual: Option<TrackVisual>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct TrackVisual {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_dist: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dist: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#static: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_model_frame: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xyz: Option<Vector3<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_yaw: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Road {
    pub name: String,
    pub width: f64,
    pub point: Vec<Vector3<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<Material>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct SphericalCoordinates {
    pub surface_model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_frame_orientation: Option<String>,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation: f64,
    pub heading_deg: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_axis_equatorial: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_axis_polar: Option<f64>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Population {
    pub name: String,
    pub model_count: usize,
    pub distribution: Distribution,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#box: Option<BoxShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cylinder: Option<Cylinder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub model: Vec<Model>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Distribution {
    // Description: Define how the objects will be placed in the specified
    // region. - random: Models placed at random. - uniform: Models
    // approximately placed in a 2D grid pattern with control over the
    // number of objects. - grid: Models evenly placed in a 2D grid pattern.
    // The number of objects is not explicitly specified, it is based on the
    // number of rows and columns of the grid. - linear-x: Models evently
    // placed in a row along the global x-axis.
    // - linear-y: Models evently placed in a row along the global y-axis. - linear-z: Models
    // evently placed in a row along the global z-axis.
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cols: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<Vector3<f64>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct BoxShape {
    // Description: Box shape
    pub size: Vector3<f64>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Cylinder {
    // Description: Cylinder shape
    pub radius: f64,
    pub length: f64,
}
