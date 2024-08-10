use std::sync::{Arc, RwLock};
use std::time::Duration;

use dashmap::DashMap;
use libutil::eventbus::EventBus;

use crate::error::{OwnResult, UniformError};
use crate::simulator::sanitation_simulator::SanitationSimulator;
use crate::simulator::{SimulatorInfo, SimulatorServiceInterface};

const _SIMULATION_STEP: Duration = Duration::from_millis(100);

pub struct SanitationSimulatorServiceImpl {
    //
    pub event_bus: EventBus,

    // 当前所有的 仿真实例
    pub simulate_instances: DashMap<String, Arc<RwLock<SanitationSimulator>>>,
}

unsafe impl Sync for SanitationSimulatorServiceImpl {}

unsafe impl Send for SanitationSimulatorServiceImpl {}

impl SanitationSimulatorServiceImpl {
    pub fn new(event_bus: EventBus) -> std::sync::Arc<Self> {
        let instance = std::sync::Arc::new(SanitationSimulatorServiceImpl {
            event_bus,
            simulate_instances: Default::default(),
        });
        return instance;
    }
}

impl SimulatorServiceInterface for SanitationSimulatorServiceImpl {
    fn list(&self) -> OwnResult<Vec<SimulatorInfo>> {
        Ok(self
            .simulate_instances
            .iter()
            .map(|v| SimulatorInfo {
                name: v.key().clone(),
            })
            .collect())
    }

    fn create(&self, name: &str) -> OwnResult<SimulatorInfo> {
        if self.simulate_instances.contains_key(name) {
            return Err(UniformError::Duplicate(name.to_string()));
        }
        self.simulate_instances.insert(
            name.to_string(),
            Arc::new(RwLock::new(SanitationSimulator::new(
                self.event_bus.clone(),
            ))),
        );
        tracing::info!("create simulator:{}", name);
        return Ok(SimulatorInfo {
            name: name.to_string(),
        });
    }
    fn delete(&self, name: &str) -> OwnResult<()> {
        if self.simulate_instances.remove(name).is_none() {
            return Err(UniformError::NotFound(name.to_string()));
        }
        tracing::info!("delete simulator:{}", name);
        return Ok(());
    }
    fn get(&self, name: &str) -> OwnResult<Arc<RwLock<SanitationSimulator>>> {
        if !self.simulate_instances.contains_key(name) {
            return Err(UniformError::NotFound(name.to_string()));
        }
        let t = self.simulate_instances.get(name).unwrap();
        tracing::info!("get simulator:{}", name);
        Ok(t.clone())
    }

    fn save(&self, name: &str) -> OwnResult<()> {
        let ins = self.simulate_instances.get(name);
        return if let Some(sim) = ins {
            // todo, 持久化，下次可以继续使用
            let _s = match sim.value().read().unwrap().dump_to_scenario() {
                Ok(s) => s,
                Err(_e) => {
                    return Err(UniformError::DeserializeErr);
                }
            };
            tracing::info!("save simulator:{}", name);
            Ok(())
        } else {
            Err(UniformError::NotFound(name.to_string()))
        };
    }

    fn load_map(
        &self, sim_name: &str, map_ref: libmap::MapRef,
    ) -> OwnResult<()> {
        let ins = self.simulate_instances.get(sim_name);
        return if let Some(sim) = ins {
            tracing::info!(
                "simulator:{}, load_map:{}",
                sim_name,
                &map_ref.header.name
            );
            _ = sim.value().write().unwrap().attach_map(map_ref);
            Ok(())
        } else {
            Err(UniformError::NotFound(sim_name.to_string()))
        };
    }

    fn load_scenario(
        &self, sim_name: &str, scenario: libmodel::sanitation::Scenario,
    ) -> OwnResult<()> {
        let ins = self.simulate_instances.get(sim_name);
        return if let Some(sim) = ins {
            tracing::info!(
                "simulator:{}, load_scenario:{}",
                sim_name,
                &scenario.header.name
            );
            _ = sim.value().write().unwrap().attach_scenario(scenario);
            Ok(())
        } else {
            Err(UniformError::NotFound(sim_name.to_string()))
        };
    }

    fn start(&self, sim_name: &str) -> OwnResult<()> {
        let ins = self.simulate_instances.get_mut(sim_name);
        return if let Some(sim) = ins {
            _ = sim.value().write().unwrap().start();
            tracing::info!("simulator:{} start", sim_name);
            Ok(())
        } else {
            Err(UniformError::NotFound(sim_name.to_string()))
        };
    }

    fn stop(&self, sim_name: &str) -> OwnResult<()> {
        let ins = self.simulate_instances.get_mut(sim_name);
        return if let Some(sim) = ins {
            _ = sim.value().write().unwrap().stop();
            tracing::info!("simulator:{} stop", sim_name);
            Ok(())
        } else {
            Err(UniformError::NotFound(sim_name.to_string()))
        };
    }

    fn stop_all(&self) -> OwnResult<()> {
        _ = self.simulate_instances.iter().map(|v| {
            _ = v.value().write().unwrap().stop();
        });
        tracing::info!("simulator stop_all");
        Ok(())
    }
}
