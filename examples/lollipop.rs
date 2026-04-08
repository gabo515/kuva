//! Lollipop plot documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example lollipop
//! ```
//!
//! SVGs are written to `docs/src/assets/lollipop/`.

use kuva::plot::LollipopPlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use std::fs;

const OUT: &str = "docs/src/assets/lollipop";

fn write(name: &str, plots: Vec<Plot>, layout: Layout) {
    fs::create_dir_all(OUT).unwrap();
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
}

fn main() {
    // ── Basic ──────────────────────────────────────────────────────────────
    let lp = LollipopPlot::new()
        .with_point(1.0, 3.0)
        .with_point(2.0, 1.0)
        .with_point(3.0, 7.0)
        .with_point(4.0, 2.0)
        .with_point(5.0, 5.0)
        .with_point(6.0, 4.0)
        .with_point(7.0, 8.0)
        .with_point(8.0, 2.0);
    let plots = vec![Plot::Lollipop(lp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Lollipop Plot")
        .with_x_label("Category")
        .with_y_label("Value");
    write("basic", plots, layout);

    // ── Mutation landscape with domain annotations ─────────────────────────
    let mutations: Vec<(f64, f64)> = vec![
        (8.0, 1.0), (12.0, 3.0), (19.0, 1.0), (25.0, 2.0),
        (41.0, 7.0), (53.0, 2.0), (55.0, 1.0), (62.0, 4.0),
        (79.0, 1.0), (88.0, 3.0), (97.0, 5.0), (110.0, 2.0),
        (124.0, 8.0), (131.0, 1.0), (138.0, 3.0), (150.0, 1.0),
        (158.0, 2.0), (163.0, 6.0), (172.0, 1.0), (180.0, 2.0),
    ];
    let lp = LollipopPlot::new()
        .with_points(mutations)
        .with_labeled_colored_point(41.0, 7.0, "R175H", "tomato")
        .with_labeled_colored_point(124.0, 8.0, "R248W", "tomato")
        .with_labeled_colored_point(163.0, 6.0, "R273H", "tomato")
        .with_domain(1.0, 67.0, Some("N-terminal"), "#4e79a7")
        .with_domain(68.0, 130.0, Some("DNA-binding"), "#f28e2b")
        .with_domain(131.0, 185.0, Some("C-terminal"), "#59a14f")
        .with_domain_height(1.5)
        .with_color("steelblue")
        .with_dot_radius(4.5)
        .with_stem_width(1.5);
    let plots = vec![Plot::Lollipop(lp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("TP53 Mutation Landscape")
        .with_x_label("Amino acid position")
        .with_y_label("Mutation count")
        .with_width(620.0)
        .with_height(360.0);
    write("mutation_landscape", plots, layout);

    // ── Mixed signs (log2 fold-change style) ──────────────────────────────
    let lp = LollipopPlot::new()
        .with_point(1.0,  2.1)
        .with_point(2.0, -1.3)
        .with_point(3.0,  3.5)
        .with_point(4.0, -2.8)
        .with_point(5.0,  1.4)
        .with_point(6.0, -0.7)
        .with_point(7.0,  2.9)
        .with_point(8.0, -1.9)
        .with_baseline(0.0)
        .with_baseline_dash("4,3")
        .with_color("steelblue");
    let plots = vec![Plot::Lollipop(lp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gene Expression Changes")
        .with_x_label("Gene")
        .with_y_label("log₂ fold change");
    write("mixed_signs", plots, layout);

    // ── Custom styling ─────────────────────────────────────────────────────
    let lp = LollipopPlot::new()
        .with_colored_point(1.0, 4.0, "#e15759")
        .with_colored_point(2.0, 7.0, "#4e79a7")
        .with_colored_point(3.0, 2.0, "#f28e2b")
        .with_colored_point(4.0, 9.0, "#76b7b2")
        .with_colored_point(5.0, 5.0, "#59a14f")
        .with_colored_point(6.0, 3.0, "#edc948")
        .with_dot_radius(7.0)
        .with_dot_stroke("white")
        .with_dot_stroke_width(1.5)
        .with_stem_width(2.0);
    let plots = vec![Plot::Lollipop(lp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Per-point colors")
        .with_x_label("Sample")
        .with_y_label("Score");
    write("per_point_colors", plots, layout);
}
