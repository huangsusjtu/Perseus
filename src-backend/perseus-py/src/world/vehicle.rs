use pyo3::{pyclass, pymethods};

use crate::map::waypoint::WayPoint;

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum VehicleType {
    Unicorn,
    Kylin,
    Hulk,
    // todo!(add more vehicle)
}

/// 车辆的固定参数
#[pyclass(set_all, get_all)]
#[derive(Clone)]
pub struct VehicleBaseParam {
    pub max_battery_capacity: f64,
    // 最大电池电量, 如果是油车，
    // 这个字段就代表 最大油量
    pub max_water_capacity: f64,   // 最大水箱容量
    pub max_recharge_mileage: f64, // 最大续航里程
    pub max_garbage_capacity: f64, // 最大垃圾箱容量
    pub max_speed: f64,            // 最大时速
    pub default_battery_consumption: f64,
    // 默认耗电量， , 如果是油车，
    // 这个字段就代表 默认油量
    pub default_water_consumption: f64, // 默认耗水量
    pub default_speed: f64,             // 默认时速  km/h
    pub charging_rate: f64,             // 充电速率, 每分钟增加多少百分比
    pub water_replenish_rate: f64,      // 加水速率, 每分钟增加多少L
}

/// 车辆的实时状态
#[pyclass(set_all, get_all)]
#[derive(Clone)]
pub struct VehicleState {
    pub real_time_battery: f64,
    // 实时电量， , 如果是油车，这个字段就代表
    // 实时油量
    pub real_time_water: f64,         // 实时水量
    pub real_time_garbage: f64,       // 实时垃圾量
    pub real_time_speed: f64,         // 实时车速
    pub real_time_mileage: f64,       // 实时里程
    pub real_time_position: WayPoint, // 实时坐标

    pub status: WorkStatus,            // 工作状态
    pub work_mode: WorkMode,           // 工作状态
    pub operation_mode: OperationMode, // 工作状态
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum WorkStatus {
    Work,
    Operation,
    Transfer, // 转场
    Park,     // 停靠
    Off,      // 关机
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum WorkMode {
    NONE = 0,      //
    DrySweep = 1,  // 干扫、
    WetSweep = 2,  // 湿扫、
    BlowSweep = 3, // 吹扫、
    WashSweep = 4, // 洗扫、
    Washing = 5,   // 冲洗、
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum OperationMode {
    NONE = 0,            //
    DumpGarbage = 1,     // 倒垃圾
    Charging = 2,        // 充电、
    WaterPump = 3,       // 补水
    DustbinCleaning = 4, // 垃圾箱自清洁
}

/// 车辆的所有信息
#[pyclass(set_all, get_all)]
#[derive(Clone)]
pub struct Vehicle {
    // 车辆类型
    pub r#type: VehicleType,
    pub name: String, // 车辆命名

    pub param: VehicleBaseParam,
    pub state: VehicleState,
}

///
#[pymethods]
impl Vehicle {
    #[new]
    fn new() -> Self {
        Vehicle {
            r#type: VehicleType::Unicorn,
            name: "".to_string(),
            param: VehicleBaseParam {
                max_battery_capacity: 0.0,
                max_water_capacity: 0.0,
                max_recharge_mileage: 0.0,
                max_garbage_capacity: 0.0,
                max_speed: 0.0,
                default_battery_consumption: 0.0,
                default_water_consumption: 0.0,
                default_speed: 0.0,
                charging_rate: 0.0,
                water_replenish_rate: 0.0,
            },
            state: VehicleState {
                real_time_battery: 0.0,
                real_time_water: 0.0,
                real_time_garbage: 0.0,
                real_time_speed: 0.0,
                real_time_mileage: 0.0,
                real_time_position: WayPoint { x: 0.0, y: 0.0 },
                status: WorkStatus::Off,
                work_mode: WorkMode::NONE,
                operation_mode: OperationMode::NONE,
            },
        }
    }
}

///

impl From<&libmodel::sanitation::vehicle::WorkMode> for WorkMode {
    fn from(value: &libmodel::sanitation::vehicle::WorkMode) -> Self {
        match value {
            libmodel::sanitation::vehicle::WorkMode::NONE => WorkMode::NONE,
            libmodel::sanitation::vehicle::WorkMode::DrySweep => {
                WorkMode::DrySweep
            }
            libmodel::sanitation::vehicle::WorkMode::WetSweep => {
                WorkMode::WetSweep
            }
            libmodel::sanitation::vehicle::WorkMode::BlowSweep => {
                WorkMode::BlowSweep
            }
            libmodel::sanitation::vehicle::WorkMode::WashSweep => {
                WorkMode::WashSweep
            }
            libmodel::sanitation::vehicle::WorkMode::Washing => {
                WorkMode::Washing
            }
        }
    }
}
impl From<&libmodel::sanitation::vehicle::OperationMode> for OperationMode {
    fn from(value: &libmodel::sanitation::vehicle::OperationMode) -> Self {
        match value {
            libmodel::sanitation::vehicle::OperationMode::NONE => {
                OperationMode::NONE
            }
            libmodel::sanitation::vehicle::OperationMode::DumpGarbage => {
                OperationMode::DumpGarbage
            }
            libmodel::sanitation::vehicle::OperationMode::Charging => {
                OperationMode::Charging
            }
            libmodel::sanitation::vehicle::OperationMode::WaterPump => {
                OperationMode::WaterPump
            }
            libmodel::sanitation::vehicle::OperationMode::DustbinCleaning => {
                OperationMode::DustbinCleaning
            }
        }
    }
}


impl From<&libmodel::sanitation::vehicle::Vehicle> for Vehicle {
    fn from(value: &libmodel::sanitation::vehicle::Vehicle) -> Self {
        let mut status = WorkStatus::Off;
        let mut work_mode = WorkMode::NONE;
        let mut operation_mode = OperationMode::NONE;
        match value.state.status.clone() {
            libmodel::sanitation::vehicle::WorkStatus::Work(r) => {
                status = WorkStatus::Work;
                work_mode = WorkMode::from(&r);
            }
            libmodel::sanitation::vehicle::WorkStatus::Operation(r) => {
                status = WorkStatus::Work;
                operation_mode = OperationMode::from(&r);
            }
            libmodel::sanitation::vehicle::WorkStatus::Transfer => {}
            libmodel::sanitation::vehicle::WorkStatus::Park => {}
            libmodel::sanitation::vehicle::WorkStatus::Off => {}
        };

        Vehicle {
            r#type: match value.r#type {
                libmodel::sanitation::vehicle::VehicleType::Unicorn => {
                    VehicleType::Unicorn
                }
                libmodel::sanitation::vehicle::VehicleType::Kylin => {
                    VehicleType::Kylin
                }
                libmodel::sanitation::vehicle::VehicleType::Hulk => {
                    VehicleType::Hulk
                }
            },
            name: value.name.clone(),
            param: VehicleBaseParam {
                max_battery_capacity: value.param.max_water_capacity,
                max_water_capacity: value.param.max_water_capacity,
                max_recharge_mileage: value.param.max_recharge_mileage,
                max_garbage_capacity: value.param.max_garbage_capacity,
                max_speed: value.param.max_speed,
                default_battery_consumption: value
                    .param
                    .default_battery_consumption,
                default_water_consumption: value
                    .param
                    .default_water_consumption,
                default_speed: value.param.default_speed,
                charging_rate: value.param.charging_rate,
                water_replenish_rate: value.param.water_replenish_rate,
            },
            state: VehicleState {
                real_time_battery: value.state.real_time_battery,
                real_time_water: value.state.real_time_water,
                real_time_garbage: value.state.real_time_garbage,
                real_time_speed: value.state.real_time_speed,
                real_time_mileage: value.state.real_time_mileage,
                real_time_position: WayPoint { x: 0.0, y: 0.0 },
                status,
                work_mode,
                operation_mode,
            },
        }
    }
}
