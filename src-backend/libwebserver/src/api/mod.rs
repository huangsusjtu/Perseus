use std::fmt::{Debug, Display, Formatter};

use actix_web::http::StatusCode;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub(crate) mod map;
pub(crate) mod scenario;
pub(crate) mod simulator;

pub(crate) fn init_api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/health").service(health));
    cfg.service(web::scope("/api/map").service(map_api()));
    cfg.service(web::scope("/api/scenario").service(scenario_api()));
    cfg.service(web::scope("/api/simulator").service(simulator_api()));
    cfg.service(
        web::scope("/api/websocket/simulator/").service(crate::ws::ws_client),
    );
}

#[get("")]
pub async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("simulator service running"))
}

pub(crate) fn map_api() -> actix_web::Scope {
    actix_web::web::scope("map")
        .service(map::list_map)
        .service(map::get_map)
        .service(map::upload_map)
        .service(map::delete_map)
}

pub(crate) fn scenario_api() -> actix_web::Scope {
    actix_web::web::scope("scenario")
        .service(scenario::list_scenario)
        .service(scenario::upload_scenario)
        .service(scenario::delete_scenario)
        .service(scenario::get_scenario)
}

pub(crate) fn simulator_api() -> actix_web::Scope {
    web::scope("simulator")
        .service(simulator::list_instance)
        .service(simulator::create_instance)
        .service(simulator::save_instance)
        .service(simulator::delete_instance)
        .service(simulator::load_map)
        .service(simulator::load_scenario)
        .service(simulator::start)
        .service(simulator::stop)
}

/// 统一一下错误处理
#[derive(Serialize, Deserialize)]
pub struct UniformError {
    pub code: u16,
    pub reason: String,
    pub message: String,
}

impl From<u16> for UniformError {
    fn from(value: u16) -> Self {
        UniformError {
            code: value,
            reason: "".to_string(),
            message: "".to_string(),
        }
    }
}

impl From<StatusCode> for UniformError {
    fn from(value: StatusCode) -> Self {
        UniformError {
            code: value.as_u16(),
            reason: value.to_string(),
            message: "".to_string(),
        }
    }
}

impl Into<StatusCode> for &UniformError {
    fn into(self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap()
    }
}

impl From<(u16, &str)> for UniformError {
    fn from(v: (u16, &str)) -> Self {
        UniformError {
            code: v.0,
            reason: v.1.to_string(),
            message: "".to_string(),
        }
    }
}

impl From<(u16, &str, &str)> for UniformError {
    fn from(v: (u16, &str, &str)) -> Self {
        UniformError {
            code: v.0,
            reason: v.1.to_string(),
            message: v.2.to_string(),
        }
    }
}
impl From<anyhow::Error> for UniformError {
    fn from(value: anyhow::Error) -> Self {
        UniformError {
            code: 500,
            reason: String::default(),
            message: value.to_string(),
        }
    }
}
impl From<libsimulator::error::UniformError> for UniformError {
    fn from(value: libsimulator::error::UniformError) -> Self {
        match value {
            libsimulator::error::UniformError::NotFound(e) => UniformError {
                code: 404,
                reason: String::default(),
                message: e,
            },
            libsimulator::error::UniformError::Duplicate(e) => UniformError {
                code: 400,
                reason: String::default(),
                message: e,
            },
            libsimulator::error::UniformError::SerializeErr => UniformError {
                code: 500,
                reason: String::default(),
                message: value.to_string(),
            },
            libsimulator::error::UniformError::DeserializeErr => UniformError {
                code: 500,
                reason: String::default(),
                message: value.to_string(),
            },
            libsimulator::error::UniformError::Unknown => UniformError {
                code: 500,
                reason: String::default(),
                message: value.to_string(),
            },
        }
    }
}

impl Display for UniformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "code:{}, reason:{}, message:{}",
            self.code, self.reason, self.message
        ))
    }
}

impl Debug for UniformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "code:{}, reason:{}, message:{}",
            self.code, self.reason, self.message
        ))
    }
}

impl actix_web::error::ResponseError for UniformError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap_or_default()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
/// 统一一下Ok
pub struct UniformOk;

impl Responder for UniformOk {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().finish()
    }
}
