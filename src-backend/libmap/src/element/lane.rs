
use crate::element::line_curve::LineCurveInfo;
use crate::proto_gen;


#[derive(Debug, Clone, Default)]
pub enum LaneType {
    #[default]
    NONE,
    CityDriving,
    SideWalk,
    Shoulder,
    Bus,
    Biking,
    Parking,
    Bidirectional,
}

/// 车道定义

#[derive(Debug, Clone)]
pub struct LaneInfo {
    pub id: i32,
    pub r#type: LaneType,

    pub central_lane_curve: LineCurveInfo,
    pub width: f32,
}

///  opendrive 格式的对接
impl From<&libformat::opendrive::lane::lane::Lane> for LaneInfo {
    fn from(value: &libformat::opendrive::lane::lane::Lane) -> Self {
        LaneInfo {
            id: value.id,
            r#type: {
                let t = value.r#type.clone().unwrap();
                t.into()
            },
            central_lane_curve: Default::default(), // lane的中心线初始化，得在road初始化后才能做
            width: if let Some(width) = value.width.as_ref() {
                width.first().unwrap().a as f32
            } else {
                5.0
            },
        }
    }
}

impl Into<libformat::opendrive::lane::lane::Lane> for &LaneInfo {
    fn into(self) -> libformat::opendrive::lane::lane::Lane {
        libformat::opendrive::lane::lane::Lane {
            id: self.id,
            r#type: None,
            advisory: None,
            direction: None,
            dynamic_lane_direction: None,
            dynamic_lane_type: None,
            level: None,
            road_works: None,
            link: None,
            road_mark: None,
            width: Some(vec![libformat::opendrive::lane::lane::LaneWidth {
                a: self.width as f64,
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
    }
}

impl From<libformat::opendrive::lane::lane::ELaneType> for LaneType {
    fn from(value: libformat::opendrive::lane::lane::ELaneType) -> Self {
        match value {
            libformat::opendrive::lane::lane::ELaneType::Bidirectional => LaneType::Bidirectional,
            libformat::opendrive::lane::lane::ELaneType::Biking => LaneType::Biking,
            libformat::opendrive::lane::lane::ELaneType::Bus => LaneType::Bus,
            libformat::opendrive::lane::lane::ELaneType::Driving => LaneType::CityDriving,
            libformat::opendrive::lane::lane::ELaneType::None => LaneType::NONE,
            libformat::opendrive::lane::lane::ELaneType::Parking => LaneType::Parking,
            libformat::opendrive::lane::lane::ELaneType::Shoulder => LaneType::Shoulder,
            libformat::opendrive::lane::lane::ELaneType::Sidewalk => LaneType::SideWalk,
            libformat::opendrive::lane::lane::ELaneType::Taxi => LaneType::CityDriving,
            libformat::opendrive::lane::lane::ELaneType::Walking => LaneType::SideWalk,
            _ => LaneType::NONE,
        }
    }
}

impl Into<libformat::opendrive::lane::lane::ELaneType> for &LaneType {
    fn into(self) -> libformat::opendrive::lane::lane::ELaneType {
        match self {
            LaneType::NONE => libformat::opendrive::lane::lane::ELaneType::None,
            LaneType::CityDriving => libformat::opendrive::lane::lane::ELaneType::Driving,
            LaneType::SideWalk => libformat::opendrive::lane::lane::ELaneType::Sidewalk,
            LaneType::Shoulder => libformat::opendrive::lane::lane::ELaneType::Shoulder,
            LaneType::Bus => libformat::opendrive::lane::lane::ELaneType::Bus,
            LaneType::Biking => libformat::opendrive::lane::lane::ELaneType::Biking,
            LaneType::Parking => libformat::opendrive::lane::lane::ELaneType::Parking,
            LaneType::Bidirectional => libformat::opendrive::lane::lane::ELaneType::Bidirectional,
        }
    }
}


/// 自定义的map proto格式，用于前后端传输的
impl From<proto_gen::map::lane::LaneType> for LaneType {
    fn from(value: proto_gen::map::lane::LaneType) -> Self {
        match value {
            proto_gen::map::lane::LaneType::NONE => { LaneType::NONE }
            proto_gen::map::lane::LaneType::CityDriving => { LaneType::CityDriving }
            proto_gen::map::lane::LaneType::SideWalk => { LaneType::SideWalk }
            proto_gen::map::lane::LaneType::Shoulder => { LaneType::Shoulder }
            proto_gen::map::lane::LaneType::Bus => { LaneType::Bus }
            proto_gen::map::lane::LaneType::Biking => { LaneType::Biking }
            proto_gen::map::lane::LaneType::Parking => { LaneType::Parking }
            proto_gen::map::lane::LaneType::Bidirectional => { LaneType::Bidirectional }
        }
    }
}

impl Into<proto_gen::map::lane::LaneType> for &LaneType {
    fn into(self) -> proto_gen::map::lane::LaneType {
        match self {
            LaneType::NONE => { proto_gen::map::lane::LaneType::NONE }
            LaneType::CityDriving => { proto_gen::map::lane::LaneType::CityDriving }
            LaneType::SideWalk => { proto_gen::map::lane::LaneType::SideWalk }
            LaneType::Shoulder => { proto_gen::map::lane::LaneType::Shoulder }
            LaneType::Bus => { proto_gen::map::lane::LaneType::Bus }
            LaneType::Biking => { proto_gen::map::lane::LaneType::Biking }
            LaneType::Parking => { proto_gen::map::lane::LaneType::Parking }
            LaneType::Bidirectional => { proto_gen::map::lane::LaneType::Bidirectional }
        }
    }
}

impl From<proto_gen::map::Lane> for LaneInfo {
    fn from(value: proto_gen::map::Lane) -> Self {
        return Self::from(&value);
    }
}

impl From<&proto_gen::map::Lane> for LaneInfo {
    fn from(value: &proto_gen::map::Lane) -> Self {
        LaneInfo {
            id: value.id,
            r#type: LaneType::from(value.type_.enum_value_or_default()),
            central_lane_curve: LineCurveInfo::from(value.central_line.as_ref().unwrap()),
            width: value.width,
        }
    }
}

impl Into<proto_gen::map::Lane> for &LaneInfo {
    fn into(self) -> proto_gen::map::Lane {
        proto_gen::map::Lane {
            id: self.id,
            type_: ::protobuf::EnumOrUnknown::new((&self.r#type).into()),
            central_line: protobuf::MessageField::some((&self.central_lane_curve).into()),
            width: self.width,
            special_fields: Default::default(),
        }
    }
}