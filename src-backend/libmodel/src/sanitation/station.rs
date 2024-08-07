/// 环卫站点的 状态
/// 在地图libmap的site只表示 站点的地理位置和类型， 这里描述具体的动态状态
#[derive(Debug, Clone)]
pub struct StationState {
    pub consume_water: f64,
    pub consume_electricity: f64,
    pub consume_gasoline: f64,
    // todo!()
}
