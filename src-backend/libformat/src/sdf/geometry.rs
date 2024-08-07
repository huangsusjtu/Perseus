/// align to doc: http://sdformat.org/spec?ver=1.11&elem=geometry
/// finished test
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Geometry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#box: Option<geometry::Box>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capsule: Option<geometry::Capsule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cylinder: Option<geometry::Cylinder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsoid: Option<geometry::Ellipsoid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heightmap: Option<geometry::HeightMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<geometry::Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<geometry::Mesh>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plane: Option<geometry::Plane>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline: Option<geometry::Polyline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sphere: Option<geometry::Sphere>,
}

pub mod geometry {
    use serde::{Deserialize, Serialize};

    use crate::sdf::util::{Vector2, Vector3};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Box {
        pub size: Vector3<f64>, // Default: 1 1 1
    }

    impl Default for Box {
        fn default() -> Self {
            Box {
                size: Vector3 {
                    content: [1.0, 1.0, 1.0],
                },
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Capsule {
        pub radius: f64, // Default: 0.5
        pub length: f64, // Default: 1.0
    }

    impl Default for Capsule {
        fn default() -> Self {
            Capsule {
                radius: 0.5,
                length: 1.0,
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Cylinder {
        pub radius: f64, // Default: 1.0
        pub length: f64, // Default: 1.0
    }

    impl Default for Cylinder {
        fn default() -> Self {
            Cylinder {
                radius: 1.0,
                length: 1.0,
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Ellipsoid {
        pub radii: Vector3<f64>, // Default: 1 1 1
    }

    impl Default for Ellipsoid {
        fn default() -> Self {
            Ellipsoid {
                radii: Vector3 {
                    content: [1.0, 1.0, 1.0],
                },
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct HeightMap {
        pub uri: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<Vector3<f64>>, // Default: 1 1 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pos: Option<Vector3<f64>>, // Default: 0 0 0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub texture: Option<Vec<height_map::Texture>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub blend: Option<Vec<height_map::Blend>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_terrain_paging: Option<bool>, // false
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sampling: Option<u32>, // Default: 1
    }

    impl Default for HeightMap {
        fn default() -> Self {
            HeightMap {
                uri: "".to_string(),
                size: Some(Vector3 {
                    content: [1.0, 1.0, 1.0],
                }),
                pos: Some(Vector3::default()),
                texture: None,
                blend: None,
                use_terrain_paging: Some(false),
                sampling: Some(1),
            }
        }
    }

    pub mod height_map {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Texture {
            pub size: f64,       // 10
            pub diffuse: String, // no default
            pub normal: String,  // no default
        }

        impl Default for Texture {
            fn default() -> Self {
                Texture {
                    size: 10.0,
                    diffuse: "".to_string(),
                    normal: "".to_string(),
                }
            }
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Blend {
            pub min_height: f64, // 0
            pub fade_dist: f64,  // 0
        }

        impl Default for Blend {
            fn default() -> Self {
                Blend {
                    min_height: 0.0,
                    fade_dist: 0.0,
                }
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Image {
        pub uri: String,
        pub scale: f64,       // Default: 1
        pub threshold: u32,   // Default: 200
        pub height: f64,      // 1
        pub granularity: u32, // 1
    }

    impl Default for Image {
        fn default() -> Self {
            Image {
                uri: "".to_string(),
                scale: 1.0,
                threshold: 200,
                height: 1.0,
                granularity: 1,
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Mesh {
        pub uri: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub submesh: Option<mesh::SubMesh>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scale: Option<Vector3<f64>>, // 1 1 1
    }

    impl Default for Mesh {
        fn default() -> Self {
            Mesh {
                uri: "".to_string(),
                submesh: None,
                scale: Some(Vector3 {
                    content: [1.0, 1.0, 1.0],
                }),
            }
        }
    }

    pub mod mesh {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, PartialEq, Debug, Default)]
        pub struct SubMesh {
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub center: Option<bool>, // false
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Plane {
        pub normal: Vector3<f64>, // 0 0 1
        pub size: Vector2<f64>,   // 1 1
    }

    impl Default for Plane {
        fn default() -> Self {
            Plane {
                normal: Vector3 {
                    content: [0.0, 0.0, 1.0],
                },
                size: Vector2 {
                    content: [1.0, 1.0],
                },
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Polyline {
        pub point: Vec<Vector2<f64>>,
        pub height: f64, // 1
    }

    impl Default for Polyline {
        fn default() -> Self {
            Polyline {
                point: vec![Vector2::default()],
                height: 1.0,
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Sphere {
        pub radius: f64, // 1.0
    }

    impl Default for Sphere {
        fn default() -> Self {
            Sphere { radius: 1.0 }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_geometry1() {
        let g = Geometry {
            empty: None,
            r#box: None,
            capsule: None,
            cylinder: None,
            ellipsoid: None,
            heightmap: None,
            image: None,
            mesh: None,
            plane: None,
            polyline: None,
            sphere: None,
        };
        let xml = quick_xml::se::to_string(&g).unwrap();
        println!("{:#?}", xml);
        assert_eq!("<Geometry/>", xml);
    }

    #[test]
    fn it_works_geometry2() {
        let g = Geometry {
            empty: None,
            r#box: Some(geometry::Box::default()),
            capsule: Some(geometry::Capsule::default()),
            cylinder: Some(geometry::Cylinder::default()),
            ellipsoid: Some(geometry::Ellipsoid::default()),
            heightmap: Some(geometry::HeightMap::default()),
            image: Some(geometry::Image::default()),
            mesh: Some(geometry::Mesh::default()),
            plane: Some(geometry::Plane::default()),
            polyline: Some(geometry::Polyline::default()),
            sphere: Some(geometry::Sphere::default()),
        };
        let xml = quick_xml::se::to_string(&g).unwrap();
        println!("{:#?}", xml);
        assert_eq!(
            r#"<Geometry><box><size>1 1 1</size></box><capsule><radius>0.5</radius><length>1</length></capsule><cylinder><radius>1</radius><length>1</length></cylinder><ellipsoid><radii>1 1 1</radii></ellipsoid><heightmap><uri/><size>1 1 1</size><pos>0 0 0</pos><use_terrain_paging>false</use_terrain_paging><sampling>1</sampling></heightmap><image><uri/><scale>1</scale><threshold>200</threshold><height>1</height><granularity>1</granularity></image><mesh><uri/><scale>1 1 1</scale></mesh><plane><normal>0 0 1</normal><size>1 1</size></plane><polyline><point>0 0</point><height>1</height></polyline><sphere><radius>1</radius></sphere></Geometry>"#,
            xml
        );
    }

    #[test]
    fn it_works_geometry3() {
        let g = Geometry {
            empty: None,
            r#box: Some(geometry::Box::default()),
            capsule: Some(geometry::Capsule::default()),
            cylinder: Some(geometry::Cylinder::default()),
            ellipsoid: Some(geometry::Ellipsoid::default()),
            heightmap: Some(geometry::HeightMap::default()),
            image: Some(geometry::Image::default()),
            mesh: Some(geometry::Mesh::default()),
            plane: Some(geometry::Plane::default()),
            polyline: Some(geometry::Polyline::default()),
            sphere: Some(geometry::Sphere::default()),
        };
        let s = r#"
                <Geometry>
                    <box>
                        <size>1 1 1</size>
                    </box>
                    <capsule>
                        <radius>0.5</radius>
                        <length>1</length>
                    </capsule>
                    <cylinder>
                        <radius>1</radius>
                        <length>1</length>
                    </cylinder>
                    <ellipsoid>
                        <radii>1 1 1</radii>
                    </ellipsoid>
                    <heightmap>
                        <uri/>
                        <size>1 1 1</size>
                        <pos>0 0 0</pos>
                        <use_terrain_paging>false</use_terrain_paging>
                        <sampling>1</sampling>
                    </heightmap>
                    <image>
                        <uri/>
                        <scale>1</scale>
                        <threshold>200</threshold>
                        <height>1</height>
                        <granularity>1</granularity>
                    </image>
                    <mesh>
                        <uri/>
                        <scale>1 1 1</scale>
                    </mesh>
                    <plane>
                        <normal>0 0 1</normal>
                        <size>1 1</size>
                    </plane>
                    <polyline>
                        <point>0 0</point>
                        <height>1</height>
                    </polyline>
                    <sphere>
                        <radius>1</radius>
                    </sphere>
                </Geometry>

        "#;

        let g1: Geometry = quick_xml::de::from_str(&s).unwrap();
        assert_eq!(g, g1);
    }
}
