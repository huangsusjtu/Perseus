use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Asset {
    #[serde(rename = "mesh", skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Vec<asset::Mesh>>,

    #[serde(rename = "hfield", skip_serializing_if = "Option::is_none")]
    pub hfield: Option<Vec<asset::HField>>,

    #[serde(rename = "texture", skip_serializing_if = "Option::is_none")]
    pub texture: Option<Vec<asset::Texture>>,

    #[serde(rename = "material", skip_serializing_if = "Option::is_none")]
    pub material: Option<Vec<asset::Material>>,

    #[serde(rename = "plugin", skip_serializing_if = "Option::is_none")]
    pub plugin: Option<asset::Plugin>,
}


pub mod asset {
    use serde::{Deserialize, Serialize};

    use crate::mjcf::util;
    use crate::mjcf::util::{BuiltinType, MarkType, TextureType};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Mesh {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
        pub file: Option<String>,
        #[serde(rename = "@scale", skip_serializing_if = "Option::is_none")]
        pub scale: Option<[f64; 3]>,
        #[serde(rename = "@smoothnormal", skip_serializing_if = "Option::is_none")]
        pub smooth_normal: Option<bool>,
        #[serde(rename = "@vertex", skip_serializing_if = "Option::is_none")]
        pub vertex: Option<Vec<[f64; 3]>>,
        #[serde(rename = "@normal", skip_serializing_if = "Option::is_none")]
        pub normal: Option<Vec<[f64; 3]>>,
        #[serde(rename = "@texcoord", skip_serializing_if = "Option::is_none")]
        pub texcoord: Option<Vec<[f64; 2]>>,
        #[serde(rename = "@face", skip_serializing_if = "Option::is_none")]
        pub face: Option<Vec<[f64; 3]>>,
        #[serde(rename = "@refpos", skip_serializing_if = "Option::is_none")]
        pub ref_pos: Option<[f64; 3]>,
        #[serde(rename = "@refquat", skip_serializing_if = "Option::is_none")]
        pub ref_quat: Option<[f64; 4]>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Texture {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<TextureType>,
        #[serde(rename = "@content_type", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,

        #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
        pub file: Option<String>,
        #[serde(rename = "@gridsize", skip_serializing_if = "Option::is_none")]
        pub grid_size: Option<[f64; 2]>,
        #[serde(rename = "@gridlayout", skip_serializing_if = "Option::is_none")]
        pub gridlayout: Option<String>,
        #[serde(rename = "@fileright", skip_serializing_if = "Option::is_none")]
        pub file_right: Option<String>,
        #[serde(rename = "@fileleft", skip_serializing_if = "Option::is_none")]
        pub file_left: Option<String>,
        #[serde(rename = "@fileup", skip_serializing_if = "Option::is_none")]
        pub file_up: Option<String>,
        #[serde(rename = "@filedown", skip_serializing_if = "Option::is_none")]
        pub file_down: Option<String>,
        #[serde(rename = "@filefront", skip_serializing_if = "Option::is_none")]
        pub file_front: Option<String>,
        #[serde(rename = "@fileback", skip_serializing_if = "Option::is_none")]
        pub file_back: Option<String>,

        #[serde(rename = "@builtin", skip_serializing_if = "Option::is_none")]
        pub builtin: Option<BuiltinType>,
        #[serde(rename = "@rgb1", skip_serializing_if = "Option::is_none")]
        pub rgb1: Option<[f64; 3]>,
        #[serde(rename = "@rgb2", skip_serializing_if = "Option::is_none")]
        pub rgb2: Option<[f64; 3]>,
        #[serde(rename = "@mark", skip_serializing_if = "Option::is_none")]
        pub mark: Option<MarkType>,
        #[serde(rename = "@markrgb", skip_serializing_if = "Option::is_none")]
        pub mark_rgb: Option<[f64; 3]>,
        #[serde(rename = "@random", skip_serializing_if = "Option::is_none")]
        pub random: Option<f64>,
        #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
        pub width: Option<i32>,
        #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
        pub height: Option<i32>,
        #[serde(rename = "@hflip", skip_serializing_if = "Option::is_none")]
        pub h_flip: Option<bool>,
        #[serde(rename = "@vflip", skip_serializing_if = "Option::is_none")]
        pub v_flip: Option<bool>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct HField {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@content_type", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
        pub file: Option<String>,
        #[serde(rename = "@nrow", skip_serializing_if = "Option::is_none")]
        pub n_row: Option<i32>,
        #[serde(rename = "@ncol", skip_serializing_if = "Option::is_none")]
        pub n_col: Option<i32>,
        #[serde(rename = "@elevation", skip_serializing_if = "Option::is_none")]
        pub elevation: Option<Vec<f64>>,
        #[serde(rename = "@size")]
        pub size: Option<[f64; 4]>,
    }


    pub type Plugin = util::Plugin;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Material {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@texture", skip_serializing_if = "Option::is_none")]
        pub texture: Option<String>,
        #[serde(rename = "@texrepeat", skip_serializing_if = "Option::is_none")]
        pub tex_repeat: Option<[f64; 2]>,
        #[serde(rename = "@texuniform", skip_serializing_if = "Option::is_none")]
        pub tex_uniform: Option<bool>,
        #[serde(rename = "@emission", skip_serializing_if = "Option::is_none")]
        pub emission: Option<f64>,
        #[serde(rename = "@specular", skip_serializing_if = "Option::is_none")]
        pub specular: Option<f64>,
        #[serde(rename = "@shininess", skip_serializing_if = "Option::is_none")]
        pub shininess: Option<f64>,
        #[serde(rename = "@reflectance", skip_serializing_if = "Option::is_none")]
        pub reflectance: Option<f64>,
        #[serde(rename = "@roughness", skip_serializing_if = "Option::is_none")]
        pub roughness: Option<f64>,
        #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
        pub rgba: Option<[f64; 4]>,
    }
}