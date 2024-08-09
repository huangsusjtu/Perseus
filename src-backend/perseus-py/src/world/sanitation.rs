use pyo3::prelude::*;

use crate::map::waypoint::WayPoint;

/// 环卫工的固定参数
#[pyclass(set_all, get_all)]
#[derive(Clone, Debug)]
pub struct SanitationManParam {
    pub max_working_time: f64,         // 一天做多工作时长
    pub max_cover_area: f64,           // 一天最大工作活动区域
    pub cover_efficiency: f64,         // 单位时间里 覆盖面积
    pub max_garbage_capacity: f64,     // 最多的垃圾收集量
    pub garbage_clean_efficiency: f64, // 单位时间里 清理垃圾速度
    pub max_speed: f64,                // 转场移动速度
}

/// 环卫工的实时状态
#[pyclass(set_all, get_all)]
#[derive(Clone, Debug)]
pub struct SanitationManState {
    pub real_time_working_time: f64, // 已经工作时长
    pub real_time_cover_area: f64,   // 已经作业面积
    pub real_time_garbage: f64,      // 已经收集垃圾量
    pub real_time_speed: f64,        // 移动速度
    pub position: WayPoint,          // 位置
}

/// 环卫工
#[pyclass(set_all, get_all)]
#[derive(Clone)]
pub struct Sanitation {
    pub name: String,

    pub param: SanitationManParam,
    pub state: SanitationManState,
}

///
#[pymethods]
impl SanitationManParam {
    #[new]
    fn new() -> Self {
        SanitationManParam {
            max_working_time: 0.0,
            max_cover_area: 0.0,
            cover_efficiency: 0.0,
            max_garbage_capacity: 0.0,
            garbage_clean_efficiency: 0.0,
            max_speed: 0.0,
        }
    }

    fn __str__(&self) -> String {
        format!(
            "SanitationManParam (max_working_time:{}, max_cover_area:{}, \
             cover_efficiency:{}, max_garbage_capacity:{}, \
             garbage_clean_efficiency:{}, max_speed:{}) ",
            self.max_working_time,
            self.max_cover_area,
            self.cover_efficiency,
            self.max_garbage_capacity,
            self.garbage_clean_efficiency,
            self.max_speed
        )
    }
}

#[pymethods]
impl SanitationManState {
    #[new]
    fn new() -> Self {
        SanitationManState {
            real_time_working_time: 0.0,
            real_time_cover_area: 0.0,
            real_time_garbage: 0.0,
            real_time_speed: 0.0,
            position: WayPoint { x: 0.0, y: 0.0 },
        }
    }

    fn __str__(&self) -> String {
        format!(
            "SanitationManState (real_time_working_time:{}, \
             real_time_cover_area:{}, real_time_garbage:{}, \
             real_time_speed:{}, position:{:?}) ",
            self.real_time_working_time,
            self.real_time_cover_area,
            self.real_time_garbage,
            self.real_time_speed,
            self.position,
        )
    }
}

#[pymethods]
impl Sanitation {
    #[new]
    fn new() -> Self {
        Sanitation {
            name: "".to_string(),
            param: SanitationManParam::new(),
            state: SanitationManState::new(),
        }
    }

    fn __str__(&self) -> String {
        format!(
            "Sanitation (name:{}, param:{:#?}, state:{:?})",
            self.name, self.param, self.state,
        )
    }
}

///
impl From<&libmodel::sanitation::sanitationman::Sanitation> for Sanitation {
    fn from(value: &libmodel::sanitation::sanitationman::Sanitation) -> Self {
        Sanitation {
            name: value.name.clone(),
            param: SanitationManParam {
                max_working_time: value.param.max_working_time,
                max_cover_area: value.param.max_cover_area,
                cover_efficiency: value.param.cover_efficiency,
                max_garbage_capacity: value.param.max_garbage_capacity,
                garbage_clean_efficiency: value.param.garbage_clean_efficiency,
                max_speed: value.param.max_speed,
            },
            state: SanitationManState {
                real_time_working_time: value.state.real_time_working_time,
                real_time_cover_area: value.state.real_time_cover_area,
                real_time_garbage: value.state.real_time_garbage,
                real_time_speed: value.state.real_time_speed,
                position: WayPoint {
                    x: value.state.position.x,
                    y: value.state.position.y,
                },
            },
        }
    }
}
