/// align to doc: http://sdformat.org/spec?ver=1.11&elem=material
/// finished test
use serde::{Deserialize, Serialize};

use crate::sdf::Color;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Material {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<material::Script>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shader: Option<material::Shader>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_order: Option<f64>, // 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lighting: Option<bool>, // true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient: Option<Color>, // Default: 0 0 0 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffuse: Option<Color>, // Default: 0 0 0 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular: Option<Color>, // Default: 0 0 0 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shininess: Option<f64>, // 0.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive: Option<Color>, // Default: 0 0 0 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_sided: Option<bool>, // false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbr: Option<material::Pbr>,
}

pub mod material {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, PartialEq, Debug, Default)]
    pub struct Script {
        pub name: String,

        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub uri: Vec<String>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Shader {
        // Description: vertex, pixel, normal_map_object_space,
        // normal_map_tangent_space Default: pixel
        #[serde(rename = "@type")]
        pub r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub normal_map: Option<String>,
    }

    impl Default for Shader {
        fn default() -> Self {
            Shader {
                r#type: "pixel".to_string(),
                normal_map: None,
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug, Default)]
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

        impl Default for Metal {
            fn default() -> Self {
                Metal {
                    albedo_map: None,
                    roughness_map: None,
                    roughness: None,
                    metalness_map: None,
                    metalness: None,
                    environment_map: None,
                    ambient_occlusion_map: None,
                    normal_map: None,
                    emissive_map: None,
                    light_map: None,
                }
            }
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct NormalMap {
            #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>, // default tangent

            #[serde(rename = "$text")]
            pub content: String,
        }

        impl Default for NormalMap {
            fn default() -> Self {
                NormalMap {
                    r#type: Some("tangent".to_string()),
                    content: "default".to_string(),
                }
            }
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct LightMap {
            #[serde(rename = "@uv_set", skip_serializing_if = "Option::is_none")]
            pub uv_set: Option<u32>, // default 0

            #[serde(rename = "$text")]
            pub content: String,
        }

        impl Default for LightMap {
            fn default() -> Self {
                LightMap {
                    uv_set: Some(0),
                    content: "default".to_string(),
                }
            }
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

#[cfg(test)]
mod tests {
    use crate::sdf::material::material::{Script, Shader};

    use super::*;

    #[test]
    fn it_works_material1() {
        let m = Material {
            script: None,
            shader: None,
            render_order: None,
            lighting: None,
            ambient: None,
            diffuse: None,
            specular: None,
            shininess: None,
            emissive: None,
            double_sided: None,
            pbr: None,
        };
        let xml = quick_xml::se::to_string(&m).unwrap();
        println!("{:#?}", xml);
        assert_eq!("<Material/>", xml);
    }

    #[test]
    fn it_works_material2() {
        let m = Material {
            script: Some(Script {
                name: "".to_string(),
                uri: vec![],
            }),
            shader: Some(Shader {
                r#type: "pixel".to_string(),
                normal_map: Some("n".to_string()),
            }),
            render_order: Some(1.0),
            lighting: Some(true),
            ambient: Some(Color {
                content: [0.0, 0.0, 1.0, 1.0],
            }),
            diffuse: Some(Color {
                content: [0.0, 1.0, 1.0, 1.0],
            }),
            specular: Some(Color {
                content: [1.0, 0.0, 1.0, 1.0],
            }),
            shininess: Some(1.0),
            emissive: Some(Color {
                content: [1.0, 0.0, 1.0, 1.0],
            }),
            double_sided: Some(false),
            pbr: Some(material::Pbr {
                metal: None,
                specular: None,
            }),
        };
        let xml = quick_xml::se::to_string(&m).unwrap();
        println!("{:#?}", xml);
        assert_eq!(
            "<Material><script><name/></script><shader \
             type=\"pixel\"><normal_map>n</normal_map></shader><render_order>1</\
             render_order><lighting>true</lighting><ambient>0 0 1 1</ambient><diffuse>0 1 1 \
             1</diffuse><specular>1 0 1 1</specular><shininess>1</shininess><emissive>1 0 1 \
             1</emissive><double_sided>false</double_sided><pbr/></Material>",
            xml
        );
    }
}
