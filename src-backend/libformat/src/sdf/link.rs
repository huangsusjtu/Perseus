/// align to doc: http://sdformat.org/spec?ver=1.11&elem=link
/// finished
use serde::{Deserialize, Serialize};

use crate::sdf::collision::Collision;
use crate::sdf::light::Light;
use crate::sdf::sensor::Sensor;
use crate::sdf::util::Pose;
use crate::sdf::visual::Visual;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Link {
    #[serde(rename = "@name")]
    pub name: String,

    // children element start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity: Option<bool>, // default true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_wind: Option<bool>, // default false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_collide: Option<bool>, // default false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinematic: Option<bool>, // default false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_be_base_link: Option<bool>, // default false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_decay: Option<link::VelocityDecay>, // default false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>, // default 0 0 0 0 0 0

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inertial: Option<link::Inertial>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub collision: Vec<Collision>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub visual: Vec<Visual>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sensor: Vec<Sensor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projector: Option<link::Projector>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audio_sink: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audio_source: Vec<link::AudioSource>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub battery: Vec<link::Battery>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub light: Vec<Light>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub particle_emitter: Vec<link::ParticleEmitter>,
}

pub mod link {
    use serde::{Deserialize, Serialize};

    use crate::sdf::Color;
    use crate::sdf::util::{Plugin, Pose, Vector3};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct VelocityDecay {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub linear: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub angular: Option<f64>, // default 0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Inertial {
        #[serde(rename = "@auto")]
        pub auto: Option<bool>, // default false

        #[serde(skip_serializing_if = "Option::is_none")]
        pub mass: Option<f64>, // default 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub density: Option<f64>, // default 1000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auto_inertia_params: Option<String>, // default

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pose: Option<Pose>, // default 000000

        #[serde(skip_serializing_if = "Option::is_none")]
        pub inertia: Option<InertialInner>, // default 000000

        #[serde(skip_serializing_if = "Option::is_none")]
        pub fluid_added_mass: Option<FluidAddedMass>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct InertialInner {
        pub ixx: f64, // 1.0
        pub ixy: f64, // 0.0
        pub ixz: f64, // 0.0
        pub iyy: f64, // 1.0
        pub iyz: f64, // 0.0
        pub izz: f64, // 1.0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct FluidAddedMass {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub xx: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub xy: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub xz: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub xp: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub xq: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub xr: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub yy: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub yz: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub yp: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub yq: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub yr: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zz: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zp: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zq: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zr: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pp: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pq: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pr: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub qq: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub qr: Option<f64>, // 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rr: Option<f64>, // 0.0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Projector {
        #[serde(rename = "@name")]
        pub name: String,

        pub texture: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fov: Option<f64>, // default 0.785
        #[serde(skip_serializing_if = "Option::is_none")]
        pub near_clip: Option<f64>, // default 0.1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub far_clip: Option<f64>, // default 10
        #[serde(skip_serializing_if = "Option::is_none")]
        pub visibility_flag: Option<u32>, // default 4294967295

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pose: Option<Pose>,

        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub plugins: Vec<Plugin>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct AudioSource {
        pub uri: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pitch: Option<f64>, // 1.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gain: Option<f64>, // 1.0

        #[serde(skip_serializing_if = "Option::is_none")]
        pub contact: Option<Contact>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#loop: Option<bool>, // false

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pose: Option<Pose>, // 0 0 0 0 0 0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Contact {
        pub collision: Vec<String>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Battery {
        #[serde(rename = "@name")]
        pub name: String,

        pub voltage: f64, // 0
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct ParticleEmitter {
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@type")]
        pub r#type: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub emitting: Option<bool>, // default true
        #[serde(skip_serializing_if = "Option::is_none")]
        pub duration: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<Vector3<f64>>, // default 1 1 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub particle_size: Option<Vector3<f64>>, // default 1 1 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lifetime: Option<f64>, // default 5
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rate: Option<f64>, // default 10
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_velocity: Option<f64>, // default 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_velocity: Option<f64>, // default 0

        #[serde(skip_serializing_if = "Option::is_none")]
        pub scale_rate: Option<f64>, // default 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color_start: Option<Color>, // default 1 1 1 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color_end: Option<Color>, // default 1 1 1 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color_range_image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub topic: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub particle_scatter_ratio: Option<f64>, // default 0.65
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pose: Option<Pose>, // 0 0 0 0 0 0

        #[serde(skip_serializing_if = "Option::is_none")]
        pub material: Option<Material>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Material {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub script: Option<Script>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shader: Option<Shader>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub render_order: Option<f64>, // 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lighting: Option<bool>, // true
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ambient: Option<Color>, // 0 0 0 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub diffuse: Option<Color>, // 0 0 0 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub specular: Option<Color>, // 0 0 0 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shininess: Option<f64>, // 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub emissive: Option<Color>, // 0 0 0 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub double_sided: Option<bool>, // false
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pbr: Option<Pbr>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Script {
        pub name: String,

        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub uri: Vec<String>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Shader {
        #[serde(rename = "@type")]
        pub r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub normal_map: Option<String>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Pbr {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metal: Option<pbr::Metal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub specular: Option<pbr::Specular>,
    }

    pub mod pbr {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Metal {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub albedo_map: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub roughness_map: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub roughness: Option<String>, // 0.5
            #[serde(skip_serializing_if = "Option::is_none")]
            pub metalness_map: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub metalness: Option<String>, // 0.5
            #[serde(skip_serializing_if = "Option::is_none")]
            pub environment_map: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ambient_occlusion_map: Option<String>,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub normal_map: Option<NormalMap>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub emissive_map: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub light_map: Option<LightMap>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct NormalMap {
            #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>, // default tangent

            #[serde(rename = "$text")]
            pub content: String,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct LightMap {
            #[serde(rename = "@uv_set", skip_serializing_if = "Option::is_none")]
            pub uv_set: Option<u32>, // default 0

            #[serde(rename = "$text")]
            pub content: String,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Specular {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub albedo_map: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub specular_map_map: Option<String>,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub glossiness_map: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub glossiness: Option<String>, // 0

            #[serde(skip_serializing_if = "Option::is_none")]
            pub environment_map: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ambient_occlusion_map: Option<String>,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub normal_map: Option<NormalMap>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub emissive_map: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub light_map: Option<LightMap>,
        }
    }
}
