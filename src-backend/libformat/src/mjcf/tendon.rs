use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Tendon {
    #[serde(rename = "connect", skip_serializing_if = "Option::is_none")]
    pub connect: Option<Vec<tendon::Spatial>>,
    #[serde(rename = "fixed", skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Vec<tendon::Fixed>>,
}

pub mod tendon {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Spatial {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>, // 0
        #[serde(rename = "@limited", skip_serializing_if = "Option::is_none")]
        pub limited: Option<bool>,
        #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
        pub range: Option<[f64; 2]>,
        #[serde(rename = "@solimplimit", skip_serializing_if = "Option::is_none")]
        pub sol_imp_limit: Option<f64>,
        #[serde(rename = "@solreflimit", skip_serializing_if = "Option::is_none")]
        pub sol_ref_limit: Option<[f64; 2]>,
        #[serde(rename = "@solimpfriction", skip_serializing_if = "Option::is_none")]
        pub sol_imp_friction: Option<f64>,
        #[serde(rename = "@solreffriction", skip_serializing_if = "Option::is_none")]
        pub sol_ref_friction: Option<[f64; 2]>,
        #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
        pub margin: Option<f64>,
        #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
        pub friction_loss: Option<f64>,
        #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
        pub width: Option<f64>,
        #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
        pub material: Option<String>,
        #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
        pub rgba: Option<[f64; 4]>,
        #[serde(rename = "@springlength", skip_serializing_if = "Option::is_none")]
        pub spring_length: Option<[f64; 2]>,
        #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
        pub stiffness: Option<f64>,
        #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
        pub damping: Option<f64>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        // element
        #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
        pub site: Option<Vec<spatial::Site>>,
        #[serde(rename = "geom", skip_serializing_if = "Option::is_none")]
        pub geom: Option<Vec<spatial::Geom>>,
        #[serde(rename = "pulley", skip_serializing_if = "Option::is_none")]
        pub pulley: Option<Vec<spatial::Pulley>>,
    }

    pub mod spatial {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Site {
            #[serde(rename = "@site")]
            pub site: String,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Geom {
            #[serde(rename = "@geom")]
            pub geom: String,
            #[serde(rename = "@sidesite", skip_serializing_if = "Option::is_none")]
            pub side_site: Option<String>,
        }

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Pulley {
            #[serde(rename = "@divisor")]
            pub divisor: f64,
        }
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Fixed {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
        pub class: Option<String>,
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<i32>,
        #[serde(rename = "@limited", skip_serializing_if = "Option::is_none")]
        pub limited: Option<bool>,
        #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
        pub range: Option<[f64; 2]>,
        #[serde(rename = "@solimplimit", skip_serializing_if = "Option::is_none")]
        pub sol_imp_limit: Option<f64>,
        #[serde(rename = "@solreflimit", skip_serializing_if = "Option::is_none")]
        pub sol_ref_limit: Option<[f64; 2]>,
        #[serde(rename = "@solimpfriction", skip_serializing_if = "Option::is_none")]
        pub sol_imp_friction: Option<f64>,
        #[serde(rename = "@solreffriction", skip_serializing_if = "Option::is_none")]
        pub sol_ref_friction: Option<[f64; 2]>,
        #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
        pub margin: Option<f64>,
        #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
        pub friction_loss: Option<f64>,
        #[serde(rename = "@springlength", skip_serializing_if = "Option::is_none")]
        pub spring_length: Option<f64>,
        #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
        pub stiffness: Option<f64>,
        #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
        pub damping: Option<f64>,
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        // element
        #[serde(rename = "joint", skip_serializing_if = "Option::is_none")]
        pub joint: Option<Vec<fixed::Joint>>,
    }

    pub mod fixed {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Joint {
            #[serde(rename = "@joint")]
            pub joint: String,
            #[serde(rename = "@coef")]
            pub coef: f64,
        }
    }
}