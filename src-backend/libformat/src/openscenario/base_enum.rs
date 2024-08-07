use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AutomaticGearType {
    N,
    P,
    R,
    D,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CloudState {
    Cloudy,
    Free,
    Overcast,
    Rainy,
    SkyOff,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ColorType {
    Other,
    Red,
    Yellow,
    Green,
    Blue,
    Violet,
    Orange,
    Brown,
    Black,
    Grey,
    White,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ConditionEdge {
    Falling,
    None,
    Rising,
    RisingOrFalling,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ControllerType {
    Lateral,
    Longitudinal,
    Lighting,
    Animation,
    Movement,
    Appearance,
    All,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CoordinateSystem {
    Entity,
    Lane,
    Road,
    Trajectory,
    World,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DirectionalDimension {
    Longitudinal,
    Lateral,
    Vertical,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DynamicsDimension {
    Distance,
    Rate,
    Time,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DynamicsShape {
    Cubic,
    Linear,
    Sinusoidal,
    Step,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FollowingMode {
    Follow,
    Position,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FractionalCloudCover {
    ZeroOktas,
    OneOktas,
    TwoOktas,
    ThreeOktas,
    FourOktas,
    FiveOktas,
    SixOktas,
    SevenOktas,
    EightOktas,
    NineOktas,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LateralDisplacement {
    Any,
    LeftToReferencedEntity,
    RightToReferencedEntity,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LightMode {
    On,
    Off,
    Flashing,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LongitudinalDisplacement {
    Any,
    TrailingReferencedEntity,
    LeadingReferencedEntity,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MiscObjectCategory {
    Barrier,
    Building,
    Crosswalk,
    Gantry,
    None,
    Obstacle,
    ParkingSpace,
    Patch,
    Pole,
    Railing,
    RoadMark,
    SoundBarrier,
    StreetLamp,
    TrafficIsland,
    Tree,
    Vegetation,
    Wind,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ObjectType {
    Miscellaneous,
    Pedestrian,
    Vehicle,
    External,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ParameterType {
    Boolean,
    DateTime,
    Double,
    Integer,
    String,
    UnsignedInt,
    UnsignedShort,
    Int,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PedestrianCategory {
    Animal,
    Pedestrian,
    Wheelchair,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PedestrianGestureType {
    PhoneCallRightHand,
    PhoneCallLeftHand,
    PhoneTextRightHand,
    PhoneTextLeftHand,
    WavingRightArm,
    WavingLeftArm,
    UmbrellaRightHand,
    UmbrellaLeftHand,
    CrossArms,
    CoffeeRightHand,
    CoffeeLeftHand,
    SandwichRightHand,
    SandwichLeftHand,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PedestrianMotionType {
    Standing,
    Sitting,
    Lying,
    Squatting,
    Walking,
    Running,
    Reeling,
    Crawling,
    Cycling,
    Jumping,
    Ducking,
    BendingDown,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PrecipitationType {
    Dry,
    Rain,
    Snow,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Priority {
    Overwrite,
    Override,
    Parallel,
    Skip,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ReferenceContext {
    Absolute,
    Relative,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RelativeDistanceType {
    Lateral,
    Longitudinal,
    CartesianDistance,
    EuclidianDistance,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    None,
    Ambulance,
    Civil,
    Fire,
    Military,
    Police,
    PublicTransport,
    RoadAssistance,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RouteStrategy {
    Fastest,
    LeastIntersections,
    Random,
    Shortest,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RoutingAlgorithm {
    AssignedRoute,
    Fastest,
    LeastIntersections,
    Shortest,
    Undefined,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Rule {
    EqualTo,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    NotEqualTo,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SpeedTargetValueType {
    Delta,
    Factor,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum StoryboardElementState {
    CompleteState,
    EndTransition,
    RunningState,
    SkipTransition,
    StandbyState,
    StartTransition,
    StopTransition,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum StoryboardElementType {
    Act,
    Action,
    Event,
    Maneuver,
    ManeuverGroup,
    Story,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TriggeringEntitiesRule {
    All,
    Any,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum VehicleCategory {
    Bicycle,
    Bus,
    Car,
    Motorbike,
    Semitrailer,
    Trailer,
    Train,
    Tram,
    Truck,
    Van,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum VehicleComponentType {
    Hood,
    Trunk,
    DoorFrontRight,
    DoorFrontLeft,
    DoorRearRight,
    DoorRearLeft,
    WindowFrontRight,
    WindowFrontLeft,
    WindowRearRight,
    WindowRearLeft,
    SideMirrors,
    SideMirrorRight,
    SideMirrorLeft,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum VehicleLightType {
    DaytimeRunningLights,
    LowBeam,
    HighBeam,
    FogLights,
    FogLightsFront,
    FogLightsRear,
    BrakeLights,
    WarningLights,
    IndicatorLeft,
    IndicatorRight,
    ReversingLights,
    LicensePlateIllumination,
    SpecialPurposeLights,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Wetness {
    Dry,
    Moist,
    WetWithPuddles,
    LowFlooded,
    HighFlooded,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AngleType {
    Heading,
    Pitch,
    Roll,
}
