use serde::{Deserialize, Serialize};

use crate::mjcf::{equality, tendon};
use crate::mjcf::actuator::actuator;
use crate::mjcf::asset::asset;
use crate::mjcf::body::body;
use crate::mjcf::contact::contact;

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultWrapper {
    #[serde(rename = "default")]
    pub child: Vec<Default>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Default {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "mesh", skip_serializing_if = "Option::is_none")]
    pub mesh: Option<asset::Mesh>,
    #[serde(rename = "material", skip_serializing_if = "Option::is_none")]
    pub material: Option<asset::Material>,
    #[serde(rename = "joint", skip_serializing_if = "Option::is_none")]
    pub joint: Option<body::Joint>,
    #[serde(rename = "geom", skip_serializing_if = "Option::is_none")]
    pub geom: Option<body::Geom>,
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<body::Site>,
    #[serde(rename = "camera", skip_serializing_if = "Option::is_none")]
    pub camera: Option<body::Camera>,
    #[serde(rename = "light", skip_serializing_if = "Option::is_none")]
    pub light: Option<body::Light>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<contact::Pair>,
    #[serde(rename = "equality", skip_serializing_if = "Option::is_none")]
    pub equality: Option<equality::Equality>,
    #[serde(rename = "tendon", skip_serializing_if = "Option::is_none")]
    pub tendon: Option<tendon::Tendon>,
    #[serde(rename = "general", skip_serializing_if = "Option::is_none")]
    pub general: Option<actuator::General>,
    #[serde(rename = "motor", skip_serializing_if = "Option::is_none")]
    pub motor: Option<actuator::Motor>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<actuator::Position>,
    #[serde(rename = "velocity", skip_serializing_if = "Option::is_none")]
    pub velocity: Option<actuator::Velocity>,
    #[serde(rename = "intvelocity", skip_serializing_if = "Option::is_none")]
    pub int_velocity: Option<actuator::IntVelocity>,
    #[serde(rename = "damper", skip_serializing_if = "Option::is_none")]
    pub damper: Option<actuator::Damper>,
    #[serde(rename = "cylinder", skip_serializing_if = "Option::is_none")]
    pub cylinder: Option<actuator::Cylinder>,
    #[serde(rename = "muscle", skip_serializing_if = "Option::is_none")]
    pub muscle: Option<actuator::Muscle>,
    #[serde(rename = "adhesion", skip_serializing_if = "Option::is_none")]
    pub adhesion: Option<actuator::Adhesion>,

    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub child: Option<Vec<Default>>,
}

#[cfg(test)]
mod tests {
    use crate::mjcf::default::DefaultWrapper;

    #[test]
    fn it_works1() {
        let xml = r#"
<default>
    <default  class="anymal_b">
        <mesh  scale="0.001 0.001 0.001"/>
        <site  group="3" rgba="1 0 0 0.66"/>
        <joint  type="hinge" damping="1" frictionloss="0.1"/>
        <geom  mass="0"/>
                
        <default  class="HAA">
            <joint  axis="1 0 0"/>
        </default>
        <default  class="HFE">
            <joint  axis="0 1 0"/>
        </default>
        <default  class="KFE">
            <joint  axis="0 1 0"/>
        </default>

        <default  class="visual">
            <geom  type="mesh" contype="0" conaffinity="0" group="2"/>
            <default  class="visual_zflip">
                <geom  quat="0 0 0 1"/>
            </default>
        </default>
        <default  class="collision">
            <geom  group="3"/>
            <default  class="haa_actuator">
                <geom  type="cylinder" size="0.05 0.05" euler="0 1.57079632679 0"/>
            </default>
            <default  class="belly_plate_bump">
                <geom  type="box" size="0.05 0.05 0.035"/>
            </default>
            <default  class="mount">
                <geom  type="box" size="0.01 0.135 0.045"/>
            </default>
            <default  class="protector">
                <geom  type="cylinder" size="0.08 0.05"/>
            </default>
            <default  class="heatfins">
                <geom  type="cylinder" size="0.045 0.015"/>
            </default>
            <default  class="thigh_with_fins">
                <geom  type="box" size="0.04 0.02 0.125"/>
            </default>
            <default  class="kfe_actuator">
                <geom  type="cylinder" size="0.06 0.06"/>
            </default>
            <default  class="upper_protector">
                <geom  type="cylinder" size="0.066 0.06"/>
            </default>
            <default  class="shank">
                <geom  type="box" size="0.04 0.034 0.065" euler="0 1.57079632679 0"/>
            </default>
            <default  class="adapter">
                <geom  type="cylinder" size="0.015 0.14" pos="0 0 -0.14"/>
            </default>
            <default  class="foot">
                <geom type="sphere" size="0.031" pos="0 0 0.02325" priority="1" solimp="0.015 1 0.031" condim="6"
            friction="0.8 0.02 0.01"/>
            </default>
        </default>
        <default  class="affine">
            <position  kp="100" ctrlrange="-6.28 6.28" forcerange="-40 40"/>
        </default>
    </default>
</default>
        "#;

        let default: DefaultWrapper = quick_xml::de::from_str(&xml).unwrap();
    }
}
