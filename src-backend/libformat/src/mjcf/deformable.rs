use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Deformable {
    #[serde(rename = "flex", skip_serializing_if = "Option::is_none")]
    pub flex: Option<Vec<deformable::Flex>>,
    #[serde(rename = "skin", skip_serializing_if = "Option::is_none")]
    pub skin: Option<Vec<deformable::Skin>>,
}

pub mod deformable {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Flex {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@dim", skip_serializing_if = "Option::is_none")]
        pub dim: Option<u32>, // 2
        #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
        pub radius: Option<f64>, // 0.005

        #[serde(rename = "@body")]
        pub body: String,

        #[serde(rename = "@vertex", skip_serializing_if = "Option::is_none")]
        pub vertex: Option<Vec<f64>>,
        #[serde(rename = "@texcoord", skip_serializing_if = "Option::is_none")]
        pub texcoord: Option<Vec<f64>>,
        #[serde(rename = "@element", skip_serializing_if = "Option::is_none")]
        pub element: Option<Vec<i32>>,
        #[serde(rename = "@flatskin", skip_serializing_if = "Option::is_none")]
        pub flat_skin: Option<bool>,
        #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
        pub material: Option<String>,
        #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
        pub rgba: Option<[f64; 4]>, // 0.5 0.5 0.5 1
        #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
        pub group: Option<u32>, // 0

        // element
        #[serde(rename = "@edge", skip_serializing_if = "Option::is_none")]
        pub edge: Option<Edge>,
        #[serde(rename = "@contact", skip_serializing_if = "Option::is_none")]
        pub contact: Option<Contact>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Edge {
        #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
        pub stiffness: Option<f64>, // 0
        #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
        pub damping: Option<f64>, // 0
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Contact {
        #[serde(rename = "@internal", skip_serializing_if = "Option::is_none")]
        pub internal: Option<bool>,
        #[serde(rename = "@selfcollide", skip_serializing_if = "Option::is_none")]
        pub self_collide: Option<String>, // [none, narrow, bvh, sap, auto]
        #[serde(rename = "@activelayers", skip_serializing_if = "Option::is_none")]
        pub active_layers: Option<u32>, // 1
        #[serde(rename = "@contype", skip_serializing_if = "Option::is_none")]
        pub con_type: Option<u32>, // 1
        #[serde(rename = "@conaffinity", skip_serializing_if = "Option::is_none")]
        pub con_affinity: Option<u32>, // 1
        #[serde(rename = "@condim", skip_serializing_if = "Option::is_none")]
        pub con_dim: Option<u32>, // 3
        #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
        pub priority: Option<u32>, // 0
        #[serde(rename = "@friction", skip_serializing_if = "Option::is_none")]
        pub friction: Option<[f64; 3]>,
        #[serde(rename = "@solmix", skip_serializing_if = "Option::is_none")]
        pub sol_mix: Option<f64>, // 1
        #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
        pub sol_ref: Option<f64>,
        #[serde(rename = "@solimp", skip_serializing_if = "Option::is_none")]
        pub sol_imp: Option<f64>,
        #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
        pub margin: Option<f64>, // 0
        #[serde(rename = "@gap", skip_serializing_if = "Option::is_none")]
        pub gap: Option<f64>, // 0
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Skin {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
        pub file: Option<String>,
        #[serde(rename = "@vertex", skip_serializing_if = "Option::is_none")]
        pub vertex: Option<Vec<f64>>,
        #[serde(rename = "@texcoord", skip_serializing_if = "Option::is_none")]
        pub texcoord: Option<Vec<f64>>,
        #[serde(rename = "@face", skip_serializing_if = "Option::is_none")]
        pub face: Option<Vec<u32>>,
        #[serde(rename = "@inflate", skip_serializing_if = "Option::is_none")]
        pub inflate: Option<f64>,
        #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
        pub material: Option<String>,
        #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
        pub group: Option<u32>,
        #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
        pub rgba: Option<[f64; 4]>,

        #[serde(rename = "bone", skip_serializing_if = "Option::is_none")]
        pub bone: Option<Vec<Bone>>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Bone {
        #[serde(rename = "@body")]
        pub body: String,
        #[serde(rename = "@bindpos")]
        pub bind_pos: [f64; 3],
        #[serde(rename = "@bindquat")]
        pub bind_quat: [f64; 4],
        #[serde(rename = "@vertid")]
        pub vertid: Vec<u32>,
        #[serde(rename = "@vertweight")]
        pub vertweight: Vec<u32>,
    }
}
