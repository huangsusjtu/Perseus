use std::f32::consts::PI;

use libformat::opendrive::common::planview::{Arc, Line};

use crate::common::math::{cross_prod, inner_prod, K_MATH_EPSILON};
use crate::common::util::LanePoint;
use crate::proto_gen;

///  地图上 2种不同的 线段
#[derive(Debug, Clone)]
pub enum GeometryLine {
    Straight(StraightLine),
    Arc(ArcLine),
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
    pub hdg: f32,
    pub length: f32,
    pub s: f32,
    pub x: f64,
    pub y: f64,
    pub curvature: f32,
}

///  opendrive 格式的对接
impl From<&libformat::opendrive::common::planview::Geometry> for GeometryLine {
    fn from(value: &libformat::opendrive::common::planview::Geometry) -> Self {
        if value.line.is_some() {
            return GeometryLine::Straight(StraightLine {
                hdg: value.hdg,
                length: value.length,
                s: value.s,
                x: value.x,
                y: value.y,
            });
        } else if value.arc.is_some() {
            return GeometryLine::Arc(ArcLine {
                hdg: value.hdg,
                length: value.length,
                s: value.s,
                x: value.x,
                y: value.y,
                curvature: value.arc.as_ref().unwrap().curvature,
            });
        } else {
            // never here
            return GeometryLine::Straight(StraightLine {
                hdg: value.hdg,
                length: value.length,
                s: value.s,
                x: value.x,
                y: value.y,
            });
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
                    straight: ::protobuf::MessageField::some(proto_gen::map::line_curve::StraightLine {
                        hdg: s.hdg,
                        length: s.length,
                        start_s: s.s,
                        position: ::protobuf::MessageField::some(proto_gen::map::LanePoint {
                            x: s.x,
                            y: s.y,
                            z: 0.0,
                            special_fields: Default::default(),
                        }),
                        special_fields: Default::default(),
                    }),
                    arc: ::protobuf::MessageField::none(),
                    special_fields: Default::default(),
                }
            }
            GeometryLine::Arc(a) => proto_gen::map::line_curve::GeometryLine {
                straight: ::protobuf::MessageField::none(),
                arc: ::protobuf::MessageField::some(proto_gen::map::line_curve::ArcLine {
                    hdg: a.hdg,
                    length: a.length,
                    start_s: a.s,
                    position: ::protobuf::MessageField::some(proto_gen::map::LanePoint {
                        x: a.x,
                        y: a.y,
                        z: 0.0,
                        special_fields: Default::default(),
                    }),
                    curvature: a.curvature,
                    special_fields: Default::default(),
                }),
                special_fields: Default::default(),
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
        }
    }

    pub fn s(&self) -> f64 {
        match self {
            GeometryLine::Straight(this) => this.s as f64,
            GeometryLine::Arc(this) => this.s as f64,
        }
    }
    pub fn length(&self) -> f64 {
        match self {
            GeometryLine::Straight(this) => this.length as f64,
            GeometryLine::Arc(this) => this.length as f64,
        }
    }
    pub fn get_smooth_point(&self, s: f64) -> LanePoint {
        match self {
            GeometryLine::Straight(this) => this.get_smooth_point(s as f32),
            GeometryLine::Arc(this) => this.get_smooth_point(s as f32),
        }
    }
    pub fn get_smooth_point_and_head(&self, s: f64) -> (LanePoint, f64) {
        match self {
            GeometryLine::Straight(this) => {
                this.get_smooth_point_and_head(s as f32)
            }
            GeometryLine::Arc(this) => this.get_smooth_point_and_head(s as f32),
        }
    }

    pub fn get_dense_point_of_range(&self, s: f32, e: f32) -> Vec<LanePoint> {
        match self {
            GeometryLine::Straight(this) => this.get_dense_point_of_range(s, e),
            GeometryLine::Arc(this) => this.get_dense_point_of_range(s, e),
        }
    }

    // return （s l）
    pub fn get_projection(&self, x: f64, y: f64) -> (f64, f64) {
        match self {
            GeometryLine::Straight(this) => this.get_projection(x, y),
            GeometryLine::Arc(this) => this.get_projection(x, y),
        }
    }

    // (distance, s, l, projection_point)
    pub fn distance(&self, x: f64, y: f64) -> (f64, LanePoint) {
        match self {
            GeometryLine::Straight(this) => this.distance(x, y),
            GeometryLine::Arc(this) => this.distance(x, y),
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
        }
    }
}

/// 直线的实现
impl StraightLine {
    pub fn get_heading(&self, _s: f32) -> f32 {
        return self.hdg;
    }

    pub fn get_smooth_point(&self, s: f32) -> LanePoint {
        if s + K_MATH_EPSILON < self.s
            || s > self.s + self.length + K_MATH_EPSILON
        {
            panic!(
                "StraightLine:get_smooth_point {}, {}, {}",
                self.s,
                s,
                self.s + self.length
            );
        }
        return LanePoint {
            x: self.x + ((s - self.s) * self.hdg.cos()) as f64,
            y: self.y + ((s - self.s) * self.hdg.sin()) as f64,
        };
    }
    pub fn get_smooth_point_and_head(&self, s: f32) -> (LanePoint, f64) {
        if s + K_MATH_EPSILON < self.s
            || s > self.s + self.length + K_MATH_EPSILON
        {
            panic!(
                "StraightLine:get_smooth_point_and_head {}, {}, {}",
                self.s,
                s,
                self.s + self.length
            );
        }
        return (
            LanePoint {
                x: self.x + ((s - self.s) * self.hdg.cos()) as f64,
                y: self.y + ((s - self.s) * self.hdg.sin()) as f64,
            },
            self.hdg as f64,
        );
    }

    pub fn get_dense_point_of_range(&self, s: f32, e: f32) -> Vec<LanePoint> {
        if s + K_MATH_EPSILON < self.s
            || e > self.s + self.length + K_MATH_EPSILON
        {
            panic!(
                "StraightLine:get_smooth_point_and_head {}, {}, {}",
                self.s,
                s,
                self.s + self.length
            );
        }

        let mut result = Vec::with_capacity(2);
        result.push(LanePoint {
            x: self.x + ((s - self.s) * self.hdg.cos()) as f64,
            y: self.y + ((s - self.s) * self.hdg.sin()) as f64,
        });
        result.push(LanePoint {
            x: self.x + ((e - self.s) * self.hdg.cos()) as f64,
            y: self.y + ((e - self.s) * self.hdg.sin()) as f64,
        });
        return result;
    }

    // return （s l）
    pub fn get_projection(&self, x: f64, y: f64) -> (f64, f64) {
        let (sin, cos) = (self.hdg as f64).sin_cos();
        let x0 = x - self.x;
        let y0 = y - self.y;
        let proj = inner_prod(cos, sin, x0, y0);
        let prod = cross_prod(cos, sin, x0, y0);

        return (proj, prod);
    }

    // (distance, projection_point)
    pub fn distance(&self, x: f64, y: f64) -> (f64, LanePoint) {
        let (sin, cos) = (self.hdg as f64).sin_cos();
        let x0 = x - self.x;
        let y0 = y - self.y;
        let proj = x0 * cos + y0 * sin;
        if proj.is_sign_negative() {
            return (x0.hypot(y0), LanePoint::new(self.x, self.y));
        } else if proj > self.length as f64 {
            return (x0.hypot(y0), LanePoint::new(self.x, self.y));
        }

        return (
            (x0 * sin - y0 * cos).abs(),
            LanePoint::new(self.x + cos * proj, self.y + sin * proj),
        );
    }

    // 直线 往左右 移动, offset < 0 表示往左
    pub fn translation(&self, offset: f32) -> Self {
        let (sin, cos) = (self.hdg - offset.signum() * PI / 2.0).sin_cos();
        let n_x = self.x + (cos * offset.abs()) as f64;
        let n_y = self.y + (sin * offset.abs()) as f64;
        return StraightLine {
            hdg: self.hdg,
            length: self.length,
            s: self.s,
            x: n_x,
            y: n_y,
        };
    }
}

/// 圆弧线的实现
impl ArcLine {
    pub fn get_heading(&self, s: f32) -> f32 {
        if s + K_MATH_EPSILON < self.s
            || s > self.s + self.length + K_MATH_EPSILON
        {
            panic!(
                "StraightLine:get_smooth_point {}, {}, {}",
                self.s,
                s,
                self.s + self.length
            );
        }

        let (r, _to_center_direction, center_x, center_y, _start_arc, _end_arc) =
            self.base_info();
        let arc = (s - self.s) / r; // 目标点 和 起点 之间弧度夹角
        let target_arc = (self.y - center_y).atan2(self.x - center_x) as f32
            + self.curvature.signum() * arc;
        return to_2pi((target_arc + self.curvature.signum() * PI / 2.0) as f64)
            as f32;
    }

    pub fn get_smooth_point(&self, s: f32) -> LanePoint {
        if s + K_MATH_EPSILON < self.s
            || s > self.s + self.length + K_MATH_EPSILON
        {
            panic!(
                "StraightLine:get_smooth_point {}, {}, {}",
                self.s,
                s,
                self.s + self.length
            );
        }
        let (r, _to_center_direction, center_x, center_y, _start_arc, _end_arc) =
            self.base_info();

        let arc = (s - self.s) / r; // 目标点 和 起点 之间弧度夹角
        let target_arc = (self.y - center_y).atan2(self.x - center_x) as f32
            + self.curvature.signum() * arc;

        return LanePoint::new(
            center_x + (r * target_arc.cos()) as f64,
            center_y + (r * target_arc.sin()) as f64,
        );
    }

    pub fn get_smooth_point_and_head(&self, s: f32) -> (LanePoint, f64) {
        if s + K_MATH_EPSILON < self.s
            || s > self.s + self.length + K_MATH_EPSILON
        {
            panic!(
                "StraightLine:get_smooth_point {}, {}, {}",
                self.s,
                s,
                self.s + self.length
            );
        }

        let (r, _to_center_direction, center_x, center_y, _start_arc, _end_arc) =
            self.base_info();

        let arc = (s - self.s) / r; // 目标点 和 起点 之间弧度夹角
        let target_arc = (self.y - center_y).atan2(self.x - center_x) as f32
            + self.curvature.signum() * arc;

        return (
            LanePoint::new(
                center_x + (r * target_arc.cos()) as f64,
                center_y + (r * target_arc.sin()) as f64,
            ),
            (target_arc + self.curvature.signum() * PI / 2.0) as f64,
        );
    }

    pub fn get_dense_point_of_range(&self, s: f32, e: f32) -> Vec<LanePoint> {
        let (r, _, center_x, center_y, base_arc, _) = self.base_info();

        let start_arc =
            base_arc + self.curvature.signum() * (s.max(self.s) - self.s) / r; // 目标起点和X轴夹角
        let end_arc = base_arc
            + self.curvature.signum() * (e.min(self.s + self.length) - self.s)
            / r; // 目标终点和X轴夹角

        let mut ret = Vec::default();
        if self.curvature.is_sign_positive() {
            let mut target_arc = start_arc;
            let end_arc = if end_arc < start_arc {
                end_arc + 2.0 * PI
            } else {
                end_arc
            };
            while target_arc < end_arc {
                ret.push(LanePoint::new(
                    center_x + (r * target_arc.cos()) as f64,
                    center_y + (r * target_arc.sin()) as f64,
                ));
                target_arc = target_arc + 0.2 / r;
            }
        } else {
            let mut target_arc = start_arc;
            let end_arc = if end_arc > start_arc {
                end_arc - 2.0 * PI
            } else {
                end_arc
            };
            while target_arc > end_arc {
                ret.push(LanePoint::new(
                    center_x + (r * target_arc.cos()) as f64,
                    center_y + (r * target_arc.sin()) as f64,
                ));
                target_arc = target_arc - 0.2 / r;
            }
        }
        return ret;
    }
    // return （s l）
    pub fn get_projection(&self, x: f64, y: f64) -> (f64, f64) {
        let (r, _to_center_direction, center_x, center_y, start_arc, end_arc) =
            self.base_info();
        let target_arc = (y - center_y).atan2(x - center_x) as f32;

        let s = if self.curvature.is_sign_positive() {
            if end_arc > start_arc {
                target_arc - start_arc
            } else {
                if target_arc > start_arc {
                    target_arc - start_arc
                } else {
                    target_arc - start_arc + 2.0 * PI
                }
            }
        } else {
            // 顺时针
            if start_arc > end_arc {
                -(target_arc - start_arc)
            } else {
                if target_arc < start_arc {
                    -(target_arc - start_arc)
                } else {
                    target_arc - start_arc - 2.0 * PI
                }
            }
        };
        let s = s * r;
        let l = (center_x + (r * target_arc.cos()) as f64 - x)
            .hypot(center_y + (r * target_arc.sin()) as f64 - y);
        return (s as f64, l);
    }

    // (distance, s, l, projection_point)
    pub fn distance(&self, x: f64, y: f64) -> (f64, LanePoint) {
        let (r, _to_center_direction, center_x, center_y, start_arc, end_arc) =
            self.base_info();
        let target_arc = (y - center_y).atan2(x - center_x) as f32;

        let s = if self.curvature.is_sign_positive() {
            if end_arc > start_arc {
                target_arc - start_arc
            } else {
                if target_arc > start_arc {
                    target_arc - start_arc
                } else {
                    target_arc - start_arc + 2.0 * PI
                }
            }
        } else {
            // 顺时针
            if start_arc > end_arc {
                -(target_arc - start_arc)
            } else {
                if target_arc < start_arc {
                    -(target_arc - start_arc)
                } else {
                    target_arc - start_arc - 2.0 * PI
                }
            }
        };
        let _s = s * r;
        let l = (center_x + (r * target_arc.cos()) as f64 - x)
            .hypot(center_y + (r * target_arc.sin()) as f64 - y);
        return (
            l,
            LanePoint::new(
                center_x + (r * target_arc.cos()) as f64,
                center_y + (r * target_arc.sin()) as f64,
            ),
        );
    }

    // 算一些基本信息
    #[inline]
    fn base_info(&self) -> (f32, f32, f64, f64, f32, f32) {
        let r = 1.0 / self.curvature.abs(); // 半径
        let to_center_direction =
            self.hdg + self.curvature.signum() * PI / 2.0; // 圆弧起点往圆心的方向
        // 圆心坐标
        let center_x = (to_center_direction.cos() * r) as f64 + self.x;
        let center_y = (to_center_direction.sin() * r) as f64 + self.y;

        let start_arc = (self.y - center_y).atan2(self.x - center_x); // 圆弧起点和X轴夹角
        let end_arc =
            to_2pi(start_arc + (self.curvature.signum() * self.length / r) as f64); // 圆弧终点 和 圆弧起点 之间弧度夹角
        return (
            r,
            to_center_direction,
            center_x,
            center_y,
            start_arc as f32,
            end_arc as f32,
        );
    }

    // 直线 往左右 移动, offset < 0 表示往左
    pub fn translation(&self, offset: f32) -> Self {
        let (r, _to_center_direction, center_x, center_y, start_arc, _end_arc) =
            self.base_info();
        let new_r = r + self.curvature.signum() * offset; // 新半径

        return ArcLine {
            hdg: self.hdg,
            length: self.length * new_r / r,
            s: self.s,
            x: center_x + (new_r * start_arc.cos()) as f64,
            y: center_y + (new_r * start_arc.sin()) as f64,
            curvature: self.curvature,
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_staight_1() {
        let line = crate::element::line::GeometryLine::Straight(StraightLine {
            hdg: 4f64.atan2(3.0) as f32,
            length: 5.0,
            s: 0.0,
            x: 0.0,
            y: 0.0,
        });

        let r = line.get_smooth_point(1.0);
        let r = line.get_smooth_point(2.0);
        let r = line.get_smooth_point(3.0);
        let r = line.get_smooth_point(4.0);
        let r = line.get_smooth_point(5.0);
        let r = line.translation(-1.0);
        let r = line.translation(1.0);

        let r = line.get_projection(0.0, 1.0);
        let r = line.get_projection(0.0, 2.0);
        let r = line.get_projection(0.0, 3.0);
        let r = line.get_projection(0.0, 4.0);
        // line.get_dense_point_of_range();
        // line.get_smooth_point_and_head();

        let d = line.distance(0.0, 0.0);
        let d = line.distance(0.0, 1.0);
        let d = line.distance(0.0, 2.0);
        let d = line.distance(0.0, 3.0);
        let d = line.distance(0.0, 4.0);
    }

    #[test]
    fn it_works_arc_1() {
        let line = crate::element::line::GeometryLine::Arc(ArcLine {
            hdg: (PI / 2.0) as f32,
            length: (2.0 * PI) as f32,
            s: 0.0,
            x: -1.0,
            y: 0.0,
            curvature: -1.0,
        });

        let r = line.get_smooth_point(0.0);
        let r = line.get_smooth_point(1.0);
        let r = line.get_smooth_point(2.0);
        let r = line.get_smooth_point(3.0);
        let r = line.get_smooth_point(4.0);
        let r = line.get_smooth_point(5.0);
        let r = line.translation(-1.0);
        let r = line.translation(1.0);

        let r = line.get_projection(0.0, 1.0);
        let r = line.get_projection(0.0, 2.0);
        let r = line.get_projection(0.0, 3.0);
        let r = line.get_projection(0.0, 4.0);
        // line.get_dense_point_of_range();
        // line.get_smooth_point_and_head();
    }
}
