use pyo3::prelude::*;

/// 描述补给站点的状态
#[pyclass(set_all,get_all)]
#[derive(Clone)]
pub struct StationState {
    pub consume_water: f64,
    pub consume_electricity: f64,
    pub consume_gasoline: f64,
}

///
#[pymethods]
impl StationState {
    #[new]
    fn new() -> Self {
        StationState {
            consume_water: 0.0,
            consume_electricity: 0.0,
            consume_gasoline: 0.0,
        }
    }

    fn __str__(&self) -> String {
        format!(
            "StationState (consume_water:{}, consume_electricity:{}, \
             consume_gasoline:{}) ",
            self.consume_water, self.consume_electricity, self.consume_gasoline,
        )
    }
}
