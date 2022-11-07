use std::ops;
use nannou::prelude::{Point2, Vec2, vec2};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub pos: Vec2,
    pub hover: bool,
}
impl Into<Point2> for Point {
    fn into(self) -> Point2 {
        Point2::new(self.pos.x, self.pos.y)
    }
}
impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        pt(self.pos.x + rhs.pos.x, self.pos.y + rhs.pos.y)
    }
}

impl ops::Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        pt(rhs.pos.x * self, rhs.pos.y * self)
    }
}

pub fn pt(x: f32, y: f32) -> Point {
    Point { pos: vec2(x, y), hover: false }
}