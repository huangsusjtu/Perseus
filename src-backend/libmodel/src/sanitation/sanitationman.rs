use serde::{Deserialize, Serialize};

use crate::sanitation::util::LanePoint;

// 静态配置参数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SanitationManParam {
    #[serde(rename = "@max_working_time")]
    pub max_working_time: f64, // 一天做多工作时长
    #[serde(rename = "@max_cover_area")]
    pub max_cover_area: f64, // 一天最大工作活动区域
    #[serde(rename = "@cover_efficiency")]
    pub cover_efficiency: f64, // 单位时间里 覆盖面积
    #[serde(rename = "@max_garbage_capacity")]
    pub max_garbage_capacity: f64, // 最多的垃圾收集量
    #[serde(rename = "@garbage_clean_efficiency")]
    pub garbage_clean_efficiency: f64, // 单位时间里 清理垃圾速度
    #[serde(rename = "@max_speed")]
    pub max_speed: f64, // 转场移动速度
}

impl Default for SanitationManParam {
    fn default() -> Self {
        SanitationManParam {
            max_working_time: 10.0,
            max_cover_area: 1000.0,
            cover_efficiency: 300.0,
            max_garbage_capacity: 100.0,
            garbage_clean_efficiency: 1.0,
            max_speed: 3.0,
        }
    }
}

// 动态活动状态
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SanitationManState {
    #[serde(rename = "@real_time_working_time")]
    pub real_time_working_time: f64, // 已经工作时长
    #[serde(rename = "@real_time_cover_area")]
    pub real_time_cover_area: f64, // 已经作业面积
    #[serde(rename = "@real_time_garbage")]
    pub real_time_garbage: f64, // 已经收集垃圾量
    #[serde(rename = "@real_time_speed")]
    pub real_time_speed: f64, // 移动速度
    #[serde(rename = "position")]
    pub position: LanePoint, // 位置
}

impl SanitationManState {
    pub fn full_state() -> Self {
        SanitationManState {
            real_time_working_time: 0.0,
            real_time_cover_area: 0.0,
            real_time_garbage: 0.0,
            real_time_speed: 0.0,
            position: Default::default(),
        }
    }
}

// 环卫工
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sanitation {
    #[serde(rename = "@name")]
    pub name: String,

    pub param: SanitationManParam,
    pub state: SanitationManState,
}

impl Default for Sanitation {
    fn default() -> Self {
        Sanitation {
            name: "".to_string(),

            param: SanitationManParam::default(),
            state: SanitationManState::full_state(),
        }
    }
}
