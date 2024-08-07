/// align to doc: http://sdformat.org/spec?ver=1.11&elem=model
/// finished
use serde::{Deserialize, Serialize};

use crate::sdf::joint::Joint;
use crate::sdf::link::Link;
use crate::sdf::util::{Frame, Include, Plugin, Pose};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Model {
    // attribute start
    #[serde(rename = "@name")]
    pub name: String,

    // The name of the model's canonical link, to which the model's implicit
    // coordinate frame is attached. If unset or set to an empty string,
    // the first `/link` listed as a direct child of this model is chosen
    // as the canonical link. If the model has no direct `/link` children, it
    // will instead be attached to the first nested (or included) model's
    // implicit frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_link: Option<String>,

    // The frame inside this model whose pose will be set by the pose element
    // of the model. i.e, the pose element specifies the pose of this frame
    // instead of the model frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_frame: Option<String>,
    // attribute end

    // element
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#static: Option<bool>, // Default: false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_collide: Option<bool>, // Default: false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_auto_disable: Option<bool>, // Default: true

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<Include>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub model: Vec<model::Model>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_wind: Option<bool>, // default false

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub frame: Vec<Frame>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>, // default 0 0 0 0 0 0

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<Link>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub joint: Vec<Joint>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugin: Vec<Plugin>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gripper: Vec<model::Gripper>,
}

pub mod model {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Model {
        #[serde(rename = "@name")]
        pub name: String,

        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub body: Vec<String>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Gripper {
        #[serde(rename = "@name")]
        pub name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub grasp_check: Option<GraspCheck>,

        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub gripper_link: Vec<String>,

        pub palm_link: String,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct GraspCheck {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub detach_steps: Option<i32>, // default 40

        #[serde(skip_serializing_if = "Option::is_none")]
        pub attach_steps: Option<i32>, // default 20

        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_contact_count: Option<u32>, // default 2
    }
}
