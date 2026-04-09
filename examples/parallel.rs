//! Parallel coordinates documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example parallel
//! ```
//!
//! SVGs are written to `docs/src/assets/parallel/`.

use kuva::plot::parallel::ParallelPlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use std::fs;

const OUT: &str = "docs/src/assets/parallel";

fn save(name: &str, plot: ParallelPlot, title: &str) {
    let plots = vec![Plot::Parallel(plot)];
    let layout = Layout::auto_from_plots(&plots).with_title(title);
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::create_dir_all(OUT).unwrap();
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
    println!("wrote {OUT}/{name}.svg");
}

fn main() {
    // ── 1. Iris (3 groups, 4 dimensions) ──────────────────────────────────────
    let iris_data: &[(&str, &[[f64; 4]])] = &[
        ("setosa", &[
            [5.1, 3.5, 1.4, 0.2], [4.9, 3.0, 1.4, 0.2], [4.7, 3.2, 1.3, 0.2],
            [4.6, 3.1, 1.5, 0.2], [5.0, 3.6, 1.4, 0.2], [5.4, 3.9, 1.7, 0.4],
            [4.6, 3.4, 1.4, 0.3], [5.0, 3.4, 1.5, 0.2], [4.4, 2.9, 1.4, 0.2],
            [4.9, 3.1, 1.5, 0.1],
        ]),
        ("versicolor", &[
            [7.0, 3.2, 4.7, 1.4], [6.4, 3.2, 4.5, 1.5], [6.9, 3.1, 4.9, 1.5],
            [5.5, 2.3, 4.0, 1.3], [6.5, 2.8, 4.6, 1.5], [5.7, 2.8, 4.5, 1.3],
            [6.3, 3.3, 4.7, 1.6], [4.9, 2.4, 3.3, 1.0], [6.6, 2.9, 4.6, 1.3],
            [5.2, 2.7, 3.9, 1.4],
        ]),
        ("virginica", &[
            [6.3, 3.3, 6.0, 2.5], [5.8, 2.7, 5.1, 1.9], [7.1, 3.0, 5.9, 2.1],
            [6.3, 2.9, 5.6, 1.8], [6.5, 3.0, 5.8, 2.2], [7.6, 3.0, 6.6, 2.1],
            [4.9, 2.5, 4.5, 1.7], [7.3, 2.9, 6.3, 1.8], [6.7, 2.5, 5.8, 1.8],
            [7.2, 3.6, 6.1, 2.5],
        ]),
    ];

    let mut iris = ParallelPlot::new()
        .with_axis_names(["Sepal Length", "Sepal Width", "Petal Length", "Petal Width"])
        .with_legend("Species")
        .with_opacity(0.65);
    for (group, rows) in iris_data {
        for row in *rows {
            iris = iris.with_row_group(*group, row.to_vec());
        }
    }
    save("iris", iris, "Iris — Parallel Coordinates");

    // ── 2. Iris — curved lines + mean overlay ─────────────────────────────────
    let mut iris_curved = ParallelPlot::new()
        .with_axis_names(["Sepal Length", "Sepal Width", "Petal Length", "Petal Width"])
        .with_legend("Species")
        .with_curved(true)
        .with_mean(true)
        .with_opacity(0.35);
    for (group, rows) in iris_data {
        for row in *rows {
            iris_curved = iris_curved.with_row_group(*group, row.to_vec());
        }
    }
    save("iris_curved", iris_curved, "Iris — Curved Lines + Group Means");

    // ── 3. Iris with axis bands and inverted axis ──────────────────────────────
    let mut iris_bands = ParallelPlot::new()
        .with_axis_names(["Sepal Length", "Sepal Width", "Petal Length", "Petal Width"])
        .with_legend("Species")
        .with_axis_bands(true)
        .with_invert_axis(1)   // invert Sepal Width so "narrow → top"
        .with_opacity(0.65);
    for (group, rows) in iris_data {
        for row in *rows {
            iris_bands = iris_bands.with_row_group(*group, row.to_vec());
        }
    }
    save("iris_bands", iris_bands, "Iris — Axis Bands + Inverted Sepal Width");

    // ── 3. Gene expression profiles ────────────────────────────────────────────
    let conditions = ["Control", "Treated", "Recovery", "Washout"];
    let mut expr = ParallelPlot::new()
        .with_axis_names(conditions)
        .with_legend("Cluster")
        .with_opacity(0.5)
        .with_stroke_width(1.5);

    // Induced cluster: low → high → low
    for i in 0..12 {
        let b = 1.5 + i as f64 * 0.1;
        expr = expr.with_row_group("Induced", vec![b, b + 3.5, b + 1.0, b + 0.2]);
    }
    // Repressed cluster: high → low
    for i in 0..10 {
        let b = 5.0 - i as f64 * 0.2;
        expr = expr.with_row_group("Repressed", vec![b, b - 2.0, b - 2.5, b - 2.0]);
    }
    // Stable cluster
    for i in 0..8 {
        let b = 3.5 + i as f64 * 0.05;
        expr = expr.with_row_group("Stable", vec![b, b + 0.1, b - 0.1, b + 0.05]);
    }
    save("gene_expression", expr, "Gene Expression Profiles");

    // ── 4. Shared scale (normalize = false) ───────────────────────────────────
    let shared = ParallelPlot::new()
        .with_axis_names(["Q1 Score", "Q2 Score", "Q3 Score", "Q4 Score"])
        .with_normalize(false)
        .with_legend("Cohort")
        .with_group_colors(["steelblue", "tomato"])
        .with_group_rows("Cohort A", vec![
            vec![70.0, 80.0, 75.0, 85.0],
            vec![65.0, 72.0, 68.0, 78.0],
            vec![80.0, 85.0, 90.0, 88.0],
        ])
        .with_group_rows("Cohort B", vec![
            vec![55.0, 60.0, 58.0, 62.0],
            vec![72.0, 78.0, 80.0, 76.0],
            vec![48.0, 52.0, 55.0, 50.0],
        ]);
    save("shared_scale", shared, "Shared Scale Parallel Plot");
}
