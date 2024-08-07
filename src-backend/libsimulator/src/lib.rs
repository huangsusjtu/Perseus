use std::sync::Arc;

mod map;
mod scenario;
mod simulator;
pub mod error;

pub use map::*;
pub use scenario::*;
pub use simulator::*;


// #[derive(Debug)]
pub struct AppState {
    // 事件总线
    pub eventbus: libutil::eventbus::EventBus,

    // 地图管理器
    pub map_svc: Arc<dyn crate::MapServiceInterface + Sync + Send>,

    // 地图管理器
    pub scenario_svc: Arc<dyn crate::ScenarioServiceInterface + Sync + Send>,

    // 仿真房间管理器
    pub sim_svc: Arc<dyn crate::SimulatorServiceInterface + Sync + Send>,
}