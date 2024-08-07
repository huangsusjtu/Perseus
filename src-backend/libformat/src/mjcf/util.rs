use serde::{Deserialize, Serialize};

use crate::mjcf::extension::extension::Instance;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FlagSimpleType {
    Enable,
    Disable,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CoordinateType {
    Local,
    Global,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AngleType {
    Degree,
    Radian,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum InertiaFromGeomType {
    False,
    True,
    Auto,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ModeType {
    None,
    Muscle,
    MuscleUser,
    All,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub enum IntegratorType {
    #[default]
    #[serde(rename = "Euler")]
    Euler,
    #[serde(rename = "RK4")]
    RK4,
    #[serde(rename = "implicit")]
    Implicit,
    #[serde(rename = "implicitfast")]
    ImplicitFast,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CollisionType {
    All,
    Predefined,
    Dynamic,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ConeType {
    Pyramidal,
    Elliptic,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum JacobianType {
    Dense,
    Sparse,
    Auto,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SolverType {
    PGS,
    CG,
    Newton,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TextureType {
    #[serde(rename = "2d")]
    TwoD,
    #[serde(rename = "cube")]
    Cube,
    #[serde(rename = "skybox")]
    Skybox,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BuiltinType {
    None,
    Gradient,
    Checker,
    Flat,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MarkType {
    None,
    Edge,
    Cross,
    Random,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum JointType {
    Free,
    Ball,
    Slide,
    Hinge,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GeomType {
    Plane,
    Hfield,
    Sphere,
    Capsule,
    Ellipsoid,
    Cylinder,
    Box,
    Mesh,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SiteType {
    Sphere,
    Capsule,
    Ellipsoid,
    Cylinder,
    Box,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CameraModeType {
    Fixed,
    Track,
    TrackCom,
    TargetBody,
    TargetBodyCom,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CompositeType {
    Particle,
    Grid,
    Rope,
    Loop,
    Cloth,
    Box,
    Cylinder,
    Ellipsoid,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum JointKindType {
    Main,
    Twist,
    Stretch,
    Particle,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TendonKindType {
    Main,
    Shear,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DynType {
    None,
    Integrator,
    Filter,
    Muscle,
    User,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GainType {
    Fixed,
    Muscle,
    User,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BiasType {
    None,
    Affine,
    Muscle,
    User,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ObjType {
    Body,
    Xbody,
    Geom,
    Site,
    Camera,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum DataType {
    #[default]
    Real,
    Positive,
    Axis,
    Quaternion,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum NeedStageType {
    Pos,
    Vel,
    Acc,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Plugin {
    #[serde(rename = "@plugin")]
    pub plugin: String,

    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<Vec<Instance>>,
}