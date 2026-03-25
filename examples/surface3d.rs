use kuva::prelude::*;

fn main() {
    // Paraboloid surface with viridis colormap
    let n = 25;
    let z_data: Vec<Vec<f64>> = (0..n).map(|i| {
        (0..n).map(|j| {
            let x = (i as f64 - n as f64 / 2.0) / (n as f64 / 4.0);
            let y = (j as f64 - n as f64 / 2.0) / (n as f64 / 4.0);
            x * x + y * y
        }).collect()
    }).collect();

    let surface = Surface3DPlot::new(z_data)
        .with_z_colormap(ColorMap::Viridis)
        .with_x_label("X")
        .with_y_label("Y")
        .with_z_label("Z");

    let plots = vec![Plot::Surface3D(surface)];
    let layout = Layout::auto_from_plots(&plots).with_title("Surface3D — Paraboloid");
    let svg = render_to_svg(plots, layout);
    std::fs::write("docs/src/assets/surface3d_paraboloid.svg", &svg).unwrap();
    println!("wrote docs/src/assets/surface3d_paraboloid.svg");

    // Sin/cos wave
    let n = 30;
    let z_data: Vec<Vec<f64>> = (0..n).map(|i| {
        (0..n).map(|j| {
            let x = i as f64 * 0.3;
            let y = j as f64 * 0.3;
            (x.sin() + y.cos()) * 2.0
        }).collect()
    }).collect();

    let surface = Surface3DPlot::new(z_data)
        .with_z_colormap(ColorMap::Inferno)
        .with_alpha(0.9);

    let plots = vec![Plot::Surface3D(surface)];
    let layout = Layout::auto_from_plots(&plots).with_title("Surface3D — Wave");
    let svg = render_to_svg(plots, layout);
    std::fs::write("docs/src/assets/surface3d_wave.svg", &svg).unwrap();
    println!("wrote docs/src/assets/surface3d_wave.svg");
}
