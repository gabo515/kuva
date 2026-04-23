//! Waffle chart documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example waffle
//! ```
//!
//! SVGs are written to `docs/src/assets/waffle/`.

use kuva::plot::waffle::{WafflePlot, CellShape};
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use std::fs;

const OUT: &str = "docs/src/assets/waffle";

fn write(name: &str, plots: Vec<Plot>, layout: Layout) {
    fs::create_dir_all(OUT).unwrap();
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
}

fn main() {
    // ── Basic waffle chart (3 categories, 10×10 grid) ─────────────────────────
    let plot = WafflePlot::new()
        .with_category("Completed", 53, "#4dac26")
        .with_category("In Progress", 27, "#d01c8b")
        .with_category("Pending", 20, "#b8b8b8")
        .with_legend("Status");
    let plots = vec![Plot::Waffle(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Project Task Status")
        .with_width(400.0)
        .with_height(380.0);
    write("basic", plots, layout);

    // ── Wide-aspect waffle (5×20 grid) ───────────────────────────────────────
    let plot = WafflePlot::new()
        .with_category("Pass", 72, "#2ca02c")
        .with_category("Fail", 18, "#d62728")
        .with_category("Skip", 10, "#aec7e8")
        .with_grid(5, 20)
        .with_legend("Result");
    let plots = vec![Plot::Waffle(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Test Suite Results")
        .with_width(560.0)
        .with_height(280.0);
    write("wide", plots, layout);

    // ── Circular cells ───────────────────────────────────────────────────────
    let plot = WafflePlot::new()
        .with_category("Vaccinated", 68, "#1f77b4")
        .with_category("Partial", 15, "#aec7e8")
        .with_category("Unvaccinated", 17, "#ff7f0e")
        .with_shape(CellShape::Circle)
        .with_legend("Status");
    let plots = vec![Plot::Waffle(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Vaccination Coverage")
        .with_width(400.0)
        .with_height(380.0);
    write("circles", plots, layout);

    // ── Unit label with count legend ─────────────────────────────────────────
    let plot = WafflePlot::new()
        .with_category("Urban", 56, "#e377c2")
        .with_category("Suburban", 28, "#7f7f7f")
        .with_category("Rural", 16, "#bcbd22")
        .with_unit_label("= 1M people")
        .with_show_counts()
        .with_legend("Area type");
    let plots = vec![Plot::Waffle(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Population Distribution")
        .with_width(400.0)
        .with_height(380.0);
    write("unit_label", plots, layout);
}
