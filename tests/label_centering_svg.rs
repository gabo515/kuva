use kuva::plot::scatter::ScatterPlot;
use kuva::plot::line::LinePlot;
use kuva::plot::{PiePlot, PieLabelPosition};
use kuva::render::layout::{Layout, ComputedLayout};
use kuva::render::plots::Plot;
use kuva::render::render::{render_multiple, render_twin_y};
use kuva::backend::svg::SvgBackend;

/// Extract the `x` attribute value from the first SVG `<text>` element whose
/// content matches `text`. Searches for `>text<` in the SVG string, walks
/// backward to the nearest `x="..."` attribute, and parses it as f64.
fn extract_text_x(svg: &str, text: &str) -> Option<f64> {
    let needle = format!(">{}<", text);
    let pos = svg.find(&needle)?;
    // Walk backwards from pos to find x="..."
    let before = &svg[..pos];
    let x_attr = before.rfind("x=\"")?;
    let after_quote = &before[x_attr + 3..];
    let end = after_quote.find('"')?;
    after_quote[..end].parse::<f64>().ok()
}

#[test]
fn test_title_centred_with_legend() {
    let data = vec![(1.0f64, 2.0f64), (3.0, 4.0), (5.0, 6.0)];
    let plot = ScatterPlot::new()
        .with_data(data)
        .with_legend("Group A");
    let plots = vec![Plot::Scatter(plot)];

    let layout = Layout::auto_from_plots(&plots)
        .with_title("MyTitle")
        .with_x_label("MyLabel");

    let computed = ComputedLayout::from_layout(&layout);
    let expected_x = computed.margin_left + computed.plot_width() / 2.0;

    let scene = render_multiple(plots, layout);
    let svg = SvgBackend.render_scene(&scene);
    std::fs::write("test_outputs/label_centering_legend.svg", &svg).unwrap();

    let title_x = extract_text_x(&svg, "MyTitle")
        .expect("title element not found in SVG");
    let label_x = extract_text_x(&svg, "MyLabel")
        .expect("x-label element not found in SVG");

    assert!(
        (title_x - expected_x).abs() < 1.0,
        "title x={title_x:.1} should equal margin_left+plot_width/2={expected_x:.1}"
    );
    assert!(
        (label_x - expected_x).abs() < 1.0,
        "x-label x={label_x:.1} should equal margin_left+plot_width/2={expected_x:.1}"
    );
}

#[test]
fn test_title_centred_twin_y() {
    let primary = vec![Plot::Line(
        LinePlot::new()
            .with_data(vec![(1.0f64, 5.0f64), (2.0, 8.0), (3.0, 14.0)])
            .with_legend("Temp"),
    )];
    let secondary = vec![Plot::Line(
        LinePlot::new()
            .with_data(vec![(1.0f64, 80.0f64), (2.0, 60.0), (3.0, 45.0)])
            .with_legend("Rain"),
    )];

    let layout = Layout::auto_from_twin_y_plots(&primary, &secondary)
        .with_title("TwinTitle")
        .with_x_label("X");

    let computed = ComputedLayout::from_layout(&layout);
    let expected_x = computed.margin_left + computed.plot_width() / 2.0;

    let scene = render_twin_y(primary, secondary, layout);
    let svg = SvgBackend.render_scene(&scene);
    std::fs::write("test_outputs/label_centering_twin_y.svg", &svg).unwrap();

    let title_x = extract_text_x(&svg, "TwinTitle")
        .expect("title element not found in SVG");
    let label_x = extract_text_x(&svg, "X")
        .expect("x-label element not found in SVG");

    assert!(
        (title_x - expected_x).abs() < 1.0,
        "twin-y title x={title_x:.1} should equal margin_left+plot_width/2={expected_x:.1}"
    );
    assert!(
        (label_x - expected_x).abs() < 1.0,
        "twin-y x-label x={label_x:.1} should equal margin_left+plot_width/2={expected_x:.1}"
    );
}

#[test]
fn test_title_centred_pie_outside_labels() {
    let pie = PiePlot::new()
        .with_slice("Alpha", 30.0, "steelblue")
        .with_slice("Beta", 25.0, "tomato")
        .with_slice("Gamma", 20.0, "seagreen")
        .with_slice("Delta", 15.0, "orange")
        .with_slice("Epsilon", 10.0, "purple")
        .with_label_position(PieLabelPosition::Outside);

    let plots = vec![Plot::Pie(pie.clone())];
    let layout = Layout::auto_from_plots(&plots).with_title("PieTitle");

    // Pre-compute margins from the layout (these are stable across the widening).
    let computed = ComputedLayout::from_layout(&layout);
    let margin_left = computed.margin_left;
    let margin_right = computed.margin_right;

    // Canvas widening happens inside render_multiple; compute expected x after that
    // by reading the final canvas width from the <svg> width attribute.
    let scene = render_multiple(plots, layout);
    let svg = SvgBackend.render_scene(&scene);
    std::fs::write("test_outputs/label_centering_pie_outside.svg", &svg).unwrap();

    // Extract the final canvas width from the SVG header: width="NNN"
    let canvas_width: f64 = {
        let w_pos = svg.find("width=\"").expect("width attr in SVG");
        let after = &svg[w_pos + 7..];
        let end = after.find('"').unwrap();
        after[..end].parse().unwrap()
    };

    // After widening: plot_width = canvas_width - margin_left - margin_right
    // expected title x = margin_left + plot_width / 2
    //                  = (canvas_width + margin_left - margin_right) / 2
    let expected_x = (canvas_width + margin_left - margin_right) / 2.0;

    let title_x = extract_text_x(&svg, "PieTitle")
        .expect("title element not found in SVG");

    assert!(
        (title_x - expected_x).abs() < 1.0,
        "pie title x={title_x:.1} should equal margin_left+plot_width/2={expected_x:.1} \
         (canvas={canvas_width:.1}, ml={margin_left:.1}, mr={margin_right:.1})"
    );
}
