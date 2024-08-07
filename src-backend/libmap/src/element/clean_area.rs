
use crate::common::util::LanePoint;
use crate::proto_gen;


#[derive(Debug, Default)]
pub struct CleanArea {
    pub id: i32,// 唯一ID
    pub name: String,

    // 位置
    pub position: LanePoint,
    pub polygon: Option<Vec<LanePoint>>,
}

///  opendrive 格式的对接
impl From<&libformat::opendrive::custom_extension::CleanArea> for CleanArea {
    fn from(value: &libformat::opendrive::custom_extension::CleanArea) -> Self {
        return CleanArea {
            id: value.id,
            name: value.name.clone(),
            position: LanePoint::new(value.x, value.y),
            polygon: if let Some(poly) = value.polygon.as_ref() {
                Some(
                    poly.iter()
                        .map(|v| {
                            crate::common::util::LanePoint::new(v[0], v[1])
                        })
                        .collect(),
                )
            } else {
                None
            },
        };
    }
}

impl Into<libformat::opendrive::custom_extension::CleanArea> for &CleanArea {
    fn into(self) -> libformat::opendrive::custom_extension::CleanArea {
        let polygon = if let Some(poly) = &self.polygon {
            let v: Vec<[f64; 2]> = poly.iter().map(|v| [v.x, v.y]).collect();
            Some(v)
        } else {
            None
        };

        return libformat::opendrive::custom_extension::CleanArea {
            name: self.name.clone(),
            id: self.id,
            x: self.position.x,
            y: self.position.y,
            polygon: polygon,
        };
    }
}

/// 自定义的map proto格式，用于前后端传输的
impl From<proto_gen::map::CleanArea> for CleanArea {
    fn from(value: proto_gen::map::CleanArea) -> Self {
        return Self::from(&value);
    }
}

impl From<&proto_gen::map::CleanArea> for CleanArea {
    fn from(value: &proto_gen::map::CleanArea) -> Self {
        return CleanArea {
            id: value.id,
            name: value.name.clone(),
            position: LanePoint::new(value.position.x, value.position.y),
            polygon: Some(
                value
                    .polygon
                    .iter()
                    .map(|v| crate::common::util::LanePoint::new(v.x, v.y))
                    .collect(),
            ),
        };
    }
}

impl Into<proto_gen::map::CleanArea> for &CleanArea {
    fn into(self) -> proto_gen::map::CleanArea {
        let polygon = if let Some(poly) = &self.polygon {
            let v: Vec<proto_gen::map::LanePoint> = poly
                .iter()
                .map(|v| {
                    let mut p = crate::proto_gen::map::LanePoint::new();
                    p.x = v.x;
                    p.y = v.y;
                    p
                })
                .collect();
            v
        } else {
            vec![]
        };

        return proto_gen::map::CleanArea {
            id: 0,
            name: self.name.clone(),
            polygon,
            position: { protobuf::MessageField::some((&self.position).into()) },
            special_fields: Default::default(),
        };
    }
}
