use pyo3::{pyclass, pymethods, PyResult};

use crate::map::waypoint::WayPoint;

///
#[pyclass]
#[derive(Clone)]
pub struct CleanArea {
    pub id: i32,
    pub name: String,
    pub position: WayPoint,
    pub polygon: Vec<WayPoint>,
}

#[pymethods]
impl CleanArea {
    #[getter]
    fn id(&self) -> PyResult<i32> {
        Ok(self.id)
    }
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }
    #[getter]
    fn position(&self) -> PyResult<WayPoint> {
        Ok(self.position.clone())
    }
    #[getter]
    fn polygon(&self) -> PyResult<Vec<WayPoint>> {
        Ok(self.polygon.clone())
    }

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
