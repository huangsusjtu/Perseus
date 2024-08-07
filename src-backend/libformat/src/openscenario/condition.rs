use serde::{Deserialize, Serialize};

use crate::openscenario::base_enum::{
    AngleType, ConditionEdge, CoordinateSystem, DirectionalDimension, ObjectType,
    RelativeDistanceType, RoutingAlgorithm, Rule, StoryboardElementState, StoryboardElementType,
    TriggeringEntitiesRule,
};
use crate::openscenario::common::{Boolean, DateTime, Double, EntityRef, Int, Position};

#[derive(Deserialize, Serialize, Debug)]
pub struct VariableCondition {
    #[serde(rename = "@variableRef")]
    pub variable_ref: String,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDefinedValueCondition {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TriggeringEntities {
    #[serde(rename = "EntityRef", skip_serializing_if = "Option::is_none")]
    pub entity_ref: Option<Vec<EntityRef>>,
    #[serde(rename = "@triggeringEntitiesRule")]
    pub triggering_entities_rule: TriggeringEntitiesRule,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ByObjectType {
    #[serde(rename = "@type")]
    pub r#type: ObjectType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TraveledDistanceCondition {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSignalControllerCondition {
    #[serde(rename = "@trafficSignalControllerRef")]
    pub traffic_signal_controller_ref: String,
    #[serde(rename = "@phase")]
    pub phase: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSignalCondition {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@state")]
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeToCollisionConditionTarget {
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(rename = "EntityRef", skip_serializing_if = "Option::is_none")]
    pub entity_ref: Option<EntityRef>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeToCollisionCondition {
    #[serde(rename = "TimeToCollisionConditionTarget")]
    pub time_to_collision_condition_target: TimeToCollisionConditionTarget,
    #[serde(rename = "@alongRoute", skip_serializing_if = "Option::is_none")]
    pub along_route: Option<Boolean>,
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(
        rename = "@relativeDistanceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_distance_type: Option<RelativeDistanceType>,
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeOfDayCondition {
    #[serde(rename = "@dateTime")]
    pub date_time: DateTime,
    #[serde(rename = "@rule")]
    pub rule: Rule,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeHeadwayCondition {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@alongRoute", skip_serializing_if = "Option::is_none")]
    pub along_route: Option<Boolean>,
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
    #[serde(
        rename = "@relativeDistanceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_distance_type: Option<RelativeDistanceType>,
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StoryboardElementStateCondition {
    #[serde(rename = "@storyboardElementRef")]
    pub storyboard_element_ref: String,
    #[serde(rename = "@state")]
    pub state: StoryboardElementState,
    #[serde(rename = "@storyboardElementType")]
    pub storyboard_element_type: StoryboardElementType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StandStillCondition {
    #[serde(rename = "@duration")]
    pub duration: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpeedCondition {
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<DirectionalDimension>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SimulationTimeCondition {
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeSpeedCondition {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<DirectionalDimension>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeLaneRange {
    #[serde(rename = "@from")]
    pub from: Int,
    #[serde(rename = "@to")]
    pub to: Int,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeDistanceCondition {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
    #[serde(rename = "@relativeDistanceType")]
    pub relative_distance_type: RelativeDistanceType,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeClearanceCondition {
    #[serde(rename = "RelativeLaneRange", skip_serializing_if = "Option::is_none")]
    pub relative_lane_range: Option<Vec<RelativeLaneRange>>,
    #[serde(rename = "EntityRef", skip_serializing_if = "Option::is_none")]
    pub entity_ref: Option<Vec<EntityRef>>,
    #[serde(rename = "@oppositeLanes")]
    pub opposite_lanes: Boolean,
    #[serde(rename = "@distanceForward", skip_serializing_if = "Option::is_none")]
    pub distance_forward: Option<Double>,
    #[serde(rename = "@distanceBackward", skip_serializing_if = "Option::is_none")]
    pub distance_backward: Option<Double>,
    #[serde(rename = "@freeSpace")]
    pub free_space: Boolean,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeAngleCondition {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@angleType")]
    pub angle_type: AngleType,
    #[serde(rename = "@angle")]
    pub angle: Double,
    #[serde(rename = "@angleTolerance")]
    pub angle_tolerance: Double,
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReachPositionCondition {
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "@tolerance")]
    pub tolerance: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterCondition {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: String,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OffroadCondition {
    #[serde(rename = "@duration")]
    pub duration: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EndOfRoadCondition {
    #[serde(rename = "@duration")]
    pub duration: Double,

    #[serde(rename = "@rule", skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DistanceCondition {
    #[serde(rename = "Position")]
    pub position: Position,

    #[serde(rename = "@alongRoute", skip_serializing_if = "Option::is_none")]
    pub along_route: Option<Boolean>,
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
    #[serde(
        rename = "@relativeDistanceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_distance_type: Option<RelativeDistanceType>,
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CollisionCondition {
    #[serde(rename = "EntityRef", skip_serializing_if = "Option::is_none")]
    pub entity_ref: Option<EntityRef>,
    #[serde(rename = "ByType", skip_serializing_if = "Option::is_none")]
    pub by_type: Option<ByObjectType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AngleCondition {
    #[serde(rename = "@angleType")]
    pub angle_type: AngleType,
    #[serde(rename = "@angle")]
    pub angle: Double,
    #[serde(rename = "@angleTolerance")]
    pub angle_tolerance: Double,
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccelerationCondition {
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<DirectionalDimension>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntityCondition {
    #[serde(rename = "EndOfRoadCondition", skip_serializing_if = "Option::is_none")]
    pub end_of_road_condition: Option<EndOfRoadCondition>,
    #[serde(rename = "CollisionCondition", skip_serializing_if = "Option::is_none")]
    pub collision_condition: Option<CollisionCondition>,
    #[serde(rename = "OffroadCondition", skip_serializing_if = "Option::is_none")]
    pub offroad_condition: Option<OffroadCondition>,
    #[serde(
        rename = "TimeHeadwayCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_headway_condition: Option<TimeHeadwayCondition>,
    #[serde(
        rename = "TimeToCollisionCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_to_collision_condition: Option<TimeToCollisionCondition>,
    #[serde(
        rename = "AccelerationCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub acceleration_condition: Option<AccelerationCondition>,
    #[serde(
        rename = "StandStillCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub stand_still_condition: Option<StandStillCondition>,
    #[serde(rename = "SpeedCondition", skip_serializing_if = "Option::is_none")]
    pub speed_condition: Option<SpeedCondition>,
    #[serde(
        rename = "RelativeSpeedCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_speed_condition: Option<RelativeSpeedCondition>,
    #[serde(
        rename = "TraveledDistanceCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub traveled_distance_condition: Option<TraveledDistanceCondition>,
    #[serde(
        rename = "ReachPositionCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub reach_position_condition: Option<ReachPositionCondition>,
    #[serde(rename = "DistanceCondition", skip_serializing_if = "Option::is_none")]
    pub distance_condition: Option<DistanceCondition>,
    #[serde(
        rename = "RelativeDistanceCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_distance_condition: Option<RelativeDistanceCondition>,
    #[serde(
        rename = "RelativeClearanceCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_clearance_condition: Option<RelativeClearanceCondition>,
    #[serde(rename = "AngleCondition", skip_serializing_if = "Option::is_none")]
    pub angle_condition: Option<AngleCondition>,
    #[serde(
        rename = "RelativeAngleCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_angle_condition: Option<RelativeAngleCondition>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Condition {
    #[serde(rename = "ByEntityCondition", skip_serializing_if = "Option::is_none")]
    pub by_entity_condition: Option<ByEntityCondition>,
    #[serde(rename = "ByValueCondition", skip_serializing_if = "Option::is_none")]
    pub by_value_condition: Option<ByValueCondition>,
    #[serde(rename = "@conditionEdge")]
    pub condition_edge: ConditionEdge,
    #[serde(rename = "@delay")]
    pub delay: Double,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ByEntityCondition {
    #[serde(rename = "TriggeringEntities")]
    pub triggering_entities: TriggeringEntities,
    #[serde(rename = "EntityCondition")]
    pub entity_condition: EntityCondition,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ByValueCondition {
    #[serde(rename = "ParameterCondition", skip_serializing_if = "Option::is_none")]
    pub parameter_condition: Option<ParameterCondition>,
    #[serde(rename = "TimeOfDayCondition", skip_serializing_if = "Option::is_none")]
    pub time_of_day_condition: Option<TimeOfDayCondition>,
    #[serde(
        rename = "SimulationTimeCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub simulation_time_condition: Option<SimulationTimeCondition>,
    #[serde(
        rename = "StoryboardElementStateCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub storyboard_element_state_condition: Option<StoryboardElementStateCondition>,
    #[serde(
        rename = "UserDefinedValueCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_value_condition: Option<UserDefinedValueCondition>,
    #[serde(
        rename = "TrafficSignalCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_signal_condition: Option<TrafficSignalCondition>,
    #[serde(
        rename = "TrafficSignalControllerCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_signal_controller_condition: Option<TrafficSignalControllerCondition>,
    #[serde(rename = "VariableCondition", skip_serializing_if = "Option::is_none")]
    pub variable_condition: Option<VariableCondition>,
}
