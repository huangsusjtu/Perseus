use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use libsimulator::MapInfo;
use serde::Serialize;

use crate::api::{UniformError, UniformOk};
use crate::AppState;

/// 作业区域/地图 API

// 列举地图列表
#[get("list")]
async fn list_map(
    app: web::Data<AppState>,
) -> Result<ListMapReply, UniformError> {
    match app.map_svc.list() {
        Ok(r) => Ok(ListMapReply { list: r }),
        Err(e) => Err(UniformError::from(e)),
    }
}

// 上传新地图， 或者替换
#[post("upload")]
async fn upload_map(
    app: web::Data<AppState>,
    MultipartForm(form): MultipartForm<UploadMultipartForm>,
) -> Result<UniformOk, UniformError> {
    let name: String = form.map_name.into_inner();
    let force_write: bool = form.force_write.into_inner();
    match app.map_svc.upload(&name, form.content.data, force_write) {
        Ok(_) => Ok(UniformOk),
        Err(e) => Err(UniformError::from(e)),
    }
}

// 删除地图
#[delete("delete")]
async fn delete_map(
    app: web::Data<AppState>, req: web::Json<MapInfo>,
) -> Result<UniformOk, UniformError> {
    match app.map_svc.delete(&req.name) {
        Ok(_) => Ok(UniformOk),
        Err(e) => Err(UniformError::from(e)),
    }
}

// 获取一张地图数据
#[get("get")]
async fn get_map(
    app: web::Data<AppState>, req: web::Query<MapInfo>,
) -> Result<bytes::Bytes, UniformError> {
    match app.map_svc.get_map_raw(&req.name) {
        Ok(r) => Ok(r),
        Err(e) => Err(UniformError::from(e)),
    }
}

// 接口数据
#[derive(Serialize, Debug)]
pub struct ListMapReply {
    pub list: Vec<MapInfo>,
}

impl Responder for ListMapReply {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create response and set content type
        HttpResponse::Ok().json(self)
    }
}

// 上传、更新 地图的数据
#[derive(MultipartForm, Debug)]
pub struct UploadMultipartForm {
    pub map_name: actix_multipart::form::text::Text<String>,
    pub force_write: actix_multipart::form::text::Text<bool>,
    #[multipart(limit = "200MB")]
    pub content: actix_multipart::form::bytes::Bytes,
}
