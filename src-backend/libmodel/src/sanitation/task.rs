use serde::{Deserialize, Serialize};

use crate::sanitation::util::LanePoint;
use crate::sanitation::vehicle::{OperationMode, WorkMode};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrenetPoint {
    #[serde(rename = "@road_id")]
    pub road_id: String,
    #[serde(rename = "@s")]
    pub s: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LaneWayPoint {
    #[serde(rename = "@work_mode", skip_serializing_if = "Option::is_none")]
    pub work_mode: Option<WorkMode>, // 从该点开始 开启的工作模式
    #[serde(
        rename = "@operation_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_mode: Option<OperationMode>, // 从该点开始 的运维模式

    #[serde(rename = "frenet_point", skip_serializing_if = "Option::is_none")]
    pub frenet_point: Option<FrenetPoint>, // 坐标点
    pub position: LanePoint,
}

// 车辆任务路径
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalPlannerPath {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(rename = "@loop")]
    pub r#loop: bool,

    pub sparse_point: Vec<LaneWayPoint>,
}

impl Default for GlobalPlannerPath {
    fn default() -> Self {
        GlobalPlannerPath {
            name: "GlobalPlannerPath".to_string(),
            timestamp: None,
            r#loop: false,
            sparse_point: vec![LaneWayPoint {
                work_mode: Some(WorkMode::DrySweep),
                operation_mode: Some(OperationMode::NONE),
                frenet_point: None,
                position: Default::default(),
            }],
        }
    }
}

// 环卫人员任务片区
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalPlannerArea {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(rename = "@loop")]
    pub r#loop: bool,
    pub clean_area_ids: Vec<String>, // 清扫区域的ID
}

impl Default for GlobalPlannerArea {
    fn default() -> Self {
        GlobalPlannerArea {
            name: "GlobalPlannerArea".to_string(),
            timestamp: None,
            r#loop: false,
            clean_area_ids: vec!["area-0".to_string()],
        }
    }
}
