mod straight;
mod arc;
mod spiral;

use libformat::opendrive::common::planview::{Arc, Line, Spiral};
use std::fmt::{Debug, Display, Formatter};

use crate::common::util::LanePoint;
use crate::proto_gen;

///  地图上 3种不同的 线段
#[derive(Clone)]
pub enum GeometryLine {
    Straight(StraightLine),
    Arc(ArcLine),
    Spiral(SpiralLine),
}

#[derive(Debug, Clone)]
pub struct StraightLine {
    pub hdg: f32,
    pub length: f32,
    pub s: f32,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct ArcLine {
    pub hdg: f32,    // 起点的斜率
    pub length: f32, // 弧线长度
    pub s: f32,      //
    pub x: f64,      // 起点坐标
    pub y: f64,
    pub curvature: f32, // 曲率=1/r, 正表示 逆时针
}

#[derive(Debug, Clone)]
pub struct SpiralLine {
    pub hdg: f32,    // 起点的斜率
    pub length: f32, // 螺旋线长度
    pub s: f32,      //
    pub x: f64,      // 起点坐标
    pub y: f64,
    pub curvature_start: f32, // 曲率=1/r,
    pub curvature_end: f32,
}

///  opendrive 格式的对接
impl From<&libformat::opendrive::common::planview::Geometry> for GeometryLine {
    fn from(value: &libformat::opendrive::common::planview::Geometry) -> Self {
        if value.line.is_some() {
            GeometryLine::Straight(StraightLine {
                hdg: value.hdg,
                length: value.length,
                s: value.s,
                x: value.x,
                y: value.y,
            })
        } else if value.arc.is_some() {
            GeometryLine::Arc(ArcLine {
                hdg: value.hdg,
                length: value.length,
                s: value.s,
                x: value.x,
                y: value.y,
                curvature: value.arc.as_ref().unwrap().curvature,
            })
        } else {
            // never here
            GeometryLine::Straight(StraightLine {
                hdg: value.hdg,
                length: value.length,
                s: value.s,
                x: value.x,
                y: value.y,
            })
        }
    }
}

impl Into<libformat::opendrive::common::planview::Geometry> for &GeometryLine {
    fn into(self) -> libformat::opendrive::common::planview::Geometry {
        match self {
            GeometryLine::Straight(v) => {
                libformat::opendrive::common::planview::Geometry {
                    hdg: v.hdg,
                    length: v.length,
                    s: v.s,
                    x: v.x,
                    y: v.y,
                    line: Some(Line),
                    spiral: None,
                    arc: None,
                    poly3: None,
                    param_poly3: None,
                }
            }

            GeometryLine::Arc(v) => {
                libformat::opendrive::common::planview::Geometry {
                    hdg: v.hdg,
                    length: v.length,
                    s: v.s,
                    x: v.x,
                    y: v.y,
                    line: None,
                    spiral: None,
                    arc: Some(Arc {
                        curvature: v.curvature,
                    }),
                    poly3: None,
                    param_poly3: None,
                }
            }
            GeometryLine::Spiral(v) =>  {
                libformat::opendrive::common::planview::Geometry {
                    hdg: v.hdg,
                    length: v.length,
                    s: v.s,
                    x: v.x,
                    y: v.y,
                    line: None,
                    spiral: Some(Spiral {
                        curve_start: v.curvature_start,
                        curve_end: v.curvature_end,
                    }),
                    arc: None,
                    poly3: None,
                    param_poly3: None,
                }
            }
        }
    }
}

/// 自定义的map proto格式，用于前后端传输的
impl From<proto_gen::map::line_curve::GeometryLine> for GeometryLine {
    fn from(value: proto_gen::map::line_curve::GeometryLine) -> Self {
        return Self::from(&value);
    }
}

impl From<&proto_gen::map::line_curve::GeometryLine> for GeometryLine {
    fn from(value: &proto_gen::map::line_curve::GeometryLine) -> Self {
        if value.straight.is_some() {
            return GeometryLine::Straight(StraightLine {
                hdg: value.straight.hdg,
                length: value.straight.length,
                s: value.straight.start_s,
                x: value.straight.position.x,
                y: value.straight.position.y,
            });
        } else if value.arc.is_some() {
            return GeometryLine::Arc(ArcLine {
                hdg: value.arc.hdg,
                length: value.arc.length,
                s: value.arc.start_s,
                x: value.arc.position.x,
                y: value.arc.position.y,
                curvature: value.arc.curvature,
            });
        }
        return GeometryLine::Straight(StraightLine {
            hdg: 0.0,
            length: 0.0,
            s: 0.0,
            x: 0.0,
            y: 0.0,
        });
    }
}

impl Into<proto_gen::map::line_curve::GeometryLine> for &GeometryLine {
    fn into(self) -> proto_gen::map::line_curve::GeometryLine {
        match self {
            GeometryLine::Straight(s) => {
                proto_gen::map::line_curve::GeometryLine {
                    straight: ::protobuf::MessageField::some(
                        proto_gen::map::line_curve::StraightLine {
                            hdg: s.hdg,
                            length: s.length,
                            start_s: s.s,
                            position: ::protobuf::MessageField::some(
                                proto_gen::map::LanePoint {
                                    x: s.x,
                                    y: s.y,
                                    z: 0.0,
                                    special_fields: Default::default(),
                                },
                            ),
                            special_fields: Default::default(),
                        },
                    ),
                    arc: ::protobuf::MessageField::none(),
                    spiral : ::protobuf::MessageField::none(),
                    special_fields: Default::default(),
                }
            }
            GeometryLine::Arc(a) => proto_gen::map::line_curve::GeometryLine {
                straight: ::protobuf::MessageField::none(),
                arc: ::protobuf::MessageField::some(
                    proto_gen::map::line_curve::ArcLine {
                        hdg: a.hdg,
                        length: a.length,
                        start_s: a.s,
                        position: ::protobuf::MessageField::some(
                            proto_gen::map::LanePoint {
                                x: a.x,
                                y: a.y,
                                z: 0.0,
                                special_fields: Default::default(),
                            },
                        ),
                        curvature: a.curvature,
                        special_fields: Default::default(),
                    },
                ),
                spiral : ::protobuf::MessageField::none(),
                special_fields: Default::default(),
            },
            GeometryLine::Spiral(v) => {
                proto_gen::map::line_curve::GeometryLine {
                    straight:  ::protobuf::MessageField::none(),
                    arc:  ::protobuf::MessageField::none(),
                    spiral : ::protobuf::MessageField::some(
                        proto_gen::map::line_curve::SpiralLine {
                            hdg: v.hdg,
                            length: v.length,
                            start_s: v.s,
                            position: ::protobuf::MessageField::some(
                                proto_gen::map::LanePoint {
                                    x: v.x,
                                    y: v.y,
                                    z: 0.0,
                                    special_fields: Default::default(),
                                },
                            ),
                            curvature_start: v.curvature_start,
                            curvature_end: v.curvature_end,
                            special_fields: Default::default(),
                        },
                    ),
                    special_fields: Default::default(),
                }
            }
        }
    }
}

/// 几何线的实现
impl GeometryLine {
    pub fn get_heading(&self, s: f64) -> f64 {
        match self {
            GeometryLine::Straight(this) => this.get_heading(s as f32) as f64,
            GeometryLine::Arc(this) => this.get_heading(s as f32) as f64,
            GeometryLine::Spiral(this) =>this.get_heading(s as f32) as f64,
        }
    }

    pub fn s(&self) -> f64 {
        match self {
            GeometryLine::Straight(this) => this.s as f64,
            GeometryLine::Arc(this) => this.s as f64,
            GeometryLine::Spiral(this) => this.s as f64,
        }
    }
    pub fn set_s(&mut self, s: f32) {
        match self {
            GeometryLine::Straight(this) => this.s = s,
            GeometryLine::Arc(this) => this.s = s,
            GeometryLine::Spiral(this) => this.s = s,
        }
    }

    pub fn length(&self) -> f64 {
        match self {
            GeometryLine::Straight(this) => this.length as f64,
            GeometryLine::Arc(this) => this.length as f64,
            GeometryLine::Spiral(this) => this.length as f64,
        }
    }
    pub fn get_smooth_point(&self, s: f64) -> LanePoint {
        match self {
            GeometryLine::Straight(this) => this.get_smooth_point(s as f32),
            GeometryLine::Arc(this) => this.get_smooth_point(s as f32),
            GeometryLine::Spiral(this) => this.get_smooth_point(s as f32),
        }
    }
    pub fn get_smooth_point_and_head(&self, s: f64) -> (LanePoint, f64) {
        match self {
            GeometryLine::Straight(this) => {
                this.get_smooth_point_and_head(s as f32)
            }
            GeometryLine::Arc(this) => this.get_smooth_point_and_head(s as f32),
            GeometryLine::Spiral(this) => this.get_smooth_point_and_head(s as f32),
        }
    }

    pub fn get_dense_point_of_range(&self, s: f32, e: f32, step: f32) -> Vec<LanePoint> {
        match self {
            GeometryLine::Straight(this) => this.get_dense_point_of_range(s, e,step),
            GeometryLine::Arc(this) => this.get_dense_point_of_range(s, e,step),
            GeometryLine::Spiral(this) => this.get_dense_point_of_range(s, e,step),
        }
    }

    // return （s l）
    pub fn get_projection(&self, x: f64, y: f64) -> (f64, f64) {
        match self {
            GeometryLine::Straight(this) => this.get_projection(x, y),
            GeometryLine::Arc(this) => this.get_projection(x, y),
            GeometryLine::Spiral(this) => this.get_projection(x, y),
        }
    }

    // (distance, s, l, projection_point)
    pub fn distance(&self, x: f64, y: f64) -> (f64, LanePoint) {
        match self {
            GeometryLine::Straight(this) => this.distance(x, y),
            GeometryLine::Arc(this) => this.distance(x, y),
            GeometryLine::Spiral(this) => this.distance(x, y),
        }
    }

    pub fn translation(&self, offset: f32) -> Self {
        match self {
            GeometryLine::Straight(this) => {
                GeometryLine::Straight(this.translation(offset))
            }
            GeometryLine::Arc(this) => {
                GeometryLine::Arc(this.translation(offset))
            }
            GeometryLine::Spiral(this) => GeometryLine::Spiral(this.translation(offset))
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            GeometryLine::Straight(this) => {
                GeometryLine::Straight(this.reverse())
            }
            GeometryLine::Arc(this) => GeometryLine::Arc(this.reverse()),
            GeometryLine::Spiral(this) => GeometryLine::Spiral(this.reverse())
        }
    }
}

impl Debug for GeometryLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GeometryLine::Straight(v) => {
                write!(
                    f,
                    "StraightLine  start:({}, {}), end:({}, {})",
                    v.x,
                    v.y,
                    v.x + (v.length * v.hdg.cos()) as f64,
                    v.y + (v.length * v.hdg.sin()) as f64,
                )
            }
            GeometryLine::Arc(v) => {
                let (
                    r,
                    to_center_direction,
                    center_x,
                    center_y,
                    start_arc,
                    end_arc,
                ) = v.base_info();
                write!(
                    f,
                    "ArcLine center:({}, {}),  start:({}, {}), end:({}, {}), hdg:{}",
                    center_x,
                    center_y,
                    v.x,
                    v.y,
                    center_x + (r * end_arc.cos()) as f64,
                    center_y + (r * end_arc.sin()) as f64,
                    v.hdg
                )
            }
            GeometryLine::Spiral(v) => {
                write!(
                    f,
                    "Spiral x,y:({}, {}), hdg:{}, length:{}, curvature:({}, {})",
                    v.x,
                    v.y,
                    v.hdg,
                    v.length,
                    v.curvature_start,
                    v.curvature_end
                )
            }
        }
    }
}

#[inline]
fn to_2pi(param: f64) -> f64 {
    let mut p = param;
    while p.is_sign_negative() {
        p = p + 2.0 * std::f64::consts::PI;
    }
    while p > 2.0 * std::f64::consts::PI {
        p = p - 2.0 * std::f64::consts::PI;
    }
    return p;
}

#[inline]
fn to_pi(param: f64) -> f64 {
    let mut p = param;
    while p > std::f64::consts::PI {
        p = p - 2.0 * std::f64::consts::PI;
    }
    while p < -std::f64::consts::PI {
        p = p + 2.0 * std::f64::consts::PI;
    }
    return p;
}
