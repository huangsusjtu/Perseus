use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Equality {
    #[serde(rename = "connect", skip_serializing_if = "Option::is_none")]
    pub connect: Option<Vec<equality::Connect>>,
    #[serde(rename = "weld", skip_serializing_if = "Option::is_none")]
    pub weld: Option<Vec<equality::Weld>>,
    #[serde(rename = "joint", skip_serializing_if = "Option::is_none")]
    pub joint: Option<Vec<equality::Joint>>,
    #[serde(rename = "tendon", skip_serializing_if = "Option::is_none")]
    pub tendon: Option<Vec<equality::Tendon>>,
    #[serde(rename = "flex", skip_serializing_if = "Option::is_none")]
    pub flex: Option<Vec<equality::Flex>>,
}

pub mod equality {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Connect {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
        #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
        pub sol_imp: Option<f64>,
        #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
        pub sol_ref: Option<[f64; 2]>,
        #[serde(rename = "@body1")]
        pub body1: String,
        #[serde(rename = "@body2", skip_serializing_if = "Option::is_none")]
        pub body2: Option<String>,
        #[serde(rename = "@anchor")]
        pub anchor: [f64; 3],
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Weld {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
        #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
        pub sol_imp: Option<f64>,
        #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
        pub sol_ref: Option<[f64; 2]>,
        #[serde(rename = "@body1")]
        pub body1: String,
        #[serde(rename = "@body2", skip_serializing_if = "Option::is_none")]
        pub body2: Option<String>,
        #[serde(rename = "@relpose", skip_serializing_if = "Option::is_none")]
        pub rel_pose: Option<[f64; 7]>,
        #[serde(rename = "@anchor")]
        pub anchor: [f64; 3],
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Joint {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
        #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
        pub sol_imp: Option<f64>,
        #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
        pub sol_ref: Option<[f64; 2]>,
        #[serde(rename = "@joint1")]
        pub joint1: String,
        #[serde(rename = "@joint2", skip_serializing_if = "Option::is_none")]
        pub joint2: Option<String>,
        #[serde(rename = "@polycoef", skip_serializing_if = "Option::is_none")]
        pub poly_coef: Option<[f64; 5]>, // 0 1 0 0 0
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Tendon {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
        #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
        pub sol_imp: Option<f64>,
        #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
        pub sol_ref: Option<[f64; 2]>,
        #[serde(rename = "@tendon1")]
        pub tendon1: String,
        #[serde(rename = "@tendon2", skip_serializing_if = "Option::is_none")]
        pub tendon2: Option<String>,
        #[serde(rename = "@polycoef", skip_serializing_if = "Option::is_none")]
        pub poly_coef: Option<[f64; 5]>, // 0 1 0 0 0
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Flex {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
        #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
        pub sol_imp: Option<f64>,
        #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
        pub sol_ref: Option<[f64; 2]>,
        #[serde(rename = "@flex")]
        pub flex: String,
    }
}
