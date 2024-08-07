use pyo3::{pyclass, pymethods};
use crate::map::waypoint::WayPoint;

///
#[pyclass(set_all)]
#[derive(Clone)]
pub struct SanitationManParam {
    pub max_working_time: f64, // 一天做多工作时长
    pub max_cover_area: f64, // 一天最大工作活动区域
    pub cover_efficiency: f64, // 单位时间里 覆盖面积
    pub max_garbage_capacity: f64, // 最多的垃圾收集量
    pub garbage_clean_efficiency: f64, // 单位时间里 清理垃圾速度
    pub max_speed: f64, // 转场移动速度
}

#[pyclass(set_all)]
#[derive(Clone)]
pub struct SanitationManState {
    pub real_time_working_time: f64, // 已经工作时长
    pub real_time_cover_area: f64, // 已经作业面积
    pub real_time_garbage: f64, // 已经收集垃圾量
    pub real_time_speed: f64, // 移动速度
    pub position: WayPoint, // 位置
}

#[pyclass(set_all)]
#[derive(Clone)]
pub struct Sanitation {
    pub name: String,

    pub param: SanitationManParam,
    pub state: SanitationManState,
}


///
#[pymethods]
impl SanitationManParam {

}




