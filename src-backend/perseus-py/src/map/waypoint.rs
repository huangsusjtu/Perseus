use pyo3::{pyclass, pymethods, PyResult};

///
#[pyclass(set_all)]
#[derive(Clone, Debug)]
pub struct WayPoint {
    pub x: f64,
    pub y: f64,
}

///
#[pymethods]
impl WayPoint {
    #[getter]
    fn x(&self) -> PyResult<f64> {
        Ok(self.x)
    }
    #[getter]
    fn y(&self) -> PyResult<f64> {
        Ok(self.y)
    }

    fn __repr__(&self) -> String {
        format!("WayPoint (x:{}, y:{})", self.x, self.y)
    }
    fn __str__(&self) -> String {
        format!("WayPoint (x:{}, y:{})", self.x, self.y)
    }
}

///
impl From<&libmap::common::util::Vec2d> for WayPoint {
    fn from(value: &libmap::common::util::Vec2d) -> Self {
        WayPoint {
            x: value.x,
            y: value.y,
        }
    }
}

impl Into<libmap::common::util::Vec2d> for &WayPoint {
    fn into(self) -> libmap::common::util::Vec2d {
        libmap::common::util::Vec2d {
            x: self.x,
            y: self.y,
        }
    }
}
