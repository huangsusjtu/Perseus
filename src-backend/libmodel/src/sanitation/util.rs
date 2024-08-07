use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Vec2d {
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@y")]
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

pub type LanePoint = Vec2d;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Range {
    #[serde(rename = "@range")]
    pub r: [f32; 2],
}

impl From<[f32; 2]> for Range {
    fn from(value: [f32; 2]) -> Self {
        Range { r: value }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tuple {
    #[serde(rename = "@value")]
    pub value: (String, String),
}

impl From<(String, String)> for Tuple {
    fn from(value: (String, String)) -> Self {
        Tuple { value }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VecTuple {
    #[serde(rename = "tuple")]
    pub value: Vec<Tuple>,
}

impl From<Vec<Tuple>> for VecTuple {
    fn from(value: Vec<Tuple>) -> Self {
        VecTuple { value }
    }
}
