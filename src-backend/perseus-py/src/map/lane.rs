use pyo3::{pyclass, pymethods, PyResult};

///
#[pyclass]
#[derive(Clone)]
pub struct LaneInfo {
    pub id: i32,
    pub r#type: LaneType,
    pub width: f32,
    // do not store LineCurveInfo in python world
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum LaneType {
    NONE,
    CityDriving,
    SideWalk,
    Shoulder,
    Bus,
    Biking,
    Parking,
    Bidirectional,
}

///
#[pymethods]
impl LaneInfo {
    #[getter]
    fn id(&self) -> PyResult<i32> {
        Ok(self.id)
    }
    #[getter]
    fn r#type(&self) -> PyResult<LaneType> {
        Ok(self.r#type.clone())
    }
    #[getter]
    fn width(&self) -> PyResult<f32> {
        Ok(self.width)
    }

    fn __repr__(&self) -> String {
        format!(
            "LaneInfo (id:{}, type:{:?}, width:{}) ",
            self.id, self.r#type, self.width
        )
    }
    fn __str__(&self) -> String {
        format!(
            "LaneInfo (id:{}, type:{:?}, width:{})",
            self.id, self.r#type, self.width
        )
    }
}

///
impl From<&libmap::LaneType> for LaneType {
    fn from(value: &libmap::LaneType) -> Self {
        match value {
            libmap::LaneType::NONE => LaneType::NONE,
            libmap::LaneType::CityDriving => LaneType::CityDriving,
            libmap::LaneType::SideWalk => LaneType::SideWalk,
            libmap::LaneType::Shoulder => LaneType::Shoulder,
            libmap::LaneType::Bus => LaneType::Bus,
            libmap::LaneType::Biking => LaneType::Biking,
            libmap::LaneType::Parking => LaneType::Parking,
            libmap::LaneType::Bidirectional => LaneType::Bidirectional,
        }
    }
}

impl From<&libmap::LaneInfo> for LaneInfo {
    fn from(value: &libmap::LaneInfo) -> Self {
        LaneInfo {
            id: value.id,
            r#type: LaneType::from(&value.r#type),
            width: value.width,
        }
    }
}
