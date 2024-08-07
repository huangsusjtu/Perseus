

pub use map::PythonMap as Map;
use once_cell::sync::OnceCell;
use perseus_api::PerseusClient;
use pyo3::prelude::*;
pub use world::World;


static PERSEUS_CLIENT: OnceCell<PerseusClient> = OnceCell::new();

/// A Python module implemented in Rust.
#[pymodule(name = "perseus_py")]
fn perseus(m: &Bound<'_, PyModule>) -> PyResult<()> {
    PERSEUS_CLIENT.set(PerseusClient::new("http://127.0.0.1:2025")).unwrap();

    m.add_function(wrap_pyfunction!(list_map, m)?)?;
    m.add_function(wrap_pyfunction!(list_scenario, m)?)?;
    m.add_function(wrap_pyfunction!(list_world, m)?)?;
    m.add_function(wrap_pyfunction!(get_world, m)?)?;

    m.add_class::<World>()?;
    m.add_class::<Map>()?;
    Ok(())
}

#[pyfunction]
fn list_map() -> PyResult<Vec<String>> {
    let t = smol::block_on(PERSEUS_CLIENT.get().unwrap().list_map());
    Ok(t.unwrap().list.iter().map(|v| v.name.clone()).collect())
}

#[pyfunction]
fn list_scenario() -> PyResult<Vec<String>> {
    let t = smol::block_on(PERSEUS_CLIENT.get().unwrap().list_scenario());
    Ok(t.unwrap().list.iter().map(|v| v.name.clone()).collect())
}

#[pyfunction]
fn list_world() -> PyResult<Vec<String>> {
    let t = smol::block_on(PERSEUS_CLIENT.get().unwrap().list_world());
    Ok(t.unwrap().list.iter().map(|v| v.name.clone()).collect())
}

#[pyfunction]
fn get_world(name: String) -> PyResult<World> {
    let t =
        smol::block_on(PERSEUS_CLIENT.get().unwrap().get_world(name)).unwrap();
    Ok(World {
        name: t.name,
        map_name: "".to_string(),
    })
}
