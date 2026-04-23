//! Radar / spider chart documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example radar
//! ```
//!
//! SVGs are written to `docs/src/assets/radar/`.

use kuva::plot::radar::RadarPlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use std::fs;

const OUT: &str = "docs/src/assets/radar";

fn write(name: &str, plots: Vec<Plot>, layout: Layout) {
    fs::create_dir_all(OUT).unwrap();
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
}

fn main() {
    // ── Basic two-series radar ────────────────────────────────────────────────
    let plot = RadarPlot::new(["Speed", "Power", "Agility", "Stamina", "Technique"])
        .with_series_labeled([0.80, 0.60, 0.90, 0.70, 0.75], "Group A")
        .with_series_labeled([0.60, 0.90, 0.50, 0.80, 0.70], "Group B")
        .with_range(0.0, 1.0)
        .with_legend(true);
    let plots = vec![Plot::Radar(plot)];
    let layout = Layout::auto_from_plots(&plots).with_title("Team Performance");
    write("basic", plots, layout);

    // ── Filled polygons with three series ────────────────────────────────────
    let plot = RadarPlot::new(["Attack", "Defense", "Speed", "Magic", "Stamina"])
        .with_series_labeled([9.0, 5.0, 7.0, 3.0, 8.0], "Warrior")
        .with_series_labeled([4.0, 7.0, 6.0, 9.0, 5.0], "Mage")
        .with_series_labeled([6.0, 4.0, 10.0, 5.0, 6.0], "Rogue")
        .with_filled(true)
        .with_opacity(0.25)
        .with_range(0.0, 10.0)
        .with_legend(true);
    let plots = vec![Plot::Radar(plot)];
    let layout = Layout::auto_from_plots(&plots).with_title("Character Stats");
    write("filled", plots, layout);
}
