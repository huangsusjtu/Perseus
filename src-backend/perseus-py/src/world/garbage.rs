use pyo3::prelude::*;

use crate::map::waypoint::WayPoint;

#[pyclass(set_all)]
pub struct GarbageConfig {}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum GarbageType {
    FallenLeaves,
    WhiteTrash,
}

/// 描述垃圾对象
#[pyclass(set_all, get_all)]
pub struct Garbage {
    pub theta: f32,
    pub width: f32,
    pub length: f32,
    pub position: WayPoint,
    pub r#type: GarbageType,
}

/// 全局的垃圾配置信息
#[pymethods]
impl GarbageConfig {
    #[new]
    fn new() -> Self {
        GarbageConfig {}
    }

    fn __str__(&self) -> String {
        "GarbageConfig () ".to_string()
    }
}

#[pymethods]
impl Garbage {
    #[new]
    fn new() -> Self {
        Garbage {
            theta: 0.0,
            width: 0.0,
            length: 0.0,
            position: WayPoint { x: 0.0, y: 0.0 },
            r#type: GarbageType::FallenLeaves,
        }
    }

    fn __str__(&self) -> String {
        format!(
            "Garbage (theta:{:?}, width:{:?}, length:{:?}, position:{:?}, \
             type:{:?}) ",
            self.theta, self.width, self.length, self.position, self.r#type
        )
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
