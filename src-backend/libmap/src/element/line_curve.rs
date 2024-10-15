use crate::common::math::binary_search;
use crate::common::util::{LanePoint, Vec2d};
use crate::element::line_segment::GeometryLine;
use crate::proto_gen;
use std::fmt::{Debug, Display, Formatter};

/// 车道线定义
#[derive(Debug, Clone)]
pub struct LineCurveInfo {
    pub segments: Vec<GeometryLine>,
    pub segments_length: Vec<f64>,
    pub length: f64,
}

impl Default for LineCurveInfo {
    fn default() -> Self {
        LineCurveInfo {
            segments: Vec::new(),
            segments_length: Vec::new(),
            length: 0.0,
        }
    }
}

impl Display for LineCurveInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LineCurveInfo  {:?}", self.segments,)
    }
}

impl LineCurveInfo {
    pub fn new(segments: Vec<GeometryLine>) -> Self {
        let mut length = 0.0;
        let mut segments_length = vec![];
        for l in segments.iter() {
            length += l.length();
            segments_length.push(length);
        }
        let l = LineCurveInfo {
            segments,
            segments_length,
            length,
        };
        l
    }

    pub fn get_smooth_point(&self, s: f64) -> LanePoint {
        let idx = binary_search(&self.segments_length, s);
        return self.segments[idx].get_smooth_point(s);
    }

    pub fn get_smooth_point_and_head(&self, s: f64) -> (LanePoint, f64) {
        let idx = binary_search(&self.segments_length, s);
        return self.segments[idx].get_smooth_point_and_head(s);
    }

    pub fn get_smooth_range_points(
        &self, s: f64, e: f64, step: f64,
    ) -> Vec<LanePoint> {
        let mut r = Vec::default();

        for seg in self.segments.iter() {
            let (sub_s, sub_e) = (seg.s(), seg.s() + seg.length());
            let (split_s, split_e) = (sub_s.max(s), sub_e.min(e));
            if split_s > split_e {
                break;
            }
            r.append(&mut seg.get_dense_point_of_range(
                sub_s.max(s) as f32,
                sub_e.min(e) as f32,
                step as f32,
            ));
        }

        return r;
    }

    pub fn get_projection(&self, point: &Vec2d) -> (f64, f64) {
        if self.segments.is_empty() {
            return (0.0, 0.0);
        }
        let mut min_dist = std::f64::MAX;
        let seg_num = self.segments.len();
        let mut min_index = 0;
        let mut i = 0;
        while i < seg_num {
            let (distance, _) = self.segments[i].distance(point.x, point.y);
            if distance < min_dist {
                min_index = i;
                min_dist = distance;
            }
            i += 1;
        }

        let lateral: f64;
        let mut accumulate_s = 0.0;
        for i in 0..min_index {
            accumulate_s += self.segments[i].length();
        }

        let nearest_seg = &self.segments[min_index];
        let (proj, prod) = nearest_seg.get_projection(point.x, point.y);
        accumulate_s += proj.min(nearest_seg.length());
        if min_index == 0 {
            accumulate_s = proj.min(nearest_seg.length());
            if proj.is_sign_negative() {
                lateral = prod;
            } else {
                if prod.is_sign_positive() {
                    lateral = min_dist;
                } else {
                    lateral = -min_dist;
                }
            }
        } else if min_index == seg_num - 1 {
            accumulate_s = accumulate_s + proj.max(0.0);
            if proj.is_sign_positive() {
                lateral = prod;
            } else {
                if prod.is_sign_positive() {
                    lateral = min_dist;
                } else {
                    lateral = -min_dist;
                }
            }
        } else {
            accumulate_s =
                accumulate_s + proj.min(nearest_seg.length()).max(0.0);
            if prod.is_sign_positive() {
                lateral = min_dist;
            } else {
                lateral = -min_dist;
            }
        }

        return (accumulate_s, lateral);
    }

    pub fn heading(&self, s: f64) -> f64 {
        let mut a_s = s;
        for seg in self.segments.iter() {
            if a_s > seg.length() {
                a_s -= seg.length();
            } else {
                return seg.get_heading(s);
            }
        }
        return self.segments.last().unwrap().get_heading(0.0);
    }

    pub fn get_projection_and_index(&self, point: &Vec2d) -> (f64, f64, usize) {
        if self.segments.is_empty() {
            return (0.0, 0.0, 0);
        }
        let mut min_dist = std::f64::MAX;
        let seg_num = self.segments.len();
        let mut min_index = 0;
        let mut i = 0;
        while i < seg_num {
            let (distance, _) = self.segments[i].distance(point.x, point.y);
            if distance < min_dist {
                min_index = i;
                min_dist = distance;
            }
            i += 1;
        }

        let lateral: f64;
        let mut accumulate_s = 0.0;
        for i in 0..min_index {
            accumulate_s += self.segments[i].length();
        }

        let nearest_seg = &self.segments[min_index];
        let (proj, prod) = nearest_seg.get_projection(point.x, point.y);
        accumulate_s += proj.min(nearest_seg.length());
        if min_index == 0 {
            accumulate_s = proj.min(nearest_seg.length());
            if proj.is_sign_negative() {
                lateral = prod;
            } else {
                if prod.is_sign_positive() {
                    lateral = min_dist;
                } else {
                    lateral = -min_dist;
                }
            }
        } else if min_index == seg_num - 1 {
            accumulate_s = accumulate_s + proj.max(0.0);
            if proj.is_sign_positive() {
                lateral = prod;
            } else {
                if prod.is_sign_positive() {
                    lateral = min_dist;
                } else {
                    lateral = -min_dist;
                }
            }
        } else {
            accumulate_s =
                accumulate_s + proj.min(nearest_seg.length()).max(0.0);
            if prod.is_sign_positive() {
                lateral = min_dist;
            } else {
                lateral = -min_dist;
            }
        }

        return (accumulate_s, lateral, min_index);
    }

    pub fn distance(&self, point: &Vec2d) -> (f64, f64, i32, LanePoint) {
        let mut min_distance = f64::MAX;
        let mut s_offset = 0.0;
        let mut s_offset_index = -1;
        let mut min_proj_point = LanePoint::default();
        let mut index = 0;
        for seg in &self.segments {
            let (distance, proj_point) = seg.distance(point.x, point.y);
            if distance < min_distance {
                let (s, _) = seg.distance(point.x, point.y);
                min_distance = distance;
                s_offset += s;
                s_offset_index = index;
                min_proj_point = proj_point;
            }
            index += 1;
            s_offset += seg.length();
        }

        (min_distance, s_offset, s_offset_index, min_proj_point)
    }

    #[inline]
    pub fn begin_point(&self) -> LanePoint {
        self.get_smooth_point(0.0)
    }

    #[inline]
    pub fn end_point(&self) -> LanePoint {
        self.get_smooth_point(self.length)
    }

    // 左/右移得到新的线条
    pub fn translation(&self, offset: f64) -> Self {
        let new_segments = self
            .segments
            .iter()
            .map(|seg| seg.translation(offset as f32))
            .collect();
        LineCurveInfo::new(new_segments)
    }

    pub fn reverse(&self) -> Self {
        let mut segments: Vec<GeometryLine> =
            self.segments.iter().map(|v| v.reverse()).collect();
        segments.reverse();

        let mut length: f64 = 0.0;
        let mut segments_length = vec![];
        for l in segments.iter_mut() {
            l.reverse();
            l.set_s(length as f32);
            length += l.length();
            segments_length.push(length);
        }
        LineCurveInfo {
            segments,
            segments_length,
            length,
        }
    }
}

/// 自定义的map proto格式，用于前后端传输的
impl From<proto_gen::map::LineCurve> for LineCurveInfo {
    fn from(value: proto_gen::map::LineCurve) -> Self {
        let seg = value.segments.iter().map(|v| v.into()).collect();
        LineCurveInfo::new(seg)
    }
}

impl From<&proto_gen::map::LineCurve> for LineCurveInfo {
    fn from(value: &proto_gen::map::LineCurve) -> Self {
        let seg = value.segments.iter().map(|v| v.into()).collect();
        LineCurveInfo::new(seg)
    }
}

impl Into<proto_gen::map::LineCurve> for &LineCurveInfo {
    fn into(self) -> proto_gen::map::LineCurve {
        proto_gen::map::LineCurve {
            segments: self.segments.iter().map(|v| v.into()).collect(),
            length: self.length as f32,
            special_fields: Default::default(),
        }
    }
}
