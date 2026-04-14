//! ECDF plot documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example ecdf
//! ```
//!
//! SVGs are written to `docs/src/assets/ecdf/`.

use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use kuva::plot::EcdfPlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::palette::Palette;
use kuva::render::plots::Plot;

const OUT: &str = "docs/src/assets/ecdf";

fn main() {
    std::fs::create_dir_all(OUT).expect("could not create docs/src/assets/ecdf");

    basic();
    multigroup();
    complementary();
    confidence_band();
    rug();
    percentile_lines();
    markers();
    smooth();

    println!("ECDF SVGs written to {OUT}/");
}

fn normal_samples(mean: f64, std: f64, n: usize, seed: u64) -> Vec<f64> {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
    Normal::new(mean, std).unwrap()
        .sample_iter(&mut rng)
        .take(n)
        .collect()
}

fn basic() {
    let data = normal_samples(0.0, 1.0, 200, 42);

    let plot = EcdfPlot::new()
        .with_data("Sample", data)
        .with_color("steelblue");

    let plots = vec![Plot::Ecdf(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("ECDF")
        .with_x_label("Value")
        .with_y_label("F(x)");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/basic.svg"), svg).unwrap();
}

fn multigroup() {
    let pal = Palette::category10();
    let control  = normal_samples(0.0, 1.0, 150, 1);
    let treated  = normal_samples(1.2, 0.9, 150, 2);

    let plot = EcdfPlot::new()
        .with_data_colored("Control", control, pal[0].to_string())
        .with_data_colored("Treated", treated, pal[1].to_string())
        .with_legend("");

    let plots = vec![Plot::Ecdf(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Treatment vs Control")
        .with_x_label("Value")
        .with_y_label("F(x)");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/multigroup.svg"), svg).unwrap();
}

fn complementary() {
    // Simulated nanopore read-length distribution (log-normal)
    let mut rng = rand::rngs::SmallRng::seed_from_u64(7);
    let data: Vec<f64> = Normal::new(7.5_f64, 1.2).unwrap()
        .sample_iter(&mut rng)
        .take(300)
        .map(|v| v.exp())
        .filter(|&v| v > 100.0)
        .take(200)
        .collect();

    let plot = EcdfPlot::new()
        .with_data("Nanopore run", data)
        .with_color("steelblue")
        .with_complementary();

    let plots = vec![Plot::Ecdf(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Read Length Distribution (CCDF)")
        .with_x_label("Read length (bp)")
        .with_y_label("Fraction ≥ length")
        .with_log_x();

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/complementary.svg"), svg).unwrap();
}

fn confidence_band() {
    let small = normal_samples(0.0, 1.0, 20, 10);
    let large = normal_samples(0.0, 1.0, 150, 11);

    let plot = EcdfPlot::new()
        .with_data("n = 20",  small)
        .with_data("n = 150", large)
        .with_confidence_band()
        .with_legend("");

    let plots = vec![Plot::Ecdf(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("DKW 95% Confidence Bands")
        .with_x_label("Value")
        .with_y_label("F(x)");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/confidence_band.svg"), svg).unwrap();
}

fn rug() {
    let data = normal_samples(0.0, 1.0, 80, 5);

    let plot = EcdfPlot::new()
        .with_data("Sample", data)
        .with_color("steelblue")
        .with_rug();

    let plots = vec![Plot::Ecdf(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("ECDF with Rug")
        .with_x_label("Value")
        .with_y_label("F(x)");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/rug.svg"), svg).unwrap();
}

fn percentile_lines() {
    let data: Vec<f64> = (1..=100).map(|i| i as f64).collect();

    let plot = EcdfPlot::new()
        .with_data("", data)
        .with_color("steelblue")
        .with_percentile_lines(vec![0.25, 0.5, 0.75]);

    let plots = vec![Plot::Ecdf(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("ECDF with Percentile Lines")
        .with_x_label("Value")
        .with_y_label("F(x)");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/percentile_lines.svg"), svg).unwrap();
}

fn markers() {
    let data = vec![1.2, 2.4, 2.9, 3.5, 4.1, 5.0, 5.8, 7.2];

    let plot = EcdfPlot::new()
        .with_data("n = 8", data)
        .with_color("steelblue")
        .with_markers()
        .with_marker_size(4.0);

    let plots = vec![Plot::Ecdf(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("ECDF with Step Markers (small n)")
        .with_x_label("Value")
        .with_y_label("F(x)");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/markers.svg"), svg).unwrap();
}

fn smooth() {
    let pal = Palette::category10();
    let a = normal_samples(0.0, 1.0, 200, 20);
    let b = normal_samples(1.5, 0.8, 200, 21);

    let plot = EcdfPlot::new()
        .with_data_colored("Group A", a, pal[0].to_string())
        .with_data_colored("Group B", b, pal[1].to_string())
        .with_smooth()
        .with_legend("");

    let plots = vec![Plot::Ecdf(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Smooth CDF")
        .with_x_label("Value")
        .with_y_label("F(x)");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/smooth.svg"), svg).unwrap();
}
