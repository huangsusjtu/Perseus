use std::ffi::CStr;

use pyo3::impl_::pyclass::{LazyTypeObject, PyClassItemsIter};
use pyo3::{pyclass, pymethods, PyResult, Python};

use crate::map::lane::LaneInfo;

///
#[pyclass]
#[derive(Clone)]
pub struct RoadInfo {
    pub id: i32, // 唯一ID
    pub name: String,
    pub length: f64,

    pub r#type: RoadType,

    // do not store LineCurveInfo in python world
    pub left_lanes: Vec<LaneInfo>,
    pub right_lanes: Vec<LaneInfo>,

    // 路的上下游
    pub link: Vec<RoadLink>,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum RoadType {
    Major = 1,    // 主道
    Minor = 2,    // 辅道
    SideWalk = 3, // 人行道
}

#[pyclass]
#[derive(Clone)]
pub struct RoadLink {
    pub link_type: String,    // predecessor or successor
    pub element_type: String, // junction or road
    pub element_id: i32,      // junction or road id
}

///
#[pymethods]
impl RoadInfo {
    #[getter]
    fn id(&self) -> PyResult<i32> {
        Ok(self.id)
    }
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }
    #[getter]
    fn length(&self) -> PyResult<f64> {
        Ok(self.length)
    }
    #[getter]
    fn r#type(&self) -> PyResult<RoadType> {
        Ok(self.r#type.clone())
    }
    #[getter]
    fn left(&self) -> PyResult<Vec<LaneInfo>> {
        Ok(self.left_lanes.clone())
    }
    #[getter]
    fn right(&self) -> PyResult<Vec<LaneInfo>> {
        Ok(self.right_lanes.clone())
    }
    #[getter]
    fn link(&self) -> PyResult<Vec<RoadLink>> {
        Ok(self.link.clone())
    }

    fn __repr__(&self) -> String {
        format!(
            "Road (id:{},name:{}, length:{}, type:{:?})",
            self.id, &self.name, self.length, self.r#type
        )
    }
    fn __str__(&self) -> String {
        format!(
            "Road (id:{},name:{}, length:{}, type:{:?})",
            self.id, &self.name, self.length, self.r#type
        )
    }
}

///
impl From<&libmap::RoadType> for RoadType {
    fn from(value: &libmap::RoadType) -> Self {
        match value {
            libmap::RoadType::Major => RoadType::Major,
            libmap::RoadType::Minor => RoadType::Minor,
            libmap::RoadType::SideWalk => RoadType::SideWalk,
        }
    }
}

impl From<&libmap::RoadLink> for RoadLink {
    fn from(value: &libmap::RoadLink) -> Self {
        RoadLink {
            link_type: value.link_type.clone(),
            element_type: value.element_type.clone(),
            element_id: value.element_id,
        }
    }
}

impl From<&libmap::RoadInfo> for RoadInfo {
    fn from(value: &libmap::RoadInfo) -> Self {
        RoadInfo {
            id: value.id,
            name: value.name.clone(),
            length: value.length,
            r#type: RoadType::from(&value.r#type),
            left_lanes: value
                .left_lanes
                .iter()
                .map(|v| LaneInfo::from(v))
                .collect(),
            right_lanes: value
                .right_lanes
                .iter()
                .map(|v| LaneInfo::from(v))
                .collect(),
            link: value.link.iter().map(|v| RoadLink::from(v)).collect(),
        }
    }
}
