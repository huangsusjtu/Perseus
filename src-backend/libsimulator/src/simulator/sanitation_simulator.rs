use std::cmp::PartialEq;
use std::collections::LinkedList;
use std::time::Duration;

use crate::error::{OwnResult, UniformError};

///  仿真主逻辑

pub struct SanitationSimulator {
    world: libmodel::sanitation::SimulatorWorld,
    snapshot: LinkedList<libmodel::sanitation::SimulatorWorld>,
    event_bus: libutil::eventbus::EventBus,

    rate: f64, // 倍速
    status: SimStatus,
}

#[derive(PartialEq)]
enum SimStatus {
    Init,
    MapReady,
    SceneReady,
    Running,
    Stop,
}

impl SanitationSimulator {
    pub fn new(event_bus: libutil::eventbus::EventBus) -> Self {
        SanitationSimulator {
            world: Default::default(),
            snapshot: Default::default(),
            event_bus: event_bus.clone(),
            rate: 1.0,
            status: SimStatus::Init,
        }
    }

    pub fn dump_to_scenario(&self) -> OwnResult<String> {
        let r = libmodel::sanitation::unparse(&self.world.scenario);
        return match r {
            Ok(r) => Ok(r),
            Err(_e) => Err(UniformError::DeserializeErr),
        };
    }

    pub fn attach_map(&mut self, map_ref: libmap::MapRef) -> OwnResult<()> {
        self.world.load_map(map_ref);
        self.status = SimStatus::MapReady;
        Ok(())
    }
    pub fn attach_scenario(
        &mut self, scenario: libmodel::sanitation::Scenario,
    ) -> OwnResult<()> {
        self.world.load_scenario(scenario);
        self.status = SimStatus::SceneReady;
        tracing::info!("attach_scenario");
        Ok(())
    }

    pub fn set_environment(
        &mut self, env: libmodel::sanitation::environment::Environment,
    ) {
        self.world.scenario.environment = env;
    }

    pub fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
    }

    // 开始仿真
    pub fn start(&mut self) -> OwnResult<()> {
        self.snapshot.push_back(self.world.clone());
        self.status = SimStatus::Running;
        Ok(())
    }

    pub fn reset(&mut self) -> OwnResult<()> {
        self.world = self.snapshot.front().unwrap().clone();
        Ok(())
    }

    pub fn step(&mut self, _duration: Duration) -> OwnResult<()> {
        //  todo

        Ok(())
    }

    pub fn stop(&mut self) -> OwnResult<()> {
        self.status = SimStatus::Stop;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        return self.status == SimStatus::Running;
    }
    pub fn report_sim_result(&mut self) -> OwnResult<()> {
        Ok(())
    }
}
