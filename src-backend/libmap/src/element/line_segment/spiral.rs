use crate::common::math::K_MATH_EPSILON;
use crate::element::line_segment::{to_pi, ArcLine, SpiralLine};
use crate::LanePoint;
use std::f32::consts::PI;

/// 螺旋线的实现
impl SpiralLine {
    pub fn get_heading(&self, s: f32) -> f32 {
        if s + K_MATH_EPSILON < self.s
            || s > self.s + self.length + K_MATH_EPSILON
        {
            panic!(
                "SpiralLine: get_smooth_point {}, {}, {}",
                self.s,
                s,
                self.s + self.length
            );
        }

        0.0
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

        LanePoint::new(0.0, 0.0)
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

        (LanePoint::new(0.0, 0.0), 0.0)
    }

    pub fn get_dense_point_of_range(&self, s: f32, e: f32, step: f32,) -> Vec<LanePoint> {
        vec![]
    }
    // return （s l）
    pub fn get_projection(&self, x: f64, y: f64) -> (f64, f64) {
        (0.0, 0.0)
    }

    // (distance, s, l, projection_point)
    pub fn distance(&self, x: f64, y: f64) -> (f64, LanePoint) {
        (0.0, LanePoint::new(0.0, 0.0))
    }

    // 直线 往左右 移动, offset < 0 表示往左
    pub fn translation(&self, offset: f32) -> Self {
        self.clone()
    }

    pub fn reverse(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::line_segment::GeometryLine;

    #[test]
    fn it_works_arc_1() {
        let line = GeometryLine::Arc(ArcLine {
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

    #[test]
    fn it_works_arc_2() {
        let line = ArcLine {
            hdg: (PI / 2.0) as f32,
            length: 2.0 * PI as f32,
            s: 0.0,
            x: 2.0,
            y: 0.0,
            curvature: 1.0 / 2.0,
        };
        let t = line.base_info();
        let line2 = line.translation(-1.0);
        let t2 = line2.base_info();
        let a = line.get_dense_point_of_range(0.0, line.length);
        let b = line2.get_dense_point_of_range(0.0, line2.length);
        println!("");
    }

    #[test]
    fn it_works_arc_3() {
        let line = ArcLine {
            // 圆心(0,0)，半径2， 上半部分
            hdg: (PI / 2.0) as f32,
            length: 2.0 * PI as f32,
            s: 0.0,
            x: 2.0,
            y: 0.0,
            curvature: 1.0 / 2.0,
        };
        let base = line.base_info();
        let r_a = line.reverse();
        println!("{:?}", line);
        println!("{:?}", r_a);
    }

    #[test]
    fn it_works_arc_4() {
        let line = ArcLine {
            // 圆心(0,0)，半径2， 上半部分
            hdg: PI as f32,
            length: 2.0 * PI as f32,
            s: 0.0,
            x: 0.0,
            y: 2.0,
            curvature: 1.0 / 2.0,
        };
        let base = line.base_info();
        let r_a = line.reverse();
        println!("{:?}", line);
        println!("{:?}", r_a);
    }

    #[test]
    fn it_works_arc_5() {
        let line = ArcLine {
            hdg: PI as f32,
            length: 100.0 * PI as f32,
            s: 0.0,
            x: 0.0,
            y: 0.0,
            curvature: 0.01,
        };
        let base = line.base_info();
        let r_a = line.reverse();
        println!("{:?}", line);
        println!("{:?}", r_a);

        let line2 = line.translation(-1.5);
        let base2 = line2.base_info();
        let r_a2 = line2.reverse();
        println!("{:?}", line2);
        println!("{:?}", r_a2);
    }

    #[test]
    fn it_works_arc_65() {
        let center_line = ArcLine {
            hdg: PI as f32,
            length: 309.5 as f32,
            s: 0.0,
            x: 0.0,
            y: -198.5,
            curvature: -0.0101522841,
        };

        let left_line = center_line.translation(-3.0 / 2.0);
        let right_line = center_line.translation(3.0 / 2.0);
        let left_points =
            left_line.get_dense_point_of_range(0.0, left_line.length);
        let right_points =
            right_line.get_dense_point_of_range(0.0, right_line.length);

        println!("");
    }
}
