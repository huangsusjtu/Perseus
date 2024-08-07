use std::cmp::Ordering;

use crate::common::math::K_MATH_EPSILON;
use crate::common::util::Vec2d;

#[derive(PartialEq, Debug)]
enum Partition {
    PartitionX = 1,
    PartitionY = 2,
}

pub trait Box2d {
    fn default() -> Self;

    #[allow(dead_code)]
    fn id(&self) -> i32; // for debug

    fn min_x(&self) -> f64;
    fn min_y(&self) -> f64;
    fn max_x(&self) -> f64;
    fn max_y(&self) -> f64;

    fn distance_square(&self, point: &Vec2d) -> f64;
}

fn compare_by_min_x<T>(a: &T, b: &T) -> Ordering
    where
        T: Box2d,
{
    if a.min_x() - b.min_x() > 0.0 {
        std::cmp::Ordering::Greater
    } else if a.min_x() - b.min_x() < 0.0 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

fn compare_by_min_y<T>(a: &T, b: &T) -> Ordering
    where
        T: Box2d,
{
    if a.min_y() - b.min_y() > 0.0 {
        std::cmp::Ordering::Greater
    } else if a.min_y() - b.min_y() < 0.0 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

fn compare_by_max_x<T>(a: &T, b: &T) -> Ordering
    where
        T: Box2d,
{
    if a.max_x() - b.max_x() > 0.0 {
        std::cmp::Ordering::Greater
    } else if a.max_x() - b.max_x() < 0.0 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

fn compare_by_max_y<T>(a: &T, b: &T) -> Ordering
    where
        T: Box2d,
{
    if a.max_y() - b.max_y() > 0.0 {
        std::cmp::Ordering::Greater
    } else if a.max_y() - b.min_y() < 0.0 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct KDTreeParams {
    /// The maximum depth of the kdtree.
    pub max_depth: i32,
    /// The maximum number of items in one leaf node.
    pub max_leaf_size: u32,
    /// The maximum dimension size of leaf node.
    pub max_leaf_dimension: f64,
}

#[derive(Debug)]
pub struct KDTree2dNode<T>
    where
        T: Box2d + Clone,
{
    num_objects_: i32,
    depth_: i32,
    // Boundary
    min_x_: f64,
    max_x_: f64,
    min_y_: f64,
    max_y_: f64,
    mid_x_: f64,
    mid_y_: f64,

    partition_: Partition,
    partition_position_: f64,
    left_subnode_: Option<Box<KDTree2dNode<T>>>,
    right_subnode_: Option<Box<KDTree2dNode<T>>>,

    objects_sorted_by_min_: Vec<T>,
    objects_sorted_by_max_: Vec<T>,
    objects_sorted_by_min_bound_: Vec<f64>,
    objects_sorted_by_max_bound_: Vec<f64>,
}

impl<T> Default for KDTree2dNode<T>
    where
        T: Box2d + Clone,
{
    fn default() -> Self {
        Self {
            num_objects_: Default::default(),
            depth_: Default::default(),
            min_x_: Default::default(),
            max_x_: Default::default(),
            min_y_: Default::default(),
            max_y_: Default::default(),
            mid_x_: Default::default(),
            mid_y_: Default::default(),
            partition_: Partition::PartitionX,
            partition_position_: Default::default(),
            left_subnode_: Default::default(),
            right_subnode_: Default::default(),
            objects_sorted_by_min_: Default::default(),
            objects_sorted_by_max_: Default::default(),
            objects_sorted_by_min_bound_: Default::default(),
            objects_sorted_by_max_bound_: Default::default(),
        }
    }
}

impl<T> KDTree2dNode<T>
    where
        T: Box2d + Clone,
{
    pub fn build(param: &KDTreeParams, objects: Vec<T>, depth: i32) -> Self {
        // todo:
        let mut s = KDTree2dNode {
            num_objects_: 0,
            depth_: depth,
            min_x_: 0.0,
            max_x_: 0.0,
            min_y_: 0.0,
            max_y_: 0.0,
            mid_x_: 0.0,
            mid_y_: 0.0,
            partition_: Partition::PartitionX,
            partition_position_: 0.0,
            left_subnode_: None,
            right_subnode_: None,
            objects_sorted_by_min_: Vec::new(),
            objects_sorted_by_max_: Vec::new(),
            objects_sorted_by_min_bound_: Vec::new(),
            objects_sorted_by_max_bound_: Vec::new(),
        };
        s.compute_boundary(&objects);
        s.compute_partition();
        if s.need_split_subnodes(&objects, param) {
            let (left_subnode_objects, right_subnode_objects) = s.partition_objects(objects);

            if !left_subnode_objects.is_empty() {
                s.left_subnode_ = Some(Box::new(KDTree2dNode::build(
                    param,
                    left_subnode_objects,
                    depth + 1,
                )))
            }
            if !right_subnode_objects.is_empty() {
                s.right_subnode_ = Some(Box::new(KDTree2dNode::build(
                    param,
                    right_subnode_objects,
                    depth + 1,
                )))
            }
        } else {
            s.init_objects(objects);
        }

        return s;
    }

    fn compute_boundary(&mut self, objects: &Vec<T>) {
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        for obj in objects {
            min_x = min_x.min(obj.min_x());
            min_y = min_y.min(obj.min_y());
            max_x = max_x.max(obj.max_x());
            max_y = max_y.max(obj.max_y());
        }

        self.min_x_ = min_x;
        self.min_y_ = min_y;
        self.max_x_ = max_x;
        self.max_y_ = max_y;
        self.mid_x_ = (min_x + max_x) / 2.0;
        self.mid_y_ = (min_y + max_y) / 2.0;
    }
    fn compute_partition(&mut self) {
        if self.max_x_ - self.min_x_ >= self.max_y_ - self.min_y_ {
            self.partition_ = Partition::PartitionX;
            self.partition_position_ = (self.min_x_ + self.max_x_) / 2.0;
        } else {
            self.partition_ = Partition::PartitionY;
            self.partition_position_ = (self.min_y_ + self.max_y_) / 2.0;
        }
    }
    fn need_split_subnodes(&mut self, objects: &Vec<T>, param: &KDTreeParams) -> bool {
        if param.max_depth >= 0 && self.depth_ > param.max_depth {
            return false;
        }
        if (objects.len() as u32) < param.max_leaf_size.max(1) {
            return false;
        }
        if param.max_leaf_dimension >= 0.0
            && (self.max_x_ - self.min_x_).max(self.max_y_ - self.min_y_)
            <= param.max_leaf_dimension
        {
            return false;
        }
        return true;
    }

    fn partition_objects(&mut self, objects: Vec<T>) -> (Vec<T>, Vec<T>) {
        let mut left_subnode_objects = Vec::new();
        let mut right_subnode_objects = Vec::new();
        let mut other_objects = Vec::new();
        if self.partition_ == Partition::PartitionX {
            for obj in &objects {
                if obj.max_x() <= self.partition_position_ {
                    left_subnode_objects.push(obj.clone());
                } else if obj.min_x() >= self.partition_position_ {
                    right_subnode_objects.push(obj.clone());
                } else {
                    other_objects.push(obj.clone());
                }
            }
        } else {
            for obj in &objects {
                if obj.max_y() <= self.partition_position_ {
                    left_subnode_objects.push(obj.clone());
                } else if obj.min_y() >= self.partition_position_ {
                    right_subnode_objects.push(obj.clone());
                } else {
                    other_objects.push(obj.clone());
                }
            }
        }
        self.init_objects(other_objects);
        return (left_subnode_objects, right_subnode_objects);
    }

    fn init_objects(&mut self, mut objects: Vec<T>) {
        self.num_objects_ = objects.len() as i32;
        if self.num_objects_ == 0 {
            return;
        }

        if self.partition_ == Partition::PartitionX {
            objects.sort_by(compare_by_min_x);
        } else {
            objects.sort_by(compare_by_min_y);
        }
        self.objects_sorted_by_min_ = objects.to_vec();
        self.objects_sorted_by_min_bound_ = Vec::with_capacity(self.objects_sorted_by_min_.len());
        if self.partition_ == Partition::PartitionX {
            for obj in &self.objects_sorted_by_min_ {
                self.objects_sorted_by_min_bound_.push(obj.min_x());
            }
        } else {
            for obj in &self.objects_sorted_by_min_ {
                self.objects_sorted_by_min_bound_.push(obj.min_y());
            }
        }

        if self.partition_ == Partition::PartitionX {
            objects.sort_by(compare_by_max_x);
        } else {
            objects.sort_by(compare_by_max_y);
        }
        self.objects_sorted_by_max_ = objects.to_vec();
        self.objects_sorted_by_max_bound_ = Vec::with_capacity(self.objects_sorted_by_max_.len());
        if self.partition_ == Partition::PartitionX {
            for obj in &self.objects_sorted_by_max_ {
                self.objects_sorted_by_max_bound_.push(obj.max_x());
            }
        } else {
            for obj in &self.objects_sorted_by_max_ {
                self.objects_sorted_by_max_bound_.push(obj.max_y());
            }
        }
    }

    // search
    pub fn find_object_by_distance(&self, point: &Vec2d, distance: f64) -> Vec<T> {
        let mut result = Vec::new();
        self.find_objects_by_distance_internal(point, distance, distance * distance, &mut result);
        return result;
    }

    pub fn find_nearest_object(&self, point: &Vec2d) -> Option<T> {
        let mut min_distance_sqr = f64::MAX;
        let mut t = T::default();
        let ret = self.find_nearest_object_internal(point, &mut min_distance_sqr, &mut t);
        if ret {
            return Some(t);
        }
        None
    }

    fn find_objects_by_distance_internal(
        &self,
        point: &Vec2d,
        distance: f64,
        distance_sqr: f64,
        result: &mut Vec<T>,
    ) {
        // for obj in &self.objects_sorted_by_min_ {
        //     if obj.id() == String::from("302") {
        //         tracing::error!("{}", obj.id());
        //         break;
        //     }
        // }

        if self.lower_distance_square_point(point) > distance_sqr {
            return;
        }
        if self.upper_distance_square_point(point) <= distance_sqr {
            self.get_all_objects(result);
            return;
        }
        let pvalue = if self.partition_ == Partition::PartitionX {
            point.x
        } else {
            point.y
        };
        if pvalue < self.partition_position_ {
            let _limit = pvalue + distance;
            let mut index = 0;
            for _bound in &self.objects_sorted_by_min_bound_ {
                // if *bound > _limit {
                //     break;
                // }
                let obj = self.objects_sorted_by_min_.get(index).unwrap();
                if obj.distance_square(point) <= distance_sqr {
                    result.push(obj.clone());
                }
                index += 1;
            }
        } else {
            let _limit = pvalue - distance;
            let mut index = 0;
            for _bound in &self.objects_sorted_by_max_bound_ {
                // if *bound < _limit {
                //     break;
                // }
                let obj = self.objects_sorted_by_max_.get(index).unwrap();
                if obj.distance_square(point) <= distance_sqr {
                    result.push(obj.clone());
                }
                index += 1;
            }
        }

        if self.left_subnode_.is_some() {
            self.left_subnode_
                .as_ref()
                .unwrap()
                .find_objects_by_distance_internal(point, distance, distance_sqr, result);
        }
        if self.right_subnode_.is_some() {
            self.right_subnode_
                .as_ref()
                .unwrap()
                .find_objects_by_distance_internal(point, distance, distance_sqr, result);
        }
    }

    fn find_nearest_object_internal(
        &self,
        point: &Vec2d,
        min_distance_sqr: &mut f64,
        t: &mut T,
    ) -> bool {
        if self.lower_distance_square_point(point) >= *min_distance_sqr - K_MATH_EPSILON as f64 {
            return false;
        }
        let mut result = false;
        let pvalue = if self.partition_ == Partition::PartitionX {
            point.x
        } else {
            point.y
        };
        let search_left_first = pvalue < self.partition_position_;
        if search_left_first {
            if self.left_subnode_.is_some() {
                result |= self
                    .left_subnode_
                    .as_ref()
                    .unwrap()
                    .find_nearest_object_internal(point, min_distance_sqr, t);
            }
        } else {
            if self.right_subnode_.is_some() {
                result |= self
                    .right_subnode_
                    .as_ref()
                    .unwrap()
                    .find_nearest_object_internal(point, min_distance_sqr, t);
            }
        }
        if *min_distance_sqr < K_MATH_EPSILON as f64 {
            return true;
        }

        if search_left_first {
            let mut index = 0;
            for _bound in &self.objects_sorted_by_min_bound_ {
                // if *bound > pvalue && (*bound - pvalue).powi(2) > *min_distance_sqr {
                //     break;
                // }
                let obj = self.objects_sorted_by_min_.get(index).unwrap();
                let distance = obj.distance_square(point);
                if distance < *min_distance_sqr {
                    *min_distance_sqr = distance;
                    *t = obj.clone();
                    result = true;
                }
                index += 1;
            }
        } else {
            let mut index = 0;
            for _bound in &self.objects_sorted_by_max_bound_ {
                // if *bound < pvalue && (*bound - pvalue).powi(2) > *min_distance_sqr {
                //     break;
                // }
                let obj = self.objects_sorted_by_max_.get(index).unwrap();
                let distance = obj.distance_square(point);
                if distance < *min_distance_sqr {
                    *min_distance_sqr = distance;
                    *t = obj.clone();
                    result = true;
                }
                index += 1;
            }
        }
        if *min_distance_sqr < K_MATH_EPSILON as f64 {
            return true;
        }
        if search_left_first {
            if self.right_subnode_.is_some() {
                result |= self
                    .right_subnode_
                    .as_ref()
                    .unwrap()
                    .find_nearest_object_internal(point, min_distance_sqr, t);
            }
        } else {
            if self.left_subnode_.is_some() {
                result |= self
                    .left_subnode_
                    .as_ref()
                    .unwrap()
                    .find_nearest_object_internal(point, min_distance_sqr, t);
            }
        }

        return result;
    }

    fn lower_distance_square_point(&self, point: &Vec2d) -> f64 {
        let mut dx = 0.0;
        if point.x < self.min_x_ {
            dx = self.min_x_ - point.x;
        } else if point.x > self.max_x_ {
            dx = point.x - self.max_x_;
        }
        let mut dy = 0.0;
        if point.y < self.min_y_ {
            dy = self.min_y_ - point.y;
        } else if point.y > self.max_y_ {
            dy = point.y - self.max_y_;
        }
        return dx * dx + dy * dy;
    }

    fn upper_distance_square_point(&self, point: &Vec2d) -> f64 {
        let dx: f64;
        if point.x > self.mid_x_ {
            dx = point.x - self.min_x_;
        } else {
            dx = self.max_x_ - point.x;
        }
        let dy: f64;
        if point.y > self.mid_y_ {
            dy = point.y - self.min_y_;
        } else {
            dy = self.max_y_ - point.y;
        }
        return dx * dx + dy * dy;
    }

    fn get_all_objects(&self, result: &mut Vec<T>) {
        for obj in &self.objects_sorted_by_min_ {
            result.push(obj.clone());
        }
        if self.left_subnode_.is_some() {
            self.left_subnode_.as_ref().unwrap().get_all_objects(result);
        }
        if self.right_subnode_.is_some() {
            self.right_subnode_
                .as_ref()
                .unwrap()
                .get_all_objects(result);
        }
    }
}

#[derive(Debug)]
pub struct KDTree2d<T>
    where
        T: Box2d + Clone,
{
    root_: KDTree2dNode<T>,
    num_objects: usize,
}

impl<T> Default for KDTree2d<T>
    where
        T: Box2d + Clone,
{
    fn default() -> Self {
        Self {
            root_: Default::default(),
            num_objects: Default::default(),
        }
    }
}

impl<T> KDTree2d<T>
    where
        T: Box2d + Clone,
{
    pub fn build(objects: Vec<T>) -> Self {
        let default = KDTreeParams {
            max_depth: 16,
            max_leaf_size: 10,
            max_leaf_dimension: -1.0,
        };

        let num = objects.len();
        KDTree2d {
            root_: KDTree2dNode::build(&default, objects, 0),
            num_objects: num,
        }
    }

    #[allow(dead_code)]
    pub fn build_with_param(objects: Vec<T>, param: &KDTreeParams) -> Self {
        let num = objects.len();
        KDTree2d {
            root_: KDTree2dNode::build(&param, objects, 0),
            num_objects: num,
        }
    }

    #[allow(dead_code)]
    pub fn get_bounding_box() {}

    pub fn find_nearest_object(&self, point: &Vec2d) -> Option<T> {
        if point.x().is_nan() || point.y().is_nan() {
            return None;
        }
        return self.root_.find_nearest_object(point);
    }

    pub fn find_objects_by_distance(&self, point: &Vec2d, distance: f64) -> Vec<T> {
        return self.root_.find_object_by_distance(point, distance);
    }

    #[allow(dead_code)]
    pub fn get_object_num(&self) -> usize {
        return self.num_objects;
    }
}

impl<T> std::fmt::Display for KDTree2dNode<T>
    where
        T: Box2d + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            " [KDTree2dNode  depth_:{}, num_objects_:{}]",
            self.depth_, self.num_objects_
        )
    }
}

impl<T> std::fmt::Display for KDTree2d<T>
    where
        T: Box2d + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "KDTree2d {}", self.root_)
    }
}
