use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Visual {
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<visual::Global>,

    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<visual::Quality>,
    #[serde(rename = "headlight", skip_serializing_if = "Option::is_none")]
    pub headlight: Option<visual::Headlight>,
    #[serde(rename = "map", skip_serializing_if = "Option::is_none")]
    pub map: Option<visual::Map>,
    #[serde(rename = "scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<visual::Scale>,
    #[serde(rename = "rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<visual::Rgba>,
}

pub mod visual {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Global {
        #[serde(rename = "@fovy", skip_serializing_if = "Option::is_none")]
        pub fovy: Option<f64>, // 45
        #[serde(rename = "@ipd", skip_serializing_if = "Option::is_none")]
        pub ipd: Option<f64>, // 0.068
        #[serde(rename = "@azimuth", skip_serializing_if = "Option::is_none")]
        pub azimuth: Option<f64>, // 90
        #[serde(rename = "@elevation", skip_serializing_if = "Option::is_none")]
        pub elevation: Option<f64>, // -45
        #[serde(rename = "@linewidth", skip_serializing_if = "Option::is_none")]
        pub line_width: Option<f64>, // 1
        #[serde(rename = "@glow", skip_serializing_if = "Option::is_none")]
        pub glow: Option<f64>, // 0.3
        #[serde(rename = "@realtime", skip_serializing_if = "Option::is_none")]
        pub realtime: Option<u32>,
        #[serde(rename = "@offwidth", skip_serializing_if = "Option::is_none")]
        pub off_width: Option<u32>,
        #[serde(rename = "@offheight", skip_serializing_if = "Option::is_none")]
        pub off_height: Option<u32>,
        #[serde(rename = "@ellipsoidinertia", skip_serializing_if = "Option::is_none")]
        pub ellipsoid_inertia: Option<bool>,
        #[serde(rename = "@bvactive", skip_serializing_if = "Option::is_none")]
        pub bv_active: Option<bool>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Quality {
        #[serde(rename = "@shadowsize", skip_serializing_if = "Option::is_none")]
        pub shadow_size: Option<u32>, // 4096
        #[serde(rename = "@offsamples", skip_serializing_if = "Option::is_none")]
        pub off_samples: Option<u32>, // 4
        #[serde(rename = "@numslices", skip_serializing_if = "Option::is_none")]
        pub num_slices: Option<u32>, // 28
        #[serde(rename = "@numstacks", skip_serializing_if = "Option::is_none")]
        pub num_stacks: Option<u32>, // 16
        #[serde(rename = "@numquads", skip_serializing_if = "Option::is_none")]
        pub num_quads: Option<u32>, // 4
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Headlight {
        #[serde(rename = "@ambient", skip_serializing_if = "Option::is_none")]
        pub ambient: Option<[f64; 3]>, // 0.1 0.1 0.1
        #[serde(rename = "@diffuse", skip_serializing_if = "Option::is_none")]
        pub diffuse: Option<[f64; 3]>, // 0.4 0.4 0.4
        #[serde(rename = "@specular", skip_serializing_if = "Option::is_none")]
        pub specular: Option<[f64; 3]>, // 0.5 0.5 0.5
        #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
        pub active: Option<i32>, // 1
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Map {
        #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
        pub stiffness: Option<f64>, // 100
        #[serde(rename = "@stiffnessrot", skip_serializing_if = "Option::is_none")]
        pub stiff_ness_rot: Option<f64>, // 500
        #[serde(rename = "@force", skip_serializing_if = "Option::is_none")]
        pub force: Option<f64>, // 0.005
        #[serde(rename = "@torque", skip_serializing_if = "Option::is_none")]
        pub torque: Option<f64>, // 0.1
        #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
        pub alpha: Option<f64>, // 0.3
        #[serde(rename = "@fogstart", skip_serializing_if = "Option::is_none")]
        pub fog_start: Option<f64>, // 3
        #[serde(rename = "@fogend", skip_serializing_if = "Option::is_none")]
        pub fog_end: Option<f64>, // 10
        #[serde(rename = "@znear", skip_serializing_if = "Option::is_none")]
        pub z_near: Option<f64>, // 0.01
        #[serde(rename = "@zfar", skip_serializing_if = "Option::is_none")]
        pub z_far: Option<f64>, // 50
        #[serde(rename = "@haze", skip_serializing_if = "Option::is_none")]
        pub haze: Option<f64>, // 0.3
        #[serde(rename = "@shadowclip", skip_serializing_if = "Option::is_none")]
        pub shadow_clip: Option<f64>, // 1
        #[serde(rename = "@shadowscale", skip_serializing_if = "Option::is_none")]
        pub shadow_scale: Option<f64>, // 0.6
        #[serde(rename = "@actuatortendon", skip_serializing_if = "Option::is_none")]
        pub actuator_tendon: Option<f64>, // 2
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Scale {
        #[serde(rename = "@forcewidth", skip_serializing_if = "Option::is_none")]
        pub force_width: Option<f64>, // 0.1
        #[serde(rename = "@contactwidth", skip_serializing_if = "Option::is_none")]
        pub contact_width: Option<f64>, // 0.3
        #[serde(rename = "@contactheight", skip_serializing_if = "Option::is_none")]
        pub contact_height: Option<f64>, // 0.1
        #[serde(rename = "@connect", skip_serializing_if = "Option::is_none")]
        pub connect: Option<f64>, // 0.2
        #[serde(rename = "@com", skip_serializing_if = "Option::is_none")]
        pub com: Option<f64>, // 0.4
        #[serde(rename = "@camera", skip_serializing_if = "Option::is_none")]
        pub camera: Option<f64>, // 0.3
        #[serde(rename = "@light", skip_serializing_if = "Option::is_none")]
        pub light: Option<f64>, // 0.3
        #[serde(rename = "@selectpoint", skip_serializing_if = "Option::is_none")]
        pub select_point: Option<f64>, // 0.2
        #[serde(rename = "@jointlength", skip_serializing_if = "Option::is_none")]
        pub joint_length: Option<f64>, // 1.0
        #[serde(rename = "@jointwidth", skip_serializing_if = "Option::is_none")]
        pub joint_width: Option<f64>, // 0.1
        #[serde(rename = "@actuatorlength", skip_serializing_if = "Option::is_none")]
        pub actuator_length: Option<f64>, // 0.7
        #[serde(rename = "@actuatorwidth", skip_serializing_if = "Option::is_none")]
        pub actuator_width: Option<f64>, // 0.2
        #[serde(rename = "@framelength", skip_serializing_if = "Option::is_none")]
        pub frame_length: Option<f64>, // 1.0
        #[serde(rename = "@framewidth", skip_serializing_if = "Option::is_none")]
        pub frame_width: Option<f64>, // 0.1
        #[serde(rename = "@constraint", skip_serializing_if = "Option::is_none")]
        pub constraint: Option<f64>, // 0.1
        #[serde(rename = "@slidercrank", skip_serializing_if = "Option::is_none")]
        pub slider_crank: Option<f64>, // 0.2
        #[serde(rename = "@frustum", skip_serializing_if = "Option::is_none")]
        pub frustum: Option<f64>, // 10
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Rgba {
        #[serde(rename = "@fog", skip_serializing_if = "Option::is_none")]
        pub fog: Option<[f64; 4]>, // 0 0 0 1
        #[serde(rename = "@haze", skip_serializing_if = "Option::is_none")]
        pub haze: Option<[f64; 4]>, // 1 1 1 1
        #[serde(rename = "@force", skip_serializing_if = "Option::is_none")]
        pub force: Option<[f64; 4]>, // 1 0.5 0.5 1
        #[serde(rename = "@inertia", skip_serializing_if = "Option::is_none")]
        pub inertia: Option<[f64; 4]>, // 0.8 0.2 0.2 0.6
        #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<[f64; 4]>, // 0.2 0.6 0.8 1
        #[serde(rename = "@actuator", skip_serializing_if = "Option::is_none")]
        pub actuator: Option<[f64; 4]>, // 0.2 0.25 0.2 1
        #[serde(rename = "@actuatornegative", skip_serializing_if = "Option::is_none")]
        pub actuator_negative: Option<[f64; 4]>, // 0.2 0.6 0.9 1
        #[serde(rename = "@actuatorpositive", skip_serializing_if = "Option::is_none")]
        pub actuator_positive: Option<[f64; 4]>, // 0.9 0.4 0.2 1
        #[serde(rename = "@com", skip_serializing_if = "Option::is_none")]
        pub com: Option<[f64; 4]>, // 0.9 0.9 0.9 1
        #[serde(rename = "@camera", skip_serializing_if = "Option::is_none")]
        pub camera: Option<[f64; 4]>, // 0.6 0.9 0.6 1
        #[serde(rename = "@light", skip_serializing_if = "Option::is_none")]
        pub light: Option<[f64; 4]>, // 0.6 0.6 0.9 1
        #[serde(rename = "@selectpoint", skip_serializing_if = "Option::is_none")]
        pub select_point: Option<[f64; 4]>, // 0.9 0.9 0.1 1
        #[serde(rename = "@connect", skip_serializing_if = "Option::is_none")]
        pub connect: Option<[f64; 4]>, // 0.2 0.2 0.8 1
        #[serde(rename = "@contactpoint", skip_serializing_if = "Option::is_none")]
        pub contact_point: Option<[f64; 4]>, // 0.9 0.6 0.2 1
        #[serde(rename = "@contactforce", skip_serializing_if = "Option::is_none")]
        pub contact_force: Option<[f64; 4]>, // 0.7 0.9 0.9 1
        #[serde(rename = "@contactfriction", skip_serializing_if = "Option::is_none")]
        pub contact_friction: Option<[f64; 4]>, // 0.9 0.8 0.4 1
        #[serde(rename = "@contacttorque", skip_serializing_if = "Option::is_none")]
        pub contact_torque: Option<[f64; 4]>, // 0.9 0.7 0.9 1
        #[serde(rename = "@contactgap", skip_serializing_if = "Option::is_none")]
        pub contact_gap: Option<[f64; 4]>, // 0.5, 0.8, 0.9, 1
        #[serde(rename = "@rangefinder", skip_serializing_if = "Option::is_none")]
        pub rangefinder: Option<[f64; 4]>, // 1 1 0.1 1
        #[serde(rename = "@constraint", skip_serializing_if = "Option::is_none")]
        pub constraint: Option<[f64; 4]>, // 0.9 0 0 1
        #[serde(rename = "@slidercrank", skip_serializing_if = "Option::is_none")]
        pub slider_crank: Option<[f64; 4]>, // 0.5 0.3 0.8 1
        #[serde(rename = "@crankbroken", skip_serializing_if = "Option::is_none")]
        pub crank_broken: Option<[f64; 4]>, // 0.9 0 0 1
        #[serde(rename = "@frustum", skip_serializing_if = "Option::is_none")]
        pub frustum: Option<[f64; 4]>, // 1 1 0 0.2
        #[serde(rename = "@bv", skip_serializing_if = "Option::is_none")]
        pub bv: Option<[f64; 4]>, // 0 1 0 0.5
        #[serde(rename = "@bvactive", skip_serializing_if = "Option::is_none")]
        pub bv_active: Option<[f64; 4]>, // 1 0 0 0.5
    }
}