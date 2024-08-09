mod map;
mod world;

use std::path::PathBuf;
use std::sync::Arc;

use libsimulator::{
    MapServiceInterface, ScenarioServiceInterface, SimulatorServiceInterface,
};
pub use map::Map;
use once_cell::sync::OnceCell;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;
pub use world::scenario::Scenario;
pub use world::World;

/// python sdk for perseus implemented in Rust.
#[pymodule(name = "perseus")]
fn perseus_init(
    py: Python<'_>, root_module: &Bound<'_, PyModule>,
) -> PyResult<()> {
    // 初始化
    root_module.add_function(wrap_pyfunction!(init, root_module)?)?;
    // 一级API
    root_module.add_function(wrap_pyfunction!(list_map, root_module)?)?;
    root_module.add_function(wrap_pyfunction!(get_map, root_module)?)?;
    root_module.add_function(wrap_pyfunction!(list_scenario, root_module)?)?;
    root_module.add_function(wrap_pyfunction!(get_scenario, root_module)?)?;
    root_module.add_function(wrap_pyfunction!(list_world, root_module)?)?;
    root_module.add_function(wrap_pyfunction!(get_world, root_module)?)?;

    // 注册子模块
    root_module.add_wrapped(wrap_pymodule!(map::map))?;
    root_module.add_wrapped(wrap_pymodule!(world::world))?;

    let sys = PyModule::import_bound(py, "sys")?;
    let sys_modules: Bound<'_, PyDict> =
        sys.getattr("modules")?.downcast_into()?;
    sys_modules.set_item("perseus.map", root_module.getattr("map")?)?;
    sys_modules.set_item("perseus.world", root_module.getattr("world")?)?;

    Ok(())
}

static mut LOG_CLEANER: Option<Box<dyn FnOnce()>> = None;

#[pyfunction]
fn init(map_dir: String, scenario_dir: String) -> PyResult<()> {
    unsafe {
        LOG_CLEANER = Some(libutil::log::init_log().unwrap());
    }

    let eventbus = libutil::eventbus::EventBus::default();
    let apps_state = libsimulator::AppState {
        eventbus: eventbus.clone(),
        map_svc: libsimulator::FileBasedMapServiceImpl::new(PathBuf::from(
            map_dir,
        ))
        .unwrap(),
        scenario_svc: libsimulator::FileSanitationScenarioServiceImpl::new(
            PathBuf::from(scenario_dir),
        )
        .unwrap(),
        sim_svc: libsimulator::SanitationSimulatorServiceImpl::new(
            eventbus.clone(),
        ),
    };
    set_data(apps_state);
    Ok(())
}

#[pyfunction]
fn list_map() -> PyResult<Vec<String>> {
    let res = get_data().map_svc.list();
    match res {
        Ok(r) => Ok(r.iter().map(|v| v.name.clone()).collect()),
        Err(e) => Err(PyTypeError::new_err(e.to_string())),
    }
}

#[pyfunction]
fn get_map(map_name: String) -> PyResult<Map> {
    let res = get_data().map_svc.get_map(&map_name);
    match res {
        Ok(r) => Ok(Map { map_inner: r }),
        Err(e) => Err(PyTypeError::new_err(e.to_string())),
    }
}

#[pyfunction]
fn list_scenario() -> PyResult<Vec<String>> {
    let res = unsafe { APP_DATA.get().unwrap().scenario_svc.list() };
    match res {
        Ok(r) => Ok(r.iter().map(|v| v.name.clone()).collect()),
        Err(e) => Err(PyTypeError::new_err(e.to_string())),
    }
}

#[pyfunction]
fn get_scenario(scenario_name: String) -> PyResult<Scenario> {
    let res =
        unsafe { APP_DATA.get().unwrap().scenario_svc.get(&scenario_name) };
    match res {
        Ok(r) => Ok(Scenario::from(&r)),
        Err(e) => Err(PyTypeError::new_err(e.to_string())),
    }
}

#[pyfunction]
fn list_world() -> PyResult<Vec<String>> {
    let res = get_data().sim_svc.list();
    match res {
        Ok(r) => Ok(r.iter().map(|v| v.name.clone()).collect()),
        Err(e) => Err(PyTypeError::new_err(e.to_string())),
    }
}

#[pyfunction]
fn get_world(name: String) -> PyResult<World> {
    let res = get_data().sim_svc.create(&name);
    match res {
        Ok(r) => Ok(World {
            name: r.name.clone(),
            map_name: "".to_string(),

            scenario_name: "".to_string(),
            simulator_ref: {
                let simulator = get_data()
                    .sim_svc
                    .get(&name)
                    .expect("create simulator but can not get");
                simulator
            },
        }),
        Err(e) => Err(PyTypeError::new_err(e.to_string())),
    }
}

///
pub(crate) static mut APP_DATA: OnceCell<libsimulator::AppState> =
    OnceCell::new();
pub(crate) fn get_data<'a>() -> &'a mut libsimulator::AppState {
    unsafe { APP_DATA.get_mut().unwrap() }
}
pub(crate) fn set_data(app: libsimulator::AppState) {
    unsafe { _ = APP_DATA.set(app) };
}
pub(crate) fn get_map_svc<'a>() -> Arc<dyn MapServiceInterface> {
    unsafe { APP_DATA.get_mut().unwrap().map_svc.clone() }
}
pub(crate) fn get_scenario_svc<'a>() -> Arc<dyn ScenarioServiceInterface> {
    unsafe { APP_DATA.get_mut().unwrap().scenario_svc.clone() }
}
pub(crate) fn get_sim_svc<'a>() -> Arc<dyn SimulatorServiceInterface> {
    unsafe { APP_DATA.get_mut().unwrap().sim_svc.clone() }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{get_data, list_map, set_data};

    #[test]
    fn it_works() {
        let eventbus = libutil::eventbus::EventBus::default();
        let apps_state = libsimulator::AppState {
            eventbus: eventbus.clone(),
            map_svc: libsimulator::FileBasedMapServiceImpl::new(PathBuf::from(
                "/home/huangsu/work/Perseus/src-backend/libformat/asset/\
                 opendrive",
            ))
            .unwrap(),
            scenario_svc: libsimulator::FileSanitationScenarioServiceImpl::new(
                PathBuf::from(
                    "/home/huangsu/work/Perseus/src-backend/libmodel/asset/\
                     sanitation",
                ),
            )
            .unwrap(),
            sim_svc: libsimulator::SanitationSimulatorServiceImpl::new(
                eventbus.clone(),
            ),
        };
        set_data(apps_state);
        let res = get_data().map_svc.list();
        let t = match res {
            Ok(r) => (),
            Err(e) => (),
        };
    }
}
