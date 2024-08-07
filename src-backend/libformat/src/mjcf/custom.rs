use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Custom {
    #[serde(rename = "numeric", skip_serializing_if = "Option::is_none")]
    pub numeric: Option<Vec<custom::Numeric>>,

    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<custom::Text>>,

    #[serde(rename = "tuple", skip_serializing_if = "Option::is_none")]
    pub tuple: Option<Vec<custom::Tuple>>,
}

pub mod custom {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Numeric {
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
        pub size: Option<usize>,

        #[serde(rename = "@data", skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<f64>>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Text {
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@data")]
        pub data: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Tuple {
        #[serde(rename = "@name")]
        pub name: String,

        #[serde(rename = "element", skip_serializing_if = "Option::is_none")]
        pub element: Option<Vec<tuple::Element>>,
    }

    pub mod tuple {
        use serde::{Deserialize, Serialize};

        use crate::mjcf::util::ObjType;

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Element {
            #[serde(rename = "@objtype")]
            pub obj_type: ObjType,
            #[serde(rename = "@objname")]
            pub obj_name: String,
            #[serde(rename = "@prm", skip_serializing_if = "Option::is_none")]
            pub prm: Option<f64>,
        }
    }
}
