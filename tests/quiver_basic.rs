use kuva::plot::{QuiverPlot, QuiverPivot, ColorMap};
use kuva::render::{plots::Plot, layout::Layout, render::render_multiple};
use kuva::backend::svg::SvgBackend;

fn render(q: QuiverPlot, title: &str) -> String {
    std::fs::create_dir_all("test_outputs").ok();
    let plots = vec![Plot::Quiver(q)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title(title)
        .with_show_grid(false);
    SvgBackend.render_scene(&render_multiple(plots, layout))
}

fn rotational_grid() -> QuiverPlot {
    QuiverPlot::from_function(
        (-2.0, 2.0, 5),
        (-2.0, 2.0, 5),
        |x, y| (-y * 0.3, x * 0.3),
    )
}

#[test]
fn test_quiver_basic_renders_arrows() {
    let svg = render(rotational_grid(), "Quiver Basic");
    std::fs::write("test_outputs/quiver_basic.svg", &svg).unwrap();
    assert!(svg.contains("<svg"), "should be valid SVG");
    let lines = svg.matches("<line").count();
    let paths = svg.matches("<path").count();
    // 25 arrows = 25 shafts (<line>) + 25 heads (<path>), minus any with zero magnitude.
    assert!(lines >= 24, "expected ≥24 shaft lines, got {lines}");
    assert!(paths >= 24, "expected ≥24 head paths, got {paths}");
}

#[test]
fn test_quiver_from_function_exact_count() {
    let q = QuiverPlot::from_function((-1.0, 1.0, 4), (-1.0, 1.0, 6), |_, _| (1.0, 0.0));
    assert_eq!(q.arrows.len(), 24, "4 × 6 = 24 arrows");
}

#[test]
fn test_quiver_from_function_endpoints_inclusive() {
    let q = QuiverPlot::from_function((-1.0, 1.0, 3), (0.0, 2.0, 3), |x, y| (x, y));
    // first: x=-1, y=0 ; last: x=+1, y=+2
    assert_eq!(q.arrows.first().map(|a| (a.x, a.y)), Some((-1.0, 0.0)));
    assert_eq!(q.arrows.last().map(|a| (a.x, a.y)), Some((1.0, 2.0)));
}

#[test]
fn test_quiver_pivot_middle_vs_tail_shifts_endpoints() {
    let arrow_tail = QuiverPlot::new()
        .with_arrow(0.0, 0.0, 2.0, 0.0)
        .with_scale(1.0)
        .with_pivot(QuiverPivot::Tail);
    let arrow_mid = QuiverPlot::new()
        .with_arrow(0.0, 0.0, 2.0, 0.0)
        .with_scale(1.0)
        .with_pivot(QuiverPivot::Middle);
    let (tail_t, tip_t) = arrow_tail.endpoints_with_scale(&arrow_tail.arrows[0], 1.0);
    let (tail_m, tip_m) = arrow_mid.endpoints_with_scale(&arrow_mid.arrows[0], 1.0);
    assert_eq!(tail_t, (0.0, 0.0));
    assert_eq!(tip_t,  (2.0, 0.0));
    assert_eq!(tail_m, (-1.0, 0.0));
    assert_eq!(tip_m,  (1.0, 0.0));
}

#[test]
fn test_quiver_pivot_tip_places_tip_at_data_point() {
    let q = QuiverPlot::new()
        .with_arrow(5.0, 5.0, 1.0, 1.0)
        .with_scale(1.0)
        .with_pivot(QuiverPivot::Tip);
    let (_tail, tip) = q.endpoints_with_scale(&q.arrows[0], 1.0);
    assert_eq!(tip, (5.0, 5.0));
}

#[test]
fn test_quiver_auto_scale_picks_sensible_length() {
    // Two huge vectors on a tiny span: the √n grid-cell heuristic should
    // shrink the scale well below 1.0 so arrows don't overlap each other.
    let q = QuiverPlot::new()
        .with_arrow(0.0, 0.0, 10.0, 0.0)
        .with_arrow(1.0, 0.0, 10.0, 0.0);
    let s = q.effective_scale();
    assert!(s > 0.0 && s < 0.1, "auto-scale should shrink huge vectors; got {s}");
}

#[test]
fn test_quiver_with_scale_pins_exact_value() {
    let q = QuiverPlot::new()
        .with_arrow(0.0, 0.0, 10.0, 0.0)
        .with_arrow(1.0, 0.0, 10.0, 0.0)
        .with_scale(0.5);
    assert_eq!(q.effective_scale(), 0.5);
}

#[test]
fn test_quiver_colormap_triggers_colorbar() {
    let q = rotational_grid().with_magnitude_colormap(ColorMap::Viridis, "Speed");
    let svg = render(q, "Quiver Colormap");
    std::fs::write("test_outputs/quiver_colormap.svg", &svg).unwrap();
    assert!(svg.contains("Speed"), "colorbar label should be in the SVG");
    // Colorbar adds a vertical gradient/rect column on the right margin.
    assert!(svg.contains("<linearGradient") || svg.contains("<rect"),
        "colorbar should emit gradient or rect primitives");
}

#[test]
fn test_quiver_tight_bounds_wraps_in_clippath() {
    let q = rotational_grid().with_tight_bounds();
    let svg = render(q, "Quiver Tight");
    std::fs::write("test_outputs/quiver_tight.svg", &svg).unwrap();
    assert!(svg.contains("clipPath") || svg.contains("clip-path"),
        "tight bounds should emit a clip path");
}

#[test]
fn test_quiver_proportional_heads_are_visible_on_small_arrows() {
    // A field where the longest arrow is tiny (all magnitudes ~0.1).
    let q = QuiverPlot::from_function(
        (-1.0, 1.0, 5),
        (-1.0, 1.0, 5),
        |_, _| (0.1, 0.1),
    );
    let svg = render(q, "Quiver Small");
    // All 25 arrows should still render heads (proportional clamp lower bound).
    assert!(svg.matches("<path").count() >= 24,
        "small arrows should still render heads");
}

#[test]
fn test_quiver_with_legend_emits_entry() {
    let q = rotational_grid().with_legend("Wind");
    let svg = render(q, "Quiver Legend");
    assert!(svg.contains("Wind"), "legend label should appear in SVG");
}

#[test]
fn test_quiver_empty_arrows_renders_empty_plot() {
    let svg = render(QuiverPlot::new(), "Empty");
    // Should produce a valid SVG with axes but no arrows.
    assert!(svg.contains("<svg"));
}

#[test]
fn test_quiver_color_fallback_priority() {
    // Per-arrow color overrides plot-level color.
    let q = QuiverPlot::new()
        .with_colored_arrow(0.0, 0.0, 1.0, 0.0, "tomato")
        .with_arrow(1.0, 0.0, 1.0, 0.0)
        .with_color("steelblue");
    let svg = render(q, "Priority");
    // Resilient to hex / named / rgb() encoding — the backend may emit any.
    let has_tomato = svg.contains("tomato") || svg.contains("#ff6347") || svg.contains("rgb(255,99,71)");
    let has_steelblue = svg.contains("steelblue") || svg.contains("#4682b4") || svg.contains("rgb(70,130,180)");
    assert!(has_tomato, "per-arrow tomato should be in SVG");
    assert!(has_steelblue, "plot-level steelblue should be in SVG");
}
