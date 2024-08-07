use serde::{Deserialize, Serialize};

use crate::sanitation::environment::Environment;
use crate::sanitation::garbage::GarbageConfig;
use crate::sanitation::header::Header;
use crate::sanitation::sanitationman::Sanitation;
use crate::sanitation::task::{GlobalPlannerArea, GlobalPlannerPath};
use crate::sanitation::traffic_flow::TrafficFlowConfig;
use crate::sanitation::util::VecTuple;
use crate::sanitation::vehicle::Vehicle;

/// 定义 环卫云仿真场景
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Scenario {
    // 元信息
    #[serde(rename = "header")]
    pub header: Header,
    // 环境信息
    #[serde(rename = "environment")]
    pub environment: Environment,

    // 环卫车
    #[serde(rename = "vehicle_group")]
    pub vehicles: VehicleGroup,
    // 环卫工
    #[serde(rename = "sanitation_group")]
    pub sanitation_man: SanitationGroup,

    // 所有车辆的作业任务路线, k-v  任务id-任务
    #[serde(rename = "vehicle_task_group")]
    pub global_vehicle_task: VehicleTaskGroup,
    // 所有人工的作业任务
    #[serde(rename = "man_task_group")]
    pub global_man_task: SanitationTaskGroup,

    // 环卫车和路线的绑定关系  车id
    #[serde(rename = "vehicle_to_task")]
    pub vehicle_to_task_mapper: VecTuple,
    // 环卫工和路线的绑定关系  车id
    #[serde(rename = "sanitation_to_task")]
    pub sanitation_to_task_mapper: VecTuple,
    // 司机和环卫车绑定关系
    #[serde(rename = "driver_to_vehicle")]
    pub driver_to_vehicle_mapper: VecTuple,

    // 其他仿真参数
    #[serde(rename = "traffic_flow_config")]
    pub traffic_flow_config: TrafficFlowConfig,
    #[serde(rename = "garbage_config")]
    pub garbage_config: GarbageConfig,
}

#[derive(Debug, Default, Clone,Deserialize, Serialize)]
pub struct VehicleGroup {
    #[serde(rename = "vehicle")]
    pub vehicles: Vec<Vehicle>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SanitationGroup {
    #[serde(rename = "sanitation")]
    pub sanitation: Vec<Sanitation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VehicleTaskGroup {
    #[serde(rename = "task")]
    pub task: Vec<GlobalPlannerPath>,
}

#[derive(Debug, Default, Clone,Deserialize, Serialize)]
pub struct SanitationTaskGroup {
    #[serde(rename = "task")]
    pub task: Vec<GlobalPlannerArea>,
}
