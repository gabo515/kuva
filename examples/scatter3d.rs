use kuva::prelude::*;

fn main() {
    // Basic 3D scatter
    let data: Vec<(f64, f64, f64)> = (0..50)
        .map(|i| {
            let t = i as f64 / 49.0 * std::f64::consts::TAU;
            (t.cos() * (1.0 + t * 0.3), t.sin() * (1.0 + t * 0.3), t)
        })
        .collect();

    let plot = Scatter3DPlot::new()
        .with_data(data)
        .with_color("steelblue")
        .with_x_label("X")
        .with_y_label("Y")
        .with_z_label("Z");

    let plots = vec![Plot::Scatter3D(plot)];
    let layout = Layout::auto_from_plots(&plots).with_title("3D Scatter — Helix");
    let svg = render_to_svg(plots, layout);
    std::fs::write("docs/assets/scatter3d_basic.svg", &svg).unwrap();
    println!("wrote docs/assets/scatter3d_basic.svg");

    // Z-colored scatter
    let data: Vec<(f64, f64, f64)> = (0..100)
        .map(|i| {
            let t = i as f64 / 99.0;
            let x = t * 10.0 - 5.0;
            let y = (t * 6.0).sin() * 3.0;
            let z = x * x + y * y;
            (x, y, z)
        })
        .collect();

    let plot = Scatter3DPlot::new()
        .with_data(data)
        .with_z_colormap(ColorMap::Viridis)
        .with_x_label("X")
        .with_y_label("Y")
        .with_z_label("Z = X² + Y²");

    let plots = vec![Plot::Scatter3D(plot)];
    let layout = Layout::auto_from_plots(&plots).with_title("3D Scatter — Z Colormap");
    let svg = render_to_svg(plots, layout);
    std::fs::write("docs/assets/scatter3d_zcolor.svg", &svg).unwrap();
    println!("wrote docs/assets/scatter3d_zcolor.svg");

    // Different view angles
    let data: Vec<(f64, f64, f64)> = (0..30)
        .map(|i| {
            let t = i as f64 / 29.0;
            (t * 10.0, (t * 4.0).sin() * 5.0, (t * 3.0).cos() * 5.0)
        })
        .collect();

    let plot = Scatter3DPlot::new()
        .with_data(data)
        .with_color("crimson")
        .with_azimuth(-120.0)
        .with_elevation(20.0)
        .with_depth_shade(true)
        .with_x_label("X")
        .with_y_label("Y")
        .with_z_label("Z");

    let plots = vec![Plot::Scatter3D(plot)];
    let layout = Layout::auto_from_plots(&plots).with_title("3D Scatter — Custom View");
    let svg = render_to_svg(plots, layout);
    std::fs::write("docs/assets/scatter3d_view.svg", &svg).unwrap();
    println!("wrote docs/assets/scatter3d_view.svg");
}
