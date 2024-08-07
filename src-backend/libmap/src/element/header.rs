
use crate::proto_gen;


#[derive(Debug, Default, Clone)]
pub struct Header {
    pub major_version: u32,
    pub minor_version: u32,
    pub name: String,
    /// 生产日期
    pub date: String,
    /// 坐标系wgs84, bd09, utm,
    pub coordinate: &'static str,
}

///  opendrive 格式的对接
impl From<libformat::opendrive::header::Header> for Header {
    fn from(value: libformat::opendrive::header::Header) -> Self {
        return Header::from(&value);
    }
}

impl From<&libformat::opendrive::header::Header> for Header {
    fn from(value: &libformat::opendrive::header::Header) -> Self {
        return Header {
            major_version: value.major_version,
            minor_version: value.minor_version,
            name: value.name.clone().unwrap_or_default(),
            date: value.date.clone().unwrap_or_default(),
            coordinate: "wgs84",
        };
    }
}

impl Into<libformat::opendrive::header::Header> for &Header {
    fn into(self) -> libformat::opendrive::header::Header {
        return libformat::opendrive::header::Header {
            major_version: self.major_version,
            minor_version: self.minor_version,
            name: Some(self.name.clone()),
            date: Some(self.date.clone()),
            east: None,
            north: None,
            south: None,
            west: None,
            vendor: None,
            version: None,
            geo_reference: None,
            offset: None,
            license: None,
            default_regulations: None,
        };
    }
}

/// 自定义的map proto格式，用于前后端传输的
impl From<proto_gen::map::Header> for Header {
    fn from(value: proto_gen::map::Header) -> Self {
        return Self::from(&value);
    }
}

impl From<&proto_gen::map::Header> for Header {
    fn from(value: &proto_gen::map::Header) -> Self {
        return Header {
            major_version: value.major_version as u32,
            minor_version: value.minor_version as u32,
            name: value.name.clone(),
            date: value.date.clone(),
            coordinate: "wgs84",
        };
    }
}

impl Into<proto_gen::map::Header> for &Header {
    fn into(self) -> proto_gen::map::Header {
        return proto_gen::map::Header {
            major_version: self.major_version as i32,
            minor_version: self.minor_version as i32,
            name: self.name.clone(),
            date: self.date.clone(),
            coordinate: Default::default(),
            special_fields: Default::default(),
        };
    }
}