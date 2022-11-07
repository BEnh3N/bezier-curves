use crate::pt::*;
use nannou::prelude::{Draw, Hsv};
use nannou::color::named::*;

pub fn draw_curve(draw: &Draw, points: &Vec<(Point, Hsv)>) {
    for i in 0..points.len() - 1 {
        draw.line().start(points[i].0.pos).end(points[i+1].0.pos).color(points[i].1).weight(10.0).caps_round();
    }
}

pub fn draw_spokes(draw: &Draw, points: &Vec<Point>, t: f32) {
    if points.len() == 2 {
        draw.ellipse().w_h(6.0, 6.0).xy(points[0].pos);
        draw.ellipse().w_h(6.0, 6.0).xy(points[1].pos);
        draw.line().start(points[0].pos).end(points[1].pos).color(WHITE).weight(2.0).caps_round();
    } else {
        let mut newpoints = vec![];
        for i in 0..points.len() - 1 {
            let pos1 = points[i].pos;
            let pos2 = points[i+1].pos;
            newpoints.push(pt(
                pos1.x + t * (pos2.x - pos1.x),
                pos1.y + t * (pos2.y - pos1.y),
            ));
        }
        for i in 0..newpoints.len() - 1 {
            draw_spokes(draw, &vec![newpoints[i], newpoints[i+1]], t)
        }
        draw_spokes(draw, &newpoints, t)
    }
}