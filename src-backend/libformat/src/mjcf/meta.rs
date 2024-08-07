use serde::{Deserialize, Serialize};

use crate::mjcf::util::{ConeType, FlagSimpleType, IntegratorType, JacobianType, SolverType};

#[derive(Deserialize, Serialize, Debug)]
pub struct Replicate {
    #[serde(rename = "@sep", skip_serializing_if = "Option::is_none")]
    pub sep: Option<String>,

    #[serde(rename = "@count")]
    pub count: i32,
    #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<[f64; 3]>,
    #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
    pub euler: Option<[f64; 3]>,
}

/// include tag
#[derive(Deserialize, Serialize, Debug)]
pub struct OptionType {
    #[serde(rename = "@timestep", skip_serializing_if = "Option::is_none")]
    pub timestep: Option<f64>, // 0.002
    #[serde(rename = "@apirate", skip_serializing_if = "Option::is_none")]
    pub api_rate: Option<f64>, // 100
    #[serde(rename = "@impratio", skip_serializing_if = "Option::is_none")]
    pub imp_ratio: Option<f64>,
    #[serde(rename = "@gravity", skip_serializing_if = "Option::is_none")]
    pub gravity: Option<[f64; 3]>,
    #[serde(rename = "@wind", skip_serializing_if = "Option::is_none")]
    pub wind: Option<[f64; 3]>,
    #[serde(rename = "@magnetic", skip_serializing_if = "Option::is_none")]
    pub magnetic: Option<[f64; 3]>,
    #[serde(rename = "@density", skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[serde(rename = "@viscosity", skip_serializing_if = "Option::is_none")]
    pub viscosity: Option<f64>,
    #[serde(rename = "@o_margin", skip_serializing_if = "Option::is_none")]
    pub o_margin: Option<f64>,
    #[serde(rename = "@o_solimp", skip_serializing_if = "Option::is_none")]
    pub o_solimp: Option<f64>,
    #[serde(rename = "@o_solref", skip_serializing_if = "Option::is_none")]
    pub o_solref: Option<[f64; 2]>,
    #[serde(rename = "@o_friction", skip_serializing_if = "Option::is_none")]
    pub o_friction: Option<[f64; 2]>,
    #[serde(rename = "@integrator", skip_serializing_if = "Option::is_none")]
    pub integrator: Option<IntegratorType>,
    #[serde(rename = "@cone", skip_serializing_if = "Option::is_none")]
    pub cone: Option<ConeType>,

    #[serde(rename = "@jacobian", skip_serializing_if = "Option::is_none")]
    pub jacobian: Option<JacobianType>,
    #[serde(rename = "@solver", skip_serializing_if = "Option::is_none")]
    pub solver: Option<SolverType>,
    #[serde(rename = "@iterations", skip_serializing_if = "Option::is_none")]
    pub iterations: Option<u32>,
    #[serde(rename = "@tolerance", skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    #[serde(rename = "@ls_iterations", skip_serializing_if = "Option::is_none")]
    pub ls_iterations: Option<u32>,
    #[serde(rename = "@ls_tolerance", skip_serializing_if = "Option::is_none")]
    pub ls_tolerance: Option<f64>,
    #[serde(rename = "@noslip_iterations", skip_serializing_if = "Option::is_none")]
    pub noslip_iterations: Option<u32>,
    #[serde(rename = "@noslip_tolerance", skip_serializing_if = "Option::is_none")]
    pub noslip_tolerance: Option<f64>,
    #[serde(rename = "@mpr_iterations", skip_serializing_if = "Option::is_none")]
    pub mpr_iterations: Option<u32>,
    #[serde(rename = "@mpr_tolerance", skip_serializing_if = "Option::is_none")]
    pub mpr_tolerance: Option<f64>,
    #[serde(rename = "@sdf_iterations", skip_serializing_if = "Option::is_none")]
    pub sdf_iterations: Option<u32>,
    #[serde(rename = "@sdf_initpoints", skip_serializing_if = "Option::is_none")]
    pub sdf_init_points: Option<u32>,
    #[serde(
        rename = "@actuatorgroupdisable",
        skip_serializing_if = "Option::is_none"
    )]
    pub actuator_group_disable: Option<u32>,

    // content
    #[serde(rename = "flag", skip_serializing_if = "Option::is_none")]
    pub flag: Option<FlagType>,

}

#[derive(Deserialize, Serialize, Debug)]
pub struct FlagType {
    #[serde(rename = "@constraint", skip_serializing_if = "Option::is_none")]
    pub constraint: Option<FlagSimpleType>,
    #[serde(rename = "@equality", skip_serializing_if = "Option::is_none")]
    pub equality: Option<FlagSimpleType>,
    #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
    pub friction_loss: Option<FlagSimpleType>,
    #[serde(rename = "@limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<FlagSimpleType>,
    #[serde(rename = "@contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<FlagSimpleType>,
    #[serde(rename = "@passive", skip_serializing_if = "Option::is_none")]
    pub passive: Option<FlagSimpleType>,
    #[serde(rename = "@gravity", skip_serializing_if = "Option::is_none")]
    pub gravity: Option<FlagSimpleType>,
    #[serde(rename = "@clampctrl", skip_serializing_if = "Option::is_none")]
    pub clamp_ctrl: Option<FlagSimpleType>,
    #[serde(rename = "@warmstart", skip_serializing_if = "Option::is_none")]
    pub warm_start: Option<FlagSimpleType>,
    #[serde(rename = "@filterparent", skip_serializing_if = "Option::is_none")]
    pub filter_parent: Option<FlagSimpleType>,
    #[serde(rename = "@actuation", skip_serializing_if = "Option::is_none")]
    pub actuation: Option<FlagSimpleType>,
    #[serde(rename = "@refsafe", skip_serializing_if = "Option::is_none")]
    pub ref_safe: Option<FlagSimpleType>,
    #[serde(rename = "@sensor", skip_serializing_if = "Option::is_none")]
    pub sensor: Option<FlagSimpleType>,
    #[serde(rename = "@midphase", skip_serializing_if = "Option::is_none")]
    pub mid_phase: Option<FlagSimpleType>,
    #[serde(rename = "@eulerdamp", skip_serializing_if = "Option::is_none")]
    pub euler_damp: Option<FlagSimpleType>,
    #[serde(rename = "@override", skip_serializing_if = "Option::is_none")]
    pub over_ride: Option<FlagSimpleType>,
    #[serde(rename = "@energy", skip_serializing_if = "Option::is_none")]
    pub energy: Option<FlagSimpleType>,
    #[serde(rename = "@fwdinv", skip_serializing_if = "Option::is_none")]
    pub fwd_inv: Option<FlagSimpleType>,
    #[serde(rename = "@invdiscrete", skip_serializing_if = "Option::is_none")]
    pub inv_discrete: Option<FlagSimpleType>,
    #[serde(rename = "@multiccd", skip_serializing_if = "Option::is_none")]
    pub multi_ccd: Option<FlagSimpleType>,
    #[serde(rename = "@island", skip_serializing_if = "Option::is_none")]
    pub island: Option<FlagSimpleType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Compiler {
    #[serde(rename = "@autolimits", skip_serializing_if = "Option::is_none")]
    pub auto_limits: Option<bool>,
    #[serde(rename = "@boundmass", skip_serializing_if = "Option::is_none")]
    pub bound_mass: Option<f64>, // 0
    #[serde(rename = "@boundinertia", skip_serializing_if = "Option::is_none")]
    pub bound_inertia: Option<f64>, // 0
    #[serde(rename = "@settotalmass", skip_serializing_if = "Option::is_none")]
    pub set_total_mass: Option<f64>, // -1
    #[serde(rename = "@balanceinertia", skip_serializing_if = "Option::is_none")]
    pub balance_inertia: Option<bool>, // 0
    #[serde(rename = "@strippath", skip_serializing_if = "Option::is_none")]
    pub strip_path: Option<bool>, // [false, true], “false” for MJCF, “true” for URDF
    #[serde(rename = "@coordinate", skip_serializing_if = "Option::is_none")]
    pub coordinate: Option<String>, //  [local, global], “local”
    #[serde(rename = "@angle", skip_serializing_if = "Option::is_none")]
    pub angle: Option<String>, // [radian, degree], “degree” for MJCF, always “radian” for URDF
    #[serde(rename = "@fitaabb", skip_serializing_if = "Option::is_none")]
    pub fit_aabb: Option<bool>, // 0
    #[serde(rename = "@eulerseq", skip_serializing_if = "Option::is_none")]
    pub euler_seq: Option<String>, // string, “xyz”
    #[serde(rename = "@meshdir", skip_serializing_if = "Option::is_none")]
    pub mesh_dir: Option<String>,
    #[serde(rename = "@texturedir", skip_serializing_if = "Option::is_none")]
    pub texture_dir: Option<String>,
    #[serde(rename = "@assetdir", skip_serializing_if = "Option::is_none")]
    pub asset_dir: Option<String>,
    #[serde(rename = "@discardvisual", skip_serializing_if = "Option::is_none")]
    pub discard_visual: Option<bool>,
    #[serde(rename = "@convexhull", skip_serializing_if = "Option::is_none")]
    pub convex_hull: Option<bool>, // true
    #[serde(rename = "@usethread", skip_serializing_if = "Option::is_none")]
    pub use_thread: Option<bool>,// true
    #[serde(rename = "@fusestatic", skip_serializing_if = "Option::is_none")]
    pub fuse_static: Option<bool>, // [false, true], “false” for MJCF, “true” for URDF
    #[serde(rename = "@inertiafromgeom", skip_serializing_if = "Option::is_none")]
    pub inertia_from_geom: Option<bool>,
    #[serde(rename = "@exactmeshinertia", skip_serializing_if = "Option::is_none")]
    pub exact_mesh_inertia: Option<bool>,
    #[serde(rename = "@inertiagrouprange", skip_serializing_if = "Option::is_none")]
    pub inertia_group_range: Option<[f64; 2]>,

    #[serde(rename = "lengthrange", skip_serializing_if = "Option::is_none")]
    pub length_range: Option<compiler::LengthRange>,
}

pub mod compiler {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct LengthRange {
        #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,  // none, muscle, muscleuser, all
        #[serde(rename = "@useexisting", skip_serializing_if = "Option::is_none")]
        pub use_existing: Option<bool>,
        #[serde(rename = "@uselimit", skip_serializing_if = "Option::is_none")]
        pub use_limit: Option<bool>,
        #[serde(rename = "@accel", skip_serializing_if = "Option::is_none")]
        pub accel: Option<f64>,// 20
        #[serde(rename = "@maxforce", skip_serializing_if = "Option::is_none")]
        pub max_force: Option<f64>,  // 0
        #[serde(rename = "@timeconst", skip_serializing_if = "Option::is_none")]
        pub time_const: Option<f64>,  // 1
        #[serde(rename = "@timestep", skip_serializing_if = "Option::is_none")]
        pub time_step: Option<f64>,  // 0.01
        #[serde(rename = "@inttotal", skip_serializing_if = "Option::is_none")]
        pub int_total: Option<f64>, // 10
        #[serde(rename = "@interval", skip_serializing_if = "Option::is_none")]
        pub interval: Option<f64>, // 2
        #[serde(rename = "@tolrange", skip_serializing_if = "Option::is_none")]
        pub tol_range: Option<f64>,  // 0.05
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Size {
    #[serde(rename = "@memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>, // “-1”
    #[serde(rename = "@njmax", skip_serializing_if = "Option::is_none")]
    pub nj_max: Option<i32>,// “-1”
    #[serde(rename = "@nconmax", skip_serializing_if = "Option::is_none")]
    pub nconmax: Option<i32>, // -1
    #[serde(rename = "@nstack", skip_serializing_if = "Option::is_none")]
    pub nstack: Option<i32>, // -1
    #[serde(rename = "@nuserdata", skip_serializing_if = "Option::is_none")]
    pub nuserdata: Option<i32>, // 0
    #[serde(rename = "@nkey", skip_serializing_if = "Option::is_none")]
    pub nkey: Option<String>,  // 0
    #[serde(rename = "@nuser_body", skip_serializing_if = "Option::is_none")]
    pub nuser_body: Option<String>, // -1
    #[serde(rename = "@nuser_jnt", skip_serializing_if = "Option::is_none")]
    pub nuser_jnt: Option<String>, // -1
    #[serde(rename = "@nuser_geom", skip_serializing_if = "Option::is_none")]
    pub nuser_geom: Option<String>, // -1
    #[serde(rename = "@nuser_site", skip_serializing_if = "Option::is_none")]
    pub nuser_site: Option<String>, // -1
    #[serde(rename = "@nuser_cam", skip_serializing_if = "Option::is_none")]
    pub nuser_cam: Option<String>, // -1
    #[serde(rename = "@nuser_tendon", skip_serializing_if = "Option::is_none")]
    pub nuser_tendon: Option<String>, // -1
    #[serde(rename = "@nuser_actuator", skip_serializing_if = "Option::is_none")]
    pub nuser_actuator: Option<String>, // -1
    #[serde(rename = "@nuser_sensor", skip_serializing_if = "Option::is_none")]
    pub nuser_sensor: Option<String>, // -1
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Statistic {
    #[serde(rename = "@meanmass", skip_serializing_if = "Option::is_none")]
    pub mean_mass: Option<f64>,
    #[serde(rename = "@meaninertia", skip_serializing_if = "Option::is_none")]
    pub mean_inertia: Option<f64>,
    #[serde(rename = "@meansize", skip_serializing_if = "Option::is_none")]
    pub mean_size: Option<f64>,
    #[serde(rename = "@extent", skip_serializing_if = "Option::is_none")]
    pub extent: Option<f64>,
    #[serde(rename = "@center", skip_serializing_if = "Option::is_none")]
    pub center: Option<[f64; 3]>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Include {
    #[serde(rename = "@file")]
    pub file: String,
}


#[cfg(test)]
mod tests {
    use crate::mjcf::meta::Replicate;

    #[test]
    fn it_works1() {
        let instance: Replicate = Replicate {
            sep: Some("sep".to_string()),
            count: 10,
            offset: Some([1.0, 2.0, 3.0]),
            euler: Some([5.0, 6.0, 7.0]),
        };

        let mut xml = String::new();
        quick_xml::se::to_writer_with_root(&mut xml, "replicate", &instance);
        println!("{}", xml);
    }

    #[test]
    fn it_works2() {
        let rep: Replicate = quick_xml::de::from_str("").unwrap();
    }
}
