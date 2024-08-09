use std::collections::HashMap;
use std::sync::Arc;

use protobuf::Message;

use crate::common::kdtree2d::KDTree2d;
use crate::common::kdtree_box::{LineSegmentBox, PointBox};
use crate::common::util::{LineSegment2d, Vec2d};
use crate::element::{CleanArea, Header, JunctionInfo, RoadInfo, SiteInfo};
use crate::proto_gen;

/// 地图sd_map
#[derive(Debug)]
pub struct SDMap {
    pub header: Header,

    // 路
    pub(crate) roads: HashMap<i32, Arc<RoadInfo>>,
    // 路口
    pub(crate) junctions: HashMap<i32, Arc<JunctionInfo>>,

    // 非道路的作业区域
    pub(crate) clean_areas: HashMap<i32, Arc<CleanArea>>,

    // 环卫的各个 站点
    pub(crate) sites: HashMap<i32, Arc<SiteInfo>>,

    // 用于 查询 车道
    pub(crate) road_infos_kdtree: KDTree2d<LineSegmentBox>,
    // 十字路口查询
    pub(crate) junctions_kdtree: KDTree2d<PointBox>,

    pub(crate) next_road_id: i32,
}

impl SDMap {
    pub fn new() -> Self {
        Self {
            header: Header {
                major_version: 0,
                minor_version: 0,
                name: "huangsu_test".to_string(),
                date: chrono::prelude::Local::now().to_string(),
                coordinate: "wgs84",
            },
            roads: Default::default(),
            junctions: Default::default(),
            clean_areas: Default::default(),
            sites: Default::default(),
            road_infos_kdtree: Default::default(),
            junctions_kdtree: Default::default(),
            next_road_id: 0,
        }
    }

    pub fn load_map_file<T: AsRef<std::path::Path>>(
        path: T,
    ) -> anyhow::Result<SDMap> {
        let now = std::time::Instant::now();

        let opendrive = libformat::opendrive::parse(&path)?;

        let mut sd_map = SDMap::from(opendrive);
        sd_map.generate_kdtree();

        tracing::debug!(
            "执行时间 SDMap load_map_file: {}ms",
            now.elapsed().as_millis()
        );
        return Ok(sd_map);
    }

    pub fn store_map_file<T: AsRef<std::path::Path>>(
        &self, path: T,
    ) -> anyhow::Result<()> {
        let opendrive: libformat::opendrive::OpenDrive = self.into();

        let content = libformat::opendrive::unparse(&opendrive)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn serialize_to_vec(&self) -> anyhow::Result<Vec<u8>> {
        let pb_map: proto_gen::map::Map = self.into();
        let bytes = pb_map.write_to_bytes()?;
        return Ok(bytes);
    }

    pub fn deserialize_from_bytes(data: &[u8]) -> anyhow::Result<Self> {
        let pb_map = proto_gen::map::Map::parse_from_bytes(data)?;
        return Ok(Self::from(pb_map));
    }

    pub fn active(&mut self) {
        self.generate_kdtree();
    }

    pub fn get_roads(&self) -> &HashMap<i32, Arc<RoadInfo>> {
        return &self.roads;
    }

    pub fn get_road_by_id(&self, id: i32) -> Arc<RoadInfo> {
        return self.roads.get(&id).unwrap().clone();
    }

    pub fn get_junctions(&self) -> &HashMap<i32, Arc<JunctionInfo>> {
        return &self.junctions;
    }

    pub fn get_junction_by_id(&self, id: i32) -> &JunctionInfo {
        return self.junctions.get(&id).unwrap();
    }

    pub fn get_clean_area(&self) -> &HashMap<i32, Arc<CleanArea>> {
        return &self.clean_areas;
    }
    pub fn get_clean_area_by_id(&self, id: i32) -> &CleanArea {
        return self.clean_areas.get(&id).unwrap();
    }

    pub fn get_sites(&self) -> &HashMap<i32, Arc<SiteInfo>> {
        return &self.sites;
    }

    /// 根据坐标找 最近道路
    pub fn find_nearest_road(&self, x: f64, y: f64) -> Option<Arc<RoadInfo>> {
        let r = self.road_infos_kdtree.find_nearest_object(&Vec2d { x, y });
        match r {
            Some(t) => {
                return Some(self.get_road_by_id(t.road_id_));
            }
            None => None,
        }
    }
    /// 根据坐标和 距离找 车道
    pub fn find_road_by_distance(
        &self, x: f64, y: f64, distance: f64,
    ) -> Vec<Arc<RoadInfo>> {
        use std::collections::HashSet;
        let mut road_ids: HashSet<i32> = HashSet::new();
        let mut v = Vec::new();
        let found = self
            .road_infos_kdtree
            .find_objects_by_distance(&Vec2d { x, y }, distance);
        for line_segment_box in &found {
            road_ids.insert(line_segment_box.road_id_);
        }
        for id in road_ids {
            let lane = self.get_road_by_id(id);
            v.push(lane);
        }
        v
    }

    pub fn find_junction_by_distance(
        &self, x: f64, y: f64, distance: f64,
    ) -> Vec<&JunctionInfo> {
        let mut v = Vec::new();
        let found = self
            .junctions_kdtree
            .find_objects_by_distance(&Vec2d { x, y }, distance);
        for crosswalk_box in &found {
            let crosswalk = self.get_junction_by_id(crosswalk_box.id_);
            v.push(crosswalk);
        }
        v
    }

    fn generate_kdtree(&mut self) {
        // 构建 道路的kdtree
        let mut now = std::time::Instant::now();
        let mut all_road_segment_box: Vec<LineSegmentBox> = Vec::new();
        for (id, road) in &self.roads {
            let mut index = 0;
            for item in road.center_line.segments.iter() {
                all_road_segment_box.push(LineSegmentBox {
                    data_: Arc::new(LineSegment2d::new(
                        item.get_smooth_point(item.s()),
                        item.get_smooth_point(item.s() + item.length()),
                    )),
                    line_segment_index: index,
                    road_id_: *id,
                    lane_id_: 0,
                });
                index += 1;
            }
        }
        self.road_infos_kdtree = KDTree2d::build(all_road_segment_box);
        tracing::debug!(
            "  generate_kdtree lane_infos_kdtree : {}ms",
            now.elapsed().as_millis()
        );

        // 路口的kdtree
        now = std::time::Instant::now();
        let mut all_junction_box: Vec<PointBox> = Vec::new();
        for (id, junction) in self.junctions.iter() {
            all_junction_box.push(PointBox {
                center_: junction.center,
                id_: id.clone(),
            });
        }
        self.junctions_kdtree = KDTree2d::build(all_junction_box);
        tracing::debug!(
            "  generate_kdtree junctions_kdtree_ : {}ms",
            now.elapsed().as_millis()
        );
    }
}

///  opendrive 格式的对接
impl From<libformat::opendrive::OpenDrive> for SDMap {
    fn from(opendrive: libformat::opendrive::OpenDrive) -> Self {
        return crate::map::SDMap::from(&opendrive);
    }
}

impl From<&libformat::opendrive::OpenDrive> for SDMap {
    fn from(opendrive: &libformat::opendrive::OpenDrive) -> Self {
        let mut next_road_id = 0;

        // road
        let mut road_hash_map: HashMap<i32, Arc<RoadInfo>> = HashMap::default();
        for r in opendrive.road.iter() {
            let road = RoadInfo::from(r);
            let my_int: i32 = r.id.parse().unwrap();
            if my_int > next_road_id {
                next_road_id = my_int + 1;
            }
            road_hash_map.insert(r.id.parse().unwrap(), Arc::new(road));
        }
        // junction
        let mut junction_hash_map: HashMap<i32, Arc<JunctionInfo>> =
            HashMap::default();
        if let Some(junctions) = opendrive.junction.as_ref() {
            for j in junctions {
                junction_hash_map.insert(
                    j.id.parse().unwrap(),
                    Arc::new(JunctionInfo::from(j)),
                );
            }
        }
        // area
        let mut area_hash_map: HashMap<i32, Arc<CleanArea>> =
            HashMap::default();
        if let Some(area) = opendrive.clean_area.as_ref() {
            for s in area {
                area_hash_map.insert(
                    s.name.parse().unwrap(),
                    Arc::new(CleanArea::from(s)),
                );
            }
        }

        // site
        let mut site_hash_map: HashMap<i32, Arc<SiteInfo>> = HashMap::default();
        if let Some(stations) = opendrive.station.as_ref() {
            for s in stations {
                site_hash_map.insert(
                    s.name.parse().unwrap(),
                    Arc::new(SiteInfo::from(s)),
                );
            }
        }

        SDMap {
            header: Header::from(opendrive.header.as_ref().unwrap()),
            roads: road_hash_map,
            junctions: junction_hash_map,
            clean_areas: area_hash_map,
            sites: site_hash_map,
            road_infos_kdtree: Default::default(),
            junctions_kdtree: Default::default(),
            next_road_id,
        }
    }
}

impl Into<libformat::opendrive::OpenDrive> for &SDMap {
    fn into(self) -> libformat::opendrive::OpenDrive {
        libformat::opendrive::OpenDrive {
            header: Some((&self.header).into()),
            road: self.roads.iter().map(|(_id, r)| r.as_ref().into()).collect(),
            controller: None,
            junction: Some(
                self.junctions
                    .iter()
                    .map(|(_id, r)| r.as_ref().into())
                    .collect(),
            ),
            junction_group: None,
            station: Some(
                self.sites.iter().map(|(_id, r)| r.as_ref().into()).collect(),
            ),
            clean_area: Some(
                self.clean_areas
                    .iter()
                    .map(|(_id, r)| r.as_ref().into())
                    .collect(),
            ),
        }
    }
}

/// 自定义的map proto格式，用于前后端传输的
impl From<proto_gen::map::Map> for SDMap {
    fn from(pb_map: proto_gen::map::Map) -> Self {
        return SDMap::from(&pb_map);
    }
}

impl From<&proto_gen::map::Map> for SDMap {
    fn from(pb_map: &proto_gen::map::Map) -> Self {
        let mut next_road_id = 0;

        // road
        let mut road_hash_map: HashMap<i32, Arc<RoadInfo>> = HashMap::default();
        for (_name, r) in pb_map.roads.iter() {
            let road = RoadInfo::from(r);
            if r.id > next_road_id {
                next_road_id = r.id + 1;
            }
            road_hash_map.insert(r.id, Arc::new(road));
        }
        // junction
        let mut junction_hash_map: HashMap<i32, Arc<JunctionInfo>> =
            HashMap::default();
        for (_name, j) in pb_map.junctions.iter() {
            junction_hash_map.insert(j.id, Arc::new(JunctionInfo::from(j)));
        }

        // area
        let mut area_hash_map: HashMap<i32, Arc<CleanArea>> =
            HashMap::default();
        for (_name, c) in pb_map.clean_areas.iter() {
            area_hash_map.insert(c.id, Arc::new(CleanArea::from(c)));
        }

        // site
        let mut site_hash_map: HashMap<i32, Arc<SiteInfo>> = HashMap::default();
        for (_name, s) in pb_map.sites.iter() {
            site_hash_map.insert(s.id, Arc::new(SiteInfo::from(s)));
        }

        SDMap {
            header: Header::from(pb_map.header.as_ref().unwrap()),
            roads: road_hash_map,
            junctions: junction_hash_map,
            clean_areas: area_hash_map,
            sites: site_hash_map,
            road_infos_kdtree: Default::default(),
            junctions_kdtree: Default::default(),
            next_road_id,
        }
    }
}

impl Into<proto_gen::map::Map> for &SDMap {
    fn into(self) -> proto_gen::map::Map {
        proto_gen::map::Map {
            header: protobuf::MessageField::some((&self.header).into()),
            roads: Default::default(),
            junctions: Default::default(),
            clean_areas: Default::default(),
            sites: Default::default(),
            special_fields: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
