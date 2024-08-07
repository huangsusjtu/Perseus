mod clean_area;
mod junction;
mod lane;
mod road;
mod site;
pub mod waypoint;

use pyo3::prelude::*;

use crate::map::clean_area::CleanArea;
use crate::map::junction::{ConnectionInfo, JunctionInfo};
use crate::map::lane::{LaneInfo, LaneType};
use crate::map::road::{RoadInfo, RoadLink, RoadType};
use crate::map::site::{SiteInfo, SiteType};
use crate::map::waypoint::WayPoint;

pub(crate) fn mod_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Map>()?;

    m.add_class::<WayPoint>()?;
    m.add_class::<CleanArea>()?;
    m.add_class::<JunctionInfo>()?;
    m.add_class::<ConnectionInfo>()?;
    m.add_class::<LaneInfo>()?;
    m.add_class::<LaneType>()?;
    m.add_class::<RoadInfo>()?;
    m.add_class::<RoadType>()?;
    m.add_class::<RoadLink>()?;
    m.add_class::<SiteInfo>()?;
    m.add_class::<SiteType>()?;
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
