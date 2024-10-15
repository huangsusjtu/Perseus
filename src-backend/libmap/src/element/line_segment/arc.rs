use crate::common::math::K_MATH_EPSILON;
use crate::element::line_segment::{to_pi, ArcLine, StraightLine};
use crate::LanePoint;
use std::f32::consts::PI;

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
        return to_pi((target_arc + self.curvature.signum() * PI / 2.0) as f64)
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

    pub fn get_dense_point_of_range(&self, s: f32, e: f32, step: f32,) -> Vec<LanePoint> {
        let (r, _, center_x, center_y, base_arc, _) = self.base_info();

        let start_arc =
            base_arc + self.curvature.signum() * (s.max(self.s) - self.s) / r; // 目标起点和X轴夹角
        let end_arc = base_arc
            + self.curvature.signum() * (e.min(self.s + self.length) - self.s)
                / r; // 目标终点和X轴夹角

        let mut ret = Vec::default();
        let step_arc: f32 = step/ r;
        if self.curvature.is_sign_positive() {
            let mut target_arc = start_arc;
            let end_arc = if end_arc < start_arc {
                end_arc + 2.0 * PI
            } else {
                end_arc
            };
            while target_arc < end_arc + step_arc {
                ret.push(LanePoint::new(
                    center_x + (r * target_arc.cos()) as f64,
                    center_y + (r * target_arc.sin()) as f64,
                ));
                target_arc = target_arc + step_arc;
            }
        } else {
            let mut target_arc = start_arc;
            let end_arc = if end_arc > start_arc {
                end_arc - 2.0 * PI
            } else {
                end_arc
            };
            while target_arc + step_arc > end_arc {
                ret.push(LanePoint::new(
                    center_x + (r * target_arc.cos()) as f64,
                    center_y + (r * target_arc.sin()) as f64,
                ));
                target_arc = target_arc - step_arc;
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
    pub fn base_info(&self) -> (f32, f32, f64, f64, f32, f32) {
        let r = 1.0 / self.curvature.abs(); // 半径
        let to_center_direction = self.hdg + self.curvature.signum() * PI / 2.0; // 圆弧起点往圆心的方向
                                                                                 // 圆心坐标
        let center_x = (to_center_direction.cos() * r) as f64 + self.x;
        let center_y = (to_center_direction.sin() * r) as f64 + self.y;

        let start_arc = (self.y - center_y).atan2(self.x - center_x); // 圆弧起点和X轴夹角
        let end_arc = to_pi(
            start_arc + (self.curvature.signum() * self.length / r) as f64,
        ); // 圆弧终点 和 圆弧起点 之间弧度夹角
        (
            r,
            to_center_direction,
            center_x,
            center_y,
            start_arc as f32,
            end_arc as f32,
        )
    }

    // 直线 往左右 移动, offset < 0 表示往左
    pub fn translation(&self, offset: f32) -> Self {
        let (r, _to_center_direction, center_x, center_y, start_arc, _end_arc) =
            self.base_info();
        let new_r = r + self.curvature.signum() * offset; // 新半径

        ArcLine {
            hdg: self.hdg,
            length: self.length * new_r / r,
            s: self.s,
            x: center_x + (new_r * start_arc.cos()) as f64,
            y: center_y + (new_r * start_arc.sin()) as f64,
            curvature: self.curvature.signum() * 1.0 / new_r,
        }
    }

    pub fn reverse(&self) -> Self {
        let (r, _to_center_direction, center_x, center_y, _start_arc, end_arc) =
            self.base_info();
        let hdg = to_pi((end_arc - self.curvature.signum() * PI / 2.0) as f64);
        let x = center_x + (r * end_arc.cos()) as f64;
        let y = center_y + (r * end_arc.sin()) as f64;
        ArcLine {
            hdg: hdg as f32,
            length: self.length,
            s: self.s,
            x,
            y,
            curvature: -self.curvature,
        }
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
        let a = line.get_dense_point_of_range(0.0, line.length, 0.1);
        let b = line2.get_dense_point_of_range(0.0, line2.length, 0.1);
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
            left_line.get_dense_point_of_range(0.0, left_line.length, 0.1);
        let right_points =
            right_line.get_dense_point_of_range(0.0, right_line.length, 0.1);

        println!("");
    }
}
