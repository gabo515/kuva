/// Radius encoding mode for [`RosePlot`] sectors.
#[derive(Debug, Clone, Default)]
pub enum RoseEncoding {
    /// Sector area is proportional to the value — perceptually accurate.  **(default)**
    #[default]
    Area,
    /// Sector radius is proportional to the value.
    Radius,
}

/// Multi-series layout mode for [`RosePlot`].
#[derive(Debug, Clone, Default)]
pub enum RoseMode {
    /// Series are stacked on top of each other within each sector.  **(default)**
    #[default]
    Stacked,
    /// Each series occupies its own sub-wedge within a sector.
    Grouped,
}

/// A single data series for a [`RosePlot`].
#[derive(Debug, Clone)]
pub struct RoseSeries {
    /// Series name (shown in legend).
    pub name: String,
    /// Per-sector values; indexed by sector position.
    pub values: Vec<f64>,
    /// Optional explicit CSS color.  Falls back to palette.
    pub color: Option<String>,
}

impl RoseSeries {
    /// Create a new named series with given values.
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        RoseSeries { name: name.into(), values, color: None }
    }
}

/// Nightingale rose / coxcomb chart — a polar bar chart where each sector's
/// **area** (or radius) is proportional to the data value.
///
/// # Basic usage
///
/// ```rust,no_run
/// use kuva::plot::rose::RosePlot;
/// use kuva::render::{plots::Plot, layout::Layout, render::{render_multiple, render_rose}};
/// use kuva::backend::svg::SvgBackend;
///
/// let plot = RosePlot::new()
///     .with_slice("Jan", 30.0)
///     .with_slice("Feb", 20.0)
///     .with_slice("Mar", 45.0)
///     .with_slice("Apr", 38.0);
///
/// let plots = vec![Plot::Rose(plot)];
/// let layout = Layout::auto_from_plots(&plots).with_title("Monthly values");
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// std::fs::write("rose.svg", svg).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct RosePlot {
    /// Data series.  Single-series plots use `series[0]`.
    pub series: Vec<RoseSeries>,
    /// Sector labels (one per sector position).
    pub labels: Vec<String>,
    /// Radius encoding mode (default: [`RoseEncoding::Area`]).
    pub encoding: RoseEncoding,
    /// Multi-series layout mode (default: [`RoseMode::Stacked`]).
    pub mode: RoseMode,
    /// Start angle in degrees clockwise from north (default: 0.0).
    pub start_angle: f64,
    /// If `true` sectors proceed clockwise (default).
    pub clockwise: bool,
    /// Inner radius as a fraction of the outer radius (0–1); 0 = no hole.  Default: 0.0.
    pub inner_radius: f64,
    /// Angular gap between adjacent sectors in degrees.  Default: 1.0.
    pub gap: f64,
    /// Draw concentric grid rings.  Default: `true`.
    pub show_grid: bool,
    /// Number of concentric grid rings.  Default: 4.
    pub grid_lines: usize,
    /// Draw radial spoke lines.  Default: `true`.
    pub show_spokes: bool,
    /// Draw sector labels around the perimeter.  Default: `true`.
    pub show_labels: bool,
    /// Draw value labels at the tip of each sector.  Default: `false`.
    pub show_values: bool,
    /// If set, a legend entry is rendered for each series.
    pub legend_label: Option<String>,
}

impl Default for RosePlot {
    fn default() -> Self { Self::new() }
}

impl RosePlot {
    /// Create a [`RosePlot`] with default settings.
    pub fn new() -> Self {
        RosePlot {
            series: vec![],
            labels: vec![],
            encoding: RoseEncoding::Area,
            mode: RoseMode::Stacked,
            start_angle: 0.0,
            clockwise: true,
            inner_radius: 0.0,
            gap: 1.0,
            show_grid: true,
            grid_lines: 4,
            show_spokes: true,
            show_labels: true,
            show_values: false,
            legend_label: None,
        }
    }

    // ── Data ─────────────────────────────────────────────────────────────────

    /// Add a single sector (label + value) to the first (default) series.
    /// Creates `series[0]` named "Values" if it doesn't exist yet.
    pub fn with_slice(mut self, label: impl Into<String>, value: impl Into<f64>) -> Self {
        self.labels.push(label.into());
        if self.series.is_empty() {
            self.series.push(RoseSeries::new("Values", vec![]));
        }
        self.series[0].values.push(value.into());
        self
    }

    /// Add multiple slices from an iterator of `(label, value)`.
    pub fn with_slices<S, V, I>(mut self, slices: I) -> Self
    where
        S: Into<String>,
        V: Into<f64>,
        I: IntoIterator<Item = (S, V)>,
    {
        for (label, value) in slices {
            self = self.with_slice(label, value);
        }
        self
    }

    /// Set all sector labels at once.
    pub fn with_x_labels<S, I>(mut self, labels: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        self.labels = labels.into_iter().map(Into::into).collect();
        self
    }

    /// Add a stacked series.  Sets mode to [`RoseMode::Stacked`].
    pub fn with_stack<S, I>(mut self, name: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        self.mode = RoseMode::Stacked;
        self.series.push(RoseSeries::new(name, values.into_iter().map(Into::into).collect()));
        self
    }

    /// Add a grouped series.  Sets mode to [`RoseMode::Grouped`].
    pub fn with_group<S, I>(mut self, name: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        self.mode = RoseMode::Grouped;
        self.series.push(RoseSeries::new(name, values.into_iter().map(Into::into).collect()));
        self
    }

    /// Bin raw bearing values (0–360°) into `n_bins` equal sectors.
    /// Sets degree labels if `labels` is currently empty; replaces (or creates)
    /// `series[0]` named "Count".
    pub fn with_bearing_data<I>(mut self, bearings: I, n_bins: usize) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        if n_bins == 0 { return self; }
        let mut counts = vec![0_u64; n_bins];
        let bin_size = 360.0 / n_bins as f64;
        for b in bearings {
            let b = ((b % 360.0) + 360.0) % 360.0;
            let idx = (b / bin_size).floor() as usize;
            let idx = idx.min(n_bins - 1);
            counts[idx] += 1;
        }
        let values: Vec<f64> = counts.iter().map(|&c| c as f64).collect();
        if self.labels.is_empty() {
            self.labels = (0..n_bins)
                .map(|i| format!("{:.0}°", i as f64 * bin_size))
                .collect();
        }
        if self.series.is_empty() {
            self.series.push(RoseSeries::new("Count", values));
        } else {
            self.series[0] = RoseSeries::new("Count", values);
        }
        self
    }

    /// Replace sector labels with compass directions derived from the current
    /// number of sectors.
    pub fn with_compass_labels(mut self) -> Self {
        let n = self.n_sectors();
        self.labels = compass_labels_for_n(n);
        self
    }

    // ── Appearance ────────────────────────────────────────────────────────────

    /// Set the radius encoding mode.
    pub fn with_encoding(mut self, enc: RoseEncoding) -> Self {
        self.encoding = enc;
        self
    }

    /// Set the multi-series layout mode.
    pub fn with_mode(mut self, mode: RoseMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the start angle in degrees clockwise from north.
    pub fn with_start_angle(mut self, deg: f64) -> Self {
        self.start_angle = deg;
        self
    }

    /// Set the rotation direction.
    pub fn with_clockwise(mut self, cw: bool) -> Self {
        self.clockwise = cw;
        self
    }

    /// Set the inner radius fraction (clamped to [0.0, 0.95]).
    pub fn with_inner_radius(mut self, r: f64) -> Self {
        self.inner_radius = r.clamp(0.0, 0.95);
        self
    }

    /// Set the angular gap between adjacent sectors in degrees.
    pub fn with_gap(mut self, gap: f64) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Show / hide concentric grid rings.
    pub fn with_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set the number of concentric grid rings.
    pub fn with_grid_lines(mut self, n: usize) -> Self {
        self.grid_lines = n;
        self
    }

    /// Show / hide radial spoke lines.
    pub fn with_spokes(mut self, show: bool) -> Self {
        self.show_spokes = show;
        self
    }

    /// Show / hide sector labels around the perimeter.
    pub fn with_show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Show / hide value labels at the tip of each sector.
    pub fn with_show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Enable legend; pass a label string (used as the legend title or group name).
    pub fn with_legend(mut self, label: impl Into<String>) -> Self {
        self.legend_label = Some(label.into());
        self
    }

    // ── Internal helpers ─────────────────────────────────────────────────────

    /// Number of sectors (the maximum of labels length and max series values length).
    pub(crate) fn n_sectors(&self) -> usize {
        let label_n = self.labels.len();
        let series_n = self.series.iter().map(|s| s.values.len()).max().unwrap_or(0);
        label_n.max(series_n)
    }

    /// Maximum data value used to scale the rings.
    /// For Stacked: maximum cumulative sum per sector.
    /// For Grouped: maximum individual value across all series and sectors.
    pub(crate) fn max_total(&self) -> f64 {
        if self.series.is_empty() { return 0.0; }
        let n = self.n_sectors();
        match self.mode {
            RoseMode::Stacked => {
                (0..n).map(|i| {
                    self.series.iter()
                        .map(|s| s.values.get(i).copied().unwrap_or(0.0).max(0.0))
                        .sum::<f64>()
                }).fold(0.0_f64, f64::max)
            }
            RoseMode::Grouped => {
                self.series.iter()
                    .flat_map(|s| s.values.iter().copied())
                    .fold(0.0_f64, f64::max)
            }
        }
    }
}

/// Return compass direction labels for `n` evenly-spaced sectors.
///
/// If `n` divides evenly into the 16-point compass rose, the standard
/// cardinal / intercardinal abbreviations are used.  Otherwise degree
/// strings like `"0°"`, `"15°"`, … are returned.
pub fn compass_labels_for_n(n: usize) -> Vec<String> {
    if n == 0 { return vec![]; }
    let compass = ["N","NNE","NE","ENE","E","ESE","SE","SSE",
                   "S","SSW","SW","WSW","W","WNW","NW","NNW"];
    if n <= 16 && 16 % n == 0 {
        let stride = 16 / n;
        (0..n).map(|i| compass[i * stride].to_string()).collect()
    } else {
        let step = 360.0 / n as f64;
        (0..n).map(|i| format!("{:.0}°", i as f64 * step)).collect()
    }
}
