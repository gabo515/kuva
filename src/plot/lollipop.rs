/// A single data point in a lollipop chart.
pub struct LollipopPoint {
    pub x: f64,
    pub y: f64,
    /// Optional text label rendered above (or below, when y < baseline) the dot.
    pub label: Option<String>,
    /// Per-point color override. `None` uses the plot-level `color`.
    pub color: Option<String>,
}

/// A colored annotation band drawn behind the stems, anchored to the baseline.
///
/// Typical use: protein domain annotations below a mutation landscape, where
/// each rect covers a functional region along the sequence x-axis.
pub struct LollipopDomain {
    pub x_start: f64,
    pub x_end: f64,
    pub label: Option<String>,
    pub color: String,
    /// Fill opacity. Default `0.35`.
    pub opacity: f64,
}

/// Builder for a lollipop chart.
///
/// Each data point is rendered as a vertical stem (line from `baseline` to `y`)
/// topped with a filled circle. Useful for mutation landscapes, ranked discrete
/// data, and any context where bar charts feel too heavy.
///
/// Optional domain annotations (`with_domain`) draw colored rectangles behind
/// the stems — the canonical presentation for protein mutation landscapes.
///
/// # Example
///
/// ```rust,no_run
/// use kuva::prelude::*;
///
/// let lollipop = LollipopPlot::new()
///     .with_point(10.0, 3.0)
///     .with_labeled_point(25.0, 7.0, "TP53")
///     .with_colored_point(42.0, 5.0, "tomato")
///     .with_domain(1.0, 50.0, Some("Kinase"), "steelblue")
///     .with_baseline(0.0);
///
/// let plots = vec![Plot::from(lollipop)];
/// let layout = Layout::auto_from_plots(&plots)
///     .with_title("Mutation Landscape")
///     .with_x_label("Position (aa)")
///     .with_y_label("Count");
/// ```
pub struct LollipopPlot {
    pub points: Vec<LollipopPoint>,
    /// Default fill color for stems and dots. Default `"steelblue"`.
    pub color: String,
    /// Y-value at which stems originate. Default `0.0`.
    pub baseline: f64,
    /// Stem stroke width in pixels. Default `1.5`.
    pub stem_width: f64,
    /// Dot radius in pixels. Default `5.0`.
    pub dot_radius: f64,
    /// Dot stroke color. `None` uses the dot fill color.
    pub dot_stroke: Option<String>,
    /// Dot stroke width in pixels. Default `1.0`.
    pub dot_stroke_width: f64,
    /// Draw a horizontal line at `baseline`. Default `true`.
    pub show_baseline: bool,
    /// Baseline line color. Default `"#888888"`.
    pub baseline_color: String,
    /// Baseline line stroke width in pixels. Default `1.0`.
    pub baseline_width: f64,
    /// Baseline line dasharray. Default `None` (solid).
    pub baseline_dash: Option<String>,
    /// Domain annotation bands rendered behind stems.
    pub domains: Vec<LollipopDomain>,
    /// Height of domain rects in data-coordinate units below the baseline. Default `0.5`.
    pub domain_height: f64,
    pub legend_label: Option<String>,
}

impl Default for LollipopPlot {
    fn default() -> Self { Self::new() }
}

impl LollipopPlot {
    /// Create a lollipop plot with default settings.
    pub fn new() -> Self {
        Self {
            points: vec![],
            color: "steelblue".into(),
            baseline: 0.0,
            stem_width: 1.5,
            dot_radius: 5.0,
            dot_stroke: None,
            dot_stroke_width: 1.0,
            show_baseline: true,
            baseline_color: "#888888".into(),
            baseline_width: 1.0,
            baseline_dash: None,
            domains: vec![],
            domain_height: 0.5,
            legend_label: None,
        }
    }

    /// Add a point at (`x`, `y`) with no label.
    pub fn with_point(mut self, x: impl Into<f64>, y: impl Into<f64>) -> Self {
        self.points.push(LollipopPoint { x: x.into(), y: y.into(), label: None, color: None });
        self
    }

    /// Add a point with a text label rendered above (or below) the dot.
    pub fn with_labeled_point(mut self, x: impl Into<f64>, y: impl Into<f64>, label: impl Into<String>) -> Self {
        self.points.push(LollipopPoint { x: x.into(), y: y.into(), label: Some(label.into()), color: None });
        self
    }

    /// Add a point with a per-point color override.
    pub fn with_colored_point(mut self, x: impl Into<f64>, y: impl Into<f64>, color: impl Into<String>) -> Self {
        self.points.push(LollipopPoint { x: x.into(), y: y.into(), label: None, color: Some(color.into()) });
        self
    }

    /// Add a point with both a label and a per-point color override.
    pub fn with_labeled_colored_point(mut self, x: impl Into<f64>, y: impl Into<f64>, label: impl Into<String>, color: impl Into<String>) -> Self {
        self.points.push(LollipopPoint { x: x.into(), y: y.into(), label: Some(label.into()), color: Some(color.into()) });
        self
    }

    /// Add multiple unlabeled points from an iterator of `(x, y)` pairs.
    pub fn with_points(mut self, pts: impl IntoIterator<Item = (f64, f64)>) -> Self {
        for (x, y) in pts {
            self.points.push(LollipopPoint { x, y, label: None, color: None });
        }
        self
    }

    /// Set the default stem and dot color (CSS color string). Default `"steelblue"`.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Set the baseline Y value (where stems originate). Default `0.0`.
    pub fn with_baseline(mut self, v: f64) -> Self {
        self.baseline = v;
        self
    }

    /// Set the stem stroke width in pixels. Default `1.5`.
    pub fn with_stem_width(mut self, w: f64) -> Self {
        self.stem_width = w;
        self
    }

    /// Set the dot radius in pixels. Default `5.0`.
    pub fn with_dot_radius(mut self, r: f64) -> Self {
        self.dot_radius = r;
        self
    }

    /// Set the dot stroke color (CSS color string). Default is same as fill.
    pub fn with_dot_stroke(mut self, color: impl Into<String>) -> Self {
        self.dot_stroke = Some(color.into());
        self
    }

    /// Set the dot stroke width in pixels. Default `1.0`.
    pub fn with_dot_stroke_width(mut self, w: f64) -> Self {
        self.dot_stroke_width = w;
        self
    }

    /// Toggle the horizontal baseline line. Default `true`.
    pub fn with_show_baseline(mut self, show: bool) -> Self {
        self.show_baseline = show;
        self
    }

    /// Set the baseline line color. Default `"#888888"`.
    pub fn with_baseline_color(mut self, c: impl Into<String>) -> Self {
        self.baseline_color = c.into();
        self
    }

    /// Set the baseline line stroke width in pixels. Default `1.0`.
    pub fn with_baseline_width(mut self, w: f64) -> Self {
        self.baseline_width = w;
        self
    }

    /// Set the baseline line dasharray (e.g. `"4,3"`). Default `None` (solid).
    pub fn with_baseline_dash(mut self, d: impl Into<String>) -> Self {
        self.baseline_dash = Some(d.into());
        self
    }

    /// Add a colored domain annotation band behind the stems.
    ///
    /// `label` is optional text centered inside the rect. `color` is a CSS color string.
    pub fn with_domain(mut self, x_start: f64, x_end: f64, label: Option<&str>, color: impl Into<String>) -> Self {
        self.domains.push(LollipopDomain {
            x_start,
            x_end,
            label: label.map(|s| s.to_string()),
            color: color.into(),
            opacity: 0.35,
        });
        self
    }

    /// Add a domain with explicit opacity.
    pub fn with_domain_opacity(mut self, x_start: f64, x_end: f64, label: Option<&str>, color: impl Into<String>, opacity: f64) -> Self {
        self.domains.push(LollipopDomain {
            x_start,
            x_end,
            label: label.map(|s| s.to_string()),
            color: color.into(),
            opacity,
        });
        self
    }

    /// Set the domain rect height in data-coordinate units below the baseline. Default `0.5`.
    pub fn with_domain_height(mut self, h: f64) -> Self {
        self.domain_height = h;
        self
    }

    /// Attach a legend label to this plot (shows a colored circle entry).
    pub fn with_legend(mut self, label: impl Into<String>) -> Self {
        self.legend_label = Some(label.into());
        self
    }
}
