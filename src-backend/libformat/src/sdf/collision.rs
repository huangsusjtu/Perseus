/// align to doc: http://sdformat.org/spec?ver=1.11&elem=collision
/// finished
use serde::{Deserialize, Serialize};

use crate::sdf::geometry::Geometry;
use crate::sdf::util::Pose;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Collision {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub laser_retro: Option<f64>, // default 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contacts: Option<u32>, // default 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>, // default 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_inertia_params: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,

    pub geometry: Geometry,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<collision::Surface>,
}

pub mod collision {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Surface {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bounce: Option<Bounce>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub friction: Option<Friction>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contact: Option<Contact>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub soft_contact: Option<SoftContact>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Bounce {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub restitution_coefficient: Option<f64>, // Default: 0.0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub threshold: Option<f64>, // Default: 100000
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Friction {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub torsional: Option<friction::Torsional>, //
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ode: Option<friction::Ode>, //
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bullet: Option<friction::Bullet>, //
    }

    pub mod friction {
        use serde::{Deserialize, Serialize};

        use crate::sdf::util::Vector3;

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Torsional {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub coefficient: Option<f64>, // 1.0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub use_patch_radius: Option<bool>, // true
            #[serde(skip_serializing_if = "Option::is_none")]
            pub patch_radius: Option<f64>, // 0.0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub surface_radius: Option<f64>,
            /* 0.0
                                                         * todo
                                                         * #[serde(skip_serializing_if
                                                         * =
                                                         * "Option::is_none")]
                                                         * pub slip: Option<f64>, // 0.0 */
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Ode {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mu: Option<f64>, // 1.0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mu2: Option<f64>, // 1.0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fdir1: Option<Vector3<f64>>, // Default: 0 0 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub slip1: Option<f64>, // 0.0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub slip2: Option<f64>, // 0.0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Bullet {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub friction: Option<f64>, // 1.0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub friction2: Option<f64>, // 1.0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fdir1: Option<Vector3<f64>>, // Default: 0 0 0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub rolling_friction: Option<f64>, // 1.0
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Contact {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub collide_without_contact: Option<bool>, // false
        #[serde(skip_serializing_if = "Option::is_none")]
        pub collide_without_contact_bitmask: Option<u32>, // 1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub collide_bitmask: Option<u32>, // 65535
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_bitmask: Option<u32>, // 65535
        #[serde(skip_serializing_if = "Option::is_none")]
        pub poissons_ratio: Option<f64>, // 0.3
        #[serde(skip_serializing_if = "Option::is_none")]
        pub elastic_modulus: Option<f64>, // -1.0

        #[serde(skip_serializing_if = "Option::is_none")]
        pub ode: Option<contact::Ode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bullet: Option<contact::Bullet>,
    }

    pub mod contact {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Ode {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub soft_cfm: Option<f64>, // 0.0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub soft_erp: Option<f64>, // 0.2
            #[serde(skip_serializing_if = "Option::is_none")]
            pub kp: Option<f64>, // 1000000000000
            #[serde(skip_serializing_if = "Option::is_none")]
            pub kd: Option<f64>, // 1
            #[serde(skip_serializing_if = "Option::is_none")]
            pub max_vel: Option<f64>, // Default: 0.01
            #[serde(skip_serializing_if = "Option::is_none")]
            pub min_depth: Option<f64>, // 0.0
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Bullet {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub soft_cfm: Option<f64>, // 0.0
            #[serde(skip_serializing_if = "Option::is_none")]
            pub soft_erp: Option<f64>, // 0.2
            #[serde(skip_serializing_if = "Option::is_none")]
            pub kp: Option<f64>, // 1000000000000
            #[serde(skip_serializing_if = "Option::is_none")]
            pub kd: Option<f64>, // 1
            #[serde(skip_serializing_if = "Option::is_none")]
            pub split_impluse: Option<bool>, // true
            #[serde(skip_serializing_if = "Option::is_none")]
            pub split_impulse_penetration_threshold: Option<f64>, // -0.01
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct SoftContact {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dart: Option<soft_contact::Dart>,
    }

    pub mod soft_contact {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        pub struct Dart {
            pub bone_attachment: f64,     // Default: 100
            pub stiffness: f64,           // Default: 100
            pub damping: f64,             // Default: 10
            pub flesh_mass_fraction: f64, // Default: 0.050000000000000003
        }
    }
}
