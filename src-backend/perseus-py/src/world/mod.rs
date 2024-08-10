pub(crate) mod environment;
pub(crate) mod garbage;
pub(crate) mod sanitation;
pub(crate) mod scenario;
pub(crate) mod station;
pub(crate) mod task;
pub(crate) mod traffic_flow;
pub(crate) mod vehicle;

use std::sync::{Arc, RwLock};

use pyo3::prelude::*;

#[pymodule]
pub fn world(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<World>()?;

    m.add_class::<crate::world::environment::Environment>()?;
    m.add_class::<crate::world::environment::SeasonType>()?;
    m.add_class::<crate::world::environment::WeatherType>()?;

    m.add_class::<crate::world::garbage::GarbageConfig>()?;
    m.add_class::<crate::world::garbage::GarbageType>()?;
    m.add_class::<crate::world::garbage::Garbage>()?;
    m.add_class::<crate::world::sanitation::SanitationManState>()?;
    m.add_class::<crate::world::sanitation::SanitationManParam>()?;
    m.add_class::<crate::world::sanitation::Sanitation>()?;
    m.add_class::<crate::world::station::StationState>()?;

    m.add_class::<crate::world::task::FrenetPoint>()?;
    m.add_class::<crate::world::task::LaneWayPoint>()?;
    m.add_class::<crate::world::task::GlobalPlannerPath>()?;
    m.add_class::<crate::world::task::GlobalPlannerArea>()?;

    m.add_class::<crate::world::vehicle::VehicleType>()?;
    m.add_class::<crate::world::vehicle::VehicleBaseParam>()?;
    m.add_class::<crate::world::vehicle::VehicleState>()?;
    m.add_class::<crate::world::vehicle::WorkMode>()?;
    m.add_class::<crate::world::vehicle::OperationMode>()?;
    m.add_class::<crate::world::vehicle::Vehicle>()?;
    Ok(())
}

#[pyclass]
pub struct World {
    pub name: String,
    pub map_name: String,
    pub scenario_name: String,

    pub simulator_ref: Arc<RwLock<libsimulator::SanitationSimulator>>,
}

#[pymethods]
impl World {
    fn set_environment(
        &mut self, env: environment::Environment,
    ) -> PyResult<()> {
        self.simulator_ref.write().unwrap().set_environment((&env).into());
        Ok(())
    }

    fn load_map(&mut self, map_name: String) -> PyResult<()> {
        let map_ref = crate::get_map_svc().get_map(&map_name).unwrap();
        self.map_name = map_name;
        crate::get_sim_svc()
            .load_map(&self.name, map_ref)
            .expect("load_map error");
        Ok(())
    }

    fn load_scenario(&mut self, scenario_name: String) -> PyResult<()> {
        let scenario_ref =
            crate::get_scenario_svc().get(&scenario_name).unwrap();
        self.scenario_name = scenario_name;
        crate::get_sim_svc()
            .load_scenario(&self.name, scenario_ref)
            .expect("load_scenario error");
        Ok(())
    }

    fn start(&mut self) -> PyResult<()> {
        self.simulator_ref.write().unwrap().start().expect("start");
        Ok(())
    }
    fn reset(&mut self) -> PyResult<()> {
        self.simulator_ref.write().unwrap().reset().expect("reset");
        Ok(())
    }
    fn stop(&mut self) -> PyResult<()> {
        self.simulator_ref.write().unwrap().stop().expect("stop");
        Ok(())
    }

    // synchronous mode
    fn step(&mut self, millis: u64) -> PyResult<()> {
        self.simulator_ref
            .write()
            .unwrap()
            .step(std::time::Duration::from_millis(millis))
            .expect("step");

        // 同步simulator_ref的数据过来
        // self.simulator_ref
        //     .read()
        //     .unwrap();
        Ok(())
    }
}
