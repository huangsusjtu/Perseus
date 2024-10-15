use std::f32::consts::PI;

use crate::common::math::{radian_of_two_radian, to_2pi};
use crate::common::util::LanePoint;
use crate::element::{LaneInfo, LaneType};

use crate::element::LineCurveInfo;
use crate::element::{RoadInfo, RoadType};
use crate::map::SDMap;

use crate::StraightLine;

const K_MATH_EPSILON: f64 = 0.01;

/// 给定一系列的点， 做一个平滑的车道线
/// 车道线是 直线和圆弧组成

impl From<&[LanePoint]> for LineCurveInfo {
    fn from(points: &[LanePoint]) -> Self {
        let mut lines: Vec<crate::GeometryLine> = Vec::new();

        let mut start_point = points[0].clone();
        let mut s = 0.0;
        for i in 1..points.len() - 1 {
            let hdg1 = (points[i - 1].y - points[i].y)
                .atan2(points[i - 1].x - points[i].x)
                as f32;
            let len1 = (points[i - 1].y - points[i].y)
                .hypot(points[i - 1].x - points[i].x)
                as f32;
            let hdg2 = (points[i + 1].y - points[i].y)
                .atan2(points[i + 1].x - points[i].x)
                as f32;
            let len2 = (points[i + 1].y - points[i].y)
                .hypot(points[i + 1].x - points[i].x)
                as f32;

            if (hdg1 + hdg2 - 2.0 * PI).abs() < 0.01 {
                // 一条直线， 所以中间这个点就忽略掉了
            } else {
                // 对于 三个点不在一条直线的情况，  前面有一条直线， 中间是
                // 圆弧线， 后续的情况根据后续的点决定
                // 1. 直线
                let trip_len = (if i == 1 { len1 } else { len1 / 2.0 }).min(
                    if i == points.len() - 2 {
                        len2
                    } else {
                        len2 / 2.0
                    },
                );
                let straight_line_end = LanePoint::new(
                    points[i].x + (trip_len * hdg1.cos()) as f64,
                    points[i].y + (trip_len * hdg1.sin()) as f64,
                );
                let straight_line_len =
                    start_point.distance(&straight_line_end) as f32;
                if straight_line_len > K_MATH_EPSILON as f32 {
                    let line = crate::GeometryLine::Straight(StraightLine {
                        hdg: to_2pi(hdg1 + PI),
                        length: straight_line_len,
                        s: s,
                        x: start_point.x,
                        y: start_point.y,
                    });
                    s += straight_line_len;
                    lines.push(line);
                    start_point = straight_line_end;
                }
                // 2. 圆弧线
                let jiajiao =
                    radian_of_two_radian(hdg1 as f64, hdg2 as f64) as f32;
                let r = trip_len * (jiajiao / 2.0).tan(); // 半径
                let len_of_radian = r * (PI - jiajiao);
                let radian = crate::GeometryLine::Arc(crate::ArcLine {
                    hdg: to_2pi(hdg1 + PI),
                    length: len_of_radian,
                    s,
                    x: start_point.x,
                    y: start_point.y,
                    curvature: 1.0 / r,
                });
                start_point = LanePoint::new(
                    points[i].x + (trip_len * hdg2.cos()) as f64,
                    points[i].y + (trip_len * hdg2.sin()) as f64,
                );
                lines.push(radian);
                s += len_of_radian;
            }
        }
        // 3. 最后一个圆弧的端点 到 最后一个点 之间是直线
        let end_point = points[points.len() - 1].clone();
        let straight_line_len = start_point.distance(&end_point);
        if straight_line_len > K_MATH_EPSILON {
            let line = crate::GeometryLine::Straight(StraightLine {
                hdg: (end_point.y - start_point.y)
                    .atan2(end_point.x - start_point.x)
                    as f32,
                length: straight_line_len as f32,
                s,
                x: start_point.x,
                y: start_point.y,
            });
            lines.push(line);
        }

        return LineCurveInfo::new(lines);
    }
}

impl SDMap {
    pub fn build_lane(
        &mut self, index: i32, r#type: LaneType,
        central_lane_curve: LineCurveInfo, width: f64,
    ) -> LaneInfo {
        LaneInfo {
            id: index,
            r#type,
            central_lane_curve,
            width: width as f32,
        }
    }

    pub fn build_road(&mut self, points: &[LanePoint]) -> RoadInfo {
        let center_curve = LineCurveInfo::from(points);
        const DRIVE_LANE_WIDTH: f64 = 3.8;
        const SHOULDER_LANE_WIDTH: f64 = 0.4;
        const WALK_LANE_WIDTH: f64 = 2.0;
        let left_drive_lane_center =
            center_curve.translation(-DRIVE_LANE_WIDTH / 2.0);
        let left_shoulder_lane_center = center_curve
            .translation(-DRIVE_LANE_WIDTH - SHOULDER_LANE_WIDTH / 2.0);
        let left_side_lane_center = center_curve.translation(
            -DRIVE_LANE_WIDTH - SHOULDER_LANE_WIDTH - WALK_LANE_WIDTH / 2.0,
        );

        let right_drive_lane_center =
            center_curve.translation(DRIVE_LANE_WIDTH / 2.0);
        let right_shoulder_lane_center = center_curve
            .translation(DRIVE_LANE_WIDTH + SHOULDER_LANE_WIDTH / 2.0);
        let right_side_lane_center = center_curve.translation(
            DRIVE_LANE_WIDTH + SHOULDER_LANE_WIDTH + WALK_LANE_WIDTH / 2.0,
        );
        let i = self.next_road_id;
        self.next_road_id += 1;
        RoadInfo {
            id: i,
            name: format!("Road-{}", i),
            length: center_curve.length,
            r#type: RoadType::Major,
            center_line: center_curve,
            left_lanes: vec![
                self.build_lane(
                    1,
                    LaneType::CityDriving,
                    left_drive_lane_center,
                    DRIVE_LANE_WIDTH,
                ),
                self.build_lane(
                    2,
                    LaneType::Shoulder,
                    left_shoulder_lane_center,
                    SHOULDER_LANE_WIDTH,
                ),
                self.build_lane(
                    3,
                    LaneType::SideWalk,
                    left_side_lane_center,
                    WALK_LANE_WIDTH,
                ),
            ],
            right_lanes: vec![
                self.build_lane(
                    -1,
                    LaneType::CityDriving,
                    right_drive_lane_center,
                    DRIVE_LANE_WIDTH,
                ),
                self.build_lane(
                    -2,
                    LaneType::Shoulder,
                    right_shoulder_lane_center,
                    SHOULDER_LANE_WIDTH,
                ),
                self.build_lane(
                    -3,
                    LaneType::SideWalk,
                    right_side_lane_center,
                    WALK_LANE_WIDTH,
                ),
            ],
            link: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let curve = LineCurveInfo::from(
            vec![
                LanePoint::new(0.0, 0.0),
                LanePoint::new(10.0, 0.0),
                LanePoint::new(10.0, 10.0),
            ]
                .as_slice(),
        );

        let mut ps = vec![];
        let mut s = 0.0;
        while s < curve.length {
            let p = curve.get_smooth_point(s);
            ps.push(p);
            s += 0.1;
        }
        // crate::common::plotter::draw_points("1", "line curve", &ps);
    }

    #[test]
    fn it_works_2() {
        let curve = LineCurveInfo::from(
            vec![LanePoint::new(0.0, 0.0), LanePoint::new(10.0, 10.0)]
                .as_slice(),
        );

        let mut ps = vec![];
        let mut s = 0.0;
        while s < curve.length {
            let p = curve.get_smooth_point(s);
            ps.push(p);
            s += 0.1;
        }
        // crate::common::plotter::draw_points("2", "line curve", &ps);
    }

    #[test]
    fn it_works_3() {
        let curve = LineCurveInfo::from(
            vec![
                LanePoint::new(0.0, 0.0),
                LanePoint::new(20.0, 0.0),
                LanePoint::new(10.0, 10.0),
                LanePoint::new(0.0, 15.0),
            ]
                .as_slice(),
        );

        let mut ps = vec![];
        let mut s = 0.0;
        while s < curve.length {
            let p = curve.get_smooth_point(s);
            ps.push(p);
            s += 0.1;
        }
        crate::common::plotter::draw_points("3", "line curve", &ps);
    }
}
