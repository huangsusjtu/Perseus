use pyo3::prelude::*;

use crate::map::waypoint::WayPoint;
use crate::world::vehicle::{OperationMode, WorkMode};

/// sl坐标点
#[pyclass(set_all, get_all)]
#[derive(PartialEq, Clone, Debug)]
pub struct FrenetPoint {
    pub road_id: String,
    pub s: f64,
}

/// 作业的坐标点
#[pyclass(set_all, get_all)]
#[derive(Debug)]
pub struct LaneWayPoint {
    pub work_mode: WorkMode, // 从该点开始 开启的工作模式
    pub operation_mode: OperationMode, // 从该点开始 的运维模式
    pub frenet_point: Option<FrenetPoint>, // 坐标点
    pub position: WayPoint,
}

/// 车辆的作业路线和任务
#[pyclass]
pub struct GlobalPlannerPath {
    #[pyo3(set, get)]
    pub name: String,
    #[pyo3(set, get)]
    pub timestamp: Option<u64>,
    #[pyo3(set, get)]
    pub r#loop: bool,
    pub sparse_point: Vec<LaneWayPoint>,
}

/// 区域清扫的人工任务
#[pyclass(set_all, get_all)]
pub struct GlobalPlannerArea {
    pub name: String,
    pub timestamp: Option<u64>,
    pub r#loop: bool,
    pub clean_area_ids: Vec<String>, // 清扫区域的ID
}

///
#[pymethods]
impl GlobalPlannerPath {
    #[new]
    #[pyo3(signature = (name="default", r#loop=false, time = None))]
    fn new(name: &str, r#loop: bool, time: Option<u64>) -> Self {
        GlobalPlannerPath {
            name: name.to_string(),
            timestamp: time,
            r#loop,
            sparse_point: vec![],
        }
    }

    fn __str__(&self) -> String {
        format!(
            "GlobalPlannerPath (name:{}, loop:{}, time:{:?}, points:{:#?}) ",
            self.name, self.r#loop, self.timestamp, self.sparse_point
        )
    }
}

#[pymethods]
impl GlobalPlannerArea {
    #[new]
    #[pyo3(signature = (name="default", r#loop=false, time=None))]
    fn new(name: &str, r#loop: bool, time: Option<u64>) -> Self {
        GlobalPlannerArea {
            name: name.to_string(),
            timestamp: time,
            r#loop,
            clean_area_ids: vec![],
        }
    }

    fn __str__(&self) -> String {
        format!(
            "GlobalPlannerArea (name:{}, loop:{}, time:{:?}, areas:{:#?}) ",
            self.name, self.r#loop, self.timestamp, self.clean_area_ids
        )
    }
}

///
impl From<&libmodel::sanitation::task::GlobalPlannerPath>
    for GlobalPlannerPath
{
    fn from(value: &libmodel::sanitation::task::GlobalPlannerPath) -> Self {
        GlobalPlannerPath {
            name: value.name.clone(),
            timestamp: value.timestamp.clone(),
            r#loop: value.r#loop,
            sparse_point: value
                .sparse_point
                .iter()
                .map(|v| LaneWayPoint {
                    work_mode: v
                        .work_mode
                        .clone()
                        .map_or(WorkMode::NONE, |v| WorkMode::from(&v)),
                    operation_mode: v
                        .operation_mode
                        .clone()
                        .map_or(OperationMode::NONE, |v| {
                            OperationMode::from(&v)
                        }),
                    frenet_point: None,
                    position: WayPoint {
                        x: v.position.x,
                        y: v.position.y,
                    },
                })
                .collect(),
        }
    }
}

impl From<libmodel::sanitation::task::LaneWayPoint> for LaneWayPoint {
    fn from(value: libmodel::sanitation::task::LaneWayPoint) -> Self {
        LaneWayPoint {
            work_mode: value
                .work_mode
                .map_or(WorkMode::NONE, |v| WorkMode::from(&v)),
            operation_mode: value
                .operation_mode
                .map_or(OperationMode::NONE, |v| OperationMode::from(&v)),
            frenet_point: value.frenet_point.map_or(None, |v| {
                Some(FrenetPoint {
                    road_id: v.road_id.clone(),
                    s: v.s,
                })
            }),
            position: WayPoint::from(&value.position),
        }
    }
}

impl From<&libmodel::sanitation::task::GlobalPlannerArea>
    for GlobalPlannerArea
{
    fn from(value: &libmodel::sanitation::task::GlobalPlannerArea) -> Self {
        GlobalPlannerArea {
            name: value.name.clone(),
            timestamp: value.timestamp,
            r#loop: value.r#loop,
            clean_area_ids: value.clean_area_ids.clone(),
        }
    }
}
