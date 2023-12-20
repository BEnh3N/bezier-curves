use crate::{bezier::*, draw::*, pt::*};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

mod bezier;
mod draw;
mod pt;

const SIZE: u32 = 550;
const X_OFF: f32 = SIZE as f32 / -2.0;
const Y_OFF: f32 = SIZE as f32 / -2.0;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}

struct Model {
    egui: Egui,
    points: Vec<Point>,
    t: f32,
    segment_count: u32,
    mouse: Vec2,
    lines: bool,
    spokes: bool,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(SIZE, SIZE)
        .event(event)
        .raw_event(raw_window_event)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    let points = vec![
        pt(220.0, 300.0),
        pt(50.0, 380.0),
        pt(420.0, 500.0),
        pt(420.0, 60.0),
        // pt(250.0, 100.0)
    ];
    let t = 0.5;
    let segment_count = 50;
    let mouse = app.mouse.position();
    let lines = true;
    let spokes = true;

    Model {
        egui,
        points,
        t,
        segment_count,
        mouse,
        lines,
        spokes,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse = app.mouse.position();
    model.mouse = vec2(mouse.x - X_OFF, mouse.y * -1.0 - Y_OFF);

    draw_gui(model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale_y(-1.0).x(X_OFF).y(Y_OFF);
    draw.background().color(BLACK);

    // Draw spokes and lines, if activated
    if model.lines {
        for i in 0..model.points.len() - 1 {
            draw.line()
                .start(model.points[i].into())
                .end(model.points[i + 1].into())
                .color(WHITE)
                .weight(2.0);
        }
        if model.spokes {
            draw_spokes(&draw, &model.points, model.t);
        }
    }

    // Draw bezier curve
    draw_curve(&draw, &flatten_curve(&model.points, model.segment_count));

    // Draw control points
    for point in model.points.iter() {
        draw.ellipse()
            .color(WHITE)
            .x_y(point.x, point.y)
            .w_h(12.0, 12.0);
        if point.hover {
            draw.ellipse()
                .w_h(12.0, 12.0)
                .no_fill()
                .stroke_color(CADETBLUE)
                .stroke_weight(2.0)
                .x_y(point.x, point.y);
        }
    }

    // Draw little colored circle at t position
    draw.ellipse()
        .w_h(15.0, 15.0)
        .color(hsv(model.t, 1.0, 1.0))
        .stroke_color(WHITE)
        .stroke_weight(2.0)
        .xy(draw_curve_point(&model.points, model.t).into());

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(MouseButton::Left) => {
            for point in model.points.iter_mut() {
                if (model.mouse.x - point.x).pow(2) + (model.mouse.y - point.y).pow(2) <= 49_f32 {
                    point.hover = true;
                }
            }
        }
        MouseReleased(MouseButton::Left) => {
            for point in model.points.iter_mut() {
                point.hover = false;
            }
        }
        MouseMoved(_) => {
            for point in model.points.iter_mut() {
                if point.hover {
                    point.x = model.mouse.x;
                    point.y = model.mouse.y;
                    // println!("{}, {}", model.mouse.x, model.mouse.y);
                }
            }
        }
        KeyPressed(Key::Space) => model.lines = !model.lines,
        _ => {}
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn draw_gui(model: &mut Model) {
    let ctx = model.egui.begin_frame();

    egui::Window::new("Bezier Curves").show(&ctx, |ui| {
        // Interval Slider
        ui.label("Interval");
        ui.add(egui::Slider::new(&mut model.t, 0.0..=1.0));

        // Segment Count Slider
        ui.label("Segment Count");
        ui.add(egui::Slider::new(&mut model.segment_count, 1..=50));

        // Show Lines Switch
        ui.add(egui::Checkbox::new(&mut model.lines, "Show Lines"));

        // Show Spokes Switch
        ui.add_enabled_ui(model.lines, |ui| {
            ui.add(egui::Checkbox::new(&mut model.spokes, "Show Spokes"));
        })
    });
}
