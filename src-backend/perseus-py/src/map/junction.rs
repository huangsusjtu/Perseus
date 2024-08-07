use pyo3::{pyclass, pymethods, PyResult};

use crate::map::waypoint::WayPoint;

///
#[pyclass]
#[derive(Clone)]
pub struct JunctionInfo {
    pub id: i32, // 唯一ID
    pub name: String,
    pub center: WayPoint,       // 坐标
    pub polygon: Vec<WayPoint>, // 区域
    pub road_connections: Vec<ConnectionInfo>,
}
#[pyclass]
#[derive(Clone)]
pub struct ConnectionInfo {
    pub id: i32,
    pub road_in: String,
    pub road_out: String,
}

//
#[pymethods]
impl JunctionInfo {
    #[getter]
    fn id(&self) -> PyResult<i32> {
        Ok(self.id)
    }
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }
    #[getter]
    fn center(&self) -> PyResult<WayPoint> {
        Ok(self.center.clone())
    }
    #[getter]
    fn road_connections(&self) -> PyResult<Vec<ConnectionInfo>> {
        Ok(self.road_connections.clone())
    }

    fn __repr__(&self) -> String {
        format!(
            "JunctionInfo (id:{}, name:{}, center:{:?}) ",
            self.id, &self.name, self.center
        )
    }
    fn __str__(&self) -> String {
        format!(
            "JunctionInfo (id:{}, name:{}, center:{:?})",
            self.id, &self.name, self.center
        )
    }
}
#[pymethods]
impl ConnectionInfo {
    #[getter]
    fn id(&self) -> PyResult<i32> {
        Ok(self.id)
    }
    #[getter]
    fn road_in(&self) -> PyResult<String> {
        Ok(self.road_in.clone())
    }
    #[getter]
    fn road_out(&self) -> PyResult<String> {
        Ok(self.road_out.clone())
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
