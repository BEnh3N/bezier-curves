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

pub fn pt(x: f32, y: f32) -> Point {
    Point { pos: vec2(x, y), hover: false }
}