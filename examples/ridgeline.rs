//! Ridgeline plot examples.
use kuva::plot::ridgeline::RidgelinePlot;
use kuva::render::plots::Plot;
use kuva::render::layout::Layout;
use kuva::render::render::render_multiple;
use kuva::backend::svg::SvgBackend;

fn main() {
    // Basic ridgeline with 3 groups
    let plot = RidgelinePlot::new()
        .with_group("Control", vec![1.2, 1.5, 1.8, 2.0, 2.2, 1.9, 1.6, 1.3])
        .with_group("Treatment A", vec![2.5, 3.0, 3.5, 4.0, 3.8, 3.2, 2.8, 3.6])
        .with_group("Treatment B", vec![4.5, 5.0, 5.5, 6.0, 5.8, 5.2, 4.8, 5.3]);

    let plots = vec![Plot::Ridgeline(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Expression by Treatment")
        .with_x_label("Expression Level")
        .with_y_label("Group");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("ridgeline_basic.svg", svg).unwrap();
    println!("Wrote ridgeline_basic.svg");
}
