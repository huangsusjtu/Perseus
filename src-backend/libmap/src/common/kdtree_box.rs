// 用于kdtree里的 数据节点， 代表一条线段所在的矩形
use std::sync::Arc;

use crate::common::kdtree2d::Box2d;
use crate::common::util::{LineSegment2d, Vec2d};

#[derive(Debug, Clone)]
pub struct LineSegmentBox {
    pub data_: Arc<LineSegment2d>,
    pub line_segment_index: i32,
    pub road_id_: i32,
    pub lane_id_: i32,
}

impl std::fmt::Display for LineSegmentBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.line_segment_index, self.road_id_, self.lane_id_)
    }
}

impl Box2d for LineSegmentBox {
    fn default() -> Self {
        LineSegmentBox {
            data_: Arc::new(LineSegment2d::default()),
            line_segment_index: -1,
            road_id_: 0,
            lane_id_: 0,
        }
    }

    fn id(&self) -> i32 {
        return self.road_id_;
    }
    fn min_x(&self) -> f64 {
        return self.data_.start_.x.min(self.data_.end_.x);
    }
    fn min_y(&self) -> f64 {
        return self.data_.start_.y.min(self.data_.end_.y);
    }
    fn max_x(&self) -> f64 {
        return self.data_.start_.x.max(self.data_.end_.x);
    }
    fn max_y(&self) -> f64 {
        return self.data_.start_.y.max(self.data_.end_.y);
    }

    fn distance_square(&self, point: &Vec2d) -> f64 {
        return self.data_.distance_square(point);
    }
}

#[derive(Debug, Clone)]
pub struct PointBox {
    pub center_: Vec2d,
    pub id_: i32,
}

impl Box2d for PointBox {
    fn default() -> Self {
        PointBox {
            center_: Vec2d { x: 0.0, y: 0.0 },
            id_: 0,
        }
    }

    fn id(&self) -> i32 {
        return self.id_;
    }

    fn min_x(&self) -> f64 {
        return self.center_.x;
    }

    fn min_y(&self) -> f64 {
        return self.center_.y;
    }

    fn max_x(&self) -> f64 {
        return self.center_.x;
    }

    fn max_y(&self) -> f64 {
        return self.center_.y;
    }

    fn distance_square(&self, point: &Vec2d) -> f64 {
        return (self.center_.x - point.x).powi(2)
            + (self.center_.y - point.y).powi(2);
    }
}
