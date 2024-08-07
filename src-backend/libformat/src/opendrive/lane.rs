use serde::{Deserialize, Serialize};

/// https://publications.pages.asam.net/standards/ASAM_OpenDRIVE/ASAM_OpenDRIVE_Specification/latest/specification/11_lanes/11_01_introduction.html#top-9c2a8ffd-2bf4-4fa9-b861-0fbf9cd6817b
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Lanes {
    #[serde(rename = "laneOffset", skip_serializing_if = "Option::is_none")]
    pub lane_offset: Option<Vec<lane::LaneOffset>>,
    #[serde(rename = "laneSection")]
    pub lane_section: Vec<lane::LaneSection>,
}

pub mod lane {
    use serde::{Deserialize, Serialize};

    use crate::opendrive::common::core::EUnitSpeed;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct LaneOffset {
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@a")]
        pub a: f64,
        #[serde(rename = "@b")]
        pub b: f64,
        #[serde(rename = "@c")]
        pub c: f64,
        #[serde(rename = "@d")]
        pub d: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct LaneSection {
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@singleSide")]
        pub single_side: Option<TBool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub left: Option<Left>,
        #[serde(rename = "center")]
        pub center: Center,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub right: Option<Right>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct LaneWrapper {
        pub lane: Vec<Lane>,
    }

    pub type Left = LaneWrapper;
    pub type Right = LaneWrapper;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Center {
        pub lane: Vec<CenterLane>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct CenterLane {
        #[serde(rename = "@id")]
        pub id: i32,
        #[serde(rename = "@level", skip_serializing_if = "Option::is_none")]
        pub level: Option<TBool>,
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<ELaneType>,

        //
        #[serde(skip_serializing_if = "Option::is_none")]
        pub link: Option<Vec<Link>>,
        #[serde(rename = "roadMark", skip_serializing_if = "Option::is_none")]
        pub road_mark: Option<Vec<RoadMark>>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Lane {
        // attr
        #[serde(rename = "@id")]
        pub id: i32,
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<ELaneType>,
        #[serde(rename = "@advisory", skip_serializing_if = "Option::is_none")]
        pub advisory: Option<ELaneAdvisory>,
        #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
        pub direction: Option<ELaneDirection>,
        #[serde(
            rename = "@dynamicLaneDirection",
            skip_serializing_if = "Option::is_none"
        )]
        pub dynamic_lane_direction: Option<TBool>,
        #[serde(rename = "@dynamicLaneType", skip_serializing_if = "Option::is_none")]
        pub dynamic_lane_type: Option<TBool>,
        #[serde(rename = "@level", skip_serializing_if = "Option::is_none")]
        pub level: Option<TBool>,
        #[serde(rename = "@roadWorks", skip_serializing_if = "Option::is_none")]
        pub road_works: Option<TBool>,

        // elements for left/center/right lane
        #[serde(skip_serializing_if = "Option::is_none")]
        pub link: Option<Vec<Link>>,
        #[serde(rename = "roadMark", skip_serializing_if = "Option::is_none")]
        pub road_mark: Option<Vec<RoadMark>>,

        // elements for left/right
        #[serde(skip_serializing_if = "Option::is_none")]
        pub width: Option<Vec<LaneWidth>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub border: Option<LaneBorder>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub material: Option<Vec<Material>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub speed: Option<Speed>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access: Option<Vec<Access>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub height: Option<Vec<Height>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rule: Option<Rule>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct PolynomialThirdOrder {
        // Polynom parameter a, width at @s (ds=0)   unit:m
        #[serde(rename = "@a")]
        pub a: f64,
        // Polynom parameter b
        #[serde(rename = "@b")]
        pub b: f64,
        // Polynom parameter c
        #[serde(rename = "@c")]
        pub c: f64,
        // Polynom parameter d
        #[serde(rename = "@d")]
        pub d: f64,
        // s-coordinate of start position of the <width> element, relative to
        // the position of the preceding <laneSection> element
        #[serde(rename = "@sOffset")]
        pub s_offset: f64,
    }

    pub type LaneWidth = PolynomialThirdOrder;
    pub type LaneBorder = PolynomialThirdOrder;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Link {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub predecessor: Option<Vec<Predecessor>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub successor: Option<Vec<Successor>>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Predecessor {
        #[serde(rename = "@id")]
        pub id: i32,
    }

    pub type Successor = Predecessor;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct RoadMark {
        #[serde(rename = "@color")]
        pub color: ERoadMarkColor,
        #[serde(rename = "@sOffset")]
        pub s_offset: f64,
        #[serde(rename = "@type")]
        pub r#type: ERoadMarkType,

        #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
        pub height: Option<f64>,
        #[serde(rename = "@laneChange", skip_serializing_if = "Option::is_none")]
        pub lane_change: Option<ERoadLanesLaneSectionLcrLaneRoadMarkLaneChange>,
        #[serde(rename = "@material", skip_serializing_if = "Option::is_none")]
        pub material: Option<String>,
        #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
        pub weight: Option<ERoadMarkWeight>,
        #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
        pub width: Option<f64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub sway: Option<road_mark::Sway>,
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub r#type1: Option<road_mark::RoadMarkType>,
        #[serde(rename = "explicit", skip_serializing_if = "Option::is_none")]
        pub explicit: Option<road_mark::Explicit>,
    }

    pub mod road_mark {
        use serde::{Deserialize, Serialize};

        use crate::opendrive::lane::lane::{ERoadMarkColor, ERoadMarkRule};

        #[derive(Deserialize, Serialize, Debug, Default)]
        pub struct RoadMarkType {
            #[serde(rename = "@name")]
            pub name: String,
            #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
            pub width: Option<f64>,

            pub line: Vec<RoadMarkTypeLine>,
        }

        #[derive(Deserialize, Serialize, Debug, Default)]
        pub struct RoadMarkTypeLine {
            #[serde(rename = "@length")]
            pub length: f64,
            #[serde(rename = "@space")]
            pub space: f64,

            #[serde(rename = "@tOffset", skip_serializing_if = "Option::is_none")]
            pub t_offset: Option<f64>,
            #[serde(rename = "@sOffset", skip_serializing_if = "Option::is_none")]
            pub s_offset: Option<f64>,

            #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
            pub color: Option<ERoadMarkColor>,
            #[serde(rename = "@rule", skip_serializing_if = "Option::is_none")]
            pub rule: Option<ERoadMarkRule>,
            #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
            pub width: Option<f64>,
        }

        #[derive(Deserialize, Serialize, Debug, Default)]
        pub struct Sway {
            #[serde(rename = "@a")]
            pub a: f64,
            #[serde(rename = "@b")]
            pub b: f64,
            #[serde(rename = "@c")]
            pub c: f64,
            #[serde(rename = "@d")]
            pub d: f64,
            #[serde(rename = "@ds")]
            pub ds: f64,
        }

        #[derive(Deserialize, Serialize, Debug, Default)]
        pub struct Explicit {
            pub line: Vec<RoadMarkExplicitLine>,
        }

        #[derive(Deserialize, Serialize, Debug, Default)]
        pub struct RoadMarkExplicitLine {
            #[serde(rename = "@length")]
            pub length: f64,
            #[serde(rename = "@sOffset")]
            pub s_offset: f64,
            #[serde(rename = "@tOffset")]
            pub t_offset: f64,

            #[serde(rename = "@rule", skip_serializing_if = "Option::is_none")]
            pub rule: Option<ERoadMarkRule>,
            #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
            pub width: Option<f64>,
        }
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Material {
        #[serde(rename = "@friction")]
        pub friction: f64,
        #[serde(rename = "@sOffset")]
        pub s_offset: f64,

        #[serde(rename = "@roughness", skip_serializing_if = "Option::is_none")]
        pub roughness: Option<f64>,
        #[serde(rename = "@surface", skip_serializing_if = "Option::is_none")]
        pub surface: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Speed {
        #[serde(rename = "@max")]
        pub max: f64, // >=0
        #[serde(rename = "@sOffset")]
        pub s_offset: f64,
        #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
        pub unit: Option<EUnitSpeed>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Access {
        #[serde(rename = "@sOffset")]
        pub s_offset: f64, // >=0
        #[serde(rename = "@restriction", skip_serializing_if = "Option::is_none")]
        pub restriction: Option<EAccessRestrictionType>,
        #[serde(rename = "@rule", skip_serializing_if = "Option::is_none")]
        pub rule: Option<ERoadLanesLaneSectionLrLaneAccessRule>,

        #[serde(rename = "restriction")]
        pub children: Vec<Restriction>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Restriction {
        #[serde(rename = "@type")]
        pub r#type: EAccessRestrictionType,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Height {
        #[serde(rename = "@inner")]
        pub inner: f64,
        #[serde(rename = "@outer")]
        pub outer: f64,
        #[serde(rename = "@sOffset")]
        pub s_offset: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Rule {
        #[serde(rename = "@sOffset")]
        pub s_offset: f64,
        // Free text; currently recommended values are
        // "no stopping at any time"
        // "disabled parking"
        // "car pool"
        #[serde(rename = "@value")]
        pub value: String,
    }

    // enum list
    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ERoadMarkWeight {
        #[default]
        Standard,
        Bold,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ELaneAdvisory {
        #[default]
        Both,
        Inner,
        None,
        Outer,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum TBool {
        #[default]
        False,
        True,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum ERoadMarkType {
        #[default]
        #[serde(rename = "botts dots")]
        BottsDots,
        #[serde(rename = "broken broken")]
        BrokenBroken,
        #[serde(rename = "broken solid")]
        BrokeSolid,
        #[serde(rename = "broken")]
        Broke,
        #[serde(rename = "curb")]
        Curb,
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "edge")]
        Edge,
        #[serde(rename = "grass")]
        Grass,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "solid broken")]
        SolidBroken,
        #[serde(rename = "solid solid")]
        SolidSolid,
        #[serde(rename = "solid")]
        Solid,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum EAccessRestrictionType {
        #[default]
        #[serde(rename = "HOV")]
        Hov,
        #[serde(rename = "autonomousTraffic")]
        AutonomousTraffic,
        #[serde(rename = "bicycle")]
        Bicycle,
        #[serde(rename = "bus")]
        Bus,
        #[serde(rename = "delivery")]
        Delivery,
        #[serde(rename = "emergency")]
        Emergency,
        #[serde(rename = "motorcycle")]
        Motorcycle,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "passengerCar")]
        PassengerCar,
        #[serde(rename = "pedestrian")]
        Pedestrian,
        #[serde(rename = "simulator")]
        Simulator,
        #[serde(rename = "taxi")]
        Taxi,
        #[serde(rename = "throughTraffic")]
        ThroughTraffic,
        #[serde(rename = "truck")]
        Truck,
        #[serde(rename = "trucks")]
        Trucks,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ERoadLanesLaneSectionLrLaneAccessRule {
        #[default]
        Allow,
        Deny,
    }

    #[derive(Deserialize, Serialize, Debug, Default, Clone)]
    pub enum ELaneType {
        #[serde(rename = "HOV")]
        Hov,
        #[serde(rename = "bidirectional")]
        Bidirectional,
        #[serde(rename = "biking")]
        Biking,
        #[serde(rename = "border")]
        Border,
        #[serde(rename = "bus")]
        Bus,
        #[serde(rename = "connectingRamp")]
        ConnectingRamp,
        #[serde(rename = "curb")]
        Curb,
        #[default]
        #[serde(rename = "driving")]
        Driving,
        #[serde(rename = "entry")]
        Entry,
        #[serde(rename = "exit")]
        Exit,
        #[serde(rename = "median")]
        Median,
        #[serde(rename = "mwyEntry")]
        MwyEntry,
        #[serde(rename = "mwyExit")]
        MwyExit,
        #[serde(rename = "none")]
        None,
        #[serde(rename = "offRamp")]
        OffRamp,
        #[serde(rename = "onRamp")]
        OnRamp,
        #[serde(rename = "parking")]
        Parking,
        #[serde(rename = "rail")]
        Rail,
        #[serde(rename = "restricted")]
        Restricted,
        #[serde(rename = "roadWorks")]
        RoadWorks,
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "shoulder")]
        Shoulder,
        #[serde(rename = "sidewalk")]
        Sidewalk,
        #[serde(rename = "slipLane")]
        SlipLane,
        #[serde(rename = "special1")]
        Special1,
        #[serde(rename = "special2")]
        Special2,
        #[serde(rename = "special3")]
        Special3,
        #[serde(rename = "stop")]
        Stop,
        #[serde(rename = "taxi")]
        Taxi,
        #[serde(rename = "tram")]
        Tram,
        #[serde(rename = "walking")]
        Walking,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ERoadLanesLaneSectionLcrLaneRoadMarkLaneChange {
        #[default]
        Both,
        Decrease,
        Increase,
        None,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ERoadMarkColor {
        #[default]
        Black,
        Blue,
        Green,
        Orange,
        Red,
        Standard,
        Violet,
        White,
        Yellow,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ELaneDirection {
        Both,
        Reversed,
        #[default]
        Standard,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum ERoadMarkRule {
        #[serde(rename = "caution")]
        Caution,
        #[serde(rename = "no passing")]
        NoPassing,
        #[default]
        #[serde(rename = "none")]
        None,
    }
}

#[cfg(test)]
mod tests {
    use crate::opendrive::lane::lane::RoadMark;

    #[test]
    fn it_works5() {
        let xml = r#"
                <roadMark sOffset="0" type="broken broken" weight="standard" color="standard" height="0.02">
                    <type name="broken broken">

                    </type>

                </roadMark>
        "#;
        let r = quick_xml::de::from_str::<RoadMark>(&xml).unwrap();
        println!("{:#?}", xml);
    }

    #[test]
    fn it_works6() {
        let xml = r#"
               <line length="3" space="3" tOffset="-0.2" width="0.2" sOffset="4.0" />
        "#;
        let r =
            quick_xml::de::from_str::<crate::opendrive::lane::lane::road_mark::RoadMarkTypeLine>(
                &xml,
            )
                .unwrap();
        println!("{:#?}", xml);
    }
}
