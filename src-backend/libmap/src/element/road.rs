
use libformat::opendrive::road::road::{Predecessor, Successor};

use crate::element::lane::{LaneInfo, LaneType};
use crate::element::line::GeometryLine;
use crate::element::line_curve::LineCurveInfo;
use crate::proto_gen;

/// 道路定义
#[derive(Debug)]
pub struct RoadInfo {
    pub id: i32, // 唯一ID
    pub name: String,
    pub length: f64,

    pub r#type: RoadType,

    pub center_line: LineCurveInfo,

    pub left_lanes: Vec<LaneInfo>,
    pub right_lanes: Vec<LaneInfo>,

    // 路的上下游
    pub link: Vec<RoadLink>,
    //  todo!() add more
    // 道路两侧 树木类型， 密度，等等
}

#[derive(Debug, Clone)]
pub enum RoadType {
    Major = 1,    // 主道
    Minor = 2,    // 辅道
    SideWalk = 3, // 人行道
}


#[derive(Debug)]
pub struct RoadLink {
    pub link_type: String,    // predecessor or successor
    pub element_type: String, // junction or road
    pub element_id: i32,      // junction or road id
}

///  opendrive 格式的对接
impl From<&libformat::opendrive::road::road::RoadType> for RoadType {
    fn from(value: &libformat::opendrive::road::road::RoadType) -> Self {
        match value.r#type {
            libformat::opendrive::road::road::ERoadType::Bicycle => {
                RoadType::Minor
            }
            libformat::opendrive::road::road::ERoadType::Motorway => {
                RoadType::Minor
            }
            libformat::opendrive::road::road::ERoadType::Pedestrian => {
                RoadType::SideWalk
            }
            libformat::opendrive::road::road::ERoadType::LowSpeed => {
                RoadType::Major
            }

            _ => RoadType::Major,
        }
    }
}

impl Into<libformat::opendrive::road::road::RoadType> for &RoadType {
    fn into(self) -> libformat::opendrive::road::road::RoadType {
        let mut t = libformat::opendrive::road::road::RoadType {
            s: 0.0,
            r#type: libformat::opendrive::road::road::ERoadType::LowSpeed,
            country: None,
            speed: None,
        };
        match self {
            RoadType::Major => {
                t.r#type = libformat::opendrive::road::road::ERoadType::LowSpeed
            }
            RoadType::Minor => {
                t.r#type = libformat::opendrive::road::road::ERoadType::Bicycle
            }
            RoadType::SideWalk => {
                t.r#type =
                    libformat::opendrive::road::road::ERoadType::Pedestrian
            }
        }
        return t;
    }
}

impl From<&libformat::opendrive::road::Road> for RoadInfo {
    fn from(value: &libformat::opendrive::road::Road) -> Self {
        // road的 中心线
        let road_center_line = {
            let v: Vec<GeometryLine> = value
                .plan_view
                .geometry
                .iter()
                .map(|v| crate::element::line::GeometryLine::from(v))
                .collect();
            LineCurveInfo::new(v)
        };

        // 这里只取第一个section； opendrive road里的lanes分多个section，
        // 咱们简化点使用；
        let section = value.lanes.lane_section.first().unwrap();
        let left_lanes = if let Some(left) = section.left.as_ref() {
            let mut left_lanes: Vec<LaneInfo> =
                left.lane.iter().map(|v| LaneInfo::from(v)).collect();
            // 由小到大， lane 1,2,3
            left_lanes.sort_by(|a, b| a.id.cmp(&b.id));
            // 左侧lane的中心线 是由road的中心线 左移动得到
            let mut shift = 0.0;
            for l in left_lanes.iter_mut() {
                shift += l.width / 2.0;
                l.central_lane_curve =
                    road_center_line.reverse().translation(shift as f64);
            }
            left_lanes
        } else {
            vec![]
        };
        let right_lanes = if let Some(right) = section.right.as_ref() {
            let mut right_lanes: Vec<LaneInfo> =
                right.lane.iter().map(|v| LaneInfo::from(v)).collect();
            // 由大到小， lane -1,-2,-3
            right_lanes.sort_by(|a, b| b.id.cmp(&a.id));
            // 右侧lane的中心线 是由road的中心线 右移动得到
            let mut shift = 0.0;
            for r in right_lanes.iter_mut() {
                shift += r.width / 2.0;
                r.central_lane_curve =
                    road_center_line.translation(shift as f64);
            }
            right_lanes
        } else {
            vec![]
        };

        RoadInfo {
            id: value.id.parse().unwrap(),
            name: value.name.clone().unwrap_or("None".to_string()),
            length: value.length,
            r#type: {
                if let Some(types) = value.r#type.as_ref() {
                    RoadType::from(types.first().unwrap())
                } else {
                    RoadType::Major
                }
            },
            center_line: road_center_line,
            left_lanes,
            right_lanes,
            link: {
                if let Some(links) = value.link.as_ref() {
                    let mut v = vec![];
                    for link in links {
                        if link.predecessor.is_some() {
                            v.push(RoadLink {
                                link_type: "predecessor".to_string(),
                                element_type: link
                                    .predecessor
                                    .as_ref()
                                    .unwrap()
                                    .element_type
                                    .clone(),
                                element_id: link
                                    .predecessor
                                    .as_ref()
                                    .unwrap()
                                    .element_id
                                    .parse()
                                    .unwrap(),
                            });
                        }
                        if link.successor.is_some() {
                            v.push(RoadLink {
                                link_type: "successor".to_string(),
                                element_type: link
                                    .successor
                                    .as_ref()
                                    .unwrap()
                                    .element_type
                                    .clone(),
                                element_id: link
                                    .successor
                                    .as_ref()
                                    .unwrap()
                                    .element_id
                                    .parse()
                                    .unwrap(),
                            });
                        }
                    }
                    v
                } else {
                    vec![]
                }
            },
        }
    }
}

impl Into<libformat::opendrive::road::Road> for &RoadInfo {
    fn into(self) -> libformat::opendrive::road::Road {
        libformat::opendrive::road::Road {
            id: self.id.to_string(),
            junction: "-1".to_string(),
            length: self.length,
            name: Some(self.name.clone()),
            rule: None,
            plan_view: libformat::opendrive::common::planview::PlanView {
                geometry: self
                    .center_line
                    .segments
                    .iter()
                    .map(|v| v.into())
                    .collect(),
            },
            lanes: libformat::opendrive::lane::Lanes {
                lane_offset: None,
                lane_section: vec![libformat::opendrive::lane::lane::LaneSection {
                    s: 0.0,
                    single_side: None,
                    left: Some(libformat::opendrive::lane::lane::Left {
                        lane: self.left_lanes.iter().map(|v| {
                            libformat::opendrive::lane::lane::Lane {
                                id: v.id,
                                r#type: match v.r#type {
                                    LaneType::NONE => { Some(libformat::opendrive::lane::lane::ELaneType::None) }
                                    LaneType::CityDriving => { Some(libformat::opendrive::lane::lane::ELaneType::Driving) }
                                    LaneType::SideWalk => { Some(libformat::opendrive::lane::lane::ELaneType::Sidewalk) }
                                    LaneType::Shoulder => { Some(libformat::opendrive::lane::lane::ELaneType::Shoulder) }
                                    LaneType::Bus => { Some(libformat::opendrive::lane::lane::ELaneType::Bus) }
                                    LaneType::Biking => { Some(libformat::opendrive::lane::lane::ELaneType::Biking) }
                                    LaneType::Parking => { Some(libformat::opendrive::lane::lane::ELaneType::Parking) }
                                    LaneType::Bidirectional => { Some(libformat::opendrive::lane::lane::ELaneType::Bidirectional) }
                                },
                                advisory: None,
                                direction: Some(libformat::opendrive::lane::lane::ELaneDirection::default()),
                                dynamic_lane_direction: None,
                                dynamic_lane_type: None,
                                level: None,
                                road_works: None,
                                link: None,
                                road_mark: None,
                                width: Some(vec![libformat::opendrive::lane::lane::LaneWidth {
                                    a: v.width as f64,
                                    b: 0.0,
                                    c: 0.0,
                                    d: 0.0,
                                    s_offset: 0.0,
                                }]),
                                border: None,
                                material: None,
                                speed: None,
                                access: None,
                                height: None,
                                rule: None,
                            }
                        }).collect()
                    }),
                    center: libformat::opendrive::lane::lane::Center {
                        lane: vec![
                            libformat::opendrive::lane::lane::CenterLane {
                                id: 0,
                                level: Some(libformat::opendrive::lane::lane::TBool::True),
                                r#type: None,
                                link: None,
                                road_mark: None,
                            }
                        ]
                    },
                    right: Some(libformat::opendrive::lane::lane::Right {
                        lane: self.right_lanes.iter().map(|v| {
                            libformat::opendrive::lane::lane::Lane {
                                id: v.id,
                                r#type: match v.r#type {
                                    LaneType::NONE => { Some(libformat::opendrive::lane::lane::ELaneType::None) }
                                    LaneType::CityDriving => { Some(libformat::opendrive::lane::lane::ELaneType::Driving) }
                                    LaneType::SideWalk => { Some(libformat::opendrive::lane::lane::ELaneType::Sidewalk) }
                                    LaneType::Shoulder => { Some(libformat::opendrive::lane::lane::ELaneType::Shoulder) }
                                    LaneType::Bus => { Some(libformat::opendrive::lane::lane::ELaneType::Bus) }
                                    LaneType::Biking => { Some(libformat::opendrive::lane::lane::ELaneType::Biking) }
                                    LaneType::Parking => { Some(libformat::opendrive::lane::lane::ELaneType::Parking) }
                                    LaneType::Bidirectional => { Some(libformat::opendrive::lane::lane::ELaneType::Bidirectional) }
                                },
                                advisory: None,
                                direction: Some(libformat::opendrive::lane::lane::ELaneDirection::default()),
                                dynamic_lane_direction: None,
                                dynamic_lane_type: None,
                                level: None,
                                road_works: None,
                                link: None,
                                road_mark: None,
                                width: Some(vec![libformat::opendrive::lane::lane::LaneWidth {
                                    a: v.width as f64,
                                    b: 0.0,
                                    c: 0.0,
                                    d: 0.0,
                                    s_offset: 0.0,
                                }]),
                                border: None,
                                material: None,
                                speed: None,
                                access: None,
                                height: None,
                                rule: None,
                            }
                        }).collect()
                    }),
                }],
            },
            link: {
                let mut v = vec![];
                for l in self.link.iter() {
                    if l.link_type.eq("predecessor") {
                        v.push(
                            libformat::opendrive::road::road::RoadLink {
                                predecessor: Some(Predecessor {
                                    element_id: l.element_id.to_string(),
                                    element_type: l.element_type.clone(),
                                    contact_point: None,
                                    element_dir: None,
                                    element_s: None,
                                }),
                                successor: None,
                            },
                        );
                    } else {
                        v.push(
                            libformat::opendrive::road::road::RoadLink {
                                successor: Some(Successor {
                                    element_id: l.element_id.to_string(),
                                    element_type: l.element_type.clone(),
                                    contact_point: None,
                                    element_dir: None,
                                    element_s: None,
                                }),
                                predecessor: None,
                            },
                        );
                    }
                }
                Some(v)
            },
            r#type: {
                let t: libformat::opendrive::road::road::RoadType =
                    (&self.r#type).into();
                Some(vec![t])
            },
            elevation_profile: None,
            lateral_profile: None,
            objects: None,
        }
    }
}


/// 自定义的map proto格式，用于前后端传输的
impl From<proto_gen::map::Road> for RoadInfo {
    fn from(value: proto_gen::map::Road) -> Self {
        return Self::from(&value);
    }
}

impl From<&proto_gen::map::Road> for RoadInfo {
    fn from(value: &proto_gen::map::Road) -> Self {
        RoadInfo {
            id: value.id,
            name: value.name.clone(),
            length: value.length as f64,
            r#type: match value.type_.enum_value_or_default() {
                proto_gen::map::road::RoadType::None => RoadType::Major,
                proto_gen::map::road::RoadType::Major => RoadType::Major,
                proto_gen::map::road::RoadType::Minor => RoadType::Minor,
                proto_gen::map::road::RoadType::SideWalk => RoadType::SideWalk,
            },
            center_line: (value.central_line.as_ref().unwrap()).into(),
            left_lanes: value.left_lanes.iter().map(|v| v.into()).collect(),
            right_lanes: value.right_lanes.iter().map(|v| v.into()).collect(),
            link: value
                .link
                .iter()
                .map(|v| RoadLink {
                    link_type: v.link_type.clone(),
                    element_type: v.element_type.clone(),
                    element_id: v.element_id,
                })
                .collect(),
        }
    }
}

impl Into<proto_gen::map::Road> for &RoadInfo {
    fn into(self) -> proto_gen::map::Road {
        proto_gen::map::Road {
            id: self.id,
            name: self.name.clone(),
            length: self.length as f32,
            type_: match self.r#type {
                RoadType::Major => protobuf::EnumOrUnknown::from(
                    proto_gen::map::road::RoadType::Major,
                ),
                RoadType::Minor => protobuf::EnumOrUnknown::from(
                    proto_gen::map::road::RoadType::Minor,
                ),
                RoadType::SideWalk => protobuf::EnumOrUnknown::from(
                    proto_gen::map::road::RoadType::SideWalk,
                ),
            },
            central_line: protobuf::MessageField::some(
                (&self.center_line).into(),
            ),
            left_lanes: self.left_lanes.iter().map(|v| v.into()).collect(),
            right_lanes: self.right_lanes.iter().map(|v| v.into()).collect(),
            link: self
                .link
                .iter()
                .map(|v| proto_gen::map::road::RoadLink {
                    link_type: v.link_type.clone(),
                    element_type: v.element_type.clone(),
                    element_id: v.element_id,
                    special_fields: Default::default(),
                })
                .collect(),
            special_fields: Default::default(),
        }
    }
}
