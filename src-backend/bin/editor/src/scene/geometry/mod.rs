use crate::scene::vertex::VertexWithColor;
use libmap::{LaneInfo, LanePoint};
use std::f32::consts::PI;

const LINE_Z_OFFSET: f32 = 0.002;
const FACE_Z_OFFSET: f32 = 0.001;

const LINE_WHITE_COLOR: [f32; 3] = [1.0, 1.0, 1.0];
const LINE_GRAY_WHITE_COLOR: [f32; 3] = [0.8, 0.8, 0.8];
const LINE_YELLOW_COLOR: [f32; 3] = [0.9, 0.9, 0.0];
const LANE_COLOR: [f32; 3] = [0.2, 0.1, 0.1];

/// 对地图上的线， 生成顶点
/// road中心线
pub(crate) fn generate_road_center_line(
    map: libmap::MapRef, thickness: f32,
) -> (Vec<VertexWithColor>, Vec<u32>) {
    let mut indices = vec![];
    let mut positions = vec![];

    for (i, road) in map.get_roads().iter() {
        let points = road.center_line.get_smooth_range_points(
            0.0,
            road.center_line.length,
            0.2,
        );

        generate_line_geometry(
            points,
            thickness,
            LINE_YELLOW_COLOR,
            &mut positions,
            &mut indices,
        );
    }

    (positions, indices)
}

// 条状形， 用于画面
pub fn generate_road_lane(
    map: libmap::MapRef,
) -> (Vec<VertexWithColor>, Vec<u32>) {
    let mut indices = vec![];
    let mut vertex = vec![];

    let fun = |v: &LaneInfo,
               vertex: &mut Vec<VertexWithColor>,
               indices: &mut Vec<u32>| {
        let left_line = v.central_lane_curve.translation(-v.width as f64 / 2.0);
        let right_line = v.central_lane_curve.translation(v.width as f64 / 2.0);
        let left_points =
            left_line.get_smooth_range_points(0.0, left_line.length, 0.2);
        let right_points =
            right_line.get_smooth_range_points(0.0, right_line.length, 0.2);
        tracing::info!("{}, {}", left_points.len(), right_points.len());
        for i in 0..left_points.len().min(right_points.len()) {
            vertex.push(VertexWithColor {
                position: [
                    left_points[i].x as f32,
                    left_points[i].y as f32,
                    FACE_Z_OFFSET,
                ],
                color: LANE_COLOR,
            });
            vertex.push(VertexWithColor {
                position: [
                    right_points[i].x as f32,
                    right_points[i].y as f32,
                    FACE_Z_OFFSET,
                ],
                color: LANE_COLOR,
            });
            if i > 0 {
                let l = vertex.len() as u32 - 1;
                indices.extend([l - 3, l - 2, l - 1]);
                indices.extend([l - 1, l - 2, l]);
            }
        }
        if left_points.len() < right_points.len() {
            let l = vertex.len() as u32 - 2; // left_last point index
            for j in left_points.len()..right_points.len() {
                vertex.push(VertexWithColor {
                    position: [
                        right_points[j].x as f32,
                        right_points[j].y as f32,
                        FACE_Z_OFFSET,
                    ],
                    color: LANE_COLOR,
                });
                indices.extend([
                    l,
                    vertex.len() as u32 - 2,
                    vertex.len() as u32 - 1,
                ]);
            }
        } else if right_points.len() < left_points.len() {
            let l = vertex.len() as u32 - 1; // right_last point index
            for j in right_points.len()..left_points.len() {
                vertex.push(VertexWithColor {
                    position: [
                        left_points[j].x as f32,
                        left_points[j].y as f32,
                        FACE_Z_OFFSET,
                    ],
                    color: LANE_COLOR,
                });
                indices.extend([
                    vertex.len() as u32 - 2,
                    l,
                    vertex.len() as u32 - 1,
                ]);
            }
        }

        // lane中心线
        let center_points = v.central_lane_curve.get_smooth_range_points(
            0.0,
            v.central_lane_curve.length,
            1.0,
        );
        generate_virtual_line_geometry(
            center_points,
            0.04,
            LINE_GRAY_WHITE_COLOR,
            vertex,
            indices,
        );
        // lane左右边界线
        generate_line_geometry(
            left_points,
            0.04,
            LINE_WHITE_COLOR,
            vertex,
            indices,
        );
        generate_line_geometry(
            right_points,
            0.04,
            LINE_WHITE_COLOR,
            vertex,
            indices,
        );
    };
    for (i, road) in map.get_roads().iter() {
        for lane in road.left_lanes.iter() {
            fun(lane, &mut vertex, &mut indices);
        }
        for lane in road.right_lanes.iter() {
            fun(lane, &mut vertex, &mut indices);
        }
    }

    (vertex, indices)
}

pub fn merge_geometry(
    mut a: (Vec<VertexWithColor>, Vec<u32>),
    mut b: (Vec<VertexWithColor>, Vec<u32>),
) -> (Vec<VertexWithColor>, Vec<u32>) {
    for i in b.1.iter_mut() {
        *i = *i + a.0.len() as u32;
    }
    a.1.extend(b.1);

    a.0.extend(b.0);
    return a;
}

// 实线
fn generate_line_geometry(
    points: Vec<LanePoint>, thickness: f32, color: [f32; 3],
    positions: &mut Vec<VertexWithColor>, indices: &mut Vec<u32>,
) {
    for i in 1..points.len() {
        let theta = {
            let mut theta = (points[i].y - points[i - 1].y)
                .atan2(points[i].x - points[i - 1].x);
            theta as f32 + PI / 2.0
        };
        let (delta_x, delta_y) =
            (theta.cos() * thickness / 2.0, theta.sin() * thickness / 2.0);
        if i == 1 {
            positions.push(VertexWithColor {
                position: [
                    points[i - 1].x as f32 + delta_x,
                    points[i - 1].y as f32 + delta_y,
                    LINE_Z_OFFSET,
                ],
                color,
            });
            positions.push(VertexWithColor {
                position: [
                    points[i - 1].x as f32 - delta_x,
                    points[i - 1].y as f32 - delta_y,
                    LINE_Z_OFFSET,
                ],
                color,
            });
        }
        positions.push(VertexWithColor {
            position: [
                points[i].x as f32 + delta_x,
                points[i].y as f32 + delta_y,
                LINE_Z_OFFSET,
            ],
            color,
        });
        positions.push(VertexWithColor {
            position: [
                points[i].x as f32 - delta_x,
                points[i].y as f32 - delta_y,
                LINE_Z_OFFSET,
            ],
            color,
        });
        let l = (positions.len() - 1) as u32;
        indices.extend([l - 3, l - 2, l - 1]);
        indices.extend([l - 1, l - 2, l]);
    }
}

// 虚线
fn generate_virtual_line_geometry(
    points: Vec<LanePoint>, thickness: f32, color: [f32; 3],
    positions: &mut Vec<VertexWithColor>, indices: &mut Vec<u32>,
) {
    let mut i = 1;
    while i < points.len() {
        let theta = {
            let mut theta = (points[i].y - points[i - 1].y)
                .atan2(points[i].x - points[i - 1].x);
            theta as f32 + PI / 2.0
        };
        let (delta_x, delta_y) =
            (theta.cos() * thickness / 2.0, theta.sin() * thickness / 2.0);

        // 先把前一个点和当前点加进去
        positions.push(VertexWithColor {
            position: [
                points[i - 1].x as f32 + delta_x,
                points[i - 1].y as f32 + delta_y,
                LINE_Z_OFFSET,
            ],
            color,
        });
        positions.push(VertexWithColor {
            position: [
                points[i - 1].x as f32 - delta_x,
                points[i - 1].y as f32 - delta_y,
                LINE_Z_OFFSET,
            ],
            color,
        });
        positions.push(VertexWithColor {
            position: [
                points[i].x as f32 + delta_x,
                points[i].y as f32 + delta_y,
                LINE_Z_OFFSET,
            ],
            color,
        });
        positions.push(VertexWithColor {
            position: [
                points[i].x as f32 - delta_x,
                points[i].y as f32 - delta_y,
                LINE_Z_OFFSET,
            ],
            color,
        });

        let l = (positions.len() - 1) as u32;
        indices.extend([l - 3, l - 2, l - 1]);
        indices.extend([l - 1, l - 2, l]);

        i = i + 2;
    }
}
