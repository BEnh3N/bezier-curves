use crate::pt::*;
use nannou::prelude::{hsv, Hsv};

pub fn draw_curve_point(points: &Vec<Point>, t: f32) -> Point {
    if points.len() == 1 {
        points[0]
    } else {
        let mut newpoints = vec![pt(0.0, 0.0); points.len() - 1];
        for i in 0..newpoints.len() {
            newpoints[i] = points[i] * (1.0 - t) + points[i + 1] * t;
        }
        draw_curve_point(&newpoints, t)
    }
}

pub fn flatten_curve(points: &Vec<Point>, segment_count: u32) -> Vec<(Point, Hsv)> {
    let step = 1.0 / segment_count as f32;
    let mut coordinates = vec![(draw_curve_point(points, 0.0), hsv(0.0, 1.0, 1.0))];
    for i in 1..=segment_count {
        let t = i as f32 * step;
        coordinates.push((draw_curve_point(points, t), hsv(t, 1.0, 1.0)))
    }
    coordinates
}
