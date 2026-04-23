//! Horizon chart documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example horizon
//! ```
//!
//! SVGs are written to `docs/src/assets/horizon/`.

use kuva::plot::horizon::HorizonPlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use std::fs;

const OUT: &str = "docs/src/assets/horizon";

fn write(name: &str, plots: Vec<Plot>, layout: Layout) {
    fs::create_dir_all(OUT).unwrap();
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
}

fn main() {
    let hours: Vec<f64> = (0..48).map(|i| i as f64).collect();

    let anomaly_a: Vec<f64> = hours.iter().map(|&t| (t * 0.3).sin() * 4.0 + (t * 0.1).cos() * 2.0).collect();
    let anomaly_b: Vec<f64> = hours.iter().map(|&t| -(t * 0.25).cos() * 3.5 + (t * 0.15).sin() * 1.5).collect();
    let anomaly_c: Vec<f64> = hours.iter().map(|&t| (t * 0.2).sin() * 5.0 - (t * 0.05).cos() * 2.5).collect();

    // ── Basic three-series horizon ────────────────────────────────────────────
    let plot = HorizonPlot::new()
        .with_series("Station A", hours.clone(), anomaly_a.clone())
        .with_series("Station B", hours.clone(), anomaly_b.clone())
        .with_series("Station C", hours.clone(), anomaly_c.clone())
        .with_n_bands(3)
        .with_row_height(40.0);
    let plots = vec![Plot::Horizon(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Temperature Anomaly (°C)")
        .with_x_label("Hour");
    write("basic", plots, layout);

    // ── With value labels (scale annotations) ────────────────────────────────
    let plot = HorizonPlot::new()
        .with_series("Station A", hours.clone(), anomaly_a)
        .with_series("Station B", hours.clone(), anomaly_b)
        .with_series("Station C", hours, anomaly_c)
        .with_n_bands(4)
        .with_row_height(40.0)
        .with_value_labels(true);
    let plots = vec![Plot::Horizon(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Temperature Anomaly with Band Labels")
        .with_x_label("Hour");
    write("value_labels", plots, layout);
}
