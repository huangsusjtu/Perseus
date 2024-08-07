use serde::{Deserialize, Serialize};

use crate::opendrive::common;
use crate::opendrive::lane::lane::TBool;
use crate::opendrive::object::object;

///
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Signals {
    #[serde(rename = "signal", skip_serializing_if = "Option::is_none")]
    pub signal: Option<Vec<Signal>>,
    #[serde(rename = "signalReference")]
    pub signal_reference: signal::SignalReference,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Signal {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@dynamic")]
    pub dynamic: TBool,
    #[serde(rename = "@orientation")]
    pub orientation: object::EOrientation,
    #[serde(rename = "@s")]
    pub s: f64,
    #[serde(rename = "@t")]
    pub t: f64,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@subtype")]
    pub subtype: String,
    #[serde(rename = "@zOffset")]
    pub z_offset: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@countryRevision", skip_serializing_if = "Option::is_none")]
    pub country_revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "@hOffset", skip_serializing_if = "Option::is_none")]
    pub h_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<common::core::EUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<common::core::Validity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency: Option<signal::Dependency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<signal::Reference>,
}

pub mod signal {
    use serde::{Deserialize, Serialize};

    use crate::opendrive::common;
    use crate::opendrive::common::core::EUnitSpeed;
    use crate::opendrive::object::object;

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Controller {
        #[serde(rename = "@id")]
        pub id: u32,
        #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "@sequence", skip_serializing_if = "Option::is_none")]
        pub sequence: Option<u32>,

        pub control: Vec<Control>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Control {
        #[serde(rename = "@signalId")]
        pub signal_id: u32,
        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Dependency {
        #[serde(rename = "@id")]
        pub id: String,

        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Reference {
        #[serde(rename = "@elementId")]
        pub element_id: String,
        #[serde(rename = "@elementType")]
        pub element_type: ERoadSignalsSignalReferenceElementType,

        #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct SignalReference {
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@orientation")]
        pub orientation: object::EOrientation,
        #[serde(rename = "@s")]
        pub s: f64,
        #[serde(rename = "@t")]
        pub t: f64,

        pub validity: common::core::Validity,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Semantics {
        #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
        pub speed: Option<Speed>,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Speed {
        #[serde(rename = "@type")]
        pub r#type: ESignalsSemanticsSpeed,
        #[serde(rename = "@unit")]
        pub unit: EUnitSpeed,
        #[serde(rename = "@value")]
        pub value: f64,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum ERoadSignalsDisplayType {
        #[default]
        #[serde(rename = "LED")]
        LED,
        #[serde(rename = "monochromGraphic")]
        MonochromGraphic,
        #[serde(rename = "other")]
        Other,
        #[serde(rename = "rotatingPrismHorizontal")]
        RotatingPrismHorizontal,
        #[serde(rename = "rotatingPrismVertical")]
        RotatingPrismVertical,
        #[serde(rename = "simpleMatrix")]
        SimpleMatrix,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ESignalsSemanticsSupplementaryTime {
        #[default]
        Day,
        Time,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub enum ESignalsSemanticsPriority {
        #[default]
        #[serde(rename = "4way")]
        FourWay,
        #[serde(rename = "keepClearLine")]
        KeepClearLine,
        #[serde(rename = "noParkingLine")]
        NoParkingLine,
        #[serde(rename = "noTurnOnRed")]
        NoTurnOnRed,
        #[serde(rename = "priorityRoadEnd")]
        PriorityRoadEnd,
        #[serde(rename = "priorityRoad")]
        PriorityRoad,
        #[serde(rename = "priorityToTheRightRule")]
        PriorityToTheRightRule,
        #[serde(rename = "stopLine")]
        StopLine,
        #[serde(rename = "stop")]
        Stop,
        #[serde(rename = "trafficLight")]
        TrafficLight,
        #[serde(rename = "turnOnRedAllowed")]
        TurnOnRedAllowed,
        #[serde(rename = "waitingLine")]
        WaitingLine,
        #[serde(rename = "yield")]
        Yield,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ERoadSignalsSignalReferenceElementType {
        #[default]
        Object,
        Signal,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum ESignalsSemanticsLane {
        #[default]
        NoOvertakeCarsEnd,
        NoOvertakeCars,
        NoOvertakeTrucksEnd,
        NoOvertakeTrucks,
        PriorityOverOncoming,
        Roundabout,
        YieldForOncoming,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ESignalsSemanticsSupplementaryEnvironment {
        #[default]
        Fog,
        Rain,
        Snow,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum ESignalsSemanticsSpeed {
        #[default]
        MaximumEnd,
        Maximum,
        MinimumEnd,
        Minimum,
        RecommendedEnd,
        Recommended,
        ZoneEnd,
        Zone,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum ESignalsSemanticsSupplementaryDistance {
        #[default]
        For,
        In,
    }
}
