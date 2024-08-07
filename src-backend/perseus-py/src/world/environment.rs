use pyo3::prelude::*;

/// 自然環境
#[pyclass(set_all)]
#[derive(Clone)]
pub struct Environment {
    pub temperature_range: [f64; 2], // 气温范围
    pub weather: WeatherType,        // 天气
    pub season: SeasonType,          // 季节
    pub defoliation: bool,           // 是否落叶
    pub work_time: Vec<[f32; 2]>,    // 当天的作业时间段
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum SeasonType {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
pub enum WeatherType {
    Fine,      // 晴
    Rainy,     // 雨
    Snow,      // 雪
    Ice,       // 冰
    Wind,      // 大风
    SandStorm, // 沙尘
    Haze,      // 雾霾
}

///
#[pymethods]
impl Environment {
    #[getter]
    fn temperature(&self) -> PyResult<[f64; 2]> {
        Ok(self.temperature_range.clone())
    }
    #[getter]
    fn weather(&self) -> PyResult<WeatherType> {
        Ok(self.weather.clone())
    }
    #[getter]
    fn season(&self) -> PyResult<SeasonType> {
        Ok(self.season.clone())
    }
    #[getter]
    fn defoliation(&self) -> PyResult<bool> {
        Ok(self.defoliation)
    }
    #[getter]
    fn work_time(&self) -> PyResult<Vec<[f32; 2]>> {
        Ok(self.work_time.clone())
    }
}

///
impl From<&libmodel::sanitation::environment::SeasonType> for SeasonType {
    fn from(value: &libmodel::sanitation::environment::SeasonType) -> Self {
        match value {
            libmodel::sanitation::environment::SeasonType::Spring => {
                SeasonType::Spring
            }
            libmodel::sanitation::environment::SeasonType::Summer => {
                SeasonType::Summer
            }
            libmodel::sanitation::environment::SeasonType::Autumn => {
                SeasonType::Autumn
            }
            libmodel::sanitation::environment::SeasonType::Winter => {
                SeasonType::Winter
            }
        }
    }
}

impl Into<libmodel::sanitation::environment::SeasonType> for &SeasonType {
    fn into(self) -> libmodel::sanitation::environment::SeasonType {
        match self {
            SeasonType::Spring => {
                libmodel::sanitation::environment::SeasonType::Spring
            }
            SeasonType::Summer => {
                libmodel::sanitation::environment::SeasonType::Summer
            }
            SeasonType::Autumn => {
                libmodel::sanitation::environment::SeasonType::Autumn
            }
            SeasonType::Winter => {
                libmodel::sanitation::environment::SeasonType::Winter
            }
        }
    }
}

impl From<&libmodel::sanitation::environment::WeatherType> for WeatherType {
    fn from(value: &libmodel::sanitation::environment::WeatherType) -> Self {
        match value {
            libmodel::sanitation::environment::WeatherType::Fine => {
                WeatherType::Fine
            }
            libmodel::sanitation::environment::WeatherType::Rainy => {
                WeatherType::Rainy
            }
            libmodel::sanitation::environment::WeatherType::Snow => {
                WeatherType::Snow
            }
            libmodel::sanitation::environment::WeatherType::Ice => {
                WeatherType::Ice
            }
            libmodel::sanitation::environment::WeatherType::Wind => {
                WeatherType::Wind
            }
            libmodel::sanitation::environment::WeatherType::SandStorm => {
                WeatherType::SandStorm
            }
            libmodel::sanitation::environment::WeatherType::Haze => {
                WeatherType::Haze
            }
        }
    }
}

impl Into<libmodel::sanitation::environment::WeatherType> for &WeatherType {
    fn into(self) -> libmodel::sanitation::environment::WeatherType {
        match self {
            WeatherType::Fine => {
                libmodel::sanitation::environment::WeatherType::Fine
            }
            WeatherType::Rainy => {
                libmodel::sanitation::environment::WeatherType::Rainy
            }
            WeatherType::Snow => {
                libmodel::sanitation::environment::WeatherType::Snow
            }
            WeatherType::Ice => {
                libmodel::sanitation::environment::WeatherType::Ice
            }
            WeatherType::Wind => {
                libmodel::sanitation::environment::WeatherType::Wind
            }
            WeatherType::SandStorm => {
                libmodel::sanitation::environment::WeatherType::SandStorm
            }
            WeatherType::Haze => {
                libmodel::sanitation::environment::WeatherType::Haze
            }
        }
    }
}

impl From<&libmodel::sanitation::environment::Environment> for Environment {
    fn from(value: &libmodel::sanitation::environment::Environment) -> Self {
        Environment {
            temperature_range: value.temperature_range.clone(),
            weather: value.weather.first().unwrap().into(),
            season: SeasonType::from(&value.season),
            defoliation: value.defoliation,
            work_time: value.work_time.iter().map(|v| v.r.clone()).collect(),
        }
    }
}

impl Into<libmodel::sanitation::environment::Environment> for &Environment {
    fn into(self) -> libmodel::sanitation::environment::Environment {
        libmodel::sanitation::environment::Environment {
            temperature_range: self.temperature_range.clone(),
            weather: vec![(&self.weather).into()],
            season: (&self.season).into(),
            defoliation: false,
            work_time: self
                .work_time
                .iter()
                .map(|v| libmodel::sanitation::util::Range { r: v.clone() })
                .collect(),
        }
    }
}
