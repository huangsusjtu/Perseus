use plotters::prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, PathElement};
use plotters::series::LineSeries;
use plotters::style::{BLACK, RED, WHITE};

use crate::common::util::LanePoint;

#[allow(dead_code)]
pub fn draw_points(file_name: &str, title: &str, paths: &Vec<LanePoint>) {
    let mut max_x = paths.first().unwrap().x.max(paths.last().unwrap().x);
    let mut min_x = paths.last().unwrap().x.min(paths.first().unwrap().x);
    let mut max_y = paths.first().unwrap().y.max(paths.last().unwrap().y);
    let mut min_y = paths.last().unwrap().y.min(paths.first().unwrap().y);

    for p in paths.iter() {
        max_x = max_x.max(p.x);
        min_x = min_x.min(p.x);
        max_y = max_y.max(p.y);
        min_y = min_y.min(p.y);
    }

    let x_width = (max_x - min_x).abs() + 10.0;
    let y_width = (max_y - min_y).abs() + 10.0;
    // let width = x_width.max(y_width);

    let file_path = std::env::current_dir()
        .unwrap()
        .join(format!("{}.png", file_name));
    // let file_path = format!("/home/huangsu/{}.png", file_name);
    let root_area = IntoDrawingArea::into_drawing_area(BitMapBackend::new(&file_path, (800, 800)));
    root_area.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(title, ("sans-serif", 40))
        .build_cartesian_2d(-5.0..(x_width + 5.0), -5.0..(y_width + 5.0))
        .unwrap();
    ctx.configure_mesh().draw().unwrap();

    let mut data: Vec<(f64, f64)> = Vec::default();
    for s in paths.iter() {
        data.push((s.x - min_x, s.y - min_y));
    }
    ctx.draw_series(LineSeries::new(data.clone(), &BLACK))
        .unwrap()
        .label("spline")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));
}