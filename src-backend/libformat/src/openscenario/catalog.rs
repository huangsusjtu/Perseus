use serde::{Deserialize, Serialize};

use crate::openscenario::common::{Controller, Environment, Route, Trajectory};
use crate::openscenario::entity::{MiscObject, Pedestrian, Vehicle};
use crate::openscenario::storyboard::Maneuver;

#[derive(Deserialize, Serialize, Debug)]
pub struct Catalog {
    #[serde(rename = "Vehicle", skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<Vec<Vehicle>>,
    #[serde(rename = "Controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<Controller>>,
    #[serde(rename = "Pedestrian", skip_serializing_if = "Option::is_none")]
    pub pedestrian: Option<Vec<Pedestrian>>,
    #[serde(rename = "MiscObject", skip_serializing_if = "Option::is_none")]
    pub misc_object: Option<Vec<MiscObject>>,
    #[serde(rename = "Environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<Environment>>,
    #[serde(rename = "Maneuver", skip_serializing_if = "Option::is_none")]
    pub maneuver: Option<Vec<Maneuver>>,
    #[serde(rename = "Trajectory", skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Vec<Trajectory>>,
    #[serde(rename = "Route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<Route>>,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CatalogLocations {
    #[serde(rename = "VehicleCatalog", skip_serializing_if = "Option::is_none")]
    pub vehicle_catalog: Option<Vec<VehicleCatalogLocation>>,
    #[serde(rename = "ControllerCatalog", skip_serializing_if = "Option::is_none")]
    pub controller_catalog: Option<Vec<ControllerCatalogLocation>>,
    #[serde(rename = "PedestrianCatalog", skip_serializing_if = "Option::is_none")]
    pub pedestrian_catalog: Option<Vec<PedestrianCatalogLocation>>,
    #[serde(rename = "MiscObjectCatalog", skip_serializing_if = "Option::is_none")]
    pub misc_object_catalog: Option<Vec<MiscObjectCatalogLocation>>,
    #[serde(rename = "EnvironmentCatalog", skip_serializing_if = "Option::is_none")]
    pub environment_catalog: Option<Vec<EnvironmentCatalogLocation>>,
    #[serde(rename = "ManeuverCatalog", skip_serializing_if = "Option::is_none")]
    pub maneuver_catalog: Option<Vec<ManeuverCatalogLocation>>,
    #[serde(rename = "TrajectoryCatalog", skip_serializing_if = "Option::is_none")]
    pub trajectory_catalog: Option<Vec<TrajectoryCatalogLocation>>,
    #[serde(rename = "RouteCatalog", skip_serializing_if = "Option::is_none")]
    pub route_catalog: Option<Vec<RouteCatalogLocation>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Directory {
    #[serde(rename = "@path")]
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VehicleCatalogLocation {
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

pub type ControllerCatalogLocation = VehicleCatalogLocation;
pub type PedestrianCatalogLocation = VehicleCatalogLocation;
pub type MiscObjectCatalogLocation = VehicleCatalogLocation;
pub type EnvironmentCatalogLocation = VehicleCatalogLocation;
pub type ManeuverCatalogLocation = VehicleCatalogLocation;
pub type RouteCatalogLocation = VehicleCatalogLocation;
pub type TrajectoryCatalogLocation = VehicleCatalogLocation;
