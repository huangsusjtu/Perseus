use serde::{Deserialize, Serialize};

use crate::sanitation::util::Range;

/// 自然環境
#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize)]
pub enum SeasonType {
    #[default]
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum WeatherType {
    Fine,      // 晴
    Rainy,     // 雨
    Snow,      // 雪
    Ice,       // 冰
    Wind,      // 大风
    SandStorm, // 沙尘
    Haze,      // 雾霾
}

/// 定义 天气等环境信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Environment {
    #[serde(rename = "@temperature")]
    pub temperature_range: [f64; 2], // 气温范围
    #[serde(rename = "@weather")]
    pub weather: Vec<WeatherType>,     // 天气
    #[serde(rename = "@season")]
    pub season: SeasonType,              // 季节
    #[serde(rename = "@defoliation")]
    pub defoliation: bool,               // 是否落叶

    #[serde(rename = "work_time")]
    pub work_time: Vec<Range>, // 当天的作业时间段
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            temperature_range: [0.0, 25.0].into(),
            weather: vec![WeatherType::Fine, WeatherType::Rainy],
            season: SeasonType::Spring,
            defoliation: false,
            work_time: vec![[5.0, 10.0].into(), [15.0, 20.0].into()],
        }
    }
}
