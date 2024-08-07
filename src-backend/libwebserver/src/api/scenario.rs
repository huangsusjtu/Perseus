use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use libsimulator::ScenarioInfo;
use serde::{Deserialize, Serialize};

use crate::api::{UniformError, UniformOk};
use crate::AppState;

/// 场景集的管理

// 列举场景列表
#[get("list")]
async fn list_scenario(
    app: web::Data<AppState>,
) -> Result<ListScenarioReply, UniformError> {
    match app.scenario_svc.list() {
        Ok(r) => Ok(crate::api::scenario::ListScenarioReply { list: r }),
        Err(e) => Err(UniformError::from(e)),
    }
}

// 上传新场景， 或者替换
#[post("upload")]
async fn upload_scenario(
    app: web::Data<AppState>, req: web::Json<UploadScenarioRequest>,
) -> Result<UniformOk, UniformError> {
    let req: UploadScenarioRequest = req.into_inner();
    match app
        .scenario_svc
        .upload(&req.name, req.data.into(), req.force_write)
        
    {
        Ok(_) => Ok(UniformOk),
        Err(e) => Err(UniformError::from(e)),
    }
}

// 删除场景
#[delete("delete")]
async fn delete_scenario(
    app: web::Data<AppState>, name: String,
) -> Result<UniformOk, UniformError> {
    match app.scenario_svc.delete(&name) {
        Ok(_) => Ok(UniformOk),
        Err(e) => Err(UniformError::from(e)),
    }
}

// 下载该场景
#[get("get")]
async fn get_scenario(
    app: web::Data<AppState>, req: web::Query<ScenarioInfo>,
) -> Result<ScenarioData, UniformError> {
    match app
        .scenario_svc
        .get(&req.name)
        
        .map(|scenario| crate::api::scenario::ScenarioData::from(&scenario))
    {
        Ok(r) => Ok(r),
        Err(e) => Err(UniformError::from(e)),
    }
}

///  接口数据
// 场景列表
#[derive(Deserialize, Serialize, Debug)]
pub struct ListScenarioReply {
    pub list: Vec<ScenarioInfo>,
}
#[derive(Serialize, Debug)]
pub struct SimulatorInfo {
    pub name: String,
}

impl Responder for ListScenarioReply {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
impl Responder for SimulatorInfo {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadScenarioRequest {
    pub name: String,
    pub force_write: bool,
    pub data: ScenarioData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScenarioData {
    pub name: String, // 名
    // where
    pub map: String, // 作业地图名

    // when
    pub temperature_range: [f64; 2], // 气温范围
    pub weather: Vec<libmodel::sanitation::environment::WeatherType>, // 天气
    pub season: libmodel::sanitation::environment::SeasonType, // 季节
    pub defoliation: bool,           // 是否落叶
    pub work_time: Vec<libmodel::sanitation::util::Range>, // 当天的作业时间段

    // who，  哪些车和人
    pub vehicles: Vec<libmodel::sanitation::vehicle::Vehicle>,
    pub sanitation: Vec<libmodel::sanitation::sanitationman::Sanitation>,

    // what， 任务路线
    pub global_vehicle_task:
        Vec<libmodel::sanitation::task::GlobalPlannerPath>,
    pub global_man_task: Vec<libmodel::sanitation::task::GlobalPlannerArea>,

    // how 任务的分配
    pub vehicle_to_task_mapper: Vec<(String, String)>,
    pub sanitation_to_task_mapper: Vec<(String, String)>,
    pub driver_to_vehicle_mapper: Vec<(String, String)>,
}

impl Responder for ScenarioData {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

// 简化，扁平化场景数据 送给到前端
impl Into<libmodel::sanitation::Scenario> for ScenarioData {
    fn into(self) -> libmodel::sanitation::Scenario {
        return (&self).into();
    }
}
impl Into<libmodel::sanitation::Scenario> for &ScenarioData {
    fn into(self) -> libmodel::sanitation::Scenario {
        libmodel::sanitation::Scenario {
            header: libmodel::sanitation::header::Header {
                name: self.name.clone(),
                map_name: self.map.clone(),
                date: chrono::prelude::Local::now().to_string(),
            },
            environment: libmodel::sanitation::environment::Environment {
                temperature_range: self.temperature_range.clone(),
                weather: self.weather.clone(),
                season: self.season.clone(),
                defoliation: self.defoliation,
                work_time: self.work_time.clone(),
            },
            vehicles: libmodel::sanitation::scenario::VehicleGroup {
                vehicles: self.vehicles.clone(),
            },
            sanitation_man: libmodel::sanitation::scenario::SanitationGroup {
                sanitation: self.sanitation.clone(),
            },
            global_vehicle_task:
                libmodel::sanitation::scenario::VehicleTaskGroup {
                    task: self.global_vehicle_task.clone(),
                },
            global_man_task:
                libmodel::sanitation::scenario::SanitationTaskGroup {
                    task: self.global_man_task.clone(),
                },
            vehicle_to_task_mapper: {
                let t: Vec<libmodel::sanitation::util::Tuple> = self
                    .vehicle_to_task_mapper
                    .iter()
                    .map(|v| libmodel::sanitation::util::Tuple {
                        value: v.clone(),
                    })
                    .collect();
                libmodel::sanitation::util::VecTuple::from(t)
            },
            sanitation_to_task_mapper: {
                let t: Vec<libmodel::sanitation::util::Tuple> = self
                    .sanitation_to_task_mapper
                    .iter()
                    .map(|v| libmodel::sanitation::util::Tuple {
                        value: v.clone(),
                    })
                    .collect();
                libmodel::sanitation::util::VecTuple::from(t)
            },
            driver_to_vehicle_mapper: {
                let t: Vec<libmodel::sanitation::util::Tuple> = self
                    .driver_to_vehicle_mapper
                    .iter()
                    .map(|v| libmodel::sanitation::util::Tuple {
                        value: v.clone(),
                    })
                    .collect();
                libmodel::sanitation::util::VecTuple::from(t)
            },
            traffic_flow_config: Default::default(),
            garbage_config: Default::default(),
        }
    }
}

impl From<&libmodel::sanitation::Scenario> for ScenarioData {
    fn from(value: &libmodel::sanitation::Scenario) -> Self {
        ScenarioData {
            name: value.header.name.clone(),
            temperature_range: value.environment.temperature_range.clone(),
            weather: value.environment.weather.clone(),
            season: value.environment.season.clone(),
            defoliation: value.environment.defoliation,
            work_time: value.environment.work_time.clone(),
            map: value.header.map_name.clone(),
            vehicles: value.vehicles.vehicles.clone(),
            sanitation: value.sanitation_man.sanitation.clone(),
            global_vehicle_task: value.global_vehicle_task.task.clone(),
            global_man_task: value.global_man_task.task.clone(),
            vehicle_to_task_mapper: value
                .vehicle_to_task_mapper
                .value
                .iter()
                .map(|v| v.value.clone())
                .collect(),
            sanitation_to_task_mapper: value
                .sanitation_to_task_mapper
                .value
                .iter()
                .map(|v| v.value.clone())
                .collect(),
            driver_to_vehicle_mapper: value
                .driver_to_vehicle_mapper
                .value
                .iter()
                .map(|v| v.value.clone())
                .collect(),
        }
    }
}
