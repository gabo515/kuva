/// Horizon chart — stacked, folded area chart for dense multi-series time series.
///
/// Each series is rendered as a single-row area chart where the value range is
/// divided into N equal-width bands.  Each band is folded onto the same row, with
/// progressively darker shading for higher value magnitudes.  A second color
/// (typically red) distinguishes negative values from positive ones.
///
/// # Example
///
/// ```rust,no_run
/// use kuva::plot::horizon::HorizonPlot;
/// use kuva::backend::svg::SvgBackend;
/// use kuva::render::render::render_horizon;
/// use kuva::render::layout::Layout;
/// use kuva::render::plots::Plot;
///
/// let x: Vec<f64> = (0..24).map(|i| i as f64).collect();
/// let y: Vec<f64> = x.iter().map(|&t| (t * 0.5).sin() * 10.0).collect();
///
/// let plot = HorizonPlot::new()
///     .with_series("Series A", x.clone(), y)
///     .with_n_bands(3);
///
/// let layout = Layout::auto_from_plots(&[Plot::Horizon(plot.clone())]).with_x_label("Time");
/// let svg = SvgBackend.render_scene(&render_horizon(plot, layout));
/// ```
#[derive(Debug, Clone)]
pub struct HorizonSeries {
    pub label: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    /// Color for positive deviations from baseline. Default: `"#4292c6"` (blue).
    pub pos_color: String,
    /// Color for negative deviations from baseline. Default: `"#d73027"` (red).
    pub neg_color: String,
}

#[derive(Debug, Clone)]
pub struct HorizonPlot {
    pub series: Vec<HorizonSeries>,
    /// Number of color bands. Default: 3.
    pub n_bands: usize,
    /// Per-row pixel height. When `None`, height is derived from the canvas.
    pub row_height: Option<f64>,
    /// Baseline value separating positive from negative regions. Default: 0.0.
    pub baseline: f64,
    /// Override the maximum absolute value used for band-width calculation.
    /// When `None`, derived from data.
    pub value_max: Option<f64>,
    /// Whether to emit a legend. Default: false.
    pub show_legend: bool,
    /// Show the full-scale value (`n_bands × band_width`) at the right end of each row.
    /// This tells the reader what "darkest band" corresponds to in data units. Default: false.
    pub show_value_labels: bool,
    /// Draw a small `+` in `pos_color` and `-` in `neg_color` alongside the row
    /// annotation so the reader can see which hue means positive vs negative.
    /// Has no visible effect unless `show_value_labels` is also true. Default: false.
    pub show_sign_colors: bool,
}

impl Default for HorizonPlot {
    fn default() -> Self { Self::new() }
}

impl HorizonPlot {
    pub fn new() -> Self {
        Self {
            series: vec![],
            n_bands: 3,
            row_height: None,
            baseline: 0.0,
            value_max: None,
            show_legend: false,
            show_value_labels: false,
            show_sign_colors: false,
        }
    }

    /// Add a series, auto-assigning a `pos_color` from the category10 palette.
    ///
    /// The color cycles with the series index, so each call to `with_series` on
    /// the same plot gets a distinct hue.  `neg_color` is always the palette's
    /// red (`#d62728`) — the universal signal for negative deviation.
    pub fn with_series<S, IX, IY, X, Y>(mut self, label: S, x: IX, y: IY) -> Self
    where
        S: Into<String>,
        IX: IntoIterator<Item = X>,
        IY: IntoIterator<Item = Y>,
        X: Into<f64>,
        Y: Into<f64>,
    {
        // category10 palette — same source as Palette::category10() in render/palette.rs
        const PALETTE: &[&str] = &[
            "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd",
            "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
        ];
        let idx = self.series.len();
        let pos_color = PALETTE[idx % PALETTE.len()].to_string();
        self.series.push(HorizonSeries {
            label: label.into(),
            x: x.into_iter().map(|v| v.into()).collect(),
            y: y.into_iter().map(|v| v.into()).collect(),
            pos_color,
            neg_color: "#d62728".to_string(),
        });
        self
    }

    /// Add a series with explicit positive and negative colors.
    pub fn with_series_colored<S, IX, IY, X, Y, CP, CN>(
        mut self,
        label: S,
        x: IX,
        y: IY,
        pos_color: CP,
        neg_color: CN,
    ) -> Self
    where
        S: Into<String>,
        IX: IntoIterator<Item = X>,
        IY: IntoIterator<Item = Y>,
        X: Into<f64>,
        Y: Into<f64>,
        CP: Into<String>,
        CN: Into<String>,
    {
        self.series.push(HorizonSeries {
            label: label.into(),
            x: x.into_iter().map(|v| v.into()).collect(),
            y: y.into_iter().map(|v| v.into()).collect(),
            pos_color: pos_color.into(),
            neg_color: neg_color.into(),
        });
        self
    }

    /// Set the number of color bands. Default: 3.
    pub fn with_n_bands(mut self, n: usize) -> Self {
        self.n_bands = n.max(1);
        self
    }

    /// Override per-row pixel height. Enables auto canvas sizing.
    pub fn with_row_height(mut self, h: f64) -> Self {
        self.row_height = Some(h);
        self
    }

    /// Set the baseline value (zero-line). Default: 0.0.
    pub fn with_baseline(mut self, b: f64) -> Self {
        self.baseline = b;
        self
    }

    /// Override the maximum absolute deviation used for band scaling.
    pub fn with_value_max(mut self, v: f64) -> Self {
        self.value_max = Some(v);
        self
    }

    /// Show a legend entry per series.
    pub fn with_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Show the full-scale value at the right end of each row.
    ///
    /// The label shows what value the darkest band represents, e.g. `+8.5` for positive
    /// and `-3.2` for series that also have negative values.
    pub fn with_value_labels(mut self, show: bool) -> Self {
        self.show_value_labels = show;
        self
    }

    /// Colorize the `+` / `-` sign characters in `pos_color` / `neg_color` in
    /// the row annotation.  Requires `with_value_labels(true)` to have any effect.
    pub fn with_sign_colors(mut self, show: bool) -> Self {
        self.show_sign_colors = show;
        self
    }

    /// Number of series.
    pub fn n_series(&self) -> usize {
        self.series.len()
    }

    /// Compute the positive band width: (max_pos_deviation / n_bands).
    /// If `value_max` is set, use that; otherwise derive from data.
    pub fn pos_band_width(&self) -> f64 {
        let vmax = self.value_max.unwrap_or_else(|| {
            self.series.iter()
                .flat_map(|s| s.y.iter())
                .map(|&v| (v - self.baseline).max(0.0))
                .fold(0.0_f64, f64::max)
        });
        if vmax <= 0.0 || self.n_bands == 0 { 1.0 } else { vmax / self.n_bands as f64 }
    }

    /// Compute the negative band width.
    pub fn neg_band_width(&self) -> f64 {
        let vmax = self.value_max.unwrap_or_else(|| {
            self.series.iter()
                .flat_map(|s| s.y.iter())
                .map(|&v| (self.baseline - v).max(0.0))
                .fold(0.0_f64, f64::max)
        });
        if vmax <= 0.0 || self.n_bands == 0 { 1.0 } else { vmax / self.n_bands as f64 }
    }

    /// x data extent across all series.
    pub fn x_range(&self) -> Option<(f64, f64)> {
        let mut xmin = f64::INFINITY;
        let mut xmax = f64::NEG_INFINITY;
        for s in &self.series {
            for &x in &s.x {
                xmin = xmin.min(x);
                xmax = xmax.max(x);
            }
        }
        if xmin.is_finite() { Some((xmin, xmax)) } else { None }
    }
}
