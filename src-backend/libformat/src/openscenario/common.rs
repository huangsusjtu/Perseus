use serde::{Deserialize, Serialize};

use crate::openscenario::base_enum::{
    CloudState, ControllerType, FractionalCloudCover, ParameterType, PrecipitationType,
    ReferenceContext, Role, RouteStrategy, Rule, VehicleCategory, Wetness,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct EntityRef {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Controller {
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Properties>>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@controllerType", skip_serializing_if = "Option::is_none")]
    pub controller_type: Option<ControllerType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WorldPosition {
    #[serde(rename = "@h", skip_serializing_if = "Option::is_none")]
    pub h: Option<Double>,
    #[serde(rename = "@p", skip_serializing_if = "Option::is_none")]
    pub p: Option<Double>,
    #[serde(rename = "@r", skip_serializing_if = "Option::is_none")]
    pub r: Option<Double>,
    #[serde(rename = "@x")]
    pub x: Double,
    #[serde(rename = "@y")]
    pub y: Double,
    #[serde(rename = "@z", skip_serializing_if = "Option::is_none")]
    pub z: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Wind {
    #[serde(rename = "@direction")]
    pub direction: Double,
    #[serde(rename = "@speed")]
    pub speed: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sun {
    #[serde(rename = "@azimuth")]
    pub azimuth: Double,
    #[serde(rename = "@elevation")]
    pub elevation: Double,
    #[serde(rename = "@intensity", skip_serializing_if = "Option::is_none")]
    pub intensity: Option<Double>,
    #[serde(rename = "@illuminance", skip_serializing_if = "Option::is_none")]
    pub illuminance: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Precipitation {
    #[serde(rename = "@intensity", skip_serializing_if = "Option::is_none")]
    pub intensity: Option<Double>,
    #[serde(rename = "@precipitationType")]
    pub precipitation_type: PrecipitationType,
    #[serde(
        rename = "@precipitationIntensity",
        skip_serializing_if = "Option::is_none"
    )]
    pub precipitation_intensity: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Fog {
    #[serde(rename = "BoundingBox", skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<Vec<BoundingBox>>,
    #[serde(rename = "@visualRange")]
    pub visual_range: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DomeImage {
    #[serde(rename = "DomeFile", skip_serializing_if = "Option::is_none")]
    pub dome_file: Option<File>,
    #[serde(rename = "@azimuthOffset", skip_serializing_if = "Option::is_none")]
    pub azimuth_offset: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Weather {
    #[serde(rename = "Sun", skip_serializing_if = "Option::is_none")]
    pub sun: Option<Vec<Sun>>,
    #[serde(rename = "Fog", skip_serializing_if = "Option::is_none")]
    pub fog: Option<Vec<Fog>>,
    #[serde(rename = "Precipitation", skip_serializing_if = "Option::is_none")]
    pub precipitation: Option<Vec<Precipitation>>,
    #[serde(rename = "Wind", skip_serializing_if = "Option::is_none")]
    pub wind: Option<Vec<Wind>>,
    #[serde(rename = "DomeImage", skip_serializing_if = "Option::is_none")]
    pub dome_image: Option<Vec<DomeImage>>,
    #[serde(rename = "@cloudState", skip_serializing_if = "Option::is_none")]
    pub cloud_state: Option<CloudState>,
    #[serde(
        rename = "@atmosphericPressure",
        skip_serializing_if = "Option::is_none"
    )]
    pub atmospheric_pressure: Option<Double>,
    #[serde(rename = "@temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Double>,
    #[serde(
        rename = "@fractionalCloudCover",
        skip_serializing_if = "Option::is_none"
    )]
    pub fractional_cloud_cover: Option<FractionalCloudCover>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VehicleRoleDistribution {
    #[serde(
        rename = "VehicleRoleDistributionEntry",
        skip_serializing_if = "Option::is_none"
    )]
    pub vehicle_role_distribution_entry: Option<Vec<VehicleRoleDistributionEntry>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VehicleRoleDistributionEntry {
    #[serde(rename = "@role")]
    pub role: Role,
    #[serde(rename = "@weight")]
    pub weight: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VehicleCategoryDistributionEntry {
    #[serde(rename = "@category")]
    pub category: VehicleCategory,
    #[serde(rename = "@weight")]
    pub weight: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VehicleCategoryDistribution {
    #[serde(
        rename = "VehicleCategoryDistributionEntry",
        skip_serializing_if = "Option::is_none"
    )]
    pub vehicle_category_distribution_entry: Option<Vec<VehicleCategoryDistributionEntry>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrajectoryRef {
    #[serde(rename = "Trajectory", skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Trajectory>,
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeOfDay {
    #[serde(rename = "@animation")]
    pub animation: Boolean,
    #[serde(rename = "@dateTime")]
    pub date_time: DateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RouteRef {
    #[serde(rename = "Route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Route>,
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoadPosition {
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Vec<Orientation>>,
    #[serde(rename = "@roadId")]
    pub road_id: String,
    #[serde(rename = "@s")]
    pub s: Double,
    #[serde(rename = "@t")]
    pub t: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoadCondition {
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Properties>>,
    #[serde(rename = "@frictionScaleFactor")]
    pub friction_scale_factor: Double,
    #[serde(rename = "@wetness", skip_serializing_if = "Option::is_none")]
    pub wetness: Option<Wetness>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PositionOfCurrentEntity {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PositionInRoadCoordinates {
    #[serde(rename = "@pathS")]
    pub path_s: Double,
    #[serde(rename = "@t")]
    pub t: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PositionInLaneCoordinates {
    #[serde(rename = "@laneId")]
    pub lane_id: String,
    #[serde(rename = "@laneOffset", skip_serializing_if = "Option::is_none")]
    pub lane_offset: Option<Double>,
    #[serde(rename = "@pathS")]
    pub path_s: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterAssignments {
    #[serde(
        rename = "ParameterAssignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_assignment: Option<Vec<ParameterAssignment>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Orientation {
    #[serde(rename = "@h", skip_serializing_if = "Option::is_none")]
    pub h: Option<Double>,
    #[serde(rename = "@p", skip_serializing_if = "Option::is_none")]
    pub p: Option<Double>,
    #[serde(rename = "@r", skip_serializing_if = "Option::is_none")]
    pub r: Option<Double>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ReferenceContext>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LanePosition {
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Vec<Orientation>>,
    #[serde(rename = "@laneId")]
    pub lane_id: String,
    #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<Double>,
    #[serde(rename = "@roadId")]
    pub road_id: String,
    #[serde(rename = "@s")]
    pub s: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InRoutePosition {
    #[serde(rename = "FromCurrentEntity", skip_serializing_if = "Option::is_none")]
    pub from_current_entity: Option<PositionOfCurrentEntity>,
    #[serde(
        rename = "FromRoadCoordinates",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_road_coordinates: Option<PositionInRoadCoordinates>,
    #[serde(
        rename = "FromLaneCoordinates",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_lane_coordinates: Option<PositionInLaneCoordinates>,
}

pub type Parameter = String; // [$][A-Za-z_][A-Za-z0-9_]*
pub type Expression = String; // [$][{][ A-Za-z0-9_\+\-\*/%$\(\)\.,]*[\}]

pub type Boolean = bool;

pub type DateTime = String;
pub type Double = f64;
pub type Int = i32;
// pub type String = String;
pub type UnsignedInt = u32;
pub type UnsignedShort = u16;

#[derive(Deserialize, Serialize, Debug)]
pub struct Center {
    #[serde(rename = "@x")]
    pub x: Double,
    #[serde(rename = "@y")]
    pub y: Double,
    #[serde(rename = "@z")]
    pub z: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ControllerDistribution {
    #[serde(
        rename = "ControllerDistributionEntry",
        skip_serializing_if = "Option::is_none"
    )]
    pub controller_distribution_entry: Option<Vec<ControllerDistributionEntry>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Properties {
    #[serde(rename = "Property", skip_serializing_if = "Option::is_none")]
    pub property: Option<Vec<Property>>,
    #[serde(rename = "File", skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<File>>,
    #[serde(rename = "CustomContent", skip_serializing_if = "Option::is_none")]
    pub custom_content: Option<Vec<CustomContent>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Property {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CustomContent {
    #[serde(rename = "$text")]
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Knot {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Range {
    #[serde(rename = "@lowerLimit")]
    pub lower_limit: Double,
    #[serde(rename = "@upperLimit")]
    pub upper_limit: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterAssignment {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct File {
    #[serde(rename = "@filepath")]
    pub filepath: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VariableDeclaration {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@variableType")]
    pub variable_type: ParameterType,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VariableDeclarations {
    #[serde(
        rename = "VariableDeclaration",
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_declaration: Option<Vec<VariableDeclaration>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Trajectory {
    #[serde(rename = "Shape", skip_serializing_if = "Option::is_none")]
    pub shape: Option<Shape>,
    #[serde(rename = "@closed")]
    pub closed: Boolean,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Shape {
    #[serde(rename = "Polyline", skip_serializing_if = "Option::is_none")]
    pub polyline: Option<Polyline>,
    #[serde(rename = "Clothoid", skip_serializing_if = "Option::is_none")]
    pub clothoid: Option<Clothoid>,
    #[serde(rename = "ClothoidSpline", skip_serializing_if = "Option::is_none")]
    pub clothoid_spline: Option<ClothoidSpline>,
    #[serde(rename = "Nurbs", skip_serializing_if = "Option::is_none")]
    pub nurbs: Option<Nurbs>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Polyline {
    #[serde(rename = "Vertex")]
    pub vertex: Vec<Vertex>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Vertex {
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(rename = "@time", skip_serializing_if = "Option::is_none")]
    pub time: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Clothoid {
    // #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    // pub position: Option<Position>,
    // TODO
    #[serde(rename = "@curvature")]
    pub curvature: Double,
    #[serde(rename = "@curvatureDot", skip_serializing_if = "Option::is_none")]
    pub curvature_dot: Option<Double>,
    #[serde(rename = "@length")]
    pub length: Double,
    #[serde(rename = "@startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Double>,
    #[serde(rename = "@stopTime", skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<Double>,
    #[serde(rename = "@curvaturePrime", skip_serializing_if = "Option::is_none")]
    pub curvature_prime: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClothoidSpline {
    #[serde(
        rename = "ClothoidSplineSegment",
        skip_serializing_if = "Option::is_none"
    )]
    pub clothoid_spline_segment: Option<Vec<ClothoidSplineSegment>>,
    #[serde(rename = "@timeEnd", skip_serializing_if = "Option::is_none")]
    pub time_end: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClothoidSplineSegment {
    #[serde(rename = "PositionStart", skip_serializing_if = "Option::is_none")]
    pub position_start: Option<Vec<Position>>,
    #[serde(rename = "@curvatureStart")]
    pub curvature_start: Double,
    #[serde(rename = "@curvatureEnd")]
    pub curvature_end: Double,
    #[serde(rename = "@length")]
    pub length: Double,
    #[serde(rename = "@hOffset", skip_serializing_if = "Option::is_none")]
    pub h_offset: Option<Double>,
    #[serde(rename = "@timeStart", skip_serializing_if = "Option::is_none")]
    pub time_start: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Nurbs {
    #[serde(rename = "ControlPoint", skip_serializing_if = "Option::is_none")]
    pub control_point: Option<Vec<ControlPoint>>,
    #[serde(rename = "Knot", skip_serializing_if = "Option::is_none")]
    pub knot: Option<Vec<Knot>>,
    #[serde(rename = "@order")]
    pub order: UnsignedInt,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValueConstraintGroup {
    #[serde(rename = "ValueConstraint", skip_serializing_if = "Option::is_none")]
    pub value_constraint: Option<Vec<ValueConstraint>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValueConstraint {
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ControlPoint {
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(rename = "@time", skip_serializing_if = "Option::is_none")]
    pub time: Option<Double>,
    #[serde(rename = "@weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Position {
    #[serde(rename = "WorldPosition", skip_serializing_if = "Option::is_none")]
    pub world_position: Option<WorldPosition>,
    #[serde(
        rename = "RelativeWorldPosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_world_position: Option<RelativeWorldPosition>,
    #[serde(
        rename = "RelativeObjectPosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_object_position: Option<RelativeObjectPosition>,
    #[serde(rename = "RoadPosition", skip_serializing_if = "Option::is_none")]
    pub road_position: Option<RoadPosition>,
    #[serde(
        rename = "RelativeRoadPosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_road_position: Option<RelativeRoadPosition>,
    #[serde(rename = "LanePosition", skip_serializing_if = "Option::is_none")]
    pub lane_position: Option<LanePosition>,
    #[serde(
        rename = "RelativeLanePosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_lane_position: Option<RelativeLanePosition>,
    #[serde(rename = "RoutePosition", skip_serializing_if = "Option::is_none")]
    pub route_position: Option<RoutePosition>,
    #[serde(rename = "GeoPosition", skip_serializing_if = "Option::is_none")]
    pub geo_position: Option<GeoPosition>,
    #[serde(rename = "TrajectoryPosition", skip_serializing_if = "Option::is_none")]
    pub trajectory_position: Option<TrajectoryPosition>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrajectoryPosition {
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Vec<Orientation>>,
    #[serde(rename = "TrajectoryRef")]
    pub trajectory_ref: TrajectoryRef,
    #[serde(rename = "@s")]
    pub s: Double,
    #[serde(rename = "@t", skip_serializing_if = "Option::is_none")]
    pub t: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoutePosition {
    #[serde(rename = "RouteRef")]
    pub route_ref: RouteRef,
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Vec<Orientation>>,
    #[serde(rename = "InRoutePosition")]
    pub in_route_position: InRoutePosition,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeLanePosition {
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Vec<Orientation>>,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@dLane")]
    pub d_lane: Int,
    #[serde(rename = "@ds", skip_serializing_if = "Option::is_none")]
    pub ds: Option<Double>,
    #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<Double>,
    #[serde(rename = "@dsLane", skip_serializing_if = "Option::is_none")]
    pub ds_lane: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeRoadPosition {
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Vec<Orientation>>,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@ds")]
    pub ds: Double,
    #[serde(rename = "@dt")]
    pub dt: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeObjectPosition {
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Vec<Orientation>>,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@dx")]
    pub dx: Double,
    #[serde(rename = "@dy")]
    pub dy: Double,
    #[serde(rename = "@dz", skip_serializing_if = "Option::is_none")]
    pub dz: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeWorldPosition {
    #[serde(rename = "Orientation")]
    pub orientation: Vec<Orientation>,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@dx")]
    pub dx: Double,
    #[serde(rename = "@dy")]
    pub dy: Double,
    #[serde(rename = "@dz", skip_serializing_if = "Option::is_none")]
    pub dz: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeoPosition {
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Vec<Orientation>>,
    #[serde(rename = "@latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Double>,
    #[serde(rename = "@longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Double>,
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<Double>,
    #[serde(rename = "@latitudeDeg", skip_serializing_if = "Option::is_none")]
    pub latitude_deg: Option<Double>,
    #[serde(rename = "@longitudeDeg", skip_serializing_if = "Option::is_none")]
    pub longitude_deg: Option<Double>,
    #[serde(rename = "@altitude", skip_serializing_if = "Option::is_none")]
    pub altitude: Option<Double>,
    #[serde(
        rename = "@verticalRoadSelection",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_road_selection: Option<Int>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MonitorDeclaration {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: Boolean,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MonitorDeclarations {
    #[serde(rename = "MonitorDeclaration", skip_serializing_if = "Option::is_none")]
    pub monitor_declaration: Option<Vec<MonitorDeclaration>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NoneStruct;

#[derive(Deserialize, Serialize, Debug)]
pub struct CatalogReference {
    #[serde(
        rename = "ParameterAssignments",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_assignments: Option<Vec<ParameterAssignments>>,
    #[serde(rename = "@catalogName")]
    pub catalog_name: String,
    #[serde(rename = "@entryName")]
    pub entry_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Route {
    #[serde(rename = "Waypoint", skip_serializing_if = "Option::is_none")]
    pub waypoint: Option<Vec<Waypoint>>,
    #[serde(rename = "@closed")]
    pub closed: Boolean,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Waypoint {
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(rename = "@routeStrategy")]
    pub route_strategy: RouteStrategy,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Environment {
    #[serde(rename = "TimeOfDay", skip_serializing_if = "Option::is_none")]
    pub time_of_day: Option<Vec<TimeOfDay>>,
    #[serde(rename = "Weather", skip_serializing_if = "Option::is_none")]
    pub weather: Option<Vec<Weather>>,
    #[serde(rename = "RoadCondition", skip_serializing_if = "Option::is_none")]
    pub road_condition: Option<Vec<RoadCondition>>,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficDefinition {
    #[serde(rename = "VehicleCategoryDistribution")]
    pub vehicle_category_distribution: VehicleCategoryDistribution,
    #[serde(
        rename = "VehicleRoleDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub vehicle_role_distribution: Option<Vec<VehicleRoleDistribution>>,
    #[serde(rename = "ControllerDistribution")]
    pub controller_distribution: ControllerDistribution,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BoundingBox {
    #[serde(rename = "Center")]
    pub center: Center,
    #[serde(rename = "Dimensions")]
    pub dimensions: Dimensions,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dimensions {
    #[serde(rename = "@height")]
    pub height: Double,
    #[serde(rename = "@length")]
    pub length: Double,
    #[serde(rename = "@width")]
    pub width: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ControllerDistributionEntry {
    #[serde(rename = "Controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<Controller>,
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference>,
    #[serde(rename = "@weight")]
    pub weight: Double,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    

    #[derive(Deserialize, Serialize, Debug)]
    enum TUnion {
        #[serde(rename = "@a")]
        A,
        #[serde(rename = "@b")]
        B,
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct Struct {
        pub u: TUnion,
    }

    #[test]
    fn it_works1() {
        let u = Struct { u: TUnion::A };
        let xml = quick_xml::se::to_string(&u).unwrap();
        println!("{:#?}", xml);
    }
}
