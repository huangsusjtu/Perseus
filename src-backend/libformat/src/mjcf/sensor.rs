use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Sensor {
    #[serde(rename = "touch", skip_serializing_if = "Option::is_none")]
    pub touch: Option<Vec<sensor::Touch>>,

    #[serde(rename = "accelerometer", skip_serializing_if = "Option::is_none")]
    pub accelerometer: Option<Vec<sensor::Accelerometer>>,
    #[serde(rename = "velocimeter", skip_serializing_if = "Option::is_none")]
    pub velocimeter: Option<Vec<sensor::Velocimeter>>,
    #[serde(rename = "gyro", skip_serializing_if = "Option::is_none")]
    pub gyro: Option<Vec<sensor::Gyro>>,
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<Vec<sensor::Force>>,
    #[serde(rename = "torque", skip_serializing_if = "Option::is_none")]
    pub torque: Option<Vec<sensor::Torque>>,
    #[serde(rename = "magnetometer", skip_serializing_if = "Option::is_none")]
    pub magnetometer: Option<Vec<sensor::Magnetometer>>,
    #[serde(rename = "rangefinder", skip_serializing_if = "Option::is_none")]
    pub rangefinder: Option<Vec<sensor::Rangefinder>>,
    #[serde(rename = "camprojection", skip_serializing_if = "Option::is_none")]
    pub cam_projection: Option<Vec<sensor::CamProjection>>,
    #[serde(rename = "jointpos", skip_serializing_if = "Option::is_none")]
    pub joint_pos: Option<Vec<sensor::JointPos>>,
    #[serde(rename = "jointvel", skip_serializing_if = "Option::is_none")]
    pub joint_vel: Option<Vec<sensor::JointVel>>,
    #[serde(rename = "tendonpos", skip_serializing_if = "Option::is_none")]
    pub tendon_pos: Option<Vec<sensor::TendonPos>>,
    #[serde(rename = "tendonvel", skip_serializing_if = "Option::is_none")]
    pub tendon_vel: Option<Vec<sensor::TendonVel>>,
    #[serde(rename = "actuatorpos", skip_serializing_if = "Option::is_none")]
    pub actuator_pos: Option<Vec<sensor::ActuatorPos>>,

    #[serde(rename = "actuatorvel", skip_serializing_if = "Option::is_none")]
    pub actuator_vel: Option<Vec<sensor::ActuatorVel>>,
    #[serde(rename = "actuatorfrc", skip_serializing_if = "Option::is_none")]
    pub actuator_frc: Option<Vec<sensor::ActuatorFrc>>,
    #[serde(rename = "jointactuatorfrc", skip_serializing_if = "Option::is_none")]
    pub joint_actuator_frc: Option<Vec<sensor::JointActuatorFrc>>,
    #[serde(rename = "ballquat", skip_serializing_if = "Option::is_none")]
    pub ball_quat: Option<Vec<sensor::BallQuat>>,
    #[serde(rename = "ballangvel", skip_serializing_if = "Option::is_none")]
    pub ball_angvel: Option<Vec<sensor::BallangVel>>,
    #[serde(rename = "jointlimitpos", skip_serializing_if = "Option::is_none")]
    pub joint_limit_pos: Option<Vec<sensor::JointLimitPos>>,
    #[serde(rename = "jointlimitvel", skip_serializing_if = "Option::is_none")]
    pub joint_limit_vel: Option<Vec<sensor::JointLimitVel>>,
    #[serde(rename = "jointlimitfrc", skip_serializing_if = "Option::is_none")]
    pub joint_limit_frc: Option<Vec<sensor::JointLimitFrc>>,
    #[serde(rename = "tendonlimitpos", skip_serializing_if = "Option::is_none")]
    pub tendon_limit_pos: Option<Vec<sensor::TendonLimitPos>>,
    #[serde(rename = "tendonlimitvel", skip_serializing_if = "Option::is_none")]
    pub tendon_limit_vel: Option<Vec<sensor::TendonLimitVel>>,
    #[serde(rename = "tendonlimitfrc", skip_serializing_if = "Option::is_none")]
    pub tendon_limit_frc: Option<Vec<sensor::TendonLimitFrc>>,
    #[serde(rename = "framepos", skip_serializing_if = "Option::is_none")]
    pub frame_pos: Option<Vec<sensor::FramePos>>,
    #[serde(rename = "framequat", skip_serializing_if = "Option::is_none")]
    pub frame_quat: Option<Vec<sensor::FrameQuat>>,
    #[serde(rename = "framexaxis", skip_serializing_if = "Option::is_none")]
    pub framexaxis: Option<Vec<sensor::FramexAxis>>,
    #[serde(rename = "frameyaxis", skip_serializing_if = "Option::is_none")]
    pub frameyaxis: Option<Vec<sensor::FrameyAxis>>,
    #[serde(rename = "framezaxis", skip_serializing_if = "Option::is_none")]
    pub framezaxis: Option<Vec<sensor::FramezAxis>>,
    #[serde(rename = "framelinvel", skip_serializing_if = "Option::is_none")]
    pub framelinvel: Option<Vec<sensor::FrameLinVel>>,
    #[serde(rename = "frameangvel", skip_serializing_if = "Option::is_none")]
    pub frameangvel: Option<Vec<sensor::FrameAngVel>>,
    #[serde(rename = "framelinacc", skip_serializing_if = "Option::is_none")]
    pub framelinacc: Option<Vec<sensor::FrameLinAcc>>,
    #[serde(rename = "frameangacc", skip_serializing_if = "Option::is_none")]
    pub frameangacc: Option<Vec<sensor::FrameAngAcc>>,
    #[serde(rename = "subtreecom", skip_serializing_if = "Option::is_none")]
    pub subtreecom: Option<Vec<sensor::SubTreeCom>>,
    #[serde(rename = "subtreelinvel", skip_serializing_if = "Option::is_none")]
    pub subtreelinvel: Option<Vec<sensor::SubTreeLinVel>>,
    #[serde(rename = "subtreeangmom", skip_serializing_if = "Option::is_none")]
    pub subtreeangmom: Option<Vec<sensor::SubTreeAngMom>>,
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<Vec<sensor::Distance>>,
    #[serde(rename = "normal", skip_serializing_if = "Option::is_none")]
    pub normal: Option<Vec<sensor::Normal>>,
    #[serde(rename = "fromto", skip_serializing_if = "Option::is_none")]
    pub fromto: Option<Vec<sensor::FromTo>>,
    #[serde(rename = "clock", skip_serializing_if = "Option::is_none")]
    pub clock: Option<Vec<sensor::Clock>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<sensor::User>>,
    #[serde(rename = "plugin", skip_serializing_if = "Option::is_none")]
    pub plugin: Option<sensor::Plugin>,
}

pub mod sensor {
    use serde::{Deserialize, Serialize};

    use crate::mjcf::util::{DataType, NeedStageType, ObjType};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct SiteSensor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@site")]
        pub site: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct CameraSensor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@site")]
        pub site: String,
        #[serde(rename = "@camera")]
        pub camera: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct JointSensor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
        #[serde(rename = "@joint")]
        pub joint: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct TendonSensor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
        #[serde(rename = "@tendon")]
        pub tendon: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ActuatorSensor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
        #[serde(rename = "@actuator")]
        pub actuator: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct FrameSensor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
        #[serde(rename = "@objtype")]
        pub obj_type: ObjType,
        #[serde(rename = "@objname")]
        pub obj_name: String,
        #[serde(rename = "@reftype", skip_serializing_if = "Option::is_none")]
        pub ref_type: Option<ObjType>,
        #[serde(rename = "@refname")]
        pub ref_name: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct FrameAccSensor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
        #[serde(rename = "@objtype")]
        pub obj_type: ObjType,
        #[serde(rename = "@objname")]
        pub obj_name: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct SubTreeSensor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
        #[serde(rename = "@body")]
        pub body: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct CollisionSensor {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@geom1", skip_serializing_if = "Option::is_none")]
        pub geom1: Option<String>,
        #[serde(rename = "@geom2", skip_serializing_if = "Option::is_none")]
        pub geom2: Option<String>,
        #[serde(rename = "@body1", skip_serializing_if = "Option::is_none")]
        pub body1: Option<String>,
        #[serde(rename = "@body2", skip_serializing_if = "Option::is_none")]
        pub body2: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Clock {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct User {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
        pub noise: Option<f64>, // 0
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,

        #[serde(rename = "@objtype", skip_serializing_if = "Option::is_none")]
        pub obj_type: Option<String>,
        #[serde(rename = "@objname", skip_serializing_if = "Option::is_none")]
        pub obj_name: Option<String>,
        #[serde(rename = "@datatype", skip_serializing_if = "Option::is_none")]
        pub data_type: Option<DataType>,
        #[serde(rename = "@needstage", skip_serializing_if = "Option::is_none")]
        pub need_stage: Option<NeedStageType>,
        #[serde(rename = "@dim")]
        pub dim: u32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Plugin {
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@cuttoff", skip_serializing_if = "Option::is_none")]
        pub cut_toff: Option<f64>, // 0
        #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
        pub user: Option<Vec<f64>>,
        #[serde(rename = "@objtype", skip_serializing_if = "Option::is_none")]
        pub obj_type: Option<ObjType>,
        #[serde(rename = "@objname", skip_serializing_if = "Option::is_none")]
        pub obj_name: Option<String>,
        #[serde(rename = "@reftype", skip_serializing_if = "Option::is_none")]
        pub ref_type: Option<ObjType>,
        #[serde(rename = "@refname", skip_serializing_if = "Option::is_none")]
        pub ref_name: Option<String>,

        #[serde(rename = "@plugin", skip_serializing_if = "Option::is_none")]
        pub plugin: Option<String>,
        #[serde(rename = "@instance", skip_serializing_if = "Option::is_none")]
        pub instance: Option<String>,
    }

    pub type Touch = SiteSensor;
    pub type Accelerometer = SiteSensor;
    pub type Velocimeter = SiteSensor;
    pub type Gyro = SiteSensor;
    pub type Force = SiteSensor;
    pub type Torque = SiteSensor;
    pub type Magnetometer = SiteSensor;
    pub type Rangefinder = SiteSensor;

    pub type CamProjection = CameraSensor;

    pub type ActuatorPos = ActuatorSensor;
    pub type ActuatorVel = ActuatorSensor;
    pub type ActuatorFrc = ActuatorSensor;

    pub type JointPos = JointSensor;
    pub type JointVel = JointSensor;
    pub type JointActuatorFrc = JointSensor;
    pub type BallQuat = JointSensor;
    pub type BallangVel = JointSensor;
    pub type JointLimitPos = JointSensor;
    pub type JointLimitVel = JointSensor;
    pub type JointLimitFrc = JointSensor;

    pub type TendonPos = TendonSensor;
    pub type TendonVel = TendonSensor;
    pub type TendonLimitPos = TendonSensor;
    pub type TendonLimitVel = TendonSensor;
    pub type TendonLimitFrc = TendonSensor;

    pub type FramePos = FrameSensor;
    pub type FrameQuat = FrameSensor;
    pub type FramexAxis = FrameSensor;
    pub type FrameyAxis = FrameSensor;
    pub type FramezAxis = FrameSensor;
    pub type FrameLinVel = FrameSensor;
    pub type FrameAngVel = FrameSensor;

    pub type FrameLinAcc = FrameAccSensor;
    pub type FrameAngAcc = FrameAccSensor;

    pub type SubTreeCom = SubTreeSensor;
    pub type SubTreeLinVel = SubTreeSensor;
    pub type SubTreeAngMom = SubTreeSensor;

    pub type Distance = CollisionSensor;
    pub type Normal = CollisionSensor;
    pub type FromTo = CollisionSensor;
}