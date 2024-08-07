use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Actuator {
    #[serde(rename = "general", skip_serializing_if = "Option::is_none")]
    pub general: Option<Vec<actuator::General>>,
    #[serde(rename = "motor", skip_serializing_if = "Option::is_none")]
    pub motor: Option<Vec<actuator::Motor>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec<actuator::Position>>,
    #[serde(rename = "velocity", skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Vec<actuator::Velocity>>,
    #[serde(rename = "intvelocity", skip_serializing_if = "Option::is_none")]
    pub int_velocity: Option<Vec<actuator::IntVelocity>>,
    #[serde(rename = "damper", skip_serializing_if = "Option::is_none")]
    pub damper: Option<Vec<actuator::Damper>>,
    #[serde(rename = "cylinder", skip_serializing_if = "Option::is_none")]
    pub cylinder: Option<Vec<actuator::Cylinder>>,
    #[serde(rename = "muscle", skip_serializing_if = "Option::is_none")]
    pub muscle: Option<Vec<actuator::Muscle>>,
    #[serde(rename = "adhesion (*", skip_serializing_if = "Option::is_none")]
    pub adhesion: Option<Vec<actuator::Adhesion>>,
    #[serde(rename = "plugin", skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Vec<actuator::Plugin>>,
}

pub mod actuator {
    use serde::{Deserialize, Serialize};

    use crate::mjcf::util::{BiasType, DynType, GainType};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct General {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@ctrllimited", skip_serializing_if = "Option::is_none")]
        pub ctrl_limited: Option<bool>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@actlimited", skip_serializing_if = "Option::is_none")]
        pub act_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>, // real(2), “0 0”
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>, // real(2), “0 0”
        #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
        pub length_range: Option<[f64; 2]>, // real(2), “0 0”
        #[serde(rename = "@gear", skip_serializing_if = "Option::is_none")]
        pub gear: Option<Vec<f64>>, // real(6), “1 0 0 0 0 0”  or real(1)
        #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
        pub crank_length: Option<f64>,
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<String>,
        #[serde(rename = "@jointinparent", skip_serializing_if = "Option::is_none")]
        pub joint_in_parent: Option<String>,
        #[serde(rename = "@site", skip_serializing_if = "Option::is_none")]
        pub site: Option<String>,
        #[serde(rename = "@refsite", skip_serializing_if = "Option::is_none")]
        pub ref_site: Option<String>,
        #[serde(rename = "@body", skip_serializing_if = "Option::is_none")]
        pub body: Option<String>,
        #[serde(rename = "@tendon", skip_serializing_if = "Option::is_none")]
        pub tendon: Option<String>,
        #[serde(rename = "@cranksite", skip_serializing_if = "Option::is_none")]
        pub crank_site: Option<String>,
        #[serde(rename = "@slidersite", skip_serializing_if = "Option::is_none")]
        pub slider_site: Option<String>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>, // real(nuser_actuator), “0 … 0”
        #[serde(rename = "@actdim", skip_serializing_if = "Option::is_none")]
        pub act_dim: Option<f64>, // default -1
        #[serde(rename = "@dyntype", skip_serializing_if = "Option::is_none")]
        pub dyn_type: Option<DynType>,
        #[serde(rename = "@gaintype", skip_serializing_if = "Option::is_none")]
        pub gain_type: Option<GainType>,
        #[serde(rename = "@biastype", skip_serializing_if = "Option::is_none")]
        pub bias_type: Option<BiasType>,
        #[serde(rename = "@dynprm", skip_serializing_if = "Option::is_none")]
        pub dyn_prm: Option<Vec<f64>>,
        #[serde(rename = "@gainprm", skip_serializing_if = "Option::is_none")]
        pub gain_prm: Option<Vec<f64>>,
        #[serde(rename = "@biasprm", skip_serializing_if = "Option::is_none")]
        pub bias_prm: Option<Vec<f64>>,
        #[serde(rename = "@actearly", skip_serializing_if = "Option::is_none")]
        pub act_early: Option<bool>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Motor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@ctrllimited", skip_serializing_if = "Option::is_none")]
        pub ctrl_limited: Option<bool>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>,
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>,
        #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
        pub length_range: Option<[f64; 2]>,
        #[serde(rename = "@gear", skip_serializing_if = "Option::is_none")]
        pub gear: Option<Vec<f64>>, // real(6), “1 0 0 0 0 0”  or real(1)
        #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
        pub crank_length: Option<f64>,
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<String>,
        #[serde(rename = "@jointinparent", skip_serializing_if = "Option::is_none")]
        pub joint_in_parent: Option<String>,
        #[serde(rename = "@site", skip_serializing_if = "Option::is_none")]
        pub site: Option<String>,
        #[serde(rename = "@refsite", skip_serializing_if = "Option::is_none")]
        pub ref_site: Option<String>,
        #[serde(rename = "@tendon", skip_serializing_if = "Option::is_none")]
        pub tendon: Option<String>,
        #[serde(rename = "@cranksite", skip_serializing_if = "Option::is_none")]
        pub crank_site: Option<String>,
        #[serde(rename = "@slidersite", skip_serializing_if = "Option::is_none")]
        pub slider_site: Option<String>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Position {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@ctrllimited", skip_serializing_if = "Option::is_none")]
        pub ctrl_limited: Option<bool>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>,
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>,
        #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
        pub length_range: Option<[f64; 2]>,
        #[serde(rename = "@gear", skip_serializing_if = "Option::is_none")]
        pub gear: Option<Vec<f64>>, // real(6), “1 0 0 0 0 0”  or real(1)
        #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
        pub crank_length: Option<f64>,
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<String>,
        #[serde(rename = "@jointinparent", skip_serializing_if = "Option::is_none")]
        pub joint_in_parent: Option<String>,
        #[serde(rename = "@site", skip_serializing_if = "Option::is_none")]
        pub site: Option<String>,
        #[serde(rename = "@refsite", skip_serializing_if = "Option::is_none")]
        pub ref_site: Option<String>,
        #[serde(rename = "@tendon", skip_serializing_if = "Option::is_none")]
        pub tendon: Option<String>,
        #[serde(rename = "@cranksite", skip_serializing_if = "Option::is_none")]
        pub crank_site: Option<String>,
        #[serde(rename = "@slidersite", skip_serializing_if = "Option::is_none")]
        pub slider_site: Option<String>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@kp", skip_serializing_if = "Option::is_none")]
        pub kp: Option<f64>, // 1
        #[serde(rename = "@kv", skip_serializing_if = "Option::is_none")]
        pub kv: Option<f64>, // 0
        #[serde(rename = "@dampratio", skip_serializing_if = "Option::is_none")]
        pub damp_ratio: Option<f64>,
        #[serde(rename = "@timeconst", skip_serializing_if = "Option::is_none")]
        pub time_const: Option<f64>,
        #[serde(rename = "@inheritrange", skip_serializing_if = "Option::is_none")]
        pub inherit_range: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Velocity {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@ctrllimited", skip_serializing_if = "Option::is_none")]
        pub ctrl_limited: Option<bool>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>,
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>,
        #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
        pub length_range: Option<[f64; 2]>,
        #[serde(rename = "@gear", skip_serializing_if = "Option::is_none")]
        pub gear: Option<Vec<f64>>, // real(6), “1 0 0 0 0 0”  or real(1)
        #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
        pub crank_length: Option<f64>,
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<String>,
        #[serde(rename = "@jointinparent", skip_serializing_if = "Option::is_none")]
        pub joint_in_parent: Option<String>,
        #[serde(rename = "@site", skip_serializing_if = "Option::is_none")]
        pub site: Option<String>,
        #[serde(rename = "@refsite", skip_serializing_if = "Option::is_none")]
        pub ref_site: Option<String>,
        #[serde(rename = "@tendon", skip_serializing_if = "Option::is_none")]
        pub tendon: Option<String>,
        #[serde(rename = "@cranksite", skip_serializing_if = "Option::is_none")]
        pub crank_site: Option<String>,
        #[serde(rename = "@slidersite", skip_serializing_if = "Option::is_none")]
        pub slider_site: Option<String>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@kv", skip_serializing_if = "Option::is_none")]
        pub kv: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct IntVelocity {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@ctrllimited", skip_serializing_if = "Option::is_none")]
        pub ctrl_limited: Option<bool>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>,
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>,
        #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
        pub length_range: Option<[f64; 2]>,
        #[serde(rename = "@gear", skip_serializing_if = "Option::is_none")]
        pub gear: Option<Vec<f64>>, // real(6), “1 0 0 0 0 0”  or real(1)
        #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
        pub crank_length: Option<f64>,
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<String>,
        #[serde(rename = "@jointinparent", skip_serializing_if = "Option::is_none")]
        pub joint_in_parent: Option<String>,
        #[serde(rename = "@site", skip_serializing_if = "Option::is_none")]
        pub site: Option<String>,
        #[serde(rename = "@refsite", skip_serializing_if = "Option::is_none")]
        pub ref_site: Option<String>,
        #[serde(rename = "@tendon", skip_serializing_if = "Option::is_none")]
        pub tendon: Option<String>,
        #[serde(rename = "@cranksite", skip_serializing_if = "Option::is_none")]
        pub crank_site: Option<String>,
        #[serde(rename = "@slidersite", skip_serializing_if = "Option::is_none")]
        pub slider_site: Option<String>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@kp", skip_serializing_if = "Option::is_none")]
        pub kp: Option<f64>, // 1.
        #[serde(rename = "@kv", skip_serializing_if = "Option::is_none")]
        pub kv: Option<f64>,
        #[serde(rename = "@dampratio", skip_serializing_if = "Option::is_none")]
        pub damp_ratio: Option<f64>,
        #[serde(rename = "@inheritrange", skip_serializing_if = "Option::is_none")]
        pub inherit_range: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Damper {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@ctrllimited", skip_serializing_if = "Option::is_none")]
        pub ctrl_limited: Option<bool>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>,
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>,
        #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
        pub length_range: Option<[f64; 2]>,
        #[serde(rename = "@gear", skip_serializing_if = "Option::is_none")]
        pub gear: Option<Vec<f64>>, // real(6), “1 0 0 0 0 0”  or real(1)
        #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
        pub crank_length: Option<f64>,
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<String>,
        #[serde(rename = "@jointinparent", skip_serializing_if = "Option::is_none")]
        pub joint_in_parent: Option<String>,
        #[serde(rename = "@site", skip_serializing_if = "Option::is_none")]
        pub site: Option<String>,
        #[serde(rename = "@refsite", skip_serializing_if = "Option::is_none")]
        pub ref_site: Option<String>,
        #[serde(rename = "@tendon", skip_serializing_if = "Option::is_none")]
        pub tendon: Option<String>,
        #[serde(rename = "@cranksite", skip_serializing_if = "Option::is_none")]
        pub crank_site: Option<String>,
        #[serde(rename = "@slidersite", skip_serializing_if = "Option::is_none")]
        pub slider_site: Option<String>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@kv", skip_serializing_if = "Option::is_none")]
        pub kv: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Cylinder {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@ctrllimited", skip_serializing_if = "Option::is_none")]
        pub ctrl_limited: Option<bool>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>,
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>,
        #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
        pub length_range: Option<[f64; 2]>,
        #[serde(rename = "@gear", skip_serializing_if = "Option::is_none")]
        pub gear: Option<Vec<f64>>, // real(6), “1 0 0 0 0 0”  or real(1)
        #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
        pub crank_length: Option<f64>,
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<String>,
        #[serde(rename = "@jointinparent", skip_serializing_if = "Option::is_none")]
        pub joint_in_parent: Option<String>,
        #[serde(rename = "@site", skip_serializing_if = "Option::is_none")]
        pub site: Option<String>,
        #[serde(rename = "@refsite", skip_serializing_if = "Option::is_none")]
        pub ref_site: Option<String>,
        #[serde(rename = "@cranksite", skip_serializing_if = "Option::is_none")]
        pub crank_site: Option<String>,
        #[serde(rename = "@slidersite", skip_serializing_if = "Option::is_none")]
        pub slider_site: Option<String>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@timeconst", skip_serializing_if = "Option::is_none")]
        pub time_const: Option<f64>,
        #[serde(rename = "@area", skip_serializing_if = "Option::is_none")]
        pub area: Option<f64>,
        #[serde(rename = "@diameter", skip_serializing_if = "Option::is_none")]
        pub diameter: Option<f64>,
        #[serde(rename = "@bias", skip_serializing_if = "Option::is_none")]
        pub bias: Option<[f64; 3]>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Muscle {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@ctrllimited", skip_serializing_if = "Option::is_none")]
        pub ctrl_limited: Option<bool>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>,
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>,
        #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
        pub length_range: Option<[f64; 2]>,
        #[serde(rename = "@gear", skip_serializing_if = "Option::is_none")]
        pub gear: Option<Vec<f64>>, // real(6), “1 0 0 0 0 0”  or real(1)
        #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
        pub crank_length: Option<f64>,
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<String>,
        #[serde(rename = "@jointinparent", skip_serializing_if = "Option::is_none")]
        pub joint_in_parent: Option<String>,
        #[serde(rename = "@tendon", skip_serializing_if = "Option::is_none")]
        pub tendon: Option<String>,
        #[serde(rename = "@cranksite", skip_serializing_if = "Option::is_none")]
        pub crank_site: Option<String>,
        #[serde(rename = "@slidersite", skip_serializing_if = "Option::is_none")]
        pub slider_site: Option<String>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@timeconst", skip_serializing_if = "Option::is_none")]
        pub time_const: Option<[f64; 2]>,
        #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
        pub range: Option<[f64; 2]>,
        #[serde(rename = "@force", skip_serializing_if = "Option::is_none")]
        pub force: Option<f64>,
        #[serde(rename = "@scale", skip_serializing_if = "Option::is_none")]
        pub scale: Option<f64>,
        #[serde(rename = "@lmin", skip_serializing_if = "Option::is_none")]
        pub l_min: Option<f64>,
        #[serde(rename = "@lmax", skip_serializing_if = "Option::is_none")]
        pub l_max: Option<f64>,
        #[serde(rename = "@vmax", skip_serializing_if = "Option::is_none")]
        pub v_max: Option<f64>,
        #[serde(rename = "@fpmax", skip_serializing_if = "Option::is_none")]
        pub fp_max: Option<f64>,
        #[serde(rename = "@fvmax", skip_serializing_if = "Option::is_none")]
        pub fv_max: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Adhesion {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>,
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@body")]
        pub body: String,
        #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
        pub gain: Option<f64>, // 1
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Plugin {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@actlimited", skip_serializing_if = "Option::is_none")]
        pub act_limited: Option<bool>,
        #[serde(rename = "@ctrllimited", skip_serializing_if = "Option::is_none")]
        pub ctrl_limited: Option<bool>,
        #[serde(rename = "@forcelimited", skip_serializing_if = "Option::is_none")]
        pub force_limited: Option<bool>,
        #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
        pub ctrl_range: Option<[f64; 2]>,
        #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
        pub force_range: Option<[f64; 2]>,
        #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
        pub length_range: Option<[f64; 2]>,
        #[serde(rename = "@gear", skip_serializing_if = "Option::is_none")]
        pub gear: Option<Vec<f64>>, // real(6), “1 0 0 0 0 0”  or real(1)
        #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
        pub crank_length: Option<f64>,
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<String>,
        #[serde(rename = "@jointinparent", skip_serializing_if = "Option::is_none")]
        pub joint_in_parent: Option<String>,
        #[serde(rename = "@site", skip_serializing_if = "Option::is_none")]
        pub site: Option<String>,
        #[serde(rename = "@refsite", skip_serializing_if = "Option::is_none")]
        pub ref_site: Option<String>,
        #[serde(rename = "@tendon", skip_serializing_if = "Option::is_none")]
        pub tendon: Option<String>,
        #[serde(rename = "@cranksite", skip_serializing_if = "Option::is_none")]
        pub crank_site: Option<String>,
        #[serde(rename = "@slidersite", skip_serializing_if = "Option::is_none")]
        pub slider_site: Option<String>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@plugin", skip_serializing_if = "Option::is_none")]
        pub plugin: Option<String>,
        #[serde(rename = "@instance", skip_serializing_if = "Option::is_none")]
        pub instance: Option<String>,
        #[serde(rename = "@dyntype", skip_serializing_if = "Option::is_none")]
        pub dyn_type: Option<String>,
        #[serde(rename = "@actrange", skip_serializing_if = "Option::is_none")]
        pub act_range: Option<[f64; 2]>,
    }
}