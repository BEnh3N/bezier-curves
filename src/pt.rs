use std::ops;
use nannou::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub hover: bool,
}
impl Into<Vec2> for Point {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        pt(self.x + rhs.x, self.y + rhs.y)
    }
}
impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        pt(self.x - rhs.x, self.y - rhs.y)
    }
}

// impl ops::Mul<Point> for f32 {
//     type Output = Point;

//     fn mul(self, rhs: Point) -> Self::Output {
//         pt(rhs.x * self, rhs.y * self)
//     }
// }
impl ops::Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        pt(self.x * rhs, self.y * rhs)
    }
}

pub fn pt(x: f32, y: f32) -> Point {
    Point { x, y, hover: false }
}