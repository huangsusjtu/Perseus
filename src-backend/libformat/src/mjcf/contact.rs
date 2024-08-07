use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Contact {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<Vec<contact::Pair>>,
    #[serde(rename = "exclude", skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<contact::Exclude>>,
}

pub mod contact {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Pair {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@geom1")]
        pub geom1: String,
        #[serde(rename = "@geom2")]
        pub geom2: String,
        #[serde(rename = "@condim", skip_serializing_if = "Option::is_none")]
        pub con_dim: Option<u32>,
        #[serde(rename = "@friction", skip_serializing_if = "Option::is_none")]
        pub friction: Option<[f64; 5]>,  // 1 1 0.005 0.0001 0.0001
        #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
        pub sol_imp: Option<f64>,
        #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
        pub sol_ref: Option<[f64; 2]>,
        #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
        pub margin: Option<f64>,
        #[serde(rename = "@gap", skip_serializing_if = "Option::is_none")]
        pub gap: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Exclude {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@body1")]
        pub body1: String,
        #[serde(rename = "@body2")]
        pub body2: String,
    }
}
