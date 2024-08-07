use pyo3::pyclass;

///
#[pyclass(set_all)]
#[derive(Clone)]
pub struct StationState {
    pub consume_water: f64,
    pub consume_electricity: f64,
    pub consume_gasoline: f64,

}