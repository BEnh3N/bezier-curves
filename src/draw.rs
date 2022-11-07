use crate::pt::*;
use nannou::prelude::{Draw, Hsv};
use nannou::color::named::*;

pub fn draw_curve(draw: &Draw, points: &Vec<(Point, Hsv)>) {
    for i in 0..points.len() - 1 {
        draw.line().start(points[i].0.into()).end(points[i+1].0.into()).color(points[i].1).weight(10.0).caps_round();
    }
}

pub fn draw_spokes(draw: &Draw, points: &Vec<Point>, t: f32) {
    if points.len() == 2 {
        draw.ellipse().w_h(6.0, 6.0).xy(points[0].into());
        draw.ellipse().w_h(6.0, 6.0).xy(points[1].into());
        draw.line().start(points[0].into()).end(points[1].into()).color(WHITE).weight(2.0).caps_round();
    } else {
        let mut newpoints = vec![];
        for i in 0..points.len() - 1 {
            newpoints.push(points[i] + (points[i+1] - points[i]) * t);
        }
        for i in 0..newpoints.len() - 1 {
            draw_spokes(draw, &vec![newpoints[i], newpoints[i+1]], t)
        }
        draw_spokes(draw, &newpoints, t)
    }
}