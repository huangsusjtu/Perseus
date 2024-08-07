use crate::common::math::{cross_prod, is_within, K_MATH_EPSILON};
use crate::proto_gen;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn default() -> Self {
        return Vec2d { x: 0.0, y: 0.0 };
    }

    pub fn new(x: f64, y: f64) -> Self {
        return Vec2d { x: x, y: y };
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    #[inline]
    pub fn length(&self) -> f64 {
        return self.x.hypot(self.y);
    }
    #[inline]
    pub fn length_square(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }
    #[inline]
    pub fn angle(&self) -> f64 {
        return self.y.atan2(self.x);
    }
    #[inline]
    pub fn normalize(&mut self) {
        let l = self.length();
        self.x /= l;
        self.y /= l;
    }
    #[inline]
    pub fn distance(&self, other: &Vec2d) -> f64 {
        return (self.x - other.x).hypot(self.y - other.y);
    }

    #[inline]
    pub fn distance_xy(&self, x: f64, y: f64) -> f64 {
        return (self.x - x).hypot(self.y - y);
    }

    #[inline]
    pub fn distance_square(&self, other: &Vec2d) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        return dx * dx + dy * dy;
    }
    #[inline]
    pub fn cross_prod(&self, other: &Vec2d) -> f64 {
        return self.x * other.y - self.y * other.x;
    }
    #[inline]
    pub fn inner_prod(&self, other: &Vec2d) -> f64 {
        return self.x * other.x + self.y * other.y;
    }
    #[inline]
    pub fn rotate(&self, angle: f64) -> Vec2d {
        return Vec2d {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
        };
    }
}

impl std::fmt::Display for Vec2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vec2d x:{}, y:{}, ", self.x, self.y)
    }
}


impl From<&proto_gen::map::LanePoint> for Vec2d {
    fn from(value: &proto_gen::map::LanePoint) -> Self {
        return LanePoint {
            x: value.x,
            y: value.y,
        };
    }
}

impl Into<proto_gen::map::LanePoint> for &Vec2d {
    fn into(self) -> proto_gen::map::LanePoint {
        proto_gen::map::LanePoint {
            x: self.x,
            y: self.y,
            z: 0.0,
            special_fields: Default::default(),
        }
    }
}

pub type LanePoint = Vec2d;

//线段
#[derive(Debug, Clone)]
pub struct LineSegment2d {
    pub start_: LanePoint,
    pub end_: LanePoint,
    pub unit_direction_: LanePoint,
    pub length_: f64,
}

impl Default for LineSegment2d {
    fn default() -> Self {
        LineSegment2d {
            start_: LanePoint { x: 0.0, y: 0.0 },
            end_: LanePoint { x: 0.0, y: 0.0 },
            unit_direction_: LanePoint { x: 0.0, y: 0.0 },
            length_: 0.0,
        }
    }
}

#[allow(dead_code)]
impl LineSegment2d {
    pub fn new(start: Vec2d, end: Vec2d) -> Self {
        let x_l = end.x - start.x;
        let y_l = end.y - start.y;
        let length = x_l.hypot(y_l);
        let mut unit_direction = Vec2d::new(0.0, 0.0);
        if length > K_MATH_EPSILON as f64 {
            unit_direction.x = x_l / length;
            unit_direction.y = y_l / length;
        }

        LineSegment2d {
            start_: start.clone(),
            end_: end.clone(),
            unit_direction_: unit_direction,
            length_: length,
        }
    }
    #[inline]
    pub fn heading(&self) -> f64 {
        return Vec2d {
            x: self.end_.x - self.start_.x,
            y: self.end_.y - self.start_.y,
        }
            .angle();
    }
    #[inline]
    pub fn center(&self) -> Vec2d {
        return Vec2d {
            x: (self.start_.x + self.end_.x) / 2.0,
            y: (self.start_.y + self.end_.y) / 2.0,
        };
    }
    #[inline]
    pub fn rotate(&self, angle: f64) -> Vec2d {
        let vec = Vec2d {
            x: self.end_.x - self.start_.x,
            y: self.end_.y - self.start_.y,
        };
        vec.rotate(angle)
    }
    #[inline]
    pub fn cos_heading(&self) -> f64 {
        return self.unit_direction_.x;
    }
    #[inline]
    pub fn sin_heading(&self) -> f64 {
        return self.unit_direction_.y;
    }
    #[inline]
    pub fn length(&self) -> f64 {
        return self.length_;
    }
    #[inline]
    pub fn distance(&self, point: &Vec2d) -> f64 {
        if self.length_ <= K_MATH_EPSILON as f64 {
            return point.distance(&self.start_);
        }
        let x0 = point.x - self.start_.x;
        let y0 = point.y - self.start_.y;
        let proj = x0 * self.unit_direction_.x + y0 * self.unit_direction_.y;
        if proj <= 0.0 {
            return x0.hypot(y0);
        } else if proj >= self.length_ {
            return point.distance(&self.end_);
        }
        return (x0 * self.unit_direction_.y - y0 * self.unit_direction_.x).abs();
    }

    /* return (distance,  proj_point)*/
    pub fn distance_with_point(&self, point: &Vec2d) -> (f64, Vec2d) {
        if self.length_ <= K_MATH_EPSILON as f64 {
            return (
                point.distance(&self.start_),
                Vec2d {
                    x: self.start_.x,
                    y: self.start_.y,
                },
            );
        }
        let x0 = point.x - self.start_.x;
        let y0 = point.y - self.start_.y;
        let proj = x0 * self.unit_direction_.x + y0 * self.unit_direction_.y;
        if proj <= 0.0 {
            return (
                x0.hypot(y0),
                Vec2d {
                    x: self.start_.x,
                    y: self.start_.y,
                },
            );
        } else if proj >= self.length_ {
            return (
                point.distance(&self.end_),
                Vec2d {
                    x: self.end_.x,
                    y: self.end_.y,
                },
            );
        }
        return (
            (x0 * self.unit_direction_.y - y0 * self.unit_direction_.x).abs(),
            Vec2d {
                x: self.start_.x + self.unit_direction_.x * proj,
                y: self.start_.y + self.unit_direction_.y * proj,
            },
        );
    }

    pub fn distance_square(&self, point: &Vec2d) -> f64 {
        if self.length_ <= K_MATH_EPSILON as f64 {
            return point.distance_square(&self.start_);
        }
        let x0 = point.x - self.start_.x;
        let y0 = point.y - self.start_.y;
        let proj = x0 * self.unit_direction_.x + y0 * self.unit_direction_.y;
        if proj <= 0.0 {
            return x0 * x0 + y0 * y0;
        } else if proj >= self.length_ {
            return point.distance_square(&self.end_);
        }
        let r = x0 * self.unit_direction_.y - y0 * self.unit_direction_.x;
        return r * r;
    }

    pub fn is_point_in(&self, point: &Vec2d) -> bool {
        if self.length_ < K_MATH_EPSILON as f64 {
            return (point.x - self.start_.x).abs() <= K_MATH_EPSILON as f64
                && (point.y - self.start_.y).abs() <= K_MATH_EPSILON as f64;
        }

        let crossprod = cross_prod(
            self.start_.x - point.x,
            self.start_.y - point.y,
            self.end_.x - point.x,
            self.end_.y - point.y,
        );
        if crossprod > K_MATH_EPSILON as f64 {
            return false;
        }

        return is_within(point.x, self.start_.x, self.end_.x)
            && is_within(point.y, self.start_.y, self.end_.y);
    }

    #[inline]
    pub fn project_and_product_on_unit(&self, point: &Vec2d) -> (f64, f64) {
        let other = &Vec2d {
            x: (point.x - self.start_.x),
            y: (point.y - self.start_.y),
        };
        return (
            self.unit_direction_.inner_prod(other),
            self.unit_direction_.cross_prod(other),
        );
    }
    #[inline]
    pub fn has_intersect(&self, other: &LineSegment2d) -> bool {
        return self.get_intersect(other).0;
    }

    pub fn get_intersect(&self, other: &LineSegment2d) -> (bool, Vec2d) {
        let mut res = Vec2d::new(0.0, 0.0);
        if self.is_point_in(&other.start_) {
            return (true, other.start_.clone());
        }
        if self.is_point_in(&other.end_) {
            return (true, other.end_.clone());
        }
        if other.is_point_in(&self.start_) {
            return (true, self.start_.clone());
        }
        if other.is_point_in(&self.end_) {
            return (true, self.end_.clone());
        }
        if self.length_ <= K_MATH_EPSILON as f64 || other.length() <= K_MATH_EPSILON as f64 {
            return (false, res);
        }

        let cc1 = cross_prod(
            &self.end_.x - &self.start_.x,
            &self.end_.y - &self.start_.y,
            other.start_.x - &self.start_.x,
            other.start_.y - &self.start_.y,
        );
        let cc2 = cross_prod(
            &self.end_.x - &self.start_.x,
            &self.end_.y - &self.start_.y,
            other.end_.x - &self.start_.x,
            other.end_.y - &self.start_.y,
        );
        if cc1 * cc2 >= -K_MATH_EPSILON as f64 {
            return (false, res);
        }
        let cc3 = cross_prod(
            other.end_.x - other.start_.x,
            other.end_.y - other.start_.y,
            &self.start_.x - other.start_.x,
            &self.start_.y - other.start_.y,
        );
        let cc4 = cross_prod(
            other.end_.x - other.start_.x,
            other.end_.y - other.start_.y,
            &self.end_.x - other.start_.x,
            &self.end_.y - other.start_.y,
        );
        if cc3 * cc4 >= -K_MATH_EPSILON as f64 {
            return (false, res);
        }
        let ratio = cc4 / (cc4 - cc3);
        res.x = &self.start_.x * ratio + &self.end_.x * (1.0 - ratio);
        res.y = &self.start_.y * ratio + &self.end_.y * (1.0 - ratio);
        return (true, res);
    }

    pub fn get_vec2d_in_s(&self, s: f64) -> Vec2d {
        let ratio = s / self.length_;
        let x = (self.end_.x - self.start_.x) * ratio + self.start_.x;
        let y = (self.end_.y - self.start_.y) * ratio + self.start_.y;
        return Vec2d { x: x, y: y };
    }

    /// 返回当前线段起始点坐标
    #[inline]
    pub fn start_point(&self) -> Vec2d {
        Vec2d {
            x: self.start_.x,
            y: self.start_.y,
        }
    }

    /// 返回当前线段截止点坐标
    #[inline]
    pub fn end_point(&self) -> Vec2d {
        Vec2d {
            x: self.end_.x,
            y: self.end_.y,
        }
    }
}

impl std::fmt::Display for LineSegment2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "start_:{}, end_:{}, length {}",
            self.start_, self.end_, self.length_,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{LineSegment2d, Vec2d};

    #[test]
    fn it_works() {
        let segment1 = LineSegment2d::new(Vec2d { x: 0.0, y: 10.0 }, Vec2d { x: 0.0, y: 20.0 });
        let segment2 = LineSegment2d::new(
            Vec2d { x: -10.0, y: 10.0 },
            Vec2d { x: 10.0, y: 10.0 },
        );

        let (has, result) = segment1.get_intersect(&segment2);
        assert!(has == true);
        assert!(result.x == 0.0 && result.y == 10.0);
    }
}
