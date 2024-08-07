mod environment;
mod garbage;
mod sanitation;
mod station;
mod task;
mod vehicle;

use std::sync::{Arc, RwLock};

use pyo3::prelude::*;

use crate::world::environment::{Environment, SeasonType, WeatherType};
use crate::world::garbage::{Garbage, GarbageConfig, GarbageType};
use crate::world::sanitation::{
    Sanitation, SanitationManParam, SanitationManState,
};
use crate::world::station::StationState;
use crate::world::vehicle::{
    OperationMode, Vehicle, VehicleBaseParam, VehicleState, VehicleType,
    WorkMode,
};

pub(crate) fn mod_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SeasonType>()?;
    m.add_class::<WeatherType>()?;
    m.add_class::<Environment>()?;

    m.add_class::<GarbageConfig>()?;
    m.add_class::<GarbageType>()?;
    m.add_class::<Garbage>()?;
    m.add_class::<SanitationManState>()?;
    m.add_class::<SanitationManParam>()?;
    m.add_class::<Sanitation>()?;
    m.add_class::<StationState>()?;
    m.add_class::<VehicleType>()?;
    m.add_class::<VehicleBaseParam>()?;
    m.add_class::<VehicleState>()?;
    m.add_class::<WorkMode>()?;
    m.add_class::<OperationMode>()?;
    m.add_class::<Vehicle>()?;

    m.add_class::<World>()?;
    Ok(())
}

#[pyclass]
pub struct World {
    pub name: String,
    pub map_name: String,

    pub simulator_ref: Arc<RwLock<libsimulator::SanitationSimulator>>,
}

#[pymethods]
impl World {
    fn set_environment(&mut self, env: Environment) -> PyResult<()> {
        self.simulator_ref.write().unwrap().set_environment((&env).into());
        Ok(())
    }

    fn set_map(&mut self, map_name: String) -> PyResult<()> {
        let map_ref = crate::get_map_svc().get_map(&map_name).unwrap();
        self.map_name = map_name;
        crate::get_sim_svc()
            .load_map(&self.name, map_ref)
            .expect("load_map error");
        Ok(())
    }

    fn spawn_actor(&mut self) -> PyResult<()> {
        unimplemented!()
    }

    fn get_actor(&mut self, _actor_id: i32) -> PyResult<()> {
        unimplemented!()
    }
    fn get_actors(&mut self, _actor_ids: Vec<i32>) -> PyResult<()> {
        unimplemented!()
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
    fn step(&mut self, frame_time: std::time::Duration) -> PyResult<()> {
        self.simulator_ref.write().unwrap().step(frame_time).expect("step");
        Ok(())
    }
}
