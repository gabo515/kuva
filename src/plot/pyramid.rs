/// How multiple series are rendered within each age-group row.
#[derive(Debug, Clone, Default)]
pub enum PyramidMode {
    /// Each series gets its own horizontal sub-band (stacked vertically within the row).
    /// This is the default — it keeps series visually distinct and easy to compare.
    #[default]
    Grouped,
    /// All series are drawn with transparency on top of each other.
    /// Useful for a single comparison (e.g., two time points).
    Overlap,
}

/// A single series (e.g., one census year) in a [`PopulationPyramid`].
#[derive(Debug, Clone)]
pub struct PyramidSeries {
    /// Series label shown in the legend (e.g., `"1960"`, `"2020"`).
    pub label: String,
    /// `(age_label, left_value, right_value)` — one entry per age group.
    pub groups: Vec<(String, f64, f64)>,
    /// Explicit CSS color for this series. Falls back to `Palette::category10()`.
    pub color: Option<String>,
    /// Fill opacity used in [`PyramidMode::Overlap`].  Default `0.6`.
    pub opacity: f64,
}

impl PyramidSeries {
    pub fn new(label: impl Into<String>) -> Self {
        PyramidSeries {
            label: label.into(),
            groups: vec![],
            color: None,
            opacity: 0.6,
        }
    }
}

/// Population pyramid — a back-to-back horizontal bar chart split by a categorical axis.
///
/// Each row is one age group; the left side shows one demographic (e.g., Male),
/// the right side shows another (e.g., Female).
///
/// **Single-series** (most common): use [`with_group`](Self::with_group) to add each row.
///
/// **Multi-series** (census comparison): use [`with_series`](Self::with_series) to add
/// named series (e.g., different years).  Each series gets its own sub-band within each
/// age group in [`PyramidMode::Grouped`] (default), or overlapping bars in
/// [`PyramidMode::Overlap`].
///
/// # Examples
///
/// ```rust,no_run
/// use kuva::plot::pyramid::{PopulationPyramid, PyramidMode};
/// use kuva::render::{plots::Plot, layout::Layout, render::render_multiple};
/// use kuva::backend::svg::SvgBackend;
///
/// // Single-series pyramid
/// let plot = PopulationPyramid::new()
///     .with_left_label("Male")
///     .with_right_label("Female")
///     .with_group("0–4",   6.5, 6.2)
///     .with_group("5–9",   6.8, 6.5)
///     .with_group("10–14", 6.9, 6.6)
///     .with_group("65+",   3.1, 4.2);
///
/// let plots = vec![Plot::Pyramid(plot)];
/// let layout = Layout::auto_from_plots(&plots).with_title("Population Pyramid");
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// std::fs::write("pyramid.svg", svg).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct PopulationPyramid {
    /// All data series (one for a simple pyramid, multiple for census comparison).
    pub series: Vec<PyramidSeries>,
    /// Label placed above the left half (e.g., `"Male"`).
    pub left_label: String,
    /// Label placed above the right half (e.g., `"Female"`).
    pub right_label: String,
    /// CSS color for the left bars in single-series mode.  Default: `"#4C72B0"`.
    pub left_color: String,
    /// CSS color for the right bars in single-series mode.  Default: `"#DD8452"`.
    pub right_color: String,
    /// Normalise values to percent of total population.  Default `false`.
    pub normalize: bool,
    /// Show value labels on each bar.  Default `false`.
    pub show_values: bool,
    /// Fraction of each row height reserved as blank space between rows.
    /// Default `0.15` (15% gap).
    pub group_gap: f64,
    /// Additional gap between sub-bands in [`PyramidMode::Grouped`].
    /// Expressed as a fraction of the row height.  Default `0.04`.
    pub bar_gap: f64,
    /// How multiple series are displayed within each age-group row.
    /// Default [`PyramidMode::Grouped`].
    pub mode: PyramidMode,
    /// Show a legend.  Default `false`.
    pub show_legend: bool,
}

impl Default for PopulationPyramid {
    fn default() -> Self { Self::new() }
}

impl PopulationPyramid {
    /// Create a [`PopulationPyramid`] with default settings.
    pub fn new() -> Self {
        PopulationPyramid {
            series: vec![],
            left_label: "Left".to_string(),
            right_label: "Right".to_string(),
            left_color: "#4C72B0".to_string(),
            right_color: "#DD8452".to_string(),
            normalize: false,
            show_values: false,
            group_gap: 0.15,
            bar_gap: 0.04,
            mode: PyramidMode::Grouped,
            show_legend: false,
        }
    }

    // ── Data builders ─────────────────────────────────────────────────────────

    /// Add a single age-group row (convenience for single-series mode).
    ///
    /// Creates an anonymous first series if none exists.
    pub fn with_group(
        mut self,
        age_label: impl Into<String>,
        left: impl Into<f64>,
        right: impl Into<f64>,
    ) -> Self {
        if self.series.is_empty() {
            self.series.push(PyramidSeries::new(""));
        }
        self.series[0].groups.push((age_label.into(), left.into(), right.into()));
        self
    }

    /// Add a named series (multi-series / census comparison mode).
    ///
    /// `groups` is an iterator of `(age_label, left_value, right_value)`.
    pub fn with_series<S, A, L, R, I>(mut self, name: S, groups: I) -> Self
    where
        S: Into<String>,
        A: Into<String>,
        L: Into<f64>,
        R: Into<f64>,
        I: IntoIterator<Item = (A, L, R)>,
    {
        let mut s = PyramidSeries::new(name);
        for (age, left, right) in groups {
            s.groups.push((age.into(), left.into(), right.into()));
        }
        self.series.push(s);
        self
    }

    // ── Labels ────────────────────────────────────────────────────────────────

    /// Set the label for the left side (e.g., `"Male"`).
    pub fn with_left_label(mut self, label: impl Into<String>) -> Self {
        self.left_label = label.into();
        self
    }

    /// Set the label for the right side (e.g., `"Female"`).
    pub fn with_right_label(mut self, label: impl Into<String>) -> Self {
        self.right_label = label.into();
        self
    }

    // ── Colors ────────────────────────────────────────────────────────────────

    /// Set the bar color for the left side (single-series mode).
    pub fn with_left_color(mut self, color: impl Into<String>) -> Self {
        self.left_color = color.into();
        self
    }

    /// Set the bar color for the right side (single-series mode).
    pub fn with_right_color(mut self, color: impl Into<String>) -> Self {
        self.right_color = color.into();
        self
    }

    /// Set an explicit CSS color for a named series (multi-series mode).
    pub fn with_series_color(mut self, name: &str, color: impl Into<String>) -> Self {
        if let Some(s) = self.series.iter_mut().find(|s| s.label == name) {
            s.color = Some(color.into());
        }
        self
    }

    // ── Options ───────────────────────────────────────────────────────────────

    /// Normalise all values to percent of total population.
    pub fn with_normalize(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }

    /// Show / hide value labels on each bar.
    pub fn with_show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set the fraction of each row height occupied by bars (0–1).
    ///
    /// `bar_width = 0.8` means bars fill 80% of the row, leaving 10% blank
    /// above and below as padding.  This is the complement of `group_gap`:
    /// `bar_width = 1.0 - group_gap`.  Default: `0.85` (i.e. `group_gap = 0.15`).
    pub fn with_bar_width(mut self, width: f64) -> Self {
        self.group_gap = (1.0 - width).clamp(0.0, 0.9);
        self
    }

    /// Set the fraction of row height reserved as blank space between rows (0–0.9).
    /// Equivalent to `with_bar_width(1.0 - gap)`.
    pub fn with_group_gap(mut self, gap: f64) -> Self {
        self.group_gap = gap.clamp(0.0, 0.9);
        self
    }

    /// Set the additional gap between sub-bands in Grouped mode (0–0.5).
    pub fn with_bar_gap(mut self, gap: f64) -> Self {
        self.bar_gap = gap.clamp(0.0, 0.5);
        self
    }

    /// Set the rendering mode (Grouped or Overlap).
    pub fn with_mode(mut self, mode: PyramidMode) -> Self {
        self.mode = mode;
        self
    }

    /// Show / hide the legend.
    pub fn with_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Age group labels taken from the first series (bottom → top order).
    pub fn age_labels(&self) -> Vec<String> {
        self.series
            .first()
            .map(|s| s.groups.iter().map(|(l, _, _)| l.clone()).collect())
            .unwrap_or_default()
    }

    /// Total population: sum of every `left + right` across all series and groups.
    /// Used as the denominator when `normalize = true`.
    pub fn total_population(&self) -> f64 {
        self.series
            .iter()
            .flat_map(|s| s.groups.iter())
            .map(|(_, l, r)| l + r)
            .sum()
    }

    /// Maximum value across all data (after normalisation if enabled).
    /// This defines the half-width of the symmetric x axis.
    pub fn max_value(&self) -> f64 {
        if self.series.is_empty() { return 1.0; }
        let denom = if self.normalize {
            self.total_population().max(1e-10) / 100.0
        } else {
            1.0
        };
        self.series
            .iter()
            .flat_map(|s| s.groups.iter())
            .map(|(_, l, r)| (l / denom).max(r / denom))
            .fold(0.0_f64, f64::max)
    }

    /// Number of age groups (from the first series).
    pub fn n_groups(&self) -> usize {
        self.series.first().map_or(0, |s| s.groups.len())
    }
}
