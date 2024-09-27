use crate::scene::vertex::VertexWithColor;
use libmap::LaneInfo;
use std::f32::consts::PI;

const LINE_Z_OFFSET: f32 = 0.002;
const FACE_Z_OFFSET: f32 = 0.001;

const LINE_WHITE_COLOR: [f32; 3] = [0.9, 0.9, 0.9];
const LANE_COLOR: [f32; 3] = [0.2, 0.1, 0.1];

/// 对地图上的线， 生成顶点
pub(crate) fn generate_road_center_line(
    map: libmap::MapRef, thickness: f32,
) -> (Vec<VertexWithColor>, Vec<u32>) {
    let mut indices = vec![];
    let mut positions = vec![];

    for (i, road) in map.get_roads().iter() {
        let points = road
            .center_line
            .get_smooth_range_points(0.0, road.center_line.length);

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
                    color: LINE_WHITE_COLOR,
                });
                positions.push(VertexWithColor {
                    position: [
                        points[i - 1].x as f32 - delta_x,
                        points[i - 1].y as f32 - delta_y,
                        LINE_Z_OFFSET,
                    ],
                    color: LINE_WHITE_COLOR,
                });
            }
            positions.push(VertexWithColor {
                position: [
                    points[i].x as f32 + delta_x,
                    points[i].y as f32 + delta_y,
                    LINE_Z_OFFSET,
                ],
                color: LINE_WHITE_COLOR,
            });
            positions.push(VertexWithColor {
                position: [
                    points[i].x as f32 - delta_x,
                    points[i].y as f32 - delta_y,
                    LINE_Z_OFFSET,
                ],
                color: LINE_WHITE_COLOR,
            });
            let l = (positions.len() - 1) as u32;
            indices.extend([l - 3, l - 2, l - 1]);
            indices.extend([l - 1, l - 2, l]);
        }
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
            left_line.get_smooth_range_points(0.0, left_line.length);
        let right_points =
            right_line.get_smooth_range_points(0.0, right_line.length);
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
    };
    for (i, road) in map.get_roads().iter() {
        for i in road.left_lanes.iter() {
            fun(i, &mut vertex, &mut indices);
        }
        for i in road.right_lanes.iter() {
            fun(i, &mut vertex, &mut indices);
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
