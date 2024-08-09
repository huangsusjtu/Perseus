use std::sync::{Arc, RwLock};

use crate::error::OwnResult;

mod sanitation_service;
pub use sanitation_service::SanitationSimulatorServiceImpl;
mod sanitation_simulator;
pub use sanitation_simulator::SanitationSimulator;

///
pub trait SimulatorServiceInterface {
    fn list(&self) -> OwnResult<Vec<SimulatorInfo>>;
    fn create(&self, name: &str) -> OwnResult<SimulatorInfo>;
    fn delete(&self, name: &str) -> OwnResult<()>;
    fn get(&self, name: &str) -> OwnResult<Arc<RwLock<SanitationSimulator>>>;
    fn save(&self, name: &str) -> OwnResult<()>;
    fn load_map(&self, sim_name: &str, map: libmap::MapRef) -> OwnResult<()>;
    fn load_scenario(
        &self, sim_name: &str, scenario: libmodel::sanitation::Scenario,
    ) -> OwnResult<()>;

    fn start(&self, name: &str) -> OwnResult<()>;
    fn stop(&self, name: &str) -> OwnResult<()>;
    fn stop_all(&self) -> OwnResult<()>;
}

#[derive(Debug)]
pub struct SimulatorInfo {
    pub name: String,
}
