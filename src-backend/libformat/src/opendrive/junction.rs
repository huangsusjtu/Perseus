use serde::{Deserialize, Serialize};

use crate::opendrive::common;
use crate::opendrive::common::planview;

///
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Junction {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<junction::EJunctionType>,
    #[serde(rename = "@name")]
    pub name: Option<String>,

    // crossing, default, direct, virtual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<common::Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<junction::Controller>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<junction::Surface>,
    #[serde(rename = "planView", skip_serializing_if = "Option::is_none")]
    pub plan_view: Option<planview::PlanView>,

    #[serde(rename = "roadSection", skip_serializing_if = "Option::is_none")]
    pub road_section: Option<junction::RoadSection>,
    #[serde(rename = "connection")]
    pub connection: Vec<junction::Connection>,
    #[serde(rename = "crossPath", skip_serializing_if = "Option::is_none")]
    pub cross_path: Option<Vec<junction::CrossPath>>,
    #[serde(rename = "boundary", skip_serializing_if = "Option::is_none")]
    pub boundary: Option<junction::Boundary>,
    #[serde(rename = "elevationGrid", skip_serializing_if = "Option::is_none")]
    pub elevation_grid: Option<junction::ElevationGrid>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct JunctionGroup {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub r#type: junction::EJunctionGroupType,
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@junctionReference")]
    pub junction_reference: Option<Vec<junction::JunctionReference>>,
}

pub mod junction {
    use serde::{Deserialize, Serialize};

    use crate::opendrive::road::road::EDirection;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct RoadSection {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@roadId")]
        pub road_id: String,
        #[serde(rename = "@sStart")]
        pub s_start: f64,
        #[serde(rename = "@sEnd")]
        pub s_end: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Controller {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@sequence")]
        pub sequence: Option<u32>,
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Surface {
        #[serde(rename = "@CRG", skip_serializing_if = "Option::is_none")]
        pub crg: Option<CRG>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct CRG {
        #[serde(rename = "@file")]
        pub file: String,
        #[serde(rename = "@hOffset", skip_serializing_if = "Option::is_none")]
        pub h_offset: Option<f64>,
        #[serde(rename = "@sOffset", skip_serializing_if = "Option::is_none")]
        pub s_offset: Option<f64>,
        #[serde(rename = "@tOffset", skip_serializing_if = "Option::is_none")]
        pub t_offset: Option<f64>,
        #[serde(rename = "@zOffset", skip_serializing_if = "Option::is_none")]
        pub z_offset: Option<f64>,
        #[serde(rename = "@zScale", skip_serializing_if = "Option::is_none")]
        pub z_scale: Option<f64>,

        #[serde(rename = "@mode")]
        pub mode: ERoadSurfaceCrgMode,
        #[serde(rename = "@orientation")]
        pub orientation: EDirection,
        #[serde(rename = "@purpose", skip_serializing_if = "Option::is_none")]
        pub purpose: Option<ERoadSurfaceCrgPurpose>,
        #[serde(rename = "@sStart")]
        pub s_start: f64,
        #[serde(rename = "@sEnd")]
        pub s_end: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Connection {
        #[serde(rename = "@id")]
        pub id: String,

        #[serde(rename = "@incomingRoad", skip_serializing_if = "Option::is_none")]
        pub incoming_road: Option<String>,
        #[serde(rename = "@connectingRoad", skip_serializing_if = "Option::is_none")]
        pub connecting_road: Option<String>,

        #[serde(rename = "@linkedRoad", skip_serializing_if = "Option::is_none")]
        pub linked_road: Option<String>,

        #[serde(rename = "@contactPoint", skip_serializing_if = "Option::is_none")]
        pub contact_point: Option<EContactPoint>,

        #[serde(rename = "laneLink", skip_serializing_if = "Option::is_none")]
        pub lane_link: Option<Vec<LaneLink>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct LaneLink {
        #[serde(rename = "@from")]
        pub from: i32,
        #[serde(rename = "@to")]
        pub to: i32,
        #[serde(rename = "@overlapZone", skip_serializing_if = "Option::is_none")]
        pub overlap_zone: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct CrossPath {
        #[serde(rename = "@crossingRoad")]
        pub crossing_road: String,
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@roadAtStart")]
        pub road_at_start: String,
        #[serde(rename = "@roadAtEnd")]
        pub road_at_end: String,

        #[serde(rename = "start_lane_link", skip_serializing_if = "Option::is_none")]
        pub start_lane_link: Option<StartLaneLink>,
        #[serde(rename = "endLaneLink", skip_serializing_if = "Option::is_none")]
        pub end_lane_link: Option<EndLaneLink>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct StartLaneLink {
        #[serde(rename = "@from")]
        pub from: i32,
        #[serde(rename = "@to")]
        pub to: i32,
        #[serde(rename = "@s")]
        pub s: f64,
    }

    pub type EndLaneLink = StartLaneLink;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Boundary {
        pub segment: Vec<Segment>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Segment {
        #[serde(rename = "@type")]
        pub r#type: EJunctionSegmentType, // type="lane", type="joint"
        #[serde(rename = "@roadId")]
        pub road_id: String,

        #[serde(rename = "@boundaryLane", skip_serializing_if = "Option::is_none")]
        boundary_lane: Option<i32>,
        #[serde(rename = "@sStart", skip_serializing_if = "Option::is_none")]
        s_start: Option<String>,
        #[serde(rename = "@sEnd", skip_serializing_if = "Option::is_none")]
        s_end: Option<String>,

        #[serde(rename = "@contactPoint", skip_serializing_if = "Option::is_none")]
        contact_point: Option<f64>,
        #[serde(rename = "@jointLaneStart", skip_serializing_if = "Option::is_none")]
        joint_lane_start: Option<i32>,
        #[serde(rename = "@jointLaneEnd", skip_serializing_if = "Option::is_none")]
        joint_lane_end: Option<i32>,
        #[serde(rename = "@transitionLength", skip_serializing_if = "Option::is_none")]
        transition_length: Option<f64>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct ElevationGrid {
        #[serde(rename = "@gridSpacing")]
        pub grid_spacing: f64,
        #[serde(rename = "@sStart")]
        pub s_start: f64,

        pub elevation: Vec<Elevation>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Elevation {
        #[serde(rename = "@center")]
        pub center: Vec<f64>,
        #[serde(rename = "@left")]
        pub left: Option<Vec<f64>>,
        #[serde(rename = "@right")]
        pub right: Option<Vec<f64>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct JunctionReference {
        pub junction: String,
    }

    // list enum
    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum EJunctionType {
        #[default]
        Crossing,
        DEFAULT,
        Direct,
        Virtual,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ERoadSurfaceCrgPurpose {
        #[default]
        Elevation,
        Friction,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ERoadSurfaceCrgMode {
        #[default]
        Attached0,
        Attached,
        Genuine,
        Global,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum EJunctionSegmentType {
        #[default]
        Joint,
        Lane,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum EJunctionGroupType {
        #[default]
        ComplexJunction,
        HighwayInterchange,
        Roundabout,
        Unknown,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EElementDir {
        #[default]
        #[serde(rename = "+")]
        Plus,
        #[serde(rename = "_")]
        Minus,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum EContactPoint {
        #[default]
        Start,
        End,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum EConnectionType {
        #[default]
        Default,
        Virtual,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works1() {
        // let r = road::RoadType::default();
        // let xml = quick_xml::se::to_string(&r).unwrap();
        // println!("{:#?}", xml);
    }

    #[test]
    fn it_works2() {
        // let mut r = Road::default();
        // r.r#type = road::RoadType::default();
        // let xml = quick_xml::se::to_string(&r).unwrap();
        // println!("{:#?}", xml);
    }
}
