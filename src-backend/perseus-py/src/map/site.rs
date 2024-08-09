use pyo3::prelude::*;

use crate::map::waypoint::WayPoint;

///
#[pyclass(set_all,get_all)]
#[derive(Clone)]
pub struct SiteInfo {
    pub id: i32, // 唯一ID
    pub name: String,
    pub r#type: SiteType,
    pub position: WayPoint,
}

#[pymethods]
impl SiteInfo {
    fn __str__(&self) -> String {
        format!(
            "SiteInfo (id:{},name:{}, type:{:?}, position:{:?})",
            self.id, &self.name, self.r#type, &self.position
        )
    }
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum SiteType {
    CleanSite,       // 清洗点
    ChargingSite,    // 充电站
    ParkingSite,     // 停车点
    TransferSite,    // 中转站
    WaterSupplySite, // 加水站
    MaintenanceSite, // 维修站
    GasSite,         // 加油站
}

impl From<&libmap::SiteInfo> for SiteInfo {
    fn from(value: &libmap::SiteInfo) -> Self {
        SiteInfo {
            id: value.id,
            name: value.name.clone(),
            r#type: match value.r#type {
                libmap::SiteType::CleanSite => SiteType::CleanSite,
                libmap::SiteType::ChargingSite => SiteType::ChargingSite,
                libmap::SiteType::ParkingSite => SiteType::ParkingSite,
                libmap::SiteType::TransferSite => SiteType::TransferSite,
                libmap::SiteType::WaterSupplySite => SiteType::WaterSupplySite,
                libmap::SiteType::MaintenanceSite => SiteType::MaintenanceSite,
                libmap::SiteType::GasSite => SiteType::GasSite,
            },
            position: WayPoint::from(&value.position),
        }
    }
}

impl Into<libmap::SiteInfo> for SiteInfo {
    fn into(self) -> libmap::SiteInfo {
        libmap::SiteInfo {
            id: self.id,
            name: self.name.clone(),
            r#type: match self.r#type {
                SiteType::CleanSite => libmap::SiteType::CleanSite,
                SiteType::ChargingSite => libmap::SiteType::ChargingSite,
                SiteType::ParkingSite => libmap::SiteType::ParkingSite,
                SiteType::TransferSite => libmap::SiteType::TransferSite,
                SiteType::WaterSupplySite => libmap::SiteType::WaterSupplySite,
                SiteType::MaintenanceSite => libmap::SiteType::MaintenanceSite,
                SiteType::GasSite => libmap::SiteType::GasSite,
            },
            position: (&self.position).into(),
        }
    }
}
