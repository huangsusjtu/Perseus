use pyo3::prelude::*;

use crate::map::waypoint::WayPoint;

/// junction which connect road
#[pyclass(set_all, get_all)]
#[derive(Clone)]
pub struct JunctionInfo {
    pub id: i32, // 唯一ID
    pub name: String,
    pub center: WayPoint,       // 坐标
    pub polygon: Vec<WayPoint>, // 区域
    pub road_connections: Vec<ConnectionInfo>,
}

/// junction connection info which road in and out
#[pyclass(set_all, get_all)]
#[derive(Clone)]
pub struct ConnectionInfo {
    pub id: i32,
    pub road_in: String,
    pub road_out: String,
}

//
#[pymethods]
impl JunctionInfo {
    fn __str__(&self) -> String {
        format!(
            "JunctionInfo (id:{}, name:{}, center:{:?})",
            self.id, &self.name, self.center
        )
    }
}
#[pymethods]
impl ConnectionInfo {
    fn __str__(&self) -> String {
        format!(
            "ConnectionInfo (id:{}, road_in:{}, road_out:{:?})",
            self.id, self.road_in, self.road_out
        )
    }
}

///
impl From<&libmap::ConnectionInfo> for ConnectionInfo {
    fn from(value: &libmap::ConnectionInfo) -> Self {
        ConnectionInfo {
            id: value.id,
            road_in: value.road_in.clone(),
            road_out: value.road_out.clone(),
        }
    }
}
impl From<&libmap::JunctionInfo> for JunctionInfo {
    fn from(value: &libmap::JunctionInfo) -> Self {
        JunctionInfo {
            id: value.id,
            name: value.name.clone(),
            center: WayPoint::from(&value.center),
            polygon: value.polygon.iter().map(|v| WayPoint::from(v)).collect(),
            road_connections: value
                .road_connections
                .iter()
                .map(|v| ConnectionInfo::from(v))
                .collect(),
        }
    }
}
