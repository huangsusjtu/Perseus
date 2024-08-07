use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::api::{UniformError, UniformOk};
use crate::AppState;

/// 仿真实例的管理
#[get("list")]
async fn list_instance(
    app: web::Data<AppState>,
) -> Result<ListSimulatorReply, UniformError> {
    match app.sim_svc.list() {
        Ok(r) => Ok(ListSimulatorReply {
            list: r
                .iter()
                .map(|v| SimulatorInfo {
                    name: v.name.clone(),
                })
                .collect(),
        }),
        Err(e) => Err(UniformError::from(e)),
    }
}

#[post("create")]
async fn create_instance(
    app: web::Data<AppState>, name: String,
) -> Result<SimulatorInfo, UniformError> {
    match app.sim_svc.create(&name) {
        Ok(r) => Ok(SimulatorInfo { name: r.name }),
        Err(e) => Err(UniformError::from(e)),
    }
}

#[delete("delete")]
async fn delete_instance(
    app: web::Data<AppState>, name: String,
) -> Result<UniformOk, UniformError> {
    match app.sim_svc.delete(&name) {
        Ok(_) => Ok(UniformOk),
        Err(e) => Err(UniformError::from(e)),
    }
}

// 保存场景
#[post("save")]
async fn save_instance(
    app: web::Data<AppState>, name: String,
) -> Result<UniformOk, UniformError> {
    match app.sim_svc.save(&name) {
        Ok(_) => Ok(UniformOk),
        Err(e) => Err(UniformError::from(e)),
    }
}

/// 对单个仿真的操作: 加载地图
#[post("load_map")]
async fn load_map(
    app: web::Data<AppState>, sim_name: String, map_name: String,
) -> Result<UniformOk, UniformError> {
    // 读地图
    let sd_map_data = app.map_svc.get_map(&map_name)?;
    app.sim_svc.load_map(&sim_name, sd_map_data)?;
    Ok(UniformOk)
}

/// 对单个仿真的操作: 加载仿真场景
#[post("load_scenario")]
async fn load_scenario(
    app: web::Data<AppState>, sim_name: String, scenario_name: String,
) -> Result<UniformOk, UniformError> {
    // 读到场景内容
    let scenario = app.scenario_svc.get(&scenario_name)?;
    // 读地图
    let sd_map_data = app.map_svc.get_map(&scenario.header.map_name)?;
    app.sim_svc.load_map(&sim_name, sd_map_data)?;
    app.sim_svc.load_scenario(&sim_name, scenario)?;
    Ok(UniformOk)
}

#[post("start")]
async fn start(
    app: web::Data<AppState>, name: String,
) -> Result<UniformOk, UniformError> {
    app.sim_svc.start(&name)?;
    Ok(UniformOk)
}

#[post("stop")]
async fn stop(
    app: web::Data<AppState>, name: String,
) -> Result<UniformOk, UniformError> {
    app.sim_svc.stop(&name)?;
    Ok(UniformOk)
}

#[post("stopall")]
async fn stop_all(app: web::Data<AppState>) -> Result<UniformOk, UniformError> {
    app.sim_svc.stop_all()?;
    Ok(UniformOk)
}

///
#[derive(Serialize, Debug)]
pub struct ListSimulatorReply {
    pub list: Vec<SimulatorInfo>,
}

#[derive(Serialize, Debug)]
pub struct SimulatorInfo {
    pub name: String,
}

impl Responder for ListSimulatorReply {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create response and set content type
        HttpResponse::Ok().json(self)
    }
}
impl Responder for SimulatorInfo {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create response and set content type
        HttpResponse::Ok().json(self)
    }
}
