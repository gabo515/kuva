//! Streamgraph documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example streamgraph
//! ```
//!
//! SVGs are written to `docs/src/assets/streamgraph/`.

use kuva::backend::svg::SvgBackend;
use kuva::plot::streamgraph::{StreamBaseline, StreamOrder, StreamgraphPlot};
use kuva::render::layout::Layout;
use kuva::render::palette::Palette;
use kuva::render::plots::Plot;
use kuva::render::render::render_multiple;

const OUT: &str = "docs/src/assets/streamgraph";

fn main() {
    std::fs::create_dir_all(OUT).expect("could not create docs/src/assets/streamgraph");

    microbiome_wiggle();
    microbiome_symmetric();
    microbiome_zero();
    microbiome_normalized();
    microbiome_by_total();
    microbiome_stroke();
    simple_linear();

    println!("Streamgraph SVGs written to {OUT}/");
}

// ── Shared data ───────────────────────────────────────────────────────────────

/// Weekly microbiome-like abundance data: 6 species × 52 weeks.
fn microbiome_data() -> (Vec<f64>, Vec<(&'static str, Vec<f64>)>) {
    use std::f64::consts::PI;
    let weeks: Vec<f64> = (1..=52).map(|w| w as f64).collect();
    let n = weeks.len();

    let mut rng_state: u64 = 42;
    let mut rng = move || -> f64 {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let bits = (rng_state >> 33) as f64;
        (bits / (u32::MAX as f64)) * 2.0 - 1.0 // [-1, 1]
    };

    let series: Vec<(&str, Vec<f64>)> = vec![
        ("Firmicutes", (0..n).map(|i| {
            let w = weeks[i];
            (350.0 + 80.0 * (w * PI / 26.0).sin() + rng() * 20.0).max(10.0)
        }).collect()),
        ("Bacteroidetes", (0..n).map(|i| {
            let w = weeks[i];
            (260.0 - 60.0 * (w * PI / 26.0).sin() + rng() * 20.0).max(10.0)
        }).collect()),
        ("Proteobacteria", (0..n).map(|i| {
            let w = weeks[i];
            (180.0 + 30.0 * (w * PI / 13.0).cos() + rng() * 15.0).max(10.0)
        }).collect()),
        ("Actinobacteria", (0..n).map(|i| {
            let w = weeks[i];
            (120.0 + 20.0 * (w * PI / 8.67).sin() + rng() * 10.0).max(5.0)
        }).collect()),
        ("Fusobacteria", (0..n).map(|_| {
            (80.0 + rng() * 12.0).max(5.0)
        }).collect()),
        ("Verrucomicrobia", (0..n).map(|i| {
            let w = weeks[i];
            (40.0 + 35.0 * (w * PI / 26.0 * 2.0 + 1.0).sin() + rng() * 8.0).max(5.0)
        }).collect()),
    ];

    (weeks, series)
}

fn build_plot(weeks: Vec<f64>, series: Vec<(&str, Vec<f64>)>) -> StreamgraphPlot {
    let pal = Palette::category10();
    let mut sg = StreamgraphPlot::new().with_x(weeks);
    for (i, (name, vals)) in series.into_iter().enumerate() {
        sg = sg.with_series(vals).with_color(pal[i].to_string()).with_label(name);
    }
    sg
}

#[allow(dead_code)]
fn save(plot: StreamgraphPlot, layout: Layout, name: &str) {
    let plots = vec![Plot::Streamgraph(plot)];
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/{name}"), svg).unwrap();
}

// ── Examples ──────────────────────────────────────────────────────────────────

/// Wiggle baseline (default) — minimises visual motion in the silhouette.
fn microbiome_wiggle() {
    let (weeks, series) = microbiome_data();
    let sg = build_plot(weeks, series);
    let plots = vec![Plot::Streamgraph(sg)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gut microbiome — wiggle baseline")
        .with_x_label("Week")
        .with_y_label("Relative abundance");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/wiggle.svg"), svg).unwrap();
}

/// ThemeRiver symmetric baseline — always centred at y = 0.
fn microbiome_symmetric() {
    let (weeks, series) = microbiome_data();
    let sg = build_plot(weeks, series).with_baseline(StreamBaseline::Symmetric);
    let plots = vec![Plot::Streamgraph(sg)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gut microbiome — symmetric baseline")
        .with_x_label("Week");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/symmetric.svg"), svg).unwrap();
}

/// Zero baseline — familiar stacked area layout with smooth curves.
fn microbiome_zero() {
    let (weeks, series) = microbiome_data();
    let sg = build_plot(weeks, series).with_baseline(StreamBaseline::Zero);
    let plots = vec![Plot::Streamgraph(sg)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gut microbiome — zero baseline")
        .with_x_label("Week")
        .with_y_label("Abundance");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/zero.svg"), svg).unwrap();
}

/// 100 % normalised — shows proportional composition.
fn microbiome_normalized() {
    let (weeks, series) = microbiome_data();
    let sg = build_plot(weeks, series).with_normalized().with_legend("");
    let plots = vec![Plot::Streamgraph(sg)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gut microbiome — 100 % normalised")
        .with_x_label("Week")
        .with_y_label("Proportion (%)");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/normalized.svg"), svg).unwrap();
}

/// ByTotal ordering with legend — largest stream at the bottom.
fn microbiome_by_total() {
    let (weeks, series) = microbiome_data();
    let sg = build_plot(weeks, series)
        .with_order(StreamOrder::ByTotal)
        .with_legend("Phylum");
    let plots = vec![Plot::Streamgraph(sg)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gut microbiome — by-total ordering")
        .with_x_label("Week");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/by_total.svg"), svg).unwrap();
}

/// With white inter-stream strokes for cleaner separation.
fn microbiome_stroke() {
    let (weeks, series) = microbiome_data();
    let sg = build_plot(weeks, series).with_stroke().with_stream_labels(false).with_legend("");
    let plots = vec![Plot::Streamgraph(sg)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gut microbiome — with strokes")
        .with_x_label("Week");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/stroke.svg"), svg).unwrap();
}

/// Linear interpolation — straight-edged "stacked area" look.
fn simple_linear() {
    let weeks: Vec<f64> = (1..=12).map(|w| w as f64).collect();
    let pal = Palette::category10();
    let sg = StreamgraphPlot::new()
        .with_x(weeks)
        .with_series([10.0, 14.0, 18.0, 22.0, 20.0, 16.0,
                      12.0, 18.0, 24.0, 28.0, 22.0, 16.0])
        .with_color(pal[0].to_string()).with_label("Alpha")
        .with_series([5.0, 8.0, 12.0, 15.0, 14.0, 10.0,
                      8.0, 11.0, 16.0, 18.0, 14.0, 9.0])
        .with_color(pal[1].to_string()).with_label("Beta")
        .with_series([3.0, 4.0, 6.0, 8.0, 9.0, 7.0,
                      5.0, 7.0, 9.0, 10.0, 8.0, 5.0])
        .with_color(pal[2].to_string()).with_label("Gamma")
        .with_linear();

    let plots = vec![Plot::Streamgraph(sg)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Linear interpolation")
        .with_x_label("Week");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/linear.svg"), svg).unwrap();
}
