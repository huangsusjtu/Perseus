use serde::{Deserialize, Serialize};

use crate::openscenario::base_enum::{
    MiscObjectCategory, ObjectType, PedestrianCategory, Role, VehicleCategory,
};
use crate::openscenario::common::{
    BoundingBox, CatalogReference, Controller, Double, EntityRef, Properties,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct TrailerHitch {
    #[serde(rename = "@dx")]
    pub dx: Double,
    #[serde(rename = "@dz", skip_serializing_if = "Option::is_none")]
    pub dz: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrailerCoupler {
    #[serde(rename = "@dx")]
    pub dx: Double,
    #[serde(rename = "@dz", skip_serializing_if = "Option::is_none")]
    pub dz: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Trailer {
    #[serde(rename = "Trailer", skip_serializing_if = "Option::is_none")]
    pub trailer: Option<ScenarioObject>,
    #[serde(rename = "TrailerRef", skip_serializing_if = "Option::is_none")]
    pub trailer_ref: Option<EntityRef>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Performance {
    #[serde(rename = "@maxAcceleration")]
    pub max_acceleration: Double,
    #[serde(
        rename = "@maxAccelerationRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_acceleration_rate: Option<Double>,
    #[serde(rename = "@maxDeceleration")]
    pub max_deceleration: Double,
    #[serde(
        rename = "@maxDecelerationRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_deceleration_rate: Option<Double>,
    #[serde(rename = "@maxSpeed")]
    pub max_speed: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Axle {
    #[serde(rename = "@maxSteering")]
    pub max_steering: Double,
    #[serde(rename = "@positionX")]
    pub position_x: Double,
    #[serde(rename = "@positionZ")]
    pub position_z: Double,
    #[serde(rename = "@trackWidth")]
    pub track_width: Double,
    #[serde(rename = "@wheelDiameter")]
    pub wheel_diameter: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Axles {
    #[serde(rename = "FrontAxle", skip_serializing_if = "Option::is_none")]
    pub front_axle: Option<Vec<Axle>>,
    #[serde(rename = "RearAxle", skip_serializing_if = "Option::is_none")]
    pub rear_axle: Option<Vec<Axle>>,
    #[serde(rename = "AdditionalAxle", skip_serializing_if = "Option::is_none")]
    pub additional_axle: Option<Vec<Axle>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entities {
    #[serde(rename = "ScenarioObject", skip_serializing_if = "Option::is_none")]
    pub scenario_object: Option<Vec<ScenarioObject>>,
    #[serde(rename = "EntitySelection", skip_serializing_if = "Option::is_none")]
    pub entity_selection: Option<Vec<EntitySelection>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntitySelection {
    #[serde(rename = "Members", skip_serializing_if = "Option::is_none")]
    pub members: Option<SelectedEntities>,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SelectedEntities {
    #[serde(rename = "EntityRef", skip_serializing_if = "Option::is_none")]
    pub entity_ref: Option<Vec<EntityRef>>,
    #[serde(rename = "ByType", skip_serializing_if = "Option::is_none")]
    pub by_type: Option<Vec<ByType>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScenarioObject {
    #[serde(rename = "ObjectController", skip_serializing_if = "Option::is_none")]
    pub object_controller: Option<Vec<ObjectController>>,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObjectController {
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference>,
    #[serde(rename = "Controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<Controller>,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ByType {
    #[serde(rename = "@objectType")]
    pub object_type: ObjectType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Vehicle {
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,
    #[serde(rename = "Performance")]
    pub performance: Performance,
    #[serde(rename = "Axles")]
    pub axles: Axles,
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Properties>>,
    #[serde(rename = "TrailerHitch", skip_serializing_if = "Option::is_none")]
    pub trailer_hitch: Option<Vec<TrailerHitch>>,
    #[serde(rename = "TrailerCoupler", skip_serializing_if = "Option::is_none")]
    pub trailer_coupler: Option<Vec<TrailerCoupler>>,
    #[serde(rename = "Trailer", skip_serializing_if = "Option::is_none")]
    pub trailer: Option<Vec<Trailer>>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@vehicleCategory")]
    pub vehicle_category: VehicleCategory,
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(rename = "@mass", skip_serializing_if = "Option::is_none")]
    pub mass: Option<Double>,
    #[serde(rename = "@model3d", skip_serializing_if = "Option::is_none")]
    pub model3d: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Pedestrian {
    #[serde(rename = "BoundingBox", skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<Vec<BoundingBox>>,
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Properties>>,
    #[serde(rename = "@mass")]
    pub mass: Double,
    #[serde(rename = "@model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@pedestrianCategory")]
    pub pedestrian_category: PedestrianCategory,
    #[serde(rename = "@model3d", skip_serializing_if = "Option::is_none")]
    pub model3d: Option<String>,
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MiscObject {
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Properties>>,
    #[serde(rename = "@mass")]
    pub mass: Double,
    #[serde(rename = "@miscObjectCategory")]
    pub misc_object_category: MiscObjectCategory,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@model3d", skip_serializing_if = "Option::is_none")]
    pub model3d: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExternalObjectReference {
    #[serde(rename = "@name")]
    pub name: String,
}
