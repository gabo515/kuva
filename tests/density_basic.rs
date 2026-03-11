use kuva::plot::DensityPlot;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::render::render::render_multiple;
use kuva::render::palette::Palette;
use kuva::backend::svg::SvgBackend;
use std::fs;

fn render_svg(plots: Vec<Plot>, layout: Layout) -> String {
    let scene = render_multiple(plots, layout);
    SvgBackend.render_scene(&scene)
}

fn outdir() {
    fs::create_dir_all("test_outputs").ok();
}

#[test]
fn test_density_basic() {
    outdir();
    let data: Vec<f64> = (0..100).map(|i| (i as f64) * 0.1).collect();
    let dp = DensityPlot::new()
        .with_data(data)
        .with_color("steelblue");
    let plots = vec![Plot::Density(dp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Density Basic")
        .with_x_label("Value")
        .with_y_label("Density");
    let svg = render_svg(plots, layout);
    fs::write("test_outputs/density_basic.svg", &svg).unwrap();
    assert!(svg.contains("<svg"), "output should contain <svg tag");
    assert!(svg.contains("<path"), "output should contain a <path element");
}

#[test]
fn test_density_filled() {
    outdir();
    let data: Vec<f64> = vec![1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 2.0, 2.5, 3.0];
    let dp = DensityPlot::new()
        .with_data(data)
        .with_color("coral")
        .with_filled(true)
        .with_opacity(0.4);
    let plots = vec![Plot::Density(dp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Density Filled");
    let svg = render_svg(plots, layout);
    fs::write("test_outputs/density_filled.svg", &svg).unwrap();
    assert!(svg.contains("<svg"));
    // The filled path should include a Z (close path) command
    assert!(svg.contains('Z'), "filled density path should contain 'Z' close command");
}

#[test]
fn test_density_bandwidth() {
    outdir();
    let data: Vec<f64> = vec![1.0, 2.0, 2.1, 2.9, 3.0, 3.1, 4.0, 4.5, 5.0];
    let dp = DensityPlot::new()
        .with_data(data)
        .with_color("purple")
        .with_bandwidth(0.3);
    let plots = vec![Plot::Density(dp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Density Custom Bandwidth");
    let svg = render_svg(plots, layout);
    fs::write("test_outputs/density_bandwidth.svg", &svg).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("<path"));
}

#[test]
fn test_density_legend() {
    outdir();
    let data: Vec<f64> = vec![1.0, 2.0, 2.5, 3.0, 3.5, 4.0, 2.2, 2.8, 3.2];
    let dp = DensityPlot::new()
        .with_data(data)
        .with_color("teal")
        .with_legend("Group A");
    let plots = vec![Plot::Density(dp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Density Legend")
        .with_legend_position(kuva::plot::LegendPosition::OutsideRightTop);
    let svg = render_svg(plots, layout);
    fs::write("test_outputs/density_legend.svg", &svg).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Group A"), "legend label 'Group A' should appear in SVG");
}

#[test]
fn test_density_multigroup() {
    outdir();
    let pal = Palette::category10();
    let group_a: Vec<f64> = vec![1.0, 1.5, 2.0, 2.5, 3.0];
    let group_b: Vec<f64> = vec![2.5, 3.0, 3.5, 4.0, 4.5];
    let group_c: Vec<f64> = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0];

    let plots = vec![
        Plot::Density(
            DensityPlot::new().with_data(group_a).with_color(pal[0].to_string()).with_legend("A")
        ),
        Plot::Density(
            DensityPlot::new().with_data(group_b).with_color(pal[1].to_string()).with_legend("B")
        ),
        Plot::Density(
            DensityPlot::new().with_data(group_c).with_color(pal[2].to_string()).with_legend("C")
        ),
    ];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Density Multigroup")
        .with_legend_position(kuva::plot::LegendPosition::OutsideRightTop);
    let svg = render_svg(plots, layout);
    fs::write("test_outputs/density_multigroup.svg", &svg).unwrap();
    assert!(svg.contains("<svg"));
    // Should have multiple path elements
    let path_count = svg.matches("<path").count();
    assert!(path_count >= 3, "expected at least 3 path elements, got {path_count}");
}

#[test]
fn test_density_precomputed() {
    outdir();
    let x = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![0.05, 0.2, 0.5, 0.4, 0.2, 0.05];
    let dp = DensityPlot::from_curve(x, y).with_color("orange");
    let plots = vec![Plot::Density(dp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Density Precomputed");
    let svg = render_svg(plots, layout);
    fs::write("test_outputs/density_precomputed.svg", &svg).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("<path"), "precomputed density should emit a path");
}
