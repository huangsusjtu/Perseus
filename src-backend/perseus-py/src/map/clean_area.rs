use pyo3::prelude::*;

use crate::map::waypoint::WayPoint;

/// clean area which should clean by sanitation on map
#[pyclass(set_all,get_all)]
#[derive(Clone)]
pub struct CleanArea {
    pub id: i32,
    pub name: String,
    pub position: WayPoint,
    pub polygon: Vec<WayPoint>,
}

#[pymethods]
impl CleanArea {
    fn __repr__(&self) -> String {
        format!(
            "CleanArea (id:{},name:{}, position:{:?}) ",
            self.id, &self.name, self.position
        )
    }
    fn __str__(&self) -> String {
        format!(
            "CleanArea (id:{},name:{}, position:{:?})",
            self.id, &self.name, self.position
        )
    }
}

impl From<&libmap::CleanArea> for CleanArea {
    fn from(value: &libmap::CleanArea) -> Self {
        CleanArea {
            id: value.id,
            name: value.name.clone(),
            position: WayPoint::from(&value.position),
            polygon: if value.polygon.is_some() {
                vec![]
            } else {
                value
                    .polygon
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|v| WayPoint::from(v))
                    .collect()
            },
        }
    }
}
