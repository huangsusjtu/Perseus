use serde::{Deserialize, Serialize};
use crate::sanitation::util::LanePoint;

/// 垃圾环境
#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct GarbageConfig {}



pub struct Garbage {
    pub theta: f32,
    pub width: f32,
    pub length: f32,
    pub position: LanePoint,
}