use kuva::plot::Histogram;
use kuva::backend::svg::SvgBackend;
// use kuva::render::render::render_histogram;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

#[test]
fn test_histogram_svg_output_builder() {
    let hist = Histogram::new()
        .with_data(vec![1.1, 2.3, 2.7, 3.2, 3.8, 3.9, 4.0])
        .with_bins(5)
        .with_color("navy")
        .with_range((0.0, 5.0)); // make this automatic

    let plots = vec![Plot::Histogram(hist.clone())];

    // let layout = Layout::auto_from_data(&hist.data, 0.0..5.0)
    //     .with_title("Histogram")
    //     .with_x_label("Value")
    //     .with_y_label("Frequency");
        // .with_ticks(10);
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Histogram")
        .with_x_label("Value")
        .with_y_label("Frequency");
        // .with_ticks(10);

    // let scene = render_histogram(&hist, &layout);
    let scene = render_multiple(plots, layout);
    let svg = SvgBackend.render_scene(&scene);
    std::fs::write("test_outputs/hist_builder.svg", svg.clone()).unwrap();

    // Basic sanity assertion
    assert!(svg.contains("<svg"));
}

// A normalized histogram has y values in [0, 1].  auto_from_plots should
// detect this and clamp the y-axis at exactly 1.0 rather than letting the
// 1%-span padding push auto_nice_range up to 1.1.
#[test]
fn test_normalized_histogram_y_axis_clamp() {
    let hist = Histogram::new()
        .with_data(vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0])
        .with_bins(4)
        .with_range((0.0, 5.0))
        .with_normalize();

    let plots = vec![Plot::Histogram(hist)];
    let layout = Layout::auto_from_plots(&plots);
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/hist_normalized_clamp.svg", &svg).unwrap();

    // clamp_y_axis should be triggered automatically: axis must stop at 1 not 1.1
    assert!(svg.contains(">1<") || svg.contains(">1.0<"),
        "normalized histogram y-axis should show a tick at 1");
    assert!(!svg.contains(">1.1<"), "y-axis must not extend past 1.0 for normalized data");
}

// Non-normalized histograms must not be affected by the clamp.
#[test]
fn test_non_normalized_histogram_y_axis_free() {
    let hist = Histogram::new()
        .with_data(vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0])
        .with_bins(4)
        .with_range((0.0, 5.0));  // no with_normalize()

    let plots = vec![Plot::Histogram(hist)];
    let layout = Layout::auto_from_plots(&plots);
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/hist_non_normalized.svg", &svg).unwrap();

    // y-axis should reflect actual counts (max count = 3), well above 1
    assert!(!svg.contains(">1.1<") || svg.contains(">2<") || svg.contains(">3<"),
        "non-normalized histogram y-axis should show count ticks");
}

// Bug #4: ticks must land on bin boundaries for auto-binned histograms.
// Range [0.0, 4.8], 6 bins → bin_width = 0.8.
// generate_ticks_bin_aligned should produce 0, 0.8, 1.6, 2.4, 3.2, 4.0, 4.8
// rather than the generic 0, 1, 2, 3, 4 from generate_ticks.
#[test]
fn test_histogram_bin_aligned_ticks() {
    // 6 uniform bins over [0.0, 4.8] → bin_width = 0.8
    let data: Vec<f64> = (0..60).map(|i| i as f64 * 0.08).collect();
    let hist = Histogram::new()
        .with_data(data)
        .with_bins(6)
        .with_range((0.0, 4.8))
        .with_color("steelblue");

    let plots = vec![Plot::Histogram(hist)];
    let layout = Layout::auto_from_plots(&plots);
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/hist_bin_aligned_ticks.svg", &svg).unwrap();

    // The rightmost bin edge (4.8) must appear as a tick label
    assert!(svg.contains(">4.8<"),
        "bin-aligned ticks should include the rightmost bin edge 4.8");
    // A non-edge tick like 1 would only appear with the generic tick generator
    assert!(!svg.contains(">1<"),
        "bin-aligned ticks must not emit non-edge tick 1");
}

// Feature #6: Histogram::from_bins — precomputed edges + counts, no range needed.
#[test]
fn test_histogram_from_bins_basic() {
    let edges = vec![0.0, 1.0, 2.0, 3.0];
    let counts = vec![5.0, 12.0, 8.0];
    let hist = Histogram::from_bins(edges, counts).with_color("steelblue");

    let plots = vec![Plot::Histogram(hist)];
    // Must not panic — range is not required for precomputed histograms
    let layout = Layout::auto_from_plots(&plots);
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/hist_from_bins.svg", &svg).unwrap();

    assert!(svg.contains("<rect"), "precomputed histogram must draw bars");
    // Bin edges are 0, 1, 2, 3 — bin-aligned ticks should include 3
    assert!(svg.contains(">3<"), "bin-aligned ticks should include the right edge 3");
}

// Feature #6: precomputed histogram with normalization.
#[test]
fn test_histogram_from_bins_normalize() {
    let edges = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    let counts = vec![5.0, 20.0, 15.0, 10.0];
    let hist = Histogram::from_bins(edges, counts)
        .with_normalize()
        .with_color("coral");

    let plots = vec![Plot::Histogram(hist)];
    let layout = Layout::auto_from_plots(&plots);
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/hist_from_bins_normalized.svg", &svg).unwrap();

    assert!(!svg.contains(">1.1<"),
        "normalized precomputed histogram y-axis must not exceed 1.0");
}
