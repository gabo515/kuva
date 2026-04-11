/// A single group of data for an ECDF plot.
#[derive(Debug, Clone)]
pub struct EcdfGroup {
    pub label: String,
    pub data: Vec<f64>,
    /// Explicit color; `None` means fall back to the palette.
    pub color: Option<String>,
}

/// Builder for an Empirical Cumulative Distribution Function (ECDF) plot.
///
/// Supports single or multiple groups, complementary CDF mode (`1 - F(x)`),
/// DKW 95% confidence bands, rug plots, percentile reference lines,
/// per-step markers, and a smooth KDE-integrated CDF.
///
/// # Simple example
///
/// ```rust,no_run
/// use kuva::plot::EcdfPlot;
/// use kuva::backend::svg::SvgBackend;
/// use kuva::render::render::render_multiple;
/// use kuva::render::layout::Layout;
/// use kuva::render::plots::Plot;
///
/// let data: Vec<f64> = vec![1.2, 3.4, 2.1, 5.6, 4.0, 0.8, 3.3, 2.7];
///
/// let plot = EcdfPlot::new()
///     .with_data("Sample", data)
///     .with_color("steelblue")
///     .with_confidence_band();
///
/// let plots = vec![Plot::Ecdf(plot)];
/// let layout = Layout::auto_from_plots(&plots)
///     .with_title("ECDF")
///     .with_x_label("Value")
///     .with_y_label("F(x)");
///
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// std::fs::write("ecdf.svg", svg).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct EcdfPlot {
    pub groups: Vec<EcdfGroup>,
    /// Plot `1 - F(x)` instead of `F(x)` (complementary / survival function).
    pub complementary: bool,
    /// Draw a DKW 95% confidence band around the step function.
    pub show_confidence_band: bool,
    /// Opacity of the confidence band fill (default `0.15`).
    pub band_alpha: f64,
    /// Draw a rug of tick marks at each data point below the x-axis.
    pub show_rug: bool,
    /// Height of rug tick marks in pixels (default `6.0`).
    pub rug_height: f64,
    /// Horizontal dashed reference lines at these percentile levels (0–1).
    pub percentile_lines: Vec<f64>,
    /// Show a circle marker at each step endpoint.
    pub show_markers: bool,
    /// Marker radius in pixels (default `3.0`).
    pub marker_size: f64,
    /// Use a smooth KDE-integrated CDF instead of the step function.
    pub smooth: bool,
    /// Number of grid points for smooth CDF estimation (default `200`).
    pub smooth_samples: usize,
    /// Line stroke width (default `1.5`).
    pub stroke_width: f64,
    /// When `Some`, the legend is rendered using group labels.
    pub legend_label: Option<String>,
    /// Default color used for single-group plots and palette auto-assignment.
    pub color: String,
    /// Optional SVG stroke-dasharray (e.g. `"6,3"`).
    pub line_dash: Option<String>,
}

impl Default for EcdfPlot {
    fn default() -> Self { Self::new() }
}

impl EcdfPlot {
    /// Create an ECDF plot with default settings.
    pub fn new() -> Self {
        Self {
            groups: vec![],
            complementary: false,
            show_confidence_band: false,
            band_alpha: 0.15,
            show_rug: false,
            rug_height: 6.0,
            percentile_lines: vec![],
            show_markers: false,
            marker_size: 3.0,
            smooth: false,
            smooth_samples: 200,
            stroke_width: 1.5,
            legend_label: None,
            color: "steelblue".into(),
            line_dash: None,
        }
    }

    /// Add a group of data values.
    ///
    /// ```rust,no_run
    /// # use kuva::plot::EcdfPlot;
    /// let plot = EcdfPlot::new()
    ///     .with_data("Group A", vec![1.0, 2.0, 3.0])
    ///     .with_data("Group B", vec![2.0, 4.0, 6.0])
    ///     .with_legend("Samples");
    /// ```
    pub fn with_data<S, I, T>(mut self, label: S, data: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = T>,
        T: Into<f64>,
    {
        let values: Vec<f64> = data.into_iter().map(|v| v.into()).collect();
        self.groups.push(EcdfGroup { label: label.into(), data: values, color: None });
        self
    }

    /// Add a group with an explicit color.
    pub fn with_data_colored<S, C, I, T>(mut self, label: S, data: I, color: C) -> Self
    where
        S: Into<String>,
        C: Into<String>,
        I: IntoIterator<Item = T>,
        T: Into<f64>,
    {
        let values: Vec<f64> = data.into_iter().map(|v| v.into()).collect();
        self.groups.push(EcdfGroup {
            label: label.into(),
            data: values,
            color: Some(color.into()),
        });
        self
    }

    /// Add multiple groups at once. Each item is a `(label, values)` pair.
    ///
    /// ```rust,no_run
    /// # use kuva::plot::EcdfPlot;
    /// let groups = vec![
    ///     ("Control", vec![1.0, 2.0, 3.0]),
    ///     ("Treated", vec![2.0, 4.0, 6.0]),
    /// ];
    /// let plot = EcdfPlot::new().with_groups(groups).with_legend("Groups");
    /// ```
    pub fn with_groups<G, S, I, T>(mut self, groups: G) -> Self
    where
        G: IntoIterator<Item = (S, I)>,
        S: Into<String>,
        I: IntoIterator<Item = T>,
        T: Into<f64>,
    {
        for (label, data) in groups {
            let values: Vec<f64> = data.into_iter().map(|v| v.into()).collect();
            self.groups.push(EcdfGroup { label: label.into(), data: values, color: None });
        }
        self
    }

    /// Plot `1 - F(x)` instead of `F(x)`.
    ///
    /// The complementary CDF (CCDF / survival function) is the standard view
    /// for heavy-tailed distributions, sequencing read lengths, and coverage
    /// distributions — it emphasises the tail rather than the bulk.
    pub fn with_complementary(mut self) -> Self {
        self.complementary = true;
        self
    }

    /// Draw a shaded DKW 95% confidence band around the ECDF.
    ///
    /// The band width is `ε = √(ln(2/0.05) / (2n))`. Useful for small samples
    /// where two curves look different but the overlap in their bands shows
    /// they are not statistically distinguishable.
    pub fn with_confidence_band(mut self) -> Self {
        self.show_confidence_band = true;
        self
    }

    /// Set the confidence band fill opacity (default `0.15`).
    pub fn with_band_alpha(mut self, alpha: f64) -> Self {
        self.band_alpha = alpha;
        self
    }

    /// Draw a rug of tick marks at each data point just below the x-axis.
    ///
    /// Shows the density of raw observations — clusters and gaps that the
    /// step function alone can obscure, especially for small samples.
    pub fn with_rug(mut self) -> Self {
        self.show_rug = true;
        self
    }

    /// Set the height of rug tick marks in pixels (default `6.0`).
    pub fn with_rug_height(mut self, px: f64) -> Self {
        self.rug_height = px;
        self
    }

    /// Draw horizontal dashed reference lines at the given percentile levels.
    ///
    /// Each value should be in `[0.0, 1.0]`. Labels are placed at the right edge.
    ///
    /// ```rust,no_run
    /// # use kuva::plot::EcdfPlot;
    /// let plot = EcdfPlot::new()
    ///     .with_data("Sample", vec![1.0, 2.0, 3.0, 4.0, 5.0])
    ///     .with_percentile_lines(vec![0.25, 0.5, 0.75]);
    /// ```
    pub fn with_percentile_lines(mut self, percentiles: Vec<f64>) -> Self {
        self.percentile_lines = percentiles;
        self
    }

    /// Show a circle marker at each step transition.
    ///
    /// Most useful for small samples (n < 30) where the discrete jumps in the
    /// ECDF indicate individual data points.
    pub fn with_markers(mut self) -> Self {
        self.show_markers = true;
        self
    }

    /// Set the marker radius in pixels (default `3.0`).
    pub fn with_marker_size(mut self, r: f64) -> Self {
        self.marker_size = r;
        self
    }

    /// Use a smooth KDE-integrated CDF instead of the empirical step function.
    ///
    /// Bandwidth is chosen by Silverman's rule-of-thumb.
    pub fn with_smooth(mut self) -> Self {
        self.smooth = true;
        self
    }

    /// Set the number of grid points for smooth CDF estimation (default `200`).
    pub fn with_smooth_samples(mut self, n: usize) -> Self {
        self.smooth_samples = n;
        self
    }

    /// Set the line stroke width in pixels (default `1.5`).
    pub fn with_stroke_width(mut self, w: f64) -> Self {
        self.stroke_width = w;
        self
    }

    /// Set a uniform color (used for single-group plots and palette auto-assignment).
    pub fn with_color<S: Into<String>>(mut self, color: S) -> Self {
        self.color = color.into();
        self
    }

    /// Show the legend. Pass an empty string `""` to show group labels without
    /// a title, or a non-empty string for a titled legend.
    pub fn with_legend<S: Into<String>>(mut self, title: S) -> Self {
        self.legend_label = Some(title.into());
        self
    }

    /// Set a dashed stroke pattern (SVG `stroke-dasharray`), e.g. `"6,3"`.
    pub fn with_line_dash<S: Into<String>>(mut self, dash: S) -> Self {
        self.line_dash = Some(dash.into());
        self
    }
}
