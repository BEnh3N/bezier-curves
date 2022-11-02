use nannou::prelude::*;


const SIZE: u32 = 550;
const X_OFF: f32 = SIZE as f32 / -2.0;
const Y_OFF: f32 = SIZE as f32 / -2.0;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug, Clone, Copy)]
struct Point {
    pos: Vec2,
    hover: bool,
}
impl Into<Point2> for Point {
    fn into(self) -> Point2 {
        Point2::new(self.pos.x, self.pos.y)
    }
}

fn pt(x: f32, y: f32) -> Point {
    Point { pos: vec2(x, y), hover: false }
}

struct Model {
    _window: window::Id,
    points: Vec<Point>,
    segment_count: u32,
    mouse: Vec2,
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
                                .size(SIZE, SIZE)
                                .event(event)
                                .view(view).build().unwrap();

    let points = vec![
        pt(220.0, 300.0),
        pt(50.0, 380.0),
        pt(420.0, 500.0),
        pt(420.0, 60.0)
    ];
    let segment_count = 25;
    let mouse = app.mouse.position();
    
    Model { _window, points, segment_count, mouse }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse = app.mouse.position();
    model.mouse = vec2(mouse.x - X_OFF, mouse.y * -1.0 - Y_OFF);

    // println!("{}", app.fps())
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale_y(-1.0).x(X_OFF).y(Y_OFF);
    draw.background().color(BLACK);

    draw.polyline().weight(10.0).points_colored(flatten_curve(&model.points, model.segment_count));
    for point in model.points.iter() {
        draw.ellipse().color(WHITE).x_y(point.pos.x, point.pos.y).w_h(12.0, 12.0);
        if point.hover {
            draw.ellipse().w_h(12.0, 12.0).no_fill().stroke_color(DEEPSKYBLUE).stroke_weight(2.0).x_y(point.pos.x, point.pos.y);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_curve_point(points: &Vec<Point>, t: f32) -> Point {
    if points.len() == 1 { points[0] }
    else {
        let mut newpoints = vec![pt(0.0, 0.0); points.len() - 1];
        for i in 0..newpoints.len() {
            newpoints[i].pos = (1.0 - t) * points[i].pos + t * points[i+1].pos;
        }
        draw_curve_point(&newpoints, t)
    }
}

fn flatten_curve(points: &Vec<Point>, segment_count: u32) -> Vec<(Point, Hsv)> {
    let step = 1.0 / segment_count as f32;
    let mut coordinates = vec![(draw_curve_point(points, 0.0), hsv(0.0, 1.0, 1.0))];
    for i in 1..=segment_count {
        let t = i as f32 * step;
        coordinates.push((draw_curve_point(points, t), hsv(t, 1.0, 1.0)))
    }
    coordinates
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(MouseButton::Left) => {
            for point in model.points.iter_mut() {
                if (model.mouse.x - point.pos.x).pow(2) + (model.mouse.y - point.pos.y).pow(2) <= 49_f32 { 
                    point.hover = true;
                }
            }
        },
        MouseReleased(MouseButton::Left) => {
            for point in model.points.iter_mut() {
                point.hover = false;
            }
        }
        MouseMoved(_) => {
            for point in model.points.iter_mut() {
                if point.hover { point.pos = model.mouse; }
            }
        }
        _ => {}
    }
}
