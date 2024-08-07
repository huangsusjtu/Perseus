use serde::{Deserialize, Serialize};

/// https://publications.pages.asam.net/standards/ASAM_OpenDRIVE/ASAM_OpenDRIVE_Specification/latest/specification/15_railroads/15_01_introduction.html
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Railroad {
    pub switch: rail_road::Switch,
}

pub mod rail_road {
    use serde::{Deserialize, Serialize};

    use crate::opendrive::junction::junction::EElementDir;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Switch {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@position")]
        pub position: ERoadRailroadSwitchPosition,

        // children
        #[serde(rename = "mainTrack", skip_serializing_if = "Option::is_none")]
        pub main_track: Option<MainTrack>,
        #[serde(rename = "sideTrack", skip_serializing_if = "Option::is_none")]
        pub side_track: Option<SideTrack>,
        #[serde(rename = "partner", skip_serializing_if = "Option::is_none")]
        pub partner: Option<Partner>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct MainTrack {
        #[serde(rename = "@dir")]
        pub dir: EElementDir,
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@s")]
        pub s: f64,
    }

    pub type SideTrack = MainTrack;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Partner {
        #[serde(rename = "@id")]
        pub id: String,

        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ERoadRailroadSwitchPosition {
        #[default]
        Dynamic,
        Straight,
        Turn,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum EStationPlatformSegmentSide {
        #[default]
        Left,
        Right,
    }
}
