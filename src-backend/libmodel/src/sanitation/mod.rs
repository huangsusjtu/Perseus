use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub use scenario::Scenario;

use crate::sanitation::sanitationman::SanitationManState;
use crate::sanitation::station::StationState;
use crate::sanitation::vehicle::VehicleState;

pub mod environment;
pub mod garbage;
pub mod header;
pub mod sanitationman;
pub mod scenario;
pub mod station;
pub mod task;
pub mod traffic_flow;
pub mod util;
pub mod vehicle;

pub fn parse<P: AsRef<Path>>(path: P) -> anyhow::Result<Scenario> {
    let xml = fs::read_to_string(path)?;
    let s: Scenario = quick_xml::de::from_str(&xml)?;
    return Ok(s);
}

pub fn unparse(instance: &Scenario) -> anyhow::Result<String> {
    let mut xml = String::new();
    quick_xml::se::to_writer_with_root(
        &mut xml,
        "SanitationScenario",
        &instance,
    )?;
    xml = libformat::pretty_xml::prettify_xml(&mut xml);
    return Ok(xml);
}

/// 仿真运行时的 核心的数据
#[derive(Debug, Clone)]
pub struct SimulatorWorld {
    /// 原始的 场景
    pub scenario: Scenario,
    /// 环卫地图
    pub sd_map: libmap::MapRef,

    pub frame_id: u64,

    /// 状态类信息
    // 环卫车状态
    pub vehicles: HashMap<String, VehicleState>,
    // 环卫工状态
    pub sanitation_man: HashMap<String, SanitationManState>,
    // 站点状态
    pub station_state: HashMap<String, StationState>,
    // 成本计算
    // ...

    // todo!()
}

impl Default for SimulatorWorld {
    fn default() -> Self {
        SimulatorWorld {
            scenario: Default::default(),
            sd_map: Arc::new(libmap::SDMap::new()),
            frame_id: 0,
            vehicles: Default::default(),

            sanitation_man: Default::default(),
            station_state: Default::default(),
        }
    }
}

impl SimulatorWorld {
    pub fn load_map(&mut self, map: libmap::MapRef) {
        self.sd_map = map;
    }

    pub fn load_scenario(&mut self, scenario: Scenario) {
        self.scenario = scenario;
        self.frame_id = 0;

        self.vehicles.clear();
        for v in self.scenario.vehicles.vehicles.iter() {
            self.vehicles.insert(v.name.clone(), v.state.clone());
        }

        self.sanitation_man.clear();
        for v in self.scenario.sanitation_man.sanitation.iter() {
            self.sanitation_man.insert(v.name.clone(), v.state.clone());
        }

        // todo : station站点状态
    }
}
