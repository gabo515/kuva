//! Population pyramid documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example pyramid
//! ```
//!
//! SVGs are written to `docs/src/assets/pyramid/`.

use kuva::plot::pyramid::PopulationPyramid;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use std::fs;

const OUT: &str = "docs/src/assets/pyramid";

fn write(name: &str, plots: Vec<Plot>, layout: Layout) {
    fs::create_dir_all(OUT).unwrap();
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
}

const AGE_GROUPS: &[&str] = &[
    "0–4", "5–9", "10–14", "15–19", "20–24", "25–29", "30–34",
    "35–39", "40–44", "45–49", "50–54", "55–59", "60–64", "65–69",
    "70–74", "75–79", "80+",
];

fn main() {
    // ── Basic single-series pyramid ───────────────────────────────────────────
    let male   = [2.1, 2.3, 2.4, 2.6, 3.0, 3.2, 3.4, 3.3, 3.1, 2.9, 2.7, 2.4, 2.0, 1.6, 1.2, 0.8, 0.5f64];
    let female = [2.0, 2.2, 2.3, 2.5, 2.9, 3.1, 3.3, 3.2, 3.0, 2.8, 2.6, 2.3, 2.0, 1.7, 1.3, 1.0, 0.7f64];

    let groups: Vec<_> = AGE_GROUPS.iter().zip(male.iter().zip(female.iter()))
        .map(|(age, (m, f))| (*age, *m, *f))
        .collect();

    let plot = PopulationPyramid::new()
        .with_series("2020", groups.clone())
        .with_left_label("Male")
        .with_right_label("Female");
    let plots = vec![Plot::Pyramid(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Population Pyramid 2020")
        .with_x_label("Population (millions)");
    write("basic", plots, layout);

    // ── Normalized (percentage) ───────────────────────────────────────────────
    let plot = PopulationPyramid::new()
        .with_series("2020", groups.clone())
        .with_normalize(true)
        .with_left_label("Male")
        .with_right_label("Female");
    let plots = vec![Plot::Pyramid(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Population Structure 2020 (%)")
        .with_x_label("Share of population (%)");
    write("normalized", plots, layout);

    // ── Multi-series comparison ───────────────────────────────────────────────
    let male_2000   = [2.4, 2.5, 2.6, 2.8, 3.1, 3.3, 3.2, 3.0, 2.8, 2.5, 2.2, 1.9, 1.5, 1.1, 0.8, 0.5, 0.3f64];
    let female_2000 = [2.3, 2.4, 2.5, 2.7, 3.0, 3.2, 3.1, 2.9, 2.7, 2.4, 2.2, 1.9, 1.6, 1.2, 0.9, 0.6, 0.4f64];

    let groups_2000: Vec<_> = AGE_GROUPS.iter().zip(male_2000.iter().zip(female_2000.iter()))
        .map(|(age, (m, f))| (*age, *m, *f))
        .collect();

    let plot = PopulationPyramid::new()
        .with_series("2000", groups_2000)
        .with_series("2020", groups)
        .with_left_label("Male")
        .with_right_label("Female")
        .with_legend(true);
    let plots = vec![Plot::Pyramid(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Population Change 2000 vs 2020")
        .with_x_label("Population (millions)");
    write("multi_series", plots, layout);
}
