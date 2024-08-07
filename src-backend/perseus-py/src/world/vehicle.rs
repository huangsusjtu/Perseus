use pyo3::pyclass;

use crate::map::waypoint::WayPoint;

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum VehicleType {
    Unicorn,
    Kylin,
    Hulk,
    // todo!(add more vehicle)
}

#[pyclass(set_all)]
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

#[pyclass(set_all)]
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
    pub word_mode: WorkMode,           // 工作状态
    pub operation_mode: OperationMode,           // 工作状态
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

#[pyclass(set_all)]
#[derive(Clone)]
pub struct Vehicle {
    // 车辆类型
    pub r#type: VehicleType,
    pub name: String, // 车辆命名

    pub param: VehicleBaseParam,
    pub state: VehicleState,
}
