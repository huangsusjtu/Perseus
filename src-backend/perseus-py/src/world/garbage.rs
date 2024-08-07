use pyo3::{pyclass, pymethods, PyResult};

use crate::map::waypoint::WayPoint;

#[pyclass(set_all)]
pub struct GarbageConfig {}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum GarbageType {
    FallenLeaves,
    WhiteTrash,
}

#[pyclass(set_all)]
pub struct Garbage {
    pub theta: f32,
    pub width: f32,
    pub length: f32,
    pub position: WayPoint,
    pub r#type: GarbageType,
}

///
#[pymethods]
impl GarbageConfig {}

#[pymethods]
impl Garbage {
    #[getter]
    fn theta(&self) -> PyResult<f32> {
        Ok(self.theta)
    }
    #[getter]
    fn width(&self) -> PyResult<f32> {
        Ok(self.width)
    }
    #[getter]
    fn length(&self) -> PyResult<f32> {
        Ok(self.length)
    }
    #[getter]
    fn position(&self) -> PyResult<WayPoint> {
        Ok(self.position.clone())
    }
    #[getter]
    fn r#type(&self) -> PyResult<GarbageType> {
        Ok(self.r#type.clone())
    }
}

///
impl From<&libmodel::sanitation::garbage::GarbageConfig> for GarbageConfig {
    fn from(_value: &libmodel::sanitation::garbage::GarbageConfig) -> Self {
        GarbageConfig {}
    }
}

impl From<&libmodel::sanitation::garbage::Garbage> for Garbage {
    fn from(value: &libmodel::sanitation::garbage::Garbage) -> Self {
        Garbage {
            theta: value.theta,
            width: value.width,
            length: value.length,
            position: WayPoint {
                x: value.position.x,
                y: value.position.y,
            },
            r#type: GarbageType::FallenLeaves,
        }
    }
}
