use std::path::PathBuf;

use dashmap::DashMap;

use crate::error::{OwnResult, UniformError};
use crate::map::MapServiceInterface;
use crate::MapInfo;

pub struct FileBasedMapServiceImpl {
    pub map_dir: PathBuf,
    pub maps_data_table: DashMap<String, libmap::MapRef>,
}

unsafe impl Sync for FileBasedMapServiceImpl {}

unsafe impl Send for FileBasedMapServiceImpl {}

impl FileBasedMapServiceImpl {
    // 读取本地目录，拿到所有地图
    pub fn new(
        map_dir: PathBuf,
    ) -> anyhow::Result<std::sync::Arc<FileBasedMapServiceImpl>> {
        let maps_data_table: DashMap<String, libmap::MapRef> =
            DashMap::default();
        let dir = std::fs::read_dir(&map_dir)?;
        for entry in dir {
            let entry = entry.unwrap();
            let map_xml_file_path = entry.path();
            if !entry.file_type().unwrap().is_file() {
                continue;
            }
            if !map_xml_file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .ends_with(".xodr")
            {
                continue;
            }

            let name = map_xml_file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let mut sd_map = libmap::SDMap::load_map_file(map_xml_file_path)?;
            sd_map.header.name = name;
            maps_data_table.insert(
                sd_map.header.name.clone(),
                std::sync::Arc::new(sd_map),
            );
        }

        Ok(std::sync::Arc::new(FileBasedMapServiceImpl {
            map_dir,
            maps_data_table,
        }))
    }
}

impl MapServiceInterface for FileBasedMapServiceImpl {
    fn list(&self) -> OwnResult<Vec<MapInfo>> {
        return Ok(self
            .maps_data_table
            .iter()
            .map(|item| MapInfo {
                name: item.value().header.name.clone(),
            })
            .collect());
    }

    fn get_map_raw(&self, name: &str) -> OwnResult<bytes::Bytes> {
        return if let Some(m) = self.maps_data_table.get(name) {
            let sd_map = m.value();
            let vec = match sd_map.serialize_to_vec() {
                Ok(r) => r,
                Err(_e) => {
                    return Err(UniformError::SerializeErr);
                }
            };
            Ok(bytes::Bytes::from(vec))
        } else {
            Err(UniformError::NotFound(name.to_string()))
        };
    }

    fn get_map(&self, name: &str) -> OwnResult<libmap::MapRef> {
        return if let Some(m) = self.maps_data_table.get(name) {
            let sd_map = m.clone();
            Ok(sd_map)
        } else {
            Err(UniformError::NotFound(name.to_string()))
        };
    }

    fn upload(
        &self, name: &str, data: bytes::Bytes, force_write: bool,
    ) -> OwnResult<()> {
        return if self.maps_data_table.contains_key(name) && !force_write {
            Err(UniformError::Duplicate(name.to_string()))
        } else {
            let sd_map =
                match libmap::SDMap::deserialize_from_bytes(data.as_ref()) {
                    Ok(r) => r,
                    Err(_e) => {
                        return Err(UniformError::DeserializeErr);
                    }
                };

            self.maps_data_table.insert(
                sd_map.header.name.clone(),
                std::sync::Arc::new(sd_map),
            );
            Ok(())
        };
    }

    fn delete(&self, name: &str) -> OwnResult<()> {
        return if !self.maps_data_table.contains_key(name) {
            Err(UniformError::NotFound(name.to_string()))
        } else {
            self.maps_data_table.remove(name);
            Ok(())
        };
    }
}
