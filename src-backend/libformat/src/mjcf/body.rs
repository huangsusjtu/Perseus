use serde::{Deserialize, Serialize};

use crate::mjcf::body::body::Plugin;

#[derive(Deserialize, Serialize, Debug)]
pub struct Body {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@childclass", skip_serializing_if = "Option::is_none")]
    pub child_class: Option<String>,
    #[serde(rename = "@mocap", skip_serializing_if = "Option::is_none")]
    pub mocap: Option<bool>,
    #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<[f64; 3]>,
    #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
    pub quat: Option<[f64; 4]>,
    #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
    pub axis_angle: Option<[f64; 4]>,
    #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
    pub euler: Option<[f64; 3]>,
    #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
    pub xy_axes: Option<[f64; 6]>,
    #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
    pub z_axis: Option<[f64; 3]>,
    #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<f64>>,

    // element
    #[serde(rename = "inertial", skip_serializing_if = "Option::is_none")]
    pub inertial: Option<body::Inertial>,
    #[serde(rename = "joint", skip_serializing_if = "Option::is_none")]
    pub joint: Option<Vec<body::Joint>>,
    #[serde(rename = "freejoint", skip_serializing_if = "Option::is_none")]
    pub free_joint: Option<Vec<body::FreeJoint>>,
    #[serde(rename = "geom", skip_serializing_if = "Option::is_none")]
    pub geom: Option<Vec<body::Geom>>,
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<Vec<body::Site>>,
    #[serde(rename = "camera", skip_serializing_if = "Option::is_none")]
    pub camera: Option<Vec<body::Camera>>,
    #[serde(rename = "light", skip_serializing_if = "Option::is_none")]
    pub light: Option<Vec<body::Light>>,
    #[serde(rename = "plugin", skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Plugin>,
    #[serde(rename = "composite", skip_serializing_if = "Option::is_none")]
    pub composite: Option<Vec<body::Composite>>,
    #[serde(rename = "flexcomp", skip_serializing_if = "Option::is_none")]
    pub flex_comp: Option<Vec<body::FlexComp>>,
    #[serde(rename = "frame", skip_serializing_if = "Option::is_none")]
    pub frame: Option<Vec<body::Frame>>,

    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<Body>>,
}

impl Body {
    pub fn merge_other(&mut self, mut other: Body) {
        if self.inertial.is_none() {
            self.inertial = other.inertial;
        }
        if other.joint.is_some() {
            if self.joint.is_some() {
                self.joint.as_mut().unwrap().append(other.joint.as_mut().unwrap());
            } else {
                self.joint = other.joint;
            }
        }

        if other.free_joint.is_some() {
            if self.free_joint.is_some() {
                self.free_joint.as_mut().unwrap().append(other.free_joint.as_mut().unwrap());
            } else {
                self.free_joint = other.free_joint;
            }
        }
        if other.geom.is_some() {
            if self.geom.is_some() {
                self.geom.as_mut().unwrap().append(other.geom.as_mut().unwrap());
            } else {
                self.geom = other.geom;
            }
        }
        if other.site.is_some() {
            if self.site.is_some() {
                self.site.as_mut().unwrap().append(other.site.as_mut().unwrap());
            } else {
                self.site = other.site;
            }
        }
        if other.camera.is_some() {
            if self.camera.is_some() {
                self.camera.as_mut().unwrap().append(other.camera.as_mut().unwrap());
            } else {
                self.camera = other.camera;
            }
        }
        if other.light.is_some() {
            if self.light.is_some() {
                self.light.as_mut().unwrap().append(other.light.as_mut().unwrap());
            } else {
                self.light = other.light;
            }
        }
        if self.plugin.is_none() {
            self.plugin = other.plugin;
        }
        if other.composite.is_some() {
            if self.composite.is_some() {
                self.composite.as_mut().unwrap().append(other.composite.as_mut().unwrap());
            } else {
                self.composite = other.composite;
            }
        }
        if other.flex_comp.is_some() {
            if self.flex_comp.is_some() {
                self.flex_comp.as_mut().unwrap().append(other.flex_comp.as_mut().unwrap());
            } else {
                self.flex_comp = other.flex_comp;
            }
        }
        if other.frame.is_some() {
            if self.frame.is_some() {
                self.frame.as_mut().unwrap().append(other.frame.as_mut().unwrap());
            } else {
                self.frame = other.frame;
            }
        }
        if other.body.is_some() {
            if self.body.is_some() {
                self.body.as_mut().unwrap().append(other.body.as_mut().unwrap());
            } else {
                self.body = other.body;
            }
        }
    }
}

pub mod body {
    use serde::{Deserialize, Serialize};

    use crate::mjcf::util;
    use crate::mjcf::util::{
        CameraModeType, CompositeType, GeomType, JointType, SiteType,
    };

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Inertial {
        #[serde(rename = "@pos")]
        pub pos: [f64; 3],
        #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
        pub quat: Option<[f64; 4]>,
        #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
        pub axis_angle: Option<[f64; 4]>,
        #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
        pub euler: Option<[f64; 3]>,
        #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
        pub xy_axes: Option<[f64; 6]>,
        #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
        pub z_axis: Option<[f64; 3]>,
        #[serde(rename = "@mass")]
        pub mass: f64,
        #[serde(rename = "@diaginertia", skip_serializing_if = "Option::is_none")]
        pub diag_inertia: Option<[f64; 3]>,
        #[serde(rename = "@fullinertia", skip_serializing_if = "Option::is_none")]
        pub full_inertia: Option<[f64; 6]>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Joint {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<JointType>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<u32>, // 0
        #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
        pub pos: Option<[f64; 3]>, // 0 0 0
        #[serde(rename = "@axis", skip_serializing_if = "Option::is_none")]
        pub axis: Option<[f64; 3]>, // 0 0 1
        #[serde(rename = "@springdamper", skip_serializing_if = "Option::is_none")]
        pub spring_damper: Option<[f64; 2]>, // 0 0
        #[serde(rename = "@solimplimit", skip_serializing_if = "Option::is_none")]
        pub sol_imp_limit: Option<[f64; 2]>,
        #[serde(rename = "@solreflimit", skip_serializing_if = "Option::is_none")]
        pub sol_ref_limit: Option<[f64; 2]>,
        #[serde(rename = "@solimpfriction", skip_serializing_if = "Option::is_none")]
        pub sol_imp_friction: Option<f64>,
        #[serde(rename = "@solreffriction", skip_serializing_if = "Option::is_none")]
        pub sol_ref_friction: Option<[f64; 2]>,
        #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
        pub stiffness: Option<f64>,
        #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
        pub range: Option<[f64; 2]>,
        #[serde(rename = "@limited", skip_serializing_if = "Option::is_none")]
        pub limited: Option<bool>,
        #[serde(rename = "@actuatorfrcrange", skip_serializing_if = "Option::is_none")]
        pub actuator_frc_range: Option<[f64; 2]>,
        #[serde(
            rename = "@actuatorfrclimited",
            skip_serializing_if = "Option::is_none"
        )]
        pub actuator_frc_limited: Option<bool>,
        #[serde(rename = "@actuatorgravcomp", skip_serializing_if = "Option::is_none")]
        pub actuator_grav_comp: Option<bool>,

        #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
        pub margin: Option<f64>,
        #[serde(rename = "@ref", skip_serializing_if = "Option::is_none")]
        pub r#ref: Option<f64>,
        #[serde(rename = "@springref", skip_serializing_if = "Option::is_none")]
        pub spring_ref: Option<f64>,
        #[serde(rename = "@armature", skip_serializing_if = "Option::is_none")]
        pub armature: Option<f64>,
        #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
        pub damping: Option<f64>,
        #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
        pub friction_loss: Option<f64>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct FreeJoint {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<u32>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Geom {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<GeomType>,
        #[serde(rename = "@contype", skip_serializing_if = "Option::is_none")]
        pub con_type: Option<u32>, // 1
        #[serde(rename = "@conaffinity", skip_serializing_if = "Option::is_none")]
        pub con_affinity: Option<u32>, // 1
        #[serde(rename = "@condim", skip_serializing_if = "Option::is_none")]
        pub con_dim: Option<u32>, // 3
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<u32>, // 0
        #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
        pub priority: Option<u32>, // 0
        #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
        pub size: Option<String>, // 0 0 0
        #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
        pub material: Option<String>,
        #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
        pub rgba: Option<[f64; 4]>, // 0.5 0.5 0.5 1
        #[serde(rename = "@friction", skip_serializing_if = "Option::is_none")]
        pub friction: Option<String>, // 1 0.005 0.0001
        #[serde(rename = "@mass", skip_serializing_if = "Option::is_none")]
        pub mass: Option<f64>,
        #[serde(rename = "@density", skip_serializing_if = "Option::is_none")]
        pub density: Option<f64>, // 1000
        #[serde(rename = "@solmix", skip_serializing_if = "Option::is_none")]
        pub sol_mix: Option<f64>,
        #[serde(rename = "@shellinertia", skip_serializing_if = "Option::is_none")]
        pub shell_inertia: Option<bool>,
        #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
        pub sol_imp: Option<[f64; 2]>,
        #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
        pub sol_ref: Option<[f64; 2]>,
        #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
        pub margin: Option<f64>,
        #[serde(rename = "@gap", skip_serializing_if = "Option::is_none")]
        pub gap: Option<f64>,
        #[serde(rename = "@fromto", skip_serializing_if = "Option::is_none")]
        pub from_to: Option<[f64; 6]>,
        #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
        pub pos: Option<[f64; 3]>,
        #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
        pub quat: Option<[f64; 4]>,
        #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
        pub axis_angle: Option<[f64; 4]>,
        #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
        pub euler: Option<[f64; 3]>,
        #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
        pub xy_axes: Option<[f64; 6]>,
        #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
        pub z_axis: Option<[f64; 3]>,
        #[serde(rename = "@hfield", skip_serializing_if = "Option::is_none")]
        pub h_field: Option<String>,
        #[serde(rename = "@mesh", skip_serializing_if = "Option::is_none")]
        pub mesh: Option<String>,
        #[serde(rename = "@fitscale", skip_serializing_if = "Option::is_none")]
        pub fit_scale: Option<f64>,
        #[serde(rename = "@fluidshape", skip_serializing_if = "Option::is_none")]
        pub fluid_shape: Option<String>, // ellipsoid none
        #[serde(rename = "@fluidcoef", skip_serializing_if = "Option::is_none")]
        pub fluid_coef: Option<[f64; 5]>, //  “0.5 0.25 1.5 1.0 1.0”
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "plugin", skip_serializing_if = "Option::is_none")]
        pub plugin: Option<Plugin>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Site {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<SiteType>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<u32>,
        #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
        pub material: Option<String>,
        #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
        pub rgba: Option<[f64; 4]>, // “0.5 0.5 0.5 1”
        #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
        pub size: Option<String>, //  “0.005 0.005 0.005”
        #[serde(rename = "@fromto", skip_serializing_if = "Option::is_none")]
        pub from_to: Option<[f64; 6]>,
        #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
        pub pos: Option<[f64; 3]>,
        #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
        pub quat: Option<[f64; 4]>,
        #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
        pub axis_angle: Option<[f64; 4]>,
        #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
        pub euler: Option<[f64; 3]>,
        #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
        pub xy_axes: Option<[f64; 6]>,
        #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
        pub z_axis: Option<[f64; 3]>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Camera {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
        pub mode: Option<CameraModeType>,
        #[serde(rename = "@target", skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[serde(rename = "@fovy", skip_serializing_if = "Option::is_none")]
        pub fovy: Option<f64>, // 45
        #[serde(rename = "@resolution", skip_serializing_if = "Option::is_none")]
        pub resolution: Option<[f64; 2]>,
        #[serde(rename = "@focal", skip_serializing_if = "Option::is_none")]
        pub focal: Option<[f64; 2]>,
        #[serde(rename = "@focalpixel", skip_serializing_if = "Option::is_none")]
        pub focal_pixel: Option<[f64; 2]>,
        #[serde(rename = "@principal", skip_serializing_if = "Option::is_none")]
        pub principal: Option<[f64; 2]>,
        #[serde(rename = "@principalpixel", skip_serializing_if = "Option::is_none")]
        pub principal_pixel: Option<[f64; 2]>,
        #[serde(rename = "@sensorsize", skip_serializing_if = "Option::is_none")]
        pub sensor_size: Option<[f64; 2]>,

        #[serde(rename = "@ipd", skip_serializing_if = "Option::is_none")]
        pub ipd: Option<f64>,
        #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
        pub pos: Option<[f64; 3]>,
        #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
        pub quat: Option<[f64; 4]>,
        #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
        pub axis_angle: Option<[f64; 4]>,
        #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
        pub euler: Option<[f64; 3]>,
        #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
        pub xy_axes: Option<[f64; 6]>,
        #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
        pub z_axis: Option<[f64; 3]>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Light {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>, //  [fixed, track, trackcom, targetbody, targetbodycom], “fixed”
        #[serde(rename = "@target", skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[serde(rename = "@directional", skip_serializing_if = "Option::is_none")]
        pub directional: Option<bool>,
        #[serde(rename = "@castshadow", skip_serializing_if = "Option::is_none")]
        pub cast_shadow: Option<bool>,
        #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
        pub radius: Option<f64>, // 0.02
        #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
        #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
        pub pos: Option<[f64; 3]>, // 0 0 0
        #[serde(rename = "@dir", skip_serializing_if = "Option::is_none")]
        pub dir: Option<[f64; 3]>, //  “0 0 -1”
        #[serde(rename = "@attenuation", skip_serializing_if = "Option::is_none")]
        pub attenuation: Option<[f64; 3]>, // 1 0 0
        #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
        pub cutoff: Option<f64>, // 45
        #[serde(rename = "@exponent", skip_serializing_if = "Option::is_none")]
        pub exponent: Option<f64>, // 10
        #[serde(rename = "@ambient", skip_serializing_if = "Option::is_none")]
        pub ambient: Option<[f64; 3]>, // 0 0 0
        #[serde(rename = "@diffuse", skip_serializing_if = "Option::is_none")]
        pub diffuse: Option<[f64; 3]>, // 0.7 0.7 0.7
        #[serde(rename = "@specular", skip_serializing_if = "Option::is_none")]
        pub specular: Option<[f64; 3]>, // 0.3 0.3 0.3
    }

    pub type Plugin = util::Plugin;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Composite {
        #[serde(rename = "@prefix", skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String>,
        #[serde(rename = "@type")]
        pub r#type: CompositeType,
        #[serde(rename = "@count")]
        pub count: [i32; 3],
        #[serde(rename = "@spacing")]
        pub spacing: f64,
        #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
        pub offset: Option<[f64; 3]>, // “0 0 0”
        #[serde(rename = "@flatinertia", skip_serializing_if = "Option::is_none")]
        pub flat_inertia: Option<f64>,
        #[serde(rename = "@solimpsmooth", skip_serializing_if = "Option::is_none")]
        pub sol_imp_smooth: Option<[f64; 2]>,
        #[serde(rename = "@solrefsmooth", skip_serializing_if = "Option::is_none")]
        pub sol_ref_smooth: Option<[f64; 2]>,

        #[serde(rename = "@vertex", skip_serializing_if = "Option::is_none")]
        pub vertex: Option<Vec<f64>>,
        #[serde(rename = "@face", skip_serializing_if = "Option::is_none")]
        pub face: Option<Vec<f64>>,
        #[serde(rename = "@initial", skip_serializing_if = "Option::is_none")]
        pub initial: Option<String>, // [free, ball, none]
        #[serde(rename = "@flatinertia", skip_serializing_if = "Option::is_none")]
        pub curve: Option<[String; 3]>,
        #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
        pub size: Option<[u32; 3]>,

        // element
        #[serde(rename = "joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<Vec<composite::Joint>>,
        #[serde(rename = "tendon", skip_serializing_if = "Option::is_none")]
        pub tendon: Option<Vec<composite::Tendon>>,
        #[serde(rename = "geom", skip_serializing_if = "Option::is_none")]
        pub geom: Option<composite::Geom>,
        #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
        pub site: Option<composite::Site>,
        #[serde(rename = "skin", skip_serializing_if = "Option::is_none")]
        pub skin: Option<composite::Skin>,
        #[serde(rename = "pin", skip_serializing_if = "Option::is_none")]
        pub pin: Option<Vec<composite::Pin>>,
        #[serde(rename = "plugin", skip_serializing_if = "Option::is_none")]
        pub plugin: Option<composite::Plugin>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct FlexComp {
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@dim", skip_serializing_if = "Option::is_none")]
        pub dim: Option<u32>, // 2
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<flex_comp::FlexCompType>,
        #[serde(rename = "@count", skip_serializing_if = "Option::is_none")]
        pub count: Option<[u32; 3]>, // 10 10 10
        #[serde(rename = "@spacing", skip_serializing_if = "Option::is_none")]
        pub spacing: Option<[f64; 3]>, // 0.02 0.02 0.02
        #[serde(rename = "@point", skip_serializing_if = "Option::is_none")]
        pub point: Option<Vec<f64>>,
        #[serde(rename = "@point", skip_serializing_if = "Option::is_none")]
        pub element: Option<Vec<u32>>,
        #[serde(rename = "@point", skip_serializing_if = "Option::is_none")]
        pub texcoord: Option<Vec<f64>>,
        #[serde(rename = "@point", skip_serializing_if = "Option::is_none")]
        pub mass: Option<u32>, // 1
        #[serde(rename = "@inertiabox", skip_serializing_if = "Option::is_none")]
        pub inertiabox: Option<u32>, // 0.005
        #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
        pub file: Option<String>,
        #[serde(rename = "@rigid", skip_serializing_if = "Option::is_none")]
        pub rigid: Option<bool>,
        #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
        pub pos: Option<[f64; 3]>,
        #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
        pub quat: Option<[f64; 4]>,
        #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
        pub axis_angle: Option<[f64; 4]>,
        #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
        pub euler: Option<[f64; 3]>,
        #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
        pub xy_axes: Option<[f64; 6]>,
        #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
        pub z_axis: Option<[f64; 3]>,
        #[serde(rename = "@scale", skip_serializing_if = "Option::is_none")]
        pub scale: Option<[f64; 3]>, // 1 1 1
        #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
        pub radius: Option<f64>, // 0.005
        #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
        pub material: Option<String>,
        #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
        pub rgba: Option<[f64; 4]>, // 0.5 0.5 0.5 1
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<u32>, // 0
        #[serde(rename = "@flatskin", skip_serializing_if = "Option::is_none")]
        pub flat_skin: Option<bool>,

        // element
        #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
        pub contact: Option<flex_comp::Contact>,
        #[serde(rename = "edge", skip_serializing_if = "Option::is_none")]
        pub edge: Option<flex_comp::Edge>,
        #[serde(rename = "pin", skip_serializing_if = "Option::is_none")]
        pub pin: Option<flex_comp::Pin>,
        #[serde(rename = "plugin", skip_serializing_if = "Option::is_none")]
        pub plugin: Option<flex_comp::Plugin>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Frame {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@childclass", skip_serializing_if = "Option::is_none")]
        pub child_class: Option<String>,
        #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
        pub pos: Option<[f64; 3]>,
        #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
        pub quat: Option<[f64; 4]>,
        #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
        pub axis_angle: Option<[f64; 4]>,
        #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
        pub euler: Option<[f64; 3]>,
        #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
        pub xy_axes: Option<[f64; 6]>,
        #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
        pub z_axis: Option<[f64; 3]>,
    }

    pub mod composite {
        use serde::{Deserialize, Serialize};

        use crate::mjcf::util;
        use crate::mjcf::util::{GeomType, JointKindType, JointType, TendonKindType};

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Joint {
            #[serde(rename = "@kind")]
            pub kind: JointKindType,
            #[serde(rename = "@solimpfix", skip_serializing_if = "Option::is_none")]
            pub sol_imp_fix: Option<[f64; 2]>,
            #[serde(rename = "@solreffix", skip_serializing_if = "Option::is_none")]
            pub sol_ref_fix: Option<[f64; 2]>,
            #[serde(rename = "@axis", skip_serializing_if = "Option::is_none")]
            pub axis: Option<String>,
            #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
            pub group: Option<u32>,
            #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
            pub stiffness: Option<f64>,
            #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
            pub damping: Option<f64>,
            #[serde(rename = "@armature", skip_serializing_if = "Option::is_none")]
            pub armature: Option<f64>,
            #[serde(rename = "@limited", skip_serializing_if = "Option::is_none")]
            pub limited: Option<bool>,
            #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
            pub range: Option<[f64; 2]>,
            #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
            pub margin: Option<f64>,
            #[serde(rename = "@solreflimit", skip_serializing_if = "Option::is_none")]
            pub sol_ref_limit: Option<[f64; 2]>,
            #[serde(rename = "@solimplimit", skip_serializing_if = "Option::is_none")]
            pub sol_imp_limit: Option<[f64; 2]>,
            #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
            pub friction_loss: Option<f64>,
            #[serde(rename = "@solreffriction", skip_serializing_if = "Option::is_none")]
            pub sol_ref_friction: Option<[f64; 2]>,
            #[serde(rename = "@solimpfriction", skip_serializing_if = "Option::is_none")]
            pub sol_imp_friction: Option<[f64; 2]>,
            #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
            pub r#type: Option<JointType>,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Tendon {
            #[serde(rename = "@kind")]
            pub kind: TendonKindType,
            #[serde(rename = "@solimpfix", skip_serializing_if = "Option::is_none")]
            pub sol_imp_fix: Option<[f64; 2]>,
            #[serde(rename = "@solreffix", skip_serializing_if = "Option::is_none")]
            pub sol_ref_fix: Option<[f64; 2]>,
            #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
            pub group: Option<u32>,
            #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
            pub stiffness: Option<f64>,
            #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
            pub damping: Option<f64>,
            #[serde(rename = "@limited", skip_serializing_if = "Option::is_none")]
            pub limited: Option<bool>,
            #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
            pub range: Option<[f64; 2]>,
            #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
            pub margin: Option<f64>,
            #[serde(rename = "@solimplimit", skip_serializing_if = "Option::is_none")]
            pub sol_imp_limit: Option<[f64; 2]>,
            #[serde(rename = "@solreflimit", skip_serializing_if = "Option::is_none")]
            pub sol_ref_limit: Option<[f64; 2]>,
            #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
            pub friction_loss: Option<f64>,
            #[serde(rename = "@solimpfriction", skip_serializing_if = "Option::is_none")]
            pub sol_imp_friction: Option<f64>,
            #[serde(rename = "@solreffriction", skip_serializing_if = "Option::is_none")]
            pub sol_ref_friction: Option<[f64; 2]>,
            #[serde(rename = "@material")]
            pub material: String,
            #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
            pub rgba: Option<[f64; 4]>,
            #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
            pub width: Option<f64>,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Geom {
            #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
            pub r#type: Option<GeomType>,
            #[serde(rename = "@contype", skip_serializing_if = "Option::is_none")]
            pub con_type: Option<u32>, // 1
            #[serde(rename = "@conaffinity", skip_serializing_if = "Option::is_none")]
            pub con_affinity: Option<u32>, // 1
            #[serde(rename = "@condim", skip_serializing_if = "Option::is_none")]
            pub con_dim: Option<u32>, // 3
            #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
            pub group: Option<u32>, // 0
            #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
            pub priority: Option<u32>, // 0
            #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
            pub size: Option<[f64; 3]>, // 0 0 0
            #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
            pub material: Option<String>,
            #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
            pub rgba: Option<[f64; 4]>, // 0.5 0.5 0.5 1
            #[serde(rename = "@friction", skip_serializing_if = "Option::is_none")]
            pub friction: Option<[f64; 3]>, // 1 0.005 0.0001
            #[serde(rename = "@mass", skip_serializing_if = "Option::is_none")]
            pub mass: Option<f64>,
            #[serde(rename = "@density", skip_serializing_if = "Option::is_none")]
            pub density: Option<f64>, // 1000
            #[serde(rename = "@solmix", skip_serializing_if = "Option::is_none")]
            pub sol_mix: Option<f64>,
            #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
            pub sol_ref: Option<[f64; 2]>,
            #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
            pub sol_imp: Option<[f64; 2]>,
            #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
            pub margin: Option<f64>,
            #[serde(rename = "@gap", skip_serializing_if = "Option::is_none")]
            pub gap: Option<f64>,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Site {
            #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
            pub group: Option<u32>,
            #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
            pub size: Option<[f64; 3]>,
            #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
            pub material: Option<String>,
            #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
            pub rgba: Option<[f64; 4]>,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Skin {
            #[serde(rename = "@texcoord", skip_serializing_if = "Option::is_none")]
            pub texcoord: Option<bool>,
            #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
            pub material: Option<String>,
            #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
            pub rgba: Option<[f64; 4]>,
            #[serde(rename = "@inflate", skip_serializing_if = "Option::is_none")]
            pub inflate: Option<f64>,
            #[serde(rename = "@subgrid", skip_serializing_if = "Option::is_none")]
            pub subgrid: Option<u32>,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Pin {
            #[serde(rename = "@coord")]
            pub coord: [f64; 2],
        }

        pub type Plugin = util::Plugin;
    }

    pub mod flex_comp {
        use serde::{Deserialize, Serialize};

        use crate::mjcf::util;

        #[derive(Deserialize, Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub enum FlexCompType {
            Grid,
            r#Box,
            Cylinder,
            Ellipsoid,
            Mesh,
            Gmsh,
            Direct,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Contact {
            #[serde(rename = "@internal", skip_serializing_if = "Option::is_none")]
            pub internal: Option<bool>,
            #[serde(rename = "@selfcollide", skip_serializing_if = "Option::is_none")]
            pub self_collide: Option<String>, // [none, narrow, bvh, sap, auto]
            #[serde(rename = "@activelayers", skip_serializing_if = "Option::is_none")]
            pub active_layers: Option<u32>, // 1
            #[serde(rename = "@contype", skip_serializing_if = "Option::is_none")]
            pub con_type: Option<u32>, // 1
            #[serde(rename = "@conaffinity", skip_serializing_if = "Option::is_none")]
            pub con_affinity: Option<u32>, // 1
            #[serde(rename = "@condim", skip_serializing_if = "Option::is_none")]
            pub con_dim: Option<u32>, // 3
            #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
            pub priority: Option<u32>, // 0
            #[serde(rename = "@friction", skip_serializing_if = "Option::is_none")]
            pub friction: Option<[f64; 3]>,
            #[serde(rename = "@solmix", skip_serializing_if = "Option::is_none")]
            pub sol_mix: Option<f64>, // 1
            #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
            pub sol_imp: Option<f64>,
            #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
            pub margin: Option<f64>, // 0
            #[serde(rename = "@gap", skip_serializing_if = "Option::is_none")]
            pub gap: Option<f64>, // 0
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Edge {
            #[serde(rename = "@equality", skip_serializing_if = "Option::is_none")]
            pub equality: Option<bool>, // 0
            #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
            pub sol_ref: Option<f64>,
            #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
            pub sol_imp: Option<f64>,
            #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
            pub stiffness: Option<f64>, // 0
            #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
            pub damping: Option<f64>, // 0
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Pin {
            #[serde(rename = "@id")]
            pub id: i32,
            #[serde(rename = "@range")]
            pub range: Vec<u32>,
            #[serde(rename = "@grid")]
            pub grid: Vec<u32>,
            #[serde(rename = "@gridrange")]
            pub grid_range: Vec<u32>,
        }

        pub type Plugin = util::Plugin;
    }
}
