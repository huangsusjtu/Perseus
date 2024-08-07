use crate::common::util::LanePoint;
use crate::proto_gen;

/// 十字路口的描述

#[derive(Debug, Clone)]
pub struct JunctionInfo {
    pub id: i32, // 唯一ID
    pub name: String,
    pub center: LanePoint,       // 坐标
    pub polygon: Vec<LanePoint>, // 区域

    pub road_connections: Vec<ConnectionInfo>, // 哪些road连接起来了
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: i32,
    pub road_in: String,
    pub road_out: String,
}

///  opendrive 格式的对接
impl From<&libformat::opendrive::junction::Junction> for JunctionInfo {
    fn from(value: &libformat::opendrive::junction::Junction) -> Self {
        let road_connections = if value.connection.is_empty() {
            vec![]
        } else {
            value
                .connection
                .iter()
                .map(|v| ConnectionInfo {
                    id: v.id.parse().unwrap(),
                    road_in: v.incoming_road.clone().unwrap(),
                    road_out: v.connecting_road.clone().unwrap(),
                })
                .collect()
        };

        let polygon: Vec<LanePoint> = if value.plan_view.is_some() {
            value
                .plan_view
                .as_ref()
                .unwrap()
                .geometry
                .iter()
                .map(|v| LanePoint::new(v.x, v.y))
                .collect()
        } else {
            vec![]
        };
        let mut center = LanePoint::default();
        for i in &polygon {
            center.x += i.x / polygon.len() as f64;
            center.x += i.y / polygon.len() as f64;
        }

        JunctionInfo {
            id: value.id.parse().unwrap(),
            name: value.name.clone().unwrap_or_default(),
            center: center,
            polygon: polygon,
            road_connections,
        }
    }
}

impl Into<libformat::opendrive::junction::Junction> for &JunctionInfo {
    fn into(self) -> libformat::opendrive::junction::Junction {
        libformat::opendrive::junction::Junction {
            id: self.id.to_string(),
            r#type: Some(libformat::opendrive::junction::junction::EJunctionType::DEFAULT),
            name: Some(self.name.clone()),
            priority: None,
            controller: None,
            surface: None,
            plan_view: {
                Some(libformat::opendrive::common::planview::PlanView {
                    geometry: self
                        .polygon
                        .iter()
                        .map(|v| libformat::opendrive::common::planview::Geometry {
                            hdg: 0.0,
                            length: 0.0,
                            s: 0.0,
                            x: v.x,
                            y: v.y,
                            line: Some(libformat::opendrive::common::planview::Line),
                            spiral: None,
                            arc: None,
                            poly3: None,
                            param_poly3: None,
                        })
                        .collect(),
                })
            },
            road_section: None,
            connection: self
                .road_connections
                .iter()
                .map(|v| libformat::opendrive::junction::junction::Connection {
                    id: v.id.to_string(),
                    incoming_road: Some(v.road_in.clone()),
                    connecting_road: Some(v.road_out.clone()),
                    linked_road: None,
                    contact_point: None,
                    lane_link: None,
                })
                .collect(),
            cross_path: None,
            boundary: None,
            elevation_grid: None,
        }
    }
}

/// 自定义的map proto格式，用于前后端传输的
impl From<proto_gen::map::Junction> for JunctionInfo {
    fn from(value: proto_gen::map::Junction) -> Self {
        return Self::from(&value);
    }
}

impl From<&proto_gen::map::Junction> for JunctionInfo {
    fn from(value: &proto_gen::map::Junction) -> Self {
        return JunctionInfo {
            id: value.id,
            name: value.name.clone(),
            center: LanePoint::from(
                value.position.as_ref().unwrap_or_default(),
            ),
            polygon: value.polygon.iter().map(|v| LanePoint::from(v)).collect(),
            road_connections: value
                .connection
                .iter()
                .map(|v| ConnectionInfo {
                    id: v.id,
                    road_in: v.road_in.clone(),
                    road_out: v.road_out.clone(),
                })
                .collect(),
        };
    }
}

impl Into<proto_gen::map::Junction> for &JunctionInfo {
    fn into(self) -> proto_gen::map::Junction {
        return proto_gen::map::Junction {
            id: self.id,
            name: self.name.clone(),
            position: {
                let p: proto_gen::map::LanePoint = (&self.center).into();
                protobuf::MessageField::some(p)
            },
            polygon: self.polygon.iter().map(|v| v.into()).collect(),
            connection: self
                .road_connections
                .iter()
                .map(|v| proto_gen::map::junction::ConnectionInfo {
                    id: v.id,
                    road_in: v.road_in.clone(),
                    road_out: v.road_out.clone(),
                    special_fields: Default::default(),
                })
                .collect(),
            special_fields: Default::default(),
        };
    }
}
