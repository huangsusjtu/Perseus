use crate::common::math::{cross_prod, inner_prod, K_MATH_EPSILON};
use crate::element::line_segment::{to_pi, ArcLine, StraightLine};
use crate::LanePoint;
use std::f32::consts::PI;

/// 直线的实现
impl StraightLine {
    pub fn get_heading(&self, _s: f32) -> f32 {
        self.hdg
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
        LanePoint {
            x: self.x + ((s - self.s) * self.hdg.cos()) as f64,
            y: self.y + ((s - self.s) * self.hdg.sin()) as f64,
        }
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
        (
            LanePoint {
                x: self.x + ((s - self.s) * self.hdg.cos()) as f64,
                y: self.y + ((s - self.s) * self.hdg.sin()) as f64,
            },
            self.hdg as f64,
        )
    }

    pub fn get_dense_point_of_range(
        &self, s: f32, e: f32, step: f32,
    ) -> Vec<LanePoint> {
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

        if step < 0.1 {
            let mut result = Vec::with_capacity(2);
            result.push(LanePoint {
                x: self.x + ((s - self.s) * self.hdg.cos()) as f64,
                y: self.y + ((s - self.s) * self.hdg.sin()) as f64,
            });
            result.push(LanePoint {
                x: self.x + ((e - self.s) * self.hdg.cos()) as f64,
                y: self.y + ((e - self.s) * self.hdg.sin()) as f64,
            });
            result
        } else {
            let mut result = vec![];
            let mut t = s;
            while t < e {
                result.push(LanePoint {
                    x: self.x + ((t - self.s) * self.hdg.cos()) as f64,
                    y: self.y + ((t - self.s) * self.hdg.sin()) as f64,
                });
                t = t + step;
            }
            result.push(LanePoint {
                x: self.x + ((e - self.s) * self.hdg.cos()) as f64,
                y: self.y + ((e - self.s) * self.hdg.sin()) as f64,
            });
            result
        }
    }

    // return （s l）
    pub fn get_projection(&self, x: f64, y: f64) -> (f64, f64) {
        let (sin, cos) = (self.hdg as f64).sin_cos();
        let x0 = x - self.x;
        let y0 = y - self.y;
        let proj = inner_prod(cos, sin, x0, y0);
        let prod = cross_prod(cos, sin, x0, y0);

        (proj, prod)
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

        (
            (x0 * sin - y0 * cos).abs(),
            LanePoint::new(self.x + cos * proj, self.y + sin * proj),
        )
    }

    // 直线 往左右 移动, offset < 0 表示往左
    pub fn translation(&self, offset: f32) -> Self {
        let (sin, cos) = (self.hdg - offset.signum() * PI / 2.0).sin_cos();
        let n_x = self.x + (cos * offset.abs()) as f64;
        let n_y = self.y + (sin * offset.abs()) as f64;
        StraightLine {
            hdg: self.hdg,
            length: self.length,
            s: self.s,
            x: n_x,
            y: n_y,
        }
    }

    pub fn reverse(&self) -> Self {
        StraightLine {
            hdg: to_pi(self.hdg as f64 + std::f64::consts::PI) as f32,
            length: self.length,
            s: self.s,
            x: self.x + self.length as f64 * self.hdg.cos() as f64,
            y: self.y + self.length as f64 * self.hdg.sin() as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::line_segment::GeometryLine;

    #[test]
    fn it_works_staight_1() {
        let line = GeometryLine::Straight(StraightLine {
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
}
