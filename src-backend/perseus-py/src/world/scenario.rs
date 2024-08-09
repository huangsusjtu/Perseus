use pyo3::prelude::*;

use crate::world::environment::Environment;
use crate::world::garbage::GarbageConfig;
use crate::world::sanitation::Sanitation;
use crate::world::task::{GlobalPlannerArea, GlobalPlannerPath};
use crate::world::traffic_flow::TrafficFlowConfig;
use crate::world::vehicle::Vehicle;

/// 场景对象
#[pyclass]
pub struct Scenario {
    pub header: Header,                              // 元信息
    pub environment: Environment,                    // 环境信息
    pub vehicles: Vec<Vehicle>,                      // 所有的车
    pub sanitation_man: Vec<Sanitation>,             // 环卫工
    pub global_vehicle_task: Vec<GlobalPlannerPath>, // 所有车辆的作业任务路线
    pub global_man_task: Vec<GlobalPlannerArea>,     // 所有人工的作业任务

    pub vehicle_to_task_mapper: Vec<(String, String)>, /* 环卫车和任务的绑定关系 */
    pub sanitation_to_task_mapper: Vec<(String, String)>, /* 环卫工和任务的绑定关系 */

    pub traffic_flow_config: TrafficFlowConfig, // 全局交通流配置
    pub garbage_config: GarbageConfig,          // 全局垃圾配置
}

#[pyclass(set_all, get_all)]
#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub map_name: String,
    pub date: String, // chrono::prelude::Local::now().to_string()
}

///
#[pymethods]
impl Scenario {
    fn __str__(&self) -> String {
        format!("Scenario (header:{:?})", self.header,)
    }
}

///
impl From<&libmodel::sanitation::Scenario> for Scenario {
    fn from(value: &libmodel::sanitation::Scenario) -> Self {
        Scenario {
            header: Header {
                name: value.header.name.clone(),
                map_name: value.header.map_name.clone(),
                date: value.header.date.clone(),
            },
            environment: Environment::from(&value.environment),
            vehicles: value
                .vehicles
                .vehicles
                .iter()
                .map(|v| Vehicle::from(v))
                .collect(),
            sanitation_man: value
                .sanitation_man
                .sanitation
                .iter()
                .map(|v| Sanitation::from(v))
                .collect(),
            global_vehicle_task: value
                .global_vehicle_task
                .task
                .iter()
                .map(|v| GlobalPlannerPath::from(v))
                .collect(),
            global_man_task: value
                .global_man_task
                .task
                .iter()
                .map(|v| GlobalPlannerArea::from(v))
                .collect(),
            vehicle_to_task_mapper: value
                .vehicle_to_task_mapper
                .value
                .iter()
                .map(|v| v.value.clone())
                .collect(),
            sanitation_to_task_mapper: value
                .sanitation_to_task_mapper
                .value
                .iter()
                .map(|v| v.value.clone())
                .collect(),
            traffic_flow_config: TrafficFlowConfig {},
            garbage_config: GarbageConfig {},
        }
    }
}
