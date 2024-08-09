use std::path::PathBuf;

use anyhow;
use dashmap::DashMap;

use crate::error::{OwnResult, UniformError};
use crate::scenario::ScenarioServiceInterface;
use crate::ScenarioInfo;

pub struct FileSanitationScenarioServiceImpl {
    pub scenario_dir: PathBuf,
    pub scenario_data_table: DashMap<String, libmodel::sanitation::Scenario>,
}

impl FileSanitationScenarioServiceImpl {
    // 读取本地目录，拿到所有场景文件
    pub fn new(scenario_dir: PathBuf) -> anyhow::Result<std::sync::Arc<Self>> {
        let scenario_data_table: DashMap<
            String,
            libmodel::sanitation::Scenario,
        > = DashMap::default();
        let dir = std::fs::read_dir(&scenario_dir)?;
        for entry in dir {
            let entry = entry.unwrap();
            let scene_xml_file_path = entry.path();
            if !entry.file_type().unwrap().is_file() {
                continue;
            }
            if !scene_xml_file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .ends_with(".xml")
            {
                continue;
            }
            let name = scene_xml_file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let mut scenario =
                libmodel::sanitation::parse(scene_xml_file_path)?;
            scenario.header.name = name;
            scenario_data_table.insert(scenario.header.name.clone(), scenario);
        }

        Ok(std::sync::Arc::new(FileSanitationScenarioServiceImpl {
            scenario_dir,
            scenario_data_table,
        }))
    }
}

impl ScenarioServiceInterface for FileSanitationScenarioServiceImpl {
    fn list(&self) -> OwnResult<Vec<ScenarioInfo>> {
        return Ok(self
            .scenario_data_table
            .iter()
            .map(|item| ScenarioInfo {
                name: item.value().header.name.clone(),
            })
            .collect());
    }

    fn upload(
        &self, name: &str, scenario: libmodel::sanitation::scenario::Scenario,
        force_write: bool,
    ) -> OwnResult<()> {
        return if self.scenario_data_table.contains_key(name) && !force_write {
            Err(UniformError::Duplicate(name.to_string()))
        } else {
            let scenario_xml_content =
                match libmodel::sanitation::unparse(&scenario) {
                    Ok(r) => r,
                    Err(_) => return Err(UniformError::DeserializeErr),
                };
            {
                if let Err(e) = std::fs::write(
                    self.scenario_dir.join(name),
                    scenario_xml_content,
                ) {
                    tracing::error!(
                        "write scenario file {}, e {:#?}",
                        &name,
                        e
                    );
                } else {
                    tracing::info!("write scenario file {}", name);
                    self.scenario_data_table
                        .insert(scenario.header.name.clone(), scenario);
                }
            }
            Ok(())
        };
    }

    fn delete(&self, name: &str) -> OwnResult<()> {
        if self.scenario_data_table.remove(name).is_none() {
            return Err(UniformError::NotFound(name.to_string()));
        }

        {
            if let Err(e) = std::fs::remove_file(self.scenario_dir.join(&name))
            {
                tracing::warn!("delete file {}, e {:#?}", &name, e);
            }
        }

        return Ok(());
    }

    fn get(
        &self, name: &str,
    ) -> OwnResult<libmodel::sanitation::scenario::Scenario> {
        return if let Some(m) = self.scenario_data_table.get(name) {
            let scenario = m.value();
            Ok(scenario.clone())
        } else {
            Err(UniformError::NotFound(name.to_string()))
        };
    }
}
