
use libformat::opendrive::station::station::Location;

use crate::common::util::LanePoint;
use crate::proto_gen;

/// 定义 公司的各种类型的 作业补给站点
#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub enum SiteType {
    CleanSite,       // 清洗点
    ChargingSite,    // 充电站
    ParkingSite,     // 停车点
    TransferSite,    // 中转站
    WaterSupplySite, // 加水站
    MaintenanceSite, // 维修站
    GasSite,         // 加油站
}


#[derive(Debug)]
pub struct SiteInfo {
    pub id: i32, // 唯一ID
    pub name: String,
    pub r#type: SiteType,
    pub position: LanePoint,
}

///  opendrive 格式的对接
impl From<&libformat::opendrive::station::Station> for SiteInfo {
    fn from(value: &libformat::opendrive::station::Station) -> Self {
        SiteInfo {
            id: value.id,
            name: value.name.to_string(),
            r#type: match value.r#type {
                libformat::opendrive::station::station::EStationType::CleanSite => {
                    SiteType::CleanSite
                }
                libformat::opendrive::station::station::EStationType::ChargingSite => {
                    SiteType::ChargingSite
                }
                libformat::opendrive::station::station::EStationType::ParkingSite => {
                    SiteType::ParkingSite
                }
                libformat::opendrive::station::station::EStationType::TransferSite => {
                    SiteType::TransferSite
                }
                libformat::opendrive::station::station::EStationType::WaterSupplySite => {
                    SiteType::WaterSupplySite
                }
                libformat::opendrive::station::station::EStationType::MaintenanceSite => {
                    SiteType::MaintenanceSite
                }
                libformat::opendrive::station::station::EStationType::GasSite => SiteType::GasSite,
                _ => SiteType::CleanSite,
            },

            position: {
                let l = value.location.as_ref().unwrap();
                LanePoint::new(l.location.0, l.location.1)
            },
        }
    }
}

impl Into<libformat::opendrive::station::Station> for &SiteInfo {
    fn into(self) -> libformat::opendrive::station::Station {
        libformat::opendrive::station::Station {
            id: self.id,
            name: self.name.to_string(),
            r#type: match self.r#type {
                SiteType::CleanSite => {
                    libformat::opendrive::station::station::EStationType::CleanSite
                }
                SiteType::ChargingSite => {
                    libformat::opendrive::station::station::EStationType::ChargingSite
                }
                SiteType::ParkingSite => {
                    libformat::opendrive::station::station::EStationType::ParkingSite
                }
                SiteType::TransferSite => {
                    libformat::opendrive::station::station::EStationType::TransferSite
                }
                SiteType::WaterSupplySite => {
                    libformat::opendrive::station::station::EStationType::WaterSupplySite
                }
                SiteType::MaintenanceSite => {
                    libformat::opendrive::station::station::EStationType::MaintenanceSite
                }
                SiteType::GasSite => libformat::opendrive::station::station::EStationType::GasSite,
            },
            platform: vec![], // todo
            location: Some(Location {
                location: (self.position.x, self.position.y),
            }),
        }
    }
}

/// 自定义的map proto格式，用于前后端传输的
impl From<proto_gen::map::Site> for SiteInfo {
    fn from(value: proto_gen::map::Site) -> Self {
        return SiteInfo::from(&value);
    }
}

impl From<&proto_gen::map::Site> for SiteInfo {
    fn from(value: &proto_gen::map::Site) -> Self {
        SiteInfo {
            id: value.id,
            name: value.name.to_string(),
            r#type: match value.type_.unwrap() {
                proto_gen::map::site::SiteType::None => SiteType::CleanSite,
                proto_gen::map::site::SiteType::CleanSite => {
                    SiteType::CleanSite
                }
                proto_gen::map::site::SiteType::ChargingSite => {
                    SiteType::ChargingSite
                }
                proto_gen::map::site::SiteType::ParkingSite => {
                    SiteType::ParkingSite
                }
                proto_gen::map::site::SiteType::TransferSite => {
                    SiteType::TransferSite
                }
                proto_gen::map::site::SiteType::WaterSupplySite => {
                    SiteType::WaterSupplySite
                }
                proto_gen::map::site::SiteType::MaintenanceSite => {
                    SiteType::MaintenanceSite
                }
                proto_gen::map::site::SiteType::GasSite => SiteType::GasSite,
            },
            position: LanePoint::from(
                value.position.as_ref().unwrap_or_default(),
            ),
        }
    }
}

impl Into<proto_gen::map::Site> for &SiteInfo {
    fn into(self) -> proto_gen::map::Site {
        proto_gen::map::Site {
            id: self.id,
            name: self.name.to_string(),
            type_: {
                protobuf::EnumOrUnknown::new(match self.r#type {
                    SiteType::CleanSite => {
                        proto_gen::map::site::SiteType::CleanSite
                    }
                    SiteType::ChargingSite => {
                        proto_gen::map::site::SiteType::ChargingSite
                    }
                    SiteType::ParkingSite => {
                        proto_gen::map::site::SiteType::ParkingSite
                    }
                    SiteType::TransferSite => {
                        proto_gen::map::site::SiteType::TransferSite
                    }
                    SiteType::WaterSupplySite => {
                        proto_gen::map::site::SiteType::WaterSupplySite
                    }
                    SiteType::MaintenanceSite => {
                        proto_gen::map::site::SiteType::MaintenanceSite
                    }
                    SiteType::GasSite => {
                        proto_gen::map::site::SiteType::GasSite
                    }
                })
            },
            position: protobuf::MessageField::some((&self.position).into()),
            special_fields: Default::default(),
        }
    }
}
