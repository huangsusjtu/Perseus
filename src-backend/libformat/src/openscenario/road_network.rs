use serde::{Deserialize, Serialize};

use crate::openscenario::common::{Double, File, Position};

#[derive(Deserialize, Serialize, Debug)]
pub struct RoadNetwork {
    #[serde(rename = "LogicFile", skip_serializing_if = "Option::is_none")]
    pub logic_file: Option<Vec<File>>,
    #[serde(rename = "SceneGraphFile", skip_serializing_if = "Option::is_none")]
    pub scene_graph_file: Option<Vec<File>>,
    #[serde(rename = "TrafficSignals", skip_serializing_if = "Option::is_none")]
    pub traffic_signals: Option<Vec<TrafficSignals>>,
    #[serde(rename = "UsedArea", skip_serializing_if = "Option::is_none")]
    pub used_area: Option<Vec<UsedArea>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSignals {
    #[serde(
        rename = "TrafficSignalController",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_signal_controller: Option<Vec<TrafficSignalController>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UsedArea {
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec<Position>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Phase {
    #[serde(rename = "TrafficSignalState", skip_serializing_if = "Option::is_none")]
    pub traffic_signal_state: Option<Vec<TrafficSignalState>>,
    #[serde(
        rename = "TrafficSignalGroupState",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_signal_group_state: Option<Vec<TrafficSignalGroupState>>,
    #[serde(rename = "@duration")]
    pub duration: Double,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSignalController {
    #[serde(rename = "Phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<Vec<Phase>>,
    #[serde(rename = "@delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<Double>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSignalGroupState {
    #[serde(rename = "@state")]
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSignalState {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(rename = "@trafficSignalId")]
    pub traffic_signal_id: String,
}
