
use serde::{Deserialize, Serialize};

mod file_sanitation_scenario_service;
pub use file_sanitation_scenario_service::FileSanitationScenarioServiceImpl;
use crate::error::OwnResult;

/// 底层服务
pub trait ScenarioServiceInterface {
    fn list(&self) -> OwnResult<Vec<ScenarioInfo>>;
    fn upload(
        &self, name: &str, data: libmodel::sanitation::scenario::Scenario,
        force_write: bool,
    ) -> OwnResult<()>;
    fn delete(&self, name: &str) -> OwnResult<()>;
    fn get(
        &self, name: &str,
    ) -> OwnResult<libmodel::sanitation::scenario::Scenario>;
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ScenarioInfo {
    pub name: String,
    // todo add more;
}
