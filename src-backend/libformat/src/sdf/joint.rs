/// align to doc: http://sdformat.org/spec?ver=1.11&elem=joint
/// finished
use serde::{Deserialize, Serialize};

use crate::sdf::sensor::Sensor;
use crate::sdf::util::Pose;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Joint {
    #[serde(rename = "@name")]
    pub name: String,
    // Description:The type of joint, which must be one of the following:
    // (continuous) a hinge joint that rotates on a single axis with a
    // continuous range of motion, (revolute) a hinge joint that rotates on
    // a single axis with a fixed range of motion, (gearbox) geared revolute
    // joints, (revolute2) same as two revolute joints connected in series,
    // (prismatic) a sliding joint that slides along an axis with a limited
    // range specified by upper and lower limits, (ball) a ball and socket
    // joint, (screw) a single degree of freedom joint with coupled sliding
    // and rotational motion, (universal) like a ball joint, but constrains one
    // degree of freedom, (fixed) a joint with zero degrees of freedom that
    // rigidly connects two links.
    #[serde(rename = "@type")]
    pub r#type: String,

    //
    pub parent: String,
    pub child: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gearbox_ratio: Option<f64>, // default 1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gearbox_reference_body: Option<String>,
    // Description: A parameter for screw joint kinematics, representing the
    // axial distance traveled for each revolution of the joint, with units
    // of meters / revolution with a positive value corresponding to a
    // right-handed thread. This parameter supersedes `thread_pitch`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screw_thread_pitch: Option<f64>, // default 1

    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<joint::Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis2: Option<joint::Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physics: Option<joint::Physics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>, // 0 0 0 0 0 0

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor: Option<Sensor>,
}

pub mod joint {
    use serde::{Deserialize, Serialize};

    use crate::sdf::util::Vector3;

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Axis {
        pub xyz: XYZ,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub dynamics: Option<Dynamics>,

        pub limit: Limit,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub mimic: Option<Mimic>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Physics {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub simbody: Option<SimBody>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ode: Option<Ode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provide_feedback: Option<bool>, // default false
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct XYZ {
        #[serde(rename = "@expressed_in", skip_serializing_if = "Option::is_none")]
        pub expressed_in: Option<String>,

        #[serde(rename = "$text")]
        pub children: Vector3<f64>, // default 0 0 1
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Dynamics {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub damping: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub friction: Option<f64>, // default 0
        //
        pub spring_reference: f64, // default 0
        pub spring_stiffness: f64, // default 0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Limit {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lower: Option<f64>, // Default: -inf
        #[serde(skip_serializing_if = "Option::is_none")]
        pub upper: Option<f64>, // Default: inf
        #[serde(skip_serializing_if = "Option::is_none")]
        pub effort: Option<f64>, // Default: inf
        #[serde(skip_serializing_if = "Option::is_none")]
        pub velocity: Option<f64>, // Default: inf
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stiffness: Option<f64>, // Default: 100000000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dissipation: Option<f64>, // Default: 1
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Mimic {
        #[serde(rename = "@joint")]
        pub joint: String,
        // Description:Name of the leader axis, i.e. the axis to be mimicked.
        // The only valid values are "axis" and "axis2", and "axis2"
        // may only be used if the leader joint has multiple axes.
        #[serde(rename = "@axis")]
        pub axis: Option<String>, // Default: axis

        pub multiplier: f64, // Default: 1
        pub offset: f64,     // Default: 0
        pub reference: f64,  // Default: 0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct SimBody {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub must_be_loop_joint: Option<bool>, // default false
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Ode {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cfm_damping: Option<bool>, // default false
        #[serde(skip_serializing_if = "Option::is_none")]
        pub implicit_spring_damper: Option<bool>, // default false
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fudge_factor: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cfm: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub erp: Option<f64>, // default 0.2
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bounce: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_force: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub velocity: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<ode::Limit>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub suspension: Option<ode::Suspension>,
    }

    pub mod ode {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Limit {
            pub cfm: f64, // default 0
            pub erp: f64, // default 0.2
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Suspension {
            pub cfm: f64, // default 0
            pub erp: f64, // default 0.2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_pose() {
        let xml_origin = r#"<axis>
        <xyz>0 0 1</xyz>
      </axis>"#;
        let axis: joint::Axis = quick_xml::de::from_str(xml_origin).unwrap();

        let xml = quick_xml::se::to_string(&axis).unwrap();
        println!("{:#?}", xml);
        // assert_eq!("<Pose>1 2 3 4 5 6</Pose>", xml);
    }
}
