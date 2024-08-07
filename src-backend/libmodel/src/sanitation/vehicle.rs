use serde::{Deserialize, Serialize};

use crate::sanitation::util::LanePoint;

/// 定义各个 作业车辆
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VehicleType {
    Unicorn,
    Kylin,
    Hulk,
    // todo!(add more vehicle)
}

/// 车辆的基础参数， 相对固定的值， 不太经常修改
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VehicleBaseParam {
    #[serde(rename = "@max_battery_capacity")]
    pub max_battery_capacity: f64,
    /* 最大电池电量, 如果是油车，
                                       * 这个字段就代表 最大油量 */
    #[serde(rename = "@max_water_capacity")]
    pub max_water_capacity: f64,   // 最大水箱容量
    #[serde(rename = "@max_recharge_mileage")]
    pub max_recharge_mileage: f64, // 最大续航里程
    #[serde(rename = "@max_garbage_capacity")]
    pub max_garbage_capacity: f64, // 最大垃圾箱容量
    #[serde(rename = "@max_speed")]
    pub max_speed: f64,            // 最大时速
    #[serde(rename = "@default_battery_consumption")]
    pub default_battery_consumption: f64,
    /* 默认耗电量， , 如果是油车，
                                              * 这个字段就代表 默认油量 */
    #[serde(rename = "@default_water_consumption")]
    pub default_water_consumption: f64, // 默认耗水量
    #[serde(rename = "@default_speed")]
    pub default_speed: f64,             // 默认时速  km/h
    #[serde(rename = "@charging_rate")]
    pub charging_rate: f64,             // 充电速率, 每分钟增加多少百分比
    #[serde(rename = "@water_replenish_rate")]
    pub water_replenish_rate: f64,      // 加水速率, 每分钟增加多少L
}

impl Default for VehicleBaseParam {
    fn default() -> Self {
        VehicleBaseParam {
            max_battery_capacity: 100.0,
            max_water_capacity: 100.0,
            max_recharge_mileage: 10.0,
            max_garbage_capacity: 10.0,
            max_speed: 10.0,
            default_battery_consumption: 0.1,
            default_water_consumption: 0.1,
            default_speed: 2.0,
            charging_rate: 20.0,
            water_replenish_rate: 0.1,
        }
    }
}

/// 车辆的动态参数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VehicleState {
    #[serde(rename = "@real_time_battery")]
    pub real_time_battery: f64,
    /* 实时电量， , 如果是油车，这个字段就代表
                                    * 实时油量 */
    #[serde(rename = "@real_time_water")]
    pub real_time_water: f64,   // 实时水量
    #[serde(rename = "@real_time_garbage")]
    pub real_time_garbage: f64, // 实时垃圾量
    #[serde(rename = "@real_time_speed")]
    pub real_time_speed: f64,   // 实时车速
    #[serde(rename = "@real_time_mileage")]
    pub real_time_mileage: f64, // 实时里程
    #[serde(rename = "position")]
    pub real_time_position: LanePoint, // 实时坐标
    #[serde(rename = "@status")]
    pub status: WorkStatus,            // 工作状态
}

impl VehicleState {
    pub fn full_state(param: &VehicleBaseParam) -> Self {
        VehicleState {
            real_time_battery: param.max_battery_capacity,
            real_time_water: param.max_water_capacity,
            real_time_garbage: 0.0,
            real_time_speed: 0.0,
            real_time_mileage: 0.0,
            real_time_position: Default::default(),
            status: WorkStatus::Transfer,
        }
    }
}

/// 车辆工作模式
///  干扫、 湿扫、 吹扫、洗扫、 冲洗
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum WorkMode {
    NONE = 0, //
    DrySweep = 1,  // 干扫、
    WetSweep = 2,  // 湿扫、
    BlowSweep = 3, // 吹扫、
    WashSweep = 4, // 洗扫、
    Washing = 5,   // 冲洗、
}

/// 车辆运维模式
///  倒垃圾、 充电、 补水、 垃圾箱自清洁、
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OperationMode {
    NONE = 0, //
    DumpGarbage = 1,     // 倒垃圾
    Charging = 2,        // 充电、
    WaterPump = 3,       // 补水
    DustbinCleaning = 4, // 垃圾箱自清洁
}

/// 车辆状态
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum WorkStatus {
    Work(WorkMode),
    Operation(OperationMode),
    Transfer, // 转场
    Park,     // 停靠
    Off,      // 关机
}

/// 车辆的基础参数， 相对固定的值， 不太经常修改
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vehicle {
    // 车辆类型
    #[serde(rename = "@type")]
    pub r#type: VehicleType,
    #[serde(rename = "@name")]
    pub name: String, // 车辆命名


    pub param: VehicleBaseParam,
    pub state: VehicleState,
}

impl Default for Vehicle {
    fn default() -> Self {
        Vehicle {
            r#type: VehicleType::Unicorn,
            name: "0".to_string(),
            param: VehicleBaseParam::default(),
            state: VehicleState::full_state(&VehicleBaseParam::default()),
        }
    }
}