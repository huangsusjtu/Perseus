pub(crate) mod clean_area;
pub(crate) mod junction;
pub(crate) mod lane;
pub(crate) mod road;
pub(crate) mod site;
pub mod waypoint;

use pyo3::prelude::*;

use crate::map::clean_area::CleanArea;
use crate::map::junction::JunctionInfo;
use crate::map::road::RoadInfo;
use crate::map::site::SiteInfo;
use crate::map::waypoint::WayPoint;




#[pymodule]
pub fn map(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Map>()?;
    m.add_class::<WayPoint>()?;
    m.add_class::<crate::map::clean_area::CleanArea>()?;
    m.add_class::<crate::map::junction::JunctionInfo>()?;
    m.add_class::<crate::map::junction::ConnectionInfo>()?;
    m.add_class::<crate::map::lane::LaneInfo>()?;
    m.add_class::<crate::map::lane::LaneType>()?;
    m.add_class::<crate::map::road::RoadInfo>()?;
    m.add_class::<crate::map::road::RoadType>()?;
    m.add_class::<crate::map::road::RoadLink>()?;
    m.add_class::<crate::map::site::SiteInfo>()?;
    m.add_class::<crate::map::site::SiteType>()?;
    Ok(())
}

#[pyclass]
pub struct Map {
    pub map_inner: libmap::MapRef,
}

#[pymethods]
impl Map {
    fn get_roads(&mut self) -> PyResult<Vec<RoadInfo>> {
        Ok(self
            .map_inner
            .get_roads()
            .iter()
            .map(|v| RoadInfo::from(v.1.as_ref()))
            .collect())
    }
    fn get_junctions(&mut self) -> PyResult<Vec<JunctionInfo>> {
        Ok(self
            .map_inner
            .get_junctions()
            .iter()
            .map(|v| JunctionInfo::from(v.1.as_ref()))
            .collect())
    }
    fn get_sites(&mut self) -> PyResult<Vec<SiteInfo>> {
        Ok(self
            .map_inner
            .get_sites()
            .iter()
            .map(|v| SiteInfo::from(v.1.as_ref()))
            .collect())
    }
    fn get_areas(&mut self) -> PyResult<Vec<CleanArea>> {
        Ok(self
            .map_inner
            .get_clean_area()
            .iter()
            .map(|v| CleanArea::from(v.1.as_ref()))
            .collect())
    }

    // todo: add more api
}
