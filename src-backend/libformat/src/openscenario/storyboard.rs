use serde::{Deserialize, Serialize};

use crate::openscenario::base_enum::{
    AutomaticGearType, ColorType, CoordinateSystem, DynamicsDimension, DynamicsShape,
    FollowingMode, LateralDisplacement, LightMode, LongitudinalDisplacement, PedestrianGestureType,
    PedestrianMotionType, Priority, ReferenceContext, SpeedTargetValueType, VehicleComponentType,
    VehicleLightType,
};
use crate::openscenario::common::{
    Boolean, CatalogReference, Controller, Double, EntityRef, Environment, File, Int, NoneStruct,
    Position, Properties, Range, Route, TrafficDefinition, Trajectory, TrajectoryRef, UnsignedInt,
};
use crate::openscenario::condition::Condition;
use crate::openscenario::entity::{
    ExternalObjectReference, MiscObject, ObjectController, Pedestrian, Vehicle,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Storyboard {
    #[serde(rename = "Init", skip_serializing_if = "Option::is_none")]
    pub init: Option<Init>,
    #[serde(rename = "Story", skip_serializing_if = "Option::is_none")]
    pub story: Option<Vec<Story>>,
    #[serde(rename = "StopTrigger", skip_serializing_if = "Option::is_none")]
    pub stop_trigger: Option<Vec<Trigger>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Init {
    #[serde(rename = "Actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<InitActions>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Story {
    #[serde(rename = "Act", skip_serializing_if = "Option::is_none")]
    pub act: Option<Vec<Act>>,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Act {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "ManeuverGroup", skip_serializing_if = "Option::is_none")]
    pub maneuver_group: Option<Vec<ManeuverGroup>>,
    #[serde(rename = "StartTrigger", skip_serializing_if = "Option::is_none")]
    pub start_trigger: Option<Vec<Trigger>>,
    #[serde(rename = "StopTrigger", skip_serializing_if = "Option::is_none")]
    pub stop_trigger: Option<Vec<Trigger>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManeuverGroup {
    #[serde(rename = "Actors", skip_serializing_if = "Option::is_none")]
    pub actors: Option<Actors>,
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<Vec<CatalogReference>>,
    #[serde(rename = "Maneuver", skip_serializing_if = "Option::is_none")]
    pub maneuver: Option<Vec<Maneuver>>,
    #[serde(rename = "@maximumExecutionCount")]
    pub maximum_execution_count: UnsignedInt,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Trigger {
    #[serde(rename = "ConditionGroup", skip_serializing_if = "Option::is_none")]
    pub condition_group: Option<Vec<ConditionGroup>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Actors {
    #[serde(rename = "EntityRef", skip_serializing_if = "Option::is_none")]
    pub entity_ref: Option<Vec<EntityRef>>,
    #[serde(rename = "@selectTriggeringEntities")]
    pub select_triggering_entities: Boolean,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    #[serde(rename = "Action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<Action>>,
    #[serde(rename = "StartTrigger", skip_serializing_if = "Option::is_none")]
    pub start_trigger: Option<Vec<Trigger>>,
    #[serde(
        rename = "@maximumExecutionCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_execution_count: Option<Int>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@priority")]
    pub priority: Priority,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Maneuver {
    #[serde(rename = "Event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Vec<Event>>,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManualGear {
    #[serde(rename = "@number")]
    pub number: Int,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FollowTrajectoryAction {
    #[serde(rename = "Trajectory", skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Vec<Trajectory>>,
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<Vec<CatalogReference>>,
    #[serde(rename = "TimeReference")]
    pub time_reference: TimeReference,
    #[serde(rename = "TrajectoryFollowingMode")]
    pub trajectory_following_mode: TrajectoryFollowingMode,
    #[serde(rename = "TrajectoryRef", skip_serializing_if = "Option::is_none")]
    pub trajectory_ref: Option<Vec<TrajectoryRef>>,
    #[serde(
        rename = "@initialDistanceOffset",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_distance_offset: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OverrideControllerValueAction {
    #[serde(rename = "Throttle", skip_serializing_if = "Option::is_none")]
    pub throttle: Option<Vec<OverrideThrottleAction>>,
    #[serde(rename = "Brake", skip_serializing_if = "Option::is_none")]
    pub brake: Option<Vec<OverrideBrakeAction>>,
    #[serde(rename = "Clutch", skip_serializing_if = "Option::is_none")]
    pub clutch: Option<Vec<OverrideClutchAction>>,
    #[serde(rename = "ParkingBrake", skip_serializing_if = "Option::is_none")]
    pub parking_brake: Option<Vec<OverrideParkingBrakeAction>>,
    #[serde(rename = "SteeringWheel", skip_serializing_if = "Option::is_none")]
    pub steering_wheel: Option<Vec<OverrideSteeringWheelAction>>,
    #[serde(rename = "Gear", skip_serializing_if = "Option::is_none")]
    pub gear: Option<Vec<OverrideGearAction>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OverrideGearAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<Double>,

    #[serde(rename = "ManualGear", skip_serializing_if = "Option::is_none")]
    pub manual_gear: Option<ManualGear>,
    #[serde(rename = "AutomaticGear", skip_serializing_if = "Option::is_none")]
    pub automatic_gear: Option<AutomaticGear>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ControllerAction {
    #[serde(
        rename = "AssignControllerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub assign_controller_action: Option<AssignControllerAction>,
    #[serde(
        rename = "OverrideControllerValueAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_controller_value_action: Option<OverrideControllerValueAction>,
    #[serde(
        rename = "ActivateControllerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_controller_action: Option<ActivateControllerAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Action {
    #[serde(rename = "GlobalAction", skip_serializing_if = "Option::is_none")]
    pub global_action: Option<GlobalAction>,
    #[serde(rename = "UserDefinedAction", skip_serializing_if = "Option::is_none")]
    pub user_defined_action: Option<UserDefinedAction>,
    #[serde(rename = "PrivateAction", skip_serializing_if = "Option::is_none")]
    pub private_action: Option<PrivateAction>,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActivateControllerAction {
    #[serde(rename = "@controllerRef", skip_serializing_if = "Option::is_none")]
    pub controller_ref: Option<String>,
    #[serde(
        rename = "@objectControllerRef",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_controller_ref: Option<String>,
    #[serde(rename = "@lateral", skip_serializing_if = "Option::is_none")]
    pub lateral: Option<Boolean>,
    #[serde(rename = "@longitudinal", skip_serializing_if = "Option::is_none")]
    pub longitudinal: Option<Boolean>,
    #[serde(rename = "@animation", skip_serializing_if = "Option::is_none")]
    pub animation: Option<Boolean>,
    #[serde(rename = "@lighting", skip_serializing_if = "Option::is_none")]
    pub lighting: Option<Boolean>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddEntityAction {
    #[serde(rename = "Position")]
    pub position: Position,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AnimationAction {
    #[serde(rename = "AnimationType", skip_serializing_if = "Option::is_none")]
    pub animation_type: Option<AnimationType>,
    #[serde(rename = "AnimationState", skip_serializing_if = "Option::is_none")]
    pub animation_state: Option<Vec<AnimationState>>,
    #[serde(rename = "@loop", skip_serializing_if = "Option::is_none")]
    pub r#loop: Option<Boolean>,
    #[serde(rename = "@animationDuration", skip_serializing_if = "Option::is_none")]
    pub animation_duration: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AbsoluteSpeed {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AbsoluteTargetLane {
    #[serde(rename = "@value")]
    pub value: String,
    // TODO add SteadyState
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AbsoluteTargetLaneOffset {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AbsoluteTargetSpeed {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AcquirePositionAction {
    #[serde(rename = "Position")]
    pub position: Position,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AnimationFile {
    #[serde(rename = "File")]
    pub file: File,
    #[serde(rename = "@timeOffset", skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AnimationState {
    #[serde(rename = "@state")]
    pub state: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AnimationType {
    #[serde(rename = "ComponentAnimation", skip_serializing_if = "Option::is_none")]
    pub component_animation: Option<ComponentAnimation>,
    #[serde(
        rename = "PedestrianAnimation",
        skip_serializing_if = "Option::is_none"
    )]
    pub pedestrian_animation: Option<PedestrianAnimation>,
    #[serde(rename = "AnimationFile", skip_serializing_if = "Option::is_none")]
    pub animation_file: Option<AnimationFile>,
    #[serde(
        rename = "UserDefinedAnimation",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_animation: Option<UserDefinedAnimation>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppearanceAction {
    #[serde(rename = "LightStateAction", skip_serializing_if = "Option::is_none")]
    pub light_state_action: Option<LightStateAction>,
    #[serde(rename = "AnimationAction", skip_serializing_if = "Option::is_none")]
    pub animation_action: Option<AnimationAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AssignControllerAction {
    #[serde(rename = "Controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<Controller>>,
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<Vec<CatalogReference>>,
    #[serde(rename = "ObjectController", skip_serializing_if = "Option::is_none")]
    pub object_controller: Option<Vec<ObjectController>>,
    #[serde(rename = "@activateLateral", skip_serializing_if = "Option::is_none")]
    pub activate_lateral: Option<Boolean>,
    #[serde(
        rename = "@activateLongitudinal",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_longitudinal: Option<Boolean>,
    #[serde(rename = "@activateAnimation", skip_serializing_if = "Option::is_none")]
    pub activate_animation: Option<Boolean>,
    #[serde(rename = "@activateLighting", skip_serializing_if = "Option::is_none")]
    pub activate_lighting: Option<Boolean>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AssignRouteAction {
    #[serde(rename = "Route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Route>,
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AutomaticGear {
    #[serde(rename = "@gear")]
    pub gear: AutomaticGearType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Brake {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@maxRate", skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CentralSwarmObject {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Color {
    #[serde(rename = "ColorRgb", skip_serializing_if = "Option::is_none")]
    pub color_rgb: Option<ColorRgb>,
    #[serde(rename = "ColorCmyk", skip_serializing_if = "Option::is_none")]
    pub color_cmyk: Option<ColorCmyk>,
    #[serde(rename = "@colorType")]
    pub color_type: ColorType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ColorCmyk {
    #[serde(rename = "@cyan")]
    pub cyan: Double,
    #[serde(rename = "@magenta")]
    pub magenta: Double,
    #[serde(rename = "@yellow")]
    pub yellow: Double,
    #[serde(rename = "@key")]
    pub key: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ColorRgb {
    #[serde(rename = "@red")]
    pub red: Double,
    #[serde(rename = "@green")]
    pub green: Double,
    #[serde(rename = "@blue")]
    pub blue: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ComponentAnimation {
    #[serde(rename = "VehicleComponent", skip_serializing_if = "Option::is_none")]
    pub vehicle_component: Option<VehicleComponent>,
    #[serde(
        rename = "UserDefinedComponent",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_component: Option<UserDefinedComponent>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConditionGroup {
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<Vec<Condition>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConnectTrailerAction {
    #[serde(rename = "@trailerRef")]
    pub trailer_ref: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CustomCommandAction {
    #[serde(rename = "@type")]
    pub r#type: String,

    #[serde(rename = "$text")]
    pub content: String,
    // #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    // pub content: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteEntityAction {}

#[derive(Deserialize, Serialize, Debug)]
pub struct DirectionOfTravelDistribution {
    #[serde(rename = "@same")]
    pub same: Double,
    #[serde(rename = "@opposite")]
    pub opposite: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DisconnectTrailerAction {}

#[derive(Deserialize, Serialize, Debug)]
pub struct DynamicConstraints {
    #[serde(rename = "@maxAcceleration", skip_serializing_if = "Option::is_none")]
    pub max_acceleration: Option<Double>,
    #[serde(
        rename = "@maxAccelerationRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_acceleration_rate: Option<Double>,
    #[serde(rename = "@maxDeceleration", skip_serializing_if = "Option::is_none")]
    pub max_deceleration: Option<Double>,
    #[serde(
        rename = "@maxDecelerationRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_deceleration_rate: Option<Double>,
    #[serde(rename = "@maxSpeed", skip_serializing_if = "Option::is_none")]
    pub max_speed: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntityAction {
    #[serde(rename = "AddEntityAction", skip_serializing_if = "Option::is_none")]
    pub add_entity_action: Option<AddEntityAction>,
    #[serde(rename = "DeleteEntityAction", skip_serializing_if = "Option::is_none")]
    pub delete_entity_action: Option<DeleteEntityAction>,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntityDistribution {
    #[serde(
        rename = "EntityDistributionEntry",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_distribution_entry: Option<Vec<EntityDistributionEntry>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntityDistributionEntry {
    #[serde(
        rename = "ScenarioObjectTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_object_template: Option<ScenarioObjectTemplate>,
    #[serde(rename = "@weight")]
    pub weight: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntityObject {
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference>,
    #[serde(rename = "Vehicle", skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<Vehicle>,
    #[serde(rename = "Pedestrian", skip_serializing_if = "Option::is_none")]
    pub pedestrian: Option<Pedestrian>,
    #[serde(rename = "MiscObject", skip_serializing_if = "Option::is_none")]
    pub misc_object: Option<MiscObject>,
    #[serde(
        rename = "ExternalObjectReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_object_reference: Option<ExternalObjectReference>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EnvironmentAction {
    #[serde(rename = "Environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FinalSpeed {
    #[serde(rename = "AbsoluteSpeed", skip_serializing_if = "Option::is_none")]
    pub absolute_speed: Option<AbsoluteSpeed>,
    #[serde(
        rename = "RelativeSpeedToMaster",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_speed_to_master: Option<RelativeSpeedToMaster>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlobalAction {
    #[serde(rename = "EnvironmentAction", skip_serializing_if = "Option::is_none")]
    pub environment_action: Option<EnvironmentAction>,
    #[serde(rename = "EntityAction", skip_serializing_if = "Option::is_none")]
    pub entity_action: Option<EntityAction>,
    #[serde(
        rename = "InfrastructureAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub infrastructure_action: Option<InfrastructureAction>,
    #[serde(rename = "SetMonitorAction", skip_serializing_if = "Option::is_none")]
    pub set_monitor_action: Option<SetMonitorAction>,
    #[serde(rename = "ParameterAction", skip_serializing_if = "Option::is_none")]
    pub parameter_action: Option<ParameterAction>,
    #[serde(rename = "TrafficAction", skip_serializing_if = "Option::is_none")]
    pub traffic_action: Option<TrafficAction>,
    #[serde(rename = "VariableAction", skip_serializing_if = "Option::is_none")]
    pub variable_action: Option<VariableAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InfrastructureAction {
    #[serde(rename = "TrafficSignalAction")]
    pub traffic_signal_action: TrafficSignalAction,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InitActions {
    #[serde(rename = "GlobalAction", skip_serializing_if = "Option::is_none")]
    pub global_action: Option<Vec<GlobalAction>>,
    #[serde(rename = "UserDefinedAction", skip_serializing_if = "Option::is_none")]
    pub user_defined_action: Option<Vec<UserDefinedAction>>,
    #[serde(rename = "Private", skip_serializing_if = "Option::is_none")]
    pub private: Option<Vec<Private>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Lane {
    #[serde(rename = "@id")]
    pub id: Int,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LaneChangeAction {
    #[serde(rename = "LaneChangeActionDynamics")]
    pub lane_change_action_dynamics: TransitionDynamics,
    #[serde(rename = "LaneChangeTarget")]
    pub lane_change_target: LaneChangeTarget,
    #[serde(rename = "@targetLaneOffset", skip_serializing_if = "Option::is_none")]
    pub target_lane_offset: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LaneChangeTarget {
    #[serde(rename = "RelativeTargetLane", skip_serializing_if = "Option::is_none")]
    pub relative_target_lane: Option<RelativeTargetLane>,
    #[serde(rename = "AbsoluteTargetLane", skip_serializing_if = "Option::is_none")]
    pub absolute_target_lane: Option<AbsoluteTargetLane>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LaneOffsetAction {
    #[serde(rename = "LaneOffsetActionDynamics")]
    pub lane_offset_action_dynamics: LaneOffsetActionDynamics,
    #[serde(rename = "LaneOffsetTarget")]
    pub lane_offset_target: LaneOffsetTarget,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LaneOffsetActionDynamics {
    #[serde(rename = "@dynamicsShape")]
    pub dynamics_shape: DynamicsShape,
    #[serde(rename = "@maxLateralAcc", skip_serializing_if = "Option::is_none")]
    pub max_lateral_acc: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LaneOffsetTarget {
    #[serde(
        rename = "RelativeTargetLaneOffset",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_target_lane_offset: Option<RelativeTargetLaneOffset>,
    #[serde(
        rename = "AbsoluteTargetLaneOffset",
        skip_serializing_if = "Option::is_none"
    )]
    pub absolute_target_lane_offset: Option<AbsoluteTargetLaneOffset>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LateralAction {
    #[serde(rename = "LaneChangeAction", skip_serializing_if = "Option::is_none")]
    pub lane_change_action: Option<LaneChangeAction>,
    #[serde(rename = "LaneOffsetAction", skip_serializing_if = "Option::is_none")]
    pub lane_offset_action: Option<LaneOffsetAction>,
    #[serde(
        rename = "LateralDistanceAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub lateral_distance_action: Option<LateralDistanceAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LateralDistanceAction {
    #[serde(rename = "DynamicConstraints", skip_serializing_if = "Option::is_none")]
    pub dynamic_constraints: Option<Vec<DynamicConstraints>>,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "@distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<Double>,
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
    #[serde(rename = "@displacement", skip_serializing_if = "Option::is_none")]
    pub displacement: Option<LateralDisplacement>,
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LightState {
    #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Vec<Color>>,
    #[serde(rename = "@mode")]
    pub mode: LightMode,
    #[serde(rename = "@luminousIntensity", skip_serializing_if = "Option::is_none")]
    pub luminous_intensity: Option<Double>,
    #[serde(
        rename = "@flashingOnDuration",
        skip_serializing_if = "Option::is_none"
    )]
    pub flashing_on_duration: Option<Double>,
    #[serde(
        rename = "@flashingOffDuration",
        skip_serializing_if = "Option::is_none"
    )]
    pub flashing_off_duration: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LightStateAction {
    #[serde(rename = "LightType", skip_serializing_if = "Option::is_none")]
    pub light_type: Option<LightType>,
    #[serde(rename = "LightState", skip_serializing_if = "Option::is_none")]
    pub light_state: Option<LightState>,
    #[serde(rename = "@transitionTime", skip_serializing_if = "Option::is_none")]
    pub transition_time: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LightType {
    #[serde(rename = "VehicleLight", skip_serializing_if = "Option::is_none")]
    pub vehicle_light: Option<VehicleLight>,
    #[serde(rename = "UserDefinedLight", skip_serializing_if = "Option::is_none")]
    pub user_defined_light: Option<UserDefinedLight>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LongitudinalAction {
    #[serde(rename = "SpeedAction", skip_serializing_if = "Option::is_none")]
    pub speed_action: Option<SpeedAction>,
    #[serde(
        rename = "LongitudinalDistanceAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub longitudinal_distance_action: Option<LongitudinalDistanceAction>,
    #[serde(rename = "SpeedProfileAction", skip_serializing_if = "Option::is_none")]
    pub speed_profile_action: Option<SpeedProfileAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LongitudinalDistanceAction {
    #[serde(rename = "DynamicConstraints", skip_serializing_if = "Option::is_none")]
    pub dynamic_constraints: Option<Vec<DynamicConstraints>>,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "@distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<Double>,
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
    #[serde(rename = "@timeGap", skip_serializing_if = "Option::is_none")]
    pub time_gap: Option<Double>,
    #[serde(rename = "@displacement", skip_serializing_if = "Option::is_none")]
    pub displacement: Option<LongitudinalDisplacement>,
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModifyRule {
    #[serde(rename = "AddValue", skip_serializing_if = "Option::is_none")]
    pub add_value: Option<ParameterAddValueRule>,
    #[serde(rename = "MultiplyByValue", skip_serializing_if = "Option::is_none")]
    pub multiply_by_value: Option<ParameterMultiplyByValueRule>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OverrideBrakeAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Double>,

    #[serde(rename = "BrakePercent", skip_serializing_if = "Option::is_none")]
    pub brake_percent: Option<Brake>,
    #[serde(rename = "BrakeForce", skip_serializing_if = "Option::is_none")]
    pub brake_force: Option<Brake>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OverrideClutchAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@maxRate", skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OverrideParkingBrakeAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Double>,

    #[serde(rename = "BrakePercent", skip_serializing_if = "Option::is_none")]
    pub brake_percent: Option<Brake>,
    #[serde(rename = "BrakeForce", skip_serializing_if = "Option::is_none")]
    pub brake_force: Option<Brake>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OverrideSteeringWheelAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@maxRate", skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<Double>,
    #[serde(rename = "@maxTorque", skip_serializing_if = "Option::is_none")]
    pub max_torque: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OverrideThrottleAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@maxRate", skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterAction {
    #[serde(rename = "SetAction", skip_serializing_if = "Option::is_none")]
    pub set_action: Option<ParameterSetAction>,
    #[serde(rename = "ModifyAction", skip_serializing_if = "Option::is_none")]
    pub modify_action: Option<ParameterModifyAction>,
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterAddValueRule {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterModifyAction {
    #[serde(rename = "Rule")]
    pub rule: ModifyRule,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterMultiplyByValueRule {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterSetAction {
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PedestrianAnimation {
    #[serde(rename = "PedestrianGesture", skip_serializing_if = "Option::is_none")]
    pub pedestrian_gesture: Option<Vec<PedestrianGesture>>,
    #[serde(rename = "@motion", skip_serializing_if = "Option::is_none")]
    pub motion: Option<PedestrianMotionType>,
    #[serde(
        rename = "@userDefinedPedestrianAnimation",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_pedestrian_animation: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PedestrianGesture {
    #[serde(rename = "@gesture")]
    pub gesture: PedestrianGestureType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Polygon {
    #[serde(rename = "Position")]
    pub position: Vec<Position>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Private {
    #[serde(rename = "PrivateAction", skip_serializing_if = "Option::is_none")]
    pub private_action: Option<Vec<PrivateAction>>,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PrivateAction {
    #[serde(rename = "LongitudinalAction", skip_serializing_if = "Option::is_none")]
    pub longitudinal_action: Option<LongitudinalAction>,
    #[serde(rename = "LateralAction", skip_serializing_if = "Option::is_none")]
    pub lateral_action: Option<LateralAction>,
    #[serde(rename = "VisibilityAction", skip_serializing_if = "Option::is_none")]
    pub visibility_action: Option<VisibilityAction>,
    #[serde(rename = "SynchronizeAction", skip_serializing_if = "Option::is_none")]
    pub synchronize_action: Option<SynchronizeAction>,
    #[serde(
        rename = "ActivateControllerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_controller_action: Option<ActivateControllerAction>,
    #[serde(rename = "ControllerAction", skip_serializing_if = "Option::is_none")]
    pub controller_action: Option<ControllerAction>,
    #[serde(rename = "TeleportAction", skip_serializing_if = "Option::is_none")]
    pub teleport_action: Option<TeleportAction>,
    #[serde(rename = "RoutingAction", skip_serializing_if = "Option::is_none")]
    pub routing_action: Option<RoutingAction>,
    #[serde(rename = "AppearanceAction", skip_serializing_if = "Option::is_none")]
    pub appearance_action: Option<AppearanceAction>,
    #[serde(rename = "TrailerAction", skip_serializing_if = "Option::is_none")]
    pub trailer_action: Option<TrailerAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RandomRouteAction;

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeSpeedToMaster {
    #[serde(
        rename = "TargetDistanceSteadyState",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_distance_steady_state: Option<TargetDistanceSteadyState>,
    #[serde(
        rename = "TargetTimeSteadyState",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_time_steady_state: Option<TargetTimeSteadyState>,

    #[serde(rename = "@speedTargetValueType")]
    pub speed_target_value_type: SpeedTargetValueType,
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeTargetLane {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@value")]
    pub value: Int,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeTargetLaneOffset {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelativeTargetSpeed {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "@speedTargetValueType")]
    pub speed_target_value_type: SpeedTargetValueType,
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoadCursor {
    #[serde(rename = "Lane", skip_serializing_if = "Option::is_none")]
    pub lane: Option<Vec<Lane>>,
    #[serde(rename = "@roadId")]
    pub road_id: String,
    #[serde(rename = "@s", skip_serializing_if = "Option::is_none")]
    pub s: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoadRange {
    #[serde(rename = "RoadCursor")]
    pub road_cursor: Vec<RoadCursor>, // length >= 2
    #[serde(rename = "@length", skip_serializing_if = "Option::is_none")]
    pub length: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoutingAction {
    #[serde(rename = "AssignRouteAction", skip_serializing_if = "Option::is_none")]
    pub assign_route_action: Option<AssignRouteAction>,
    #[serde(
        rename = "FollowTrajectoryAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_trajectory_action: Option<FollowTrajectoryAction>,
    #[serde(
        rename = "AcquirePositionAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub acquire_position_action: Option<AcquirePositionAction>,
    #[serde(rename = "RandomRouteAction", skip_serializing_if = "Option::is_none")]
    pub random_route_action: Option<RandomRouteAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScenarioObjectTemplate {
    #[serde(rename = "ObjectController", skip_serializing_if = "Option::is_none")]
    pub object_controller: Option<Vec<ObjectController>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SensorReference {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SensorReferenceSet {
    #[serde(rename = "SensorReference", skip_serializing_if = "Option::is_none")]
    pub sensor_reference: Option<Vec<SensorReference>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetMonitorAction {
    #[serde(rename = "@monitorRef")]
    pub monitor_ref: String,
    #[serde(rename = "@value")]
    pub value: Boolean,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpeedAction {
    #[serde(rename = "SpeedActionDynamics")]
    pub speed_action_dynamics: TransitionDynamics,
    #[serde(rename = "SpeedActionTarget")]
    pub speed_action_target: SpeedActionTarget,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpeedActionTarget {
    #[serde(
        rename = "RelativeTargetSpeed",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_target_speed: Option<RelativeTargetSpeed>,
    #[serde(
        rename = "AbsoluteTargetSpeed",
        skip_serializing_if = "Option::is_none"
    )]
    pub absolute_target_speed: Option<AbsoluteTargetSpeed>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpeedProfileAction {
    #[serde(rename = "DynamicConstraints", skip_serializing_if = "Option::is_none")]
    pub dynamic_constraints: Option<Vec<DynamicConstraints>>,
    #[serde(rename = "SpeedProfileEntry", skip_serializing_if = "Option::is_none")]
    pub speed_profile_entry: Option<Vec<SpeedProfileEntry>>,
    #[serde(rename = "@entityRef", skip_serializing_if = "Option::is_none")]
    pub entity_ref: Option<String>,
    #[serde(rename = "@followingMode")]
    pub following_mode: FollowingMode,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpeedProfileEntry {
    #[serde(rename = "@speed")]
    pub speed: Double,
    #[serde(rename = "@time", skip_serializing_if = "Option::is_none")]
    pub time: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SteadyState {
    #[serde(
        rename = "TargetDistanceSteadyState",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_distance_steady_state: Option<TargetDistanceSteadyState>,
    #[serde(
        rename = "TargetTimeSteadyState",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_time_steady_state: Option<TargetTimeSteadyState>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SynchronizeAction {
    #[serde(rename = "TargetPositionMaster")]
    pub target_position_master: Position,
    #[serde(rename = "TargetPosition")]
    pub target_position: Position,
    #[serde(rename = "FinalSpeed", skip_serializing_if = "Option::is_none")]
    pub final_speed: Option<Vec<FinalSpeed>>,
    #[serde(rename = "@masterEntityRef")]
    pub master_entity_ref: String,
    #[serde(
        rename = "@targetToleranceMaster",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_tolerance_master: Option<Double>,
    #[serde(rename = "@targetTolerance", skip_serializing_if = "Option::is_none")]
    pub target_tolerance: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TargetDistanceSteadyState {
    #[serde(rename = "@distance")]
    pub distance: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TargetTimeSteadyState {
    #[serde(rename = "@time")]
    pub time: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeleportAction {
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeReference {
    #[serde(rename = "None", skip_serializing_if = "Option::is_none")]
    pub none: Option<NoneStruct>,
    #[serde(rename = "Timing", skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Timing {
    #[serde(rename = "@domainAbsoluteRelative")]
    pub domain_absolute_relative: ReferenceContext,
    #[serde(rename = "@offset")]
    pub offset: Double,
    #[serde(rename = "@scale")]
    pub scale: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficAction {
    #[serde(
        rename = "TrafficSourceAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_source_action: Option<TrafficSourceAction>,
    #[serde(rename = "TrafficSinkAction", skip_serializing_if = "Option::is_none")]
    pub traffic_sink_action: Option<TrafficSinkAction>,
    #[serde(rename = "TrafficSwarmAction", skip_serializing_if = "Option::is_none")]
    pub traffic_swarm_action: Option<TrafficSwarmAction>,
    #[serde(rename = "TrafficAreaAction", skip_serializing_if = "Option::is_none")]
    pub traffic_area_action: Option<TrafficAreaAction>,
    #[serde(rename = "TrafficStopAction", skip_serializing_if = "Option::is_none")]
    pub traffic_stop_action: Option<TrafficStopAction>,
    #[serde(rename = "@trafficName", skip_serializing_if = "Option::is_none")]
    pub traffic_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficArea {
    #[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Polygon>,
    #[serde(rename = "RoadRange", skip_serializing_if = "Option::is_none")]
    pub road_range: Option<Vec<RoadRange>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficAreaAction {
    #[serde(rename = "TrafficDistribution")]
    pub traffic_distribution: TrafficDistribution,
    #[serde(rename = "TrafficArea")]
    pub traffic_area: TrafficArea,
    #[serde(rename = "@numberOfEntities")]
    pub number_of_entities: UnsignedInt,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficDistribution {
    #[serde(
        rename = "TrafficDistributionEntry",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_distribution_entry: Option<Vec<TrafficDistributionEntry>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficDistributionEntry {
    #[serde(rename = "EntityDistribution", skip_serializing_if = "Option::is_none")]
    pub entity_distribution: Option<EntityDistribution>,
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Properties>>,
    #[serde(rename = "@weight")]
    pub weight: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSignalAction {
    #[serde(
        rename = "TrafficSignalControllerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_signal_controller_action: Option<TrafficSignalControllerAction>,
    #[serde(
        rename = "TrafficSignalStateAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_signal_state_action: Option<TrafficSignalStateAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSignalControllerAction {
    #[serde(rename = "@trafficSignalControllerRef")]
    pub traffic_signal_controller_ref: String,
    #[serde(rename = "@phase")]
    pub phase: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSignalStateAction {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@state")]
    pub state: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSinkAction {
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "TrafficDefinition", skip_serializing_if = "Option::is_none")]
    pub traffic_definition: Option<Vec<TrafficDefinition>>,
    #[serde(rename = "@radius")]
    pub radius: Double,
    #[serde(rename = "@rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSourceAction {
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "TrafficDefinition", skip_serializing_if = "Option::is_none")]
    pub traffic_definition: Option<Vec<TrafficDefinition>>,
    #[serde(
        rename = "TrafficDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_distribution: Option<Vec<TrafficDistribution>>,
    #[serde(rename = "@radius")]
    pub radius: Double,
    #[serde(rename = "@rate")]
    pub rate: Double,
    #[serde(rename = "@velocity", skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Double>,
    #[serde(rename = "@speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficStopAction {}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrafficSwarmAction {
    #[serde(rename = "CentralObject")]
    pub central_object: CentralSwarmObject,
    #[serde(rename = "TrafficDefinition", skip_serializing_if = "Option::is_none")]
    pub traffic_definition: Option<Vec<TrafficDefinition>>,
    #[serde(
        rename = "TrafficDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_distribution: Option<Vec<TrafficDistribution>>,
    #[serde(rename = "InitialSpeedRange", skip_serializing_if = "Option::is_none")]
    pub initial_speed_range: Option<Vec<Range>>,
    #[serde(
        rename = "DirectionOfTravelDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub direction_of_travel_distribution: Option<Vec<DirectionOfTravelDistribution>>,
    #[serde(rename = "@innerRadius")]
    pub inner_radius: Double,
    #[serde(rename = "@numberOfVehicles")]
    pub number_of_vehicles: UnsignedInt,
    #[serde(rename = "@offset")]
    pub offset: Double,
    #[serde(rename = "@semiMajorAxis")]
    pub semi_major_axis: Double,
    #[serde(rename = "@semiMinorAxis")]
    pub semi_minor_axis: Double,
    #[serde(rename = "@velocity", skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrailerAction {
    #[serde(
        rename = "ConnectTrailerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub connect_trailer_action: Option<ConnectTrailerAction>,
    #[serde(
        rename = "DisconnectTrailerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub disconnect_trailer_action: Option<DisconnectTrailerAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrajectoryFollowingMode {
    #[serde(rename = "@followingMode")]
    pub following_mode: FollowingMode,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransitionDynamics {
    #[serde(rename = "@dynamicsDimension")]
    pub dynamics_dimension: DynamicsDimension,
    #[serde(rename = "@dynamicsShape")]
    pub dynamics_shape: DynamicsShape,
    #[serde(rename = "@followingMode", skip_serializing_if = "Option::is_none")]
    pub following_mode: Option<FollowingMode>,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Double>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDefinedAction {
    #[serde(
        rename = "CustomCommandAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_command_action: Option<CustomCommandAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDefinedAnimation {
    #[serde(rename = "@userDefinedAnimationType")]
    pub user_defined_animation_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDefinedComponent {
    #[serde(rename = "@userDefinedComponentType")]
    pub user_defined_component_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDefinedLight {
    #[serde(rename = "@userDefinedLightType")]
    pub user_defined_light_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VariableAction {
    #[serde(rename = "SetAction", skip_serializing_if = "Option::is_none")]
    pub set_action: Option<VariableSetAction>,
    #[serde(rename = "ModifyAction", skip_serializing_if = "Option::is_none")]
    pub modify_action: Option<VariableModifyAction>,
    #[serde(rename = "@variableRef")]
    pub variable_ref: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VariableAddValueRule {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VariableModifyAction {
    #[serde(rename = "Rule")]
    pub rule: VariableModifyRule,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VariableModifyRule {
    #[serde(rename = "AddValue", skip_serializing_if = "Option::is_none")]
    pub add_value: Option<VariableAddValueRule>,
    #[serde(rename = "MultiplyByValue", skip_serializing_if = "Option::is_none")]
    pub multiply_by_value: Option<VariableMultiplyByValueRule>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VariableMultiplyByValueRule {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VariableSetAction {
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VehicleComponent {
    #[serde(rename = "@vehicleComponentType")]
    pub vehicle_component_type: VehicleComponentType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VehicleLight {
    #[serde(rename = "@vehicleLightType")]
    pub vehicle_light_type: VehicleLightType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VisibilityAction {
    #[serde(rename = "SensorReferenceSet", skip_serializing_if = "Option::is_none")]
    pub sensor_reference_set: Option<Vec<SensorReferenceSet>>,
    #[serde(rename = "@graphics")]
    pub graphics: Boolean,
    #[serde(rename = "@sensors")]
    pub sensors: Boolean,
    #[serde(rename = "@traffic")]
    pub traffic: Boolean,
}

#[cfg(test)]
mod tests {
    use crate::openscenario::storyboard::UserDefinedAction;

    #[test]
    fn it_works_opendrive6() {
        let xml = r#"<UserDefinedAction>
                                    <CustomCommandAction type="noop">Wait</CustomCommandAction>
                                </UserDefinedAction>"#;
        let r = quick_xml::de::from_str::<UserDefinedAction>(&xml).unwrap();
        println!("{:#?}", r);
    }
}
