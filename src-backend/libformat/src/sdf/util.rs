/// finished test
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Default)]
pub struct Vector2<T> {
    #[serde(rename = "$text")]
    pub content: [T; 2],
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 { content: [x, y] }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default)]
pub struct Vector3<T> {
    #[serde(rename = "$text")]
    pub content: [T; 3],
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { content: [x, y, z] }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default)]
pub struct Vector4<T> {
    #[serde(rename = "$text")]
    pub content: [T; 4],
}

impl<T> Vector4<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Vector4 {
            content: [a, b, c, d],
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default)]
pub struct Vector6<T> {
    #[serde(rename = "$text")]
    pub content: [T; 6],
}

/// Description: Description: A pose (translation, rotation) expressed in the
/// frame named by @relative_to. The first three components (x, y, z) represent
/// the position of the element's origin (in the @relative_to frame). The
/// rotation component represents the orientation of the element as either a sequence of Euler rotations (r, p, y), see http://sdformat.org/tutorials?tut=specify_pose,
/// or as a quaternion (x, y, z, w), where w is the real component.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Pose {
    // attribute start
    // Description:If specified, this pose is expressed in the named frame.
    // The named frame must be declared within the same scope (world/model) as
    // the element that has its pose specified by this tag.
    // If missing, the pose is expressed in the frame of the parent XML element
    // of the element that contains the pose. For exceptions to this rule
    // and more details on the default behavior, see http://sdformat.org/tutorials?tut=pose_frame_semantics. Note that @relative_to merely affects an element's initial pose and does not affect the element's dynamic movement thereafter. New in v1.8: @relative_to may use frames of nested scopes. In this case, the frame is specified using `::` as delimiter to define the scope of the frame, e.g. `nested_model_A::nested_model_B::awesome_frame`
    #[serde(rename = "@relative_to", skip_serializing_if = "Option::is_none")]
    pub relative_to: Option<String>,

    // Description:'euler_rpy' by default. Supported rotation formats are
    // 'euler_rpy', Euler angles representation in roll, pitch, yaw. The
    // pose is expected to have 6 values. 'quat_xyzw', Quaternion
    // representation in x, y, z, w. The pose is expected to have 7 values.
    #[serde(
        rename = "@rotation_format",
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation_format: Option<String>,
    #[serde(rename = "@degrees", skip_serializing_if = "Option::is_none")]
    pub degrees: Option<bool>,
    // attribute end

    // children element start
    #[serde(rename = "$text")]
    pub content: [f64; 6],
}

/// Description: Include resources from a URI. Included resources can only
/// contain one 'model', 'light' or 'actor' element. The URI can point to a
/// directory or a file. If the URI is a directory, it must conform to the model
/// database structure (see /tutorials?tut=composition&cat=specification&#
/// defining-models-in-separate-files).
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Include {
    // children element start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<bool>,
    // Merge the included model into the top model.
    // Only elements valid
    // in 'world' are allowed in the
    // included model
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#static: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_frame: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<Plugin>>,
    // children element end
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Frame {
    pub name: String,

    // Description: If specified, this frame is attached to the specified
    // frame. The specified frame must be within the same scope and may be
    // defined implicitly, i.e., the name of any //frame, //model, //joint,
    // or //link within the same scope may be used. If missing, this
    // frame is attached to the containing scope's frame. Within a //world
    // scope this is the implicit world frame, and within a //model scope
    // this is the implicit model frame. A frame moves jointly with the
    // frame it is @attached_to. This is different from //pose/@relative_to.
    // @attached_to defines how the frame is attached to a //link, //model, or
    // //world frame, while //pose/@relative_to defines how the frame's
    // pose is represented numerically. As a result, following the chain of
    // @attached_to attributes must always lead to a //link, //model, //world,
    // or //joint (implicitly attached_to its child //link).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
}

/// Description: A plugin is a dynamically loaded chunk of code. It can exist as
/// a child of world, model, and sensor.
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Plugin {
    // attribute start
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@filename")]
    pub filename: String,
    // attribute end

    // children element start
    // Description: Arbitrary elements and attributes that can be used to
    // configure the plugin
    #[serde(rename = "$text")]
    pub content: String,
    // children element end
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            relative_to: None,
            rotation_format: Some("euler_rpy".to_string()),
            degrees: Some(false),
            content: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_pose1() {
        let pose = Pose {
            relative_to: None,
            rotation_format: None,
            degrees: None,
            content: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        };
        let xml = quick_xml::se::to_string(&pose).unwrap();
        println!("{:#?}", xml);
        assert_eq!("<Pose>1 2 3 4 5 6</Pose>", xml);
    }

    #[test]
    fn it_works_pose2() {
        let pose = Pose {
            relative_to: Some(String::default()),
            rotation_format: Some("euler_rpy".to_string()),
            degrees: Some(true),
            content: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        };
        let xml = quick_xml::se::to_string(&pose).unwrap();
        println!("{:#?}", xml);
        assert_eq!(
            "<Pose relative_to=\"\" rotation_format=\"euler_rpy\" \
             degrees=\"true\">1 2 3 4 5 6</Pose>",
            xml
        );
    }

    #[test]
    fn it_works_pose3() {
        let pose = Pose::default();
        let xml = quick_xml::se::to_string(&pose).unwrap();
        println!("{:#?}", xml);
        assert_eq!(
            "<Pose rotation_format=\"euler_rpy\" degrees=\"false\">0 0 0 0 0 \
             0</Pose>",
            xml
        );

        let d_pose: Pose = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(d_pose, pose);
    }

    #[test]
    fn it_works_pose4() {
        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        struct T {
            #[serde(skip_serializing_if = "Option::is_none")]
            pose: Option<Pose>,
        }

        let t = T {
            pose: Some(Pose::default()),
        };
        let xml = quick_xml::se::to_string(&t).unwrap();
        println!("{:#?}", xml);
        assert_eq!(
            "<T><pose rotation_format=\"euler_rpy\" degrees=\"false\">0 0 0 0 \
             0 0</pose></T>",
            xml
        );

        let s = r#"<T><pose rotation_format=\"euler_rpy\" degrees=\"false\">0 0 0 0 0 0</pose></T>"#;
        let t1: T = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(t, t1);
    }

    #[test]
    fn it_works_frame1() {
        let frame = Frame {
            name: "f".to_string(),
            attached_to: Some("g".to_string()),
            pose: Some(Pose::default()),
        };
        let xml = quick_xml::se::to_string(&frame).unwrap();
        println!("{:#?}", xml);
        assert_eq!(
            "<Frame><name>f</name><attached_to>g</attached_to><pose \
             rotation_format=\"euler_rpy\" degrees=\"false\">0 0 0 0 0 \
             0</pose></Frame>",
            xml
        );
    }

    #[test]
    fn it_works_include1() {
        let include = Include {
            merge: None,
            uri: "123".to_string(),
            name: None,
            r#static: None,
            placement_frame: None,
            pose: Some(Pose::default()),
            plugins: None,
        };
        let xml = quick_xml::se::to_string(&include).unwrap();
        println!("{:#?}", xml);
        assert_eq!(
            "<Include><uri>123</uri><pose rotation_format=\"euler_rpy\" \
             degrees=\"false\">0 0 0 0 0 0</pose></Include>",
            xml
        );
    }

    #[test]
    fn it_works_include2() {
        let include = Include {
            merge: None,
            uri: "123".to_string(),
            name: None,
            r#static: None,
            placement_frame: None,
            pose: Some(Pose {
                relative_to: None,
                rotation_format: None,
                degrees: None,
                content: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            }),
            plugins: None,
        };
        let s = r#"
            <Include>
                <uri>123</uri>
                <pose>0 0 0 0 0 0</pose>
            </Include>"#;

        let include1: Include = quick_xml::de::from_str(&s).unwrap();
        assert_eq!(include, include1);
    }

    #[test]
    fn it_works_plugin1() {
        let plugin = Plugin {
            name: Some("123".to_string()),
            filename: "a.so".to_string(),
            content: "content".to_string(),
        };
        let xml = quick_xml::se::to_string(&plugin).unwrap();
        println!("{:#?}", xml);
        assert_eq!(
            "<Plugin name=\"123\" filename=\"a.so\">content</Plugin>",
            xml
        );
    }

    #[test]
    fn it_works_plugin2() {
        let plugin = Plugin {
            name: Some("123".to_string()),
            filename: "a.so".to_string(),
            content: "content".to_string(),
        };
        let s = "<Plugin name=\"123\" filename=\"a.so\">content</Plugin>";
        let plugin1: Plugin = quick_xml::de::from_str(&s).unwrap();
        assert_eq!(plugin, plugin1);
    }
}
