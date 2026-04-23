//! Precision-recall curve documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example pr
//! ```
//!
//! SVGs are written to `docs/src/assets/pr/`.

use kuva::plot::pr::{PrPlot, PrGroup};
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use std::fs;

const OUT: &str = "docs/src/assets/pr";

fn write(name: &str, plots: Vec<Plot>, layout: Layout) {
    fs::create_dir_all(OUT).unwrap();
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
}

fn logistic_dataset(n: usize, mu: f64, scale: f64) -> Vec<(f64, bool)> {
    let mut data = Vec::with_capacity(2 * n);
    for i in 1..=n {
        let p = i as f64 / (n + 1) as f64;
        let logit = (p / (1.0 - p)).ln();
        let pos_score = 1.0 / (1.0 + (-(mu + scale * logit)).exp());
        let neg_score = 1.0 / (1.0 + (-(-mu + scale * logit)).exp());
        data.push((pos_score, true));
        data.push((neg_score, false));
    }
    data
}

fn main() {
    // ── Basic single-group PR curve ───────────────────────────────────────────
    let group = PrGroup::new("Classifier").with_raw(logistic_dataset(150, 1.0, 0.5));
    let pr = PrPlot::new().with_group(group);
    let plots = vec![Plot::Pr(pr)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Precision-Recall Curve")
        .with_x_label("Recall")
        .with_y_label("Precision");
    write("basic", plots, layout);

    // ── With optimal F1 threshold marker ─────────────────────────────────────
    let group = PrGroup::new("Classifier")
        .with_raw(logistic_dataset(150, 1.0, 0.5))
        .with_optimal_point()
        .with_auc_label(true);
    let pr = PrPlot::new().with_group(group);
    let plots = vec![Plot::Pr(pr)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("PR Curve with Optimal F1 Threshold")
        .with_x_label("Recall")
        .with_y_label("Precision");
    write("optimal", plots, layout);

    // ── Three-model comparison ────────────────────────────────────────────────
    let g1 = PrGroup::new("Model A")
        .with_raw(logistic_dataset(150, 1.2, 0.5))
        .with_auc_label(true);
    let g2 = PrGroup::new("Model B")
        .with_raw(logistic_dataset(150, 0.6, 0.5))
        .with_auc_label(true);
    let g3 = PrGroup::new("Model C")
        .with_raw(logistic_dataset(150, 0.2, 0.5))
        .with_auc_label(true);
    let pr = PrPlot::new()
        .with_group(g1)
        .with_group(g2)
        .with_group(g3)
        .with_legend("Classifier");
    let plots = vec![Plot::Pr(pr)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Multi-model Precision-Recall Comparison")
        .with_x_label("Recall")
        .with_y_label("Precision")
        .with_width(520.0)
        .with_height(440.0);
    write("multi_model", plots, layout);
}
