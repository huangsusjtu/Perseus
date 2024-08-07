
mod file_map_service;
pub use file_map_service::FileBasedMapServiceImpl;
use serde::{Deserialize, Serialize};

use crate::error::OwnResult;

/// 底层服务接口
pub trait MapServiceInterface {
    fn list(&self) -> OwnResult<Vec<MapInfo>>;
    fn get_map_raw(&self, name: &str) -> OwnResult<bytes::Bytes>;
    fn get_map(&self, name: &str) -> OwnResult<libmap::MapRef>;
    fn upload(
        &self, name: &str, data: bytes::Bytes, force_write: bool,
    ) -> OwnResult<()>;
    fn delete(&self, name: &str) -> OwnResult<()>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapInfo {
    pub name: String,
    // todo add more;
}
