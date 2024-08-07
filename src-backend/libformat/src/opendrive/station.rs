use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Station {
    #[serde(rename = "@id")]
    pub id: i32,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub r#type: station::EStationType,

    #[serde(rename = "platform")]
    pub platform: Vec<station::Platform>,

    // custom, extention
    #[serde(rename = "location")]
    pub location: Option<station::Location>,
}

pub mod station {
    use serde::{Deserialize, Serialize};

    use crate::opendrive::railroad::rail_road::EStationPlatformSegmentSide;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Platform {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        #[serde(rename = "segment")]
        pub segment: Vec<Segment>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Segment {
        #[serde(rename = "@roadId")]
        pub road_id: String,
        #[serde(rename = "@sStart")]
        pub s_start: String,
        #[serde(rename = "@sEnd")]
        pub s_end: String,
        #[serde(rename = "@side")]
        pub side: EStationPlatformSegmentSide,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum EStationType {
        #[default]
        Large,
        Medium,
        Small,
        CleanSite,       // 清洗点
        ChargingSite,    // 充电站
        ParkingSite,     // 停车点
        TransferSite,    // 中转站
        WaterSupplySite, // 加水站
        MaintenanceSite, // 维修站
        GasSite,         // 加油站
    }

    // custom, extention
    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Location {
        #[serde(rename = "$text")]
        pub location: (f64, f64),
    }
}
