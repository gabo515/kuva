/// A single observation (row) in a parallel coordinates plot.
#[derive(Debug, Clone)]
pub struct ParallelRow {
    /// Values for each axis, in axis order.
    pub values: Vec<f64>,
    /// Optional group label for coloring.
    pub group: Option<String>,
}

/// A parallel coordinates plot.
///
/// Each row in the dataset is drawn as a polyline (or bezier curve) passing
/// through one vertical axis per dimension.  Axes are independently normalised
/// to \[0, 1\] by default so that differently-scaled dimensions can be compared.
///
/// # Example
///
/// ```rust,no_run
/// use kuva::plot::parallel::ParallelPlot;
/// use kuva::render::plots::Plot;
/// use kuva::render::layout::Layout;
/// use kuva::render::render::render_multiple;
/// use kuva::backend::svg::SvgBackend;
///
/// let plot = ParallelPlot::new()
///     .with_axis_names(["Sepal.L", "Sepal.W", "Petal.L", "Petal.W"])
///     .with_row_group("setosa",     vec![5.1, 3.5, 1.4, 0.2])
///     .with_row_group("versicolor", vec![7.0, 3.2, 4.7, 1.4])
///     .with_row_group("virginica",  vec![6.3, 3.3, 6.0, 2.5])
///     .with_curved(true)
///     .with_mean(true);
///
/// let plots = vec![Plot::Parallel(plot)];
/// let layout = Layout::auto_from_plots(&plots).with_title("Iris");
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// std::fs::write("parallel.svg", svg).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct ParallelPlot {
    /// Name for each axis (column).  Length must equal the number of values per row.
    pub axis_names: Vec<String>,
    /// All rows.
    pub rows: Vec<ParallelRow>,

    // Display options
    /// Normalise each axis independently to \[0, 1\] before drawing (default: `true`).
    /// Set to `false` when all axes share a common scale.
    pub normalize: bool,
    /// Draw smooth S-shaped bezier curves instead of straight polylines (default: `false`).
    pub curved: bool,
    /// Stroke width for polylines (default: `1.2`).
    pub stroke_width: f64,
    /// Global opacity for polylines (default: `0.6`).
    pub opacity: f64,
    /// Fallback color when no groups are provided (default: `"steelblue"`).
    pub color: String,
    /// Explicit per-group colors (CSS color strings).
    /// Falls back to `category10` palette when `None` or shorter than number of groups.
    pub group_colors: Option<Vec<String>>,
    /// Whether to draw tick marks + value labels on each axis (default: `true`).
    pub show_axis_ticks: bool,
    /// Number of ticks per axis (default: `5`).
    pub axis_ticks: usize,
    /// Draw a bold mean line for each group (default: `false`).
    ///
    /// One thick polyline (or curve when `curved = true`) is drawn per group at the
    /// mean value on each axis, making group-level patterns clearly visible even when
    /// individual lines are dense.
    pub show_mean: bool,
    /// Stroke width used for mean lines (default: `3.0`).
    pub mean_stroke_width: f64,
    /// Per-axis inversion flags.  When `inverted_axes[i]` is `true`, axis `i` is drawn
    /// bottom-to-top (high values at bottom).  An inverted axis is indicated by a small
    /// downward-pointing triangle beneath the axis label.
    pub inverted_axes: Vec<bool>,
    /// Legend group title.  When set, a legend entry per group is added.
    pub legend_label: Option<String>,
    /// Draw a thin grey background band behind each axis line (default: `false`).
    pub show_axis_bands: bool,
}

impl Default for ParallelPlot {
    fn default() -> Self { Self::new() }
}

impl ParallelPlot {
    /// Create a parallel coordinates plot with default settings.
    pub fn new() -> Self {
        Self {
            axis_names: vec![],
            rows: vec![],
            normalize: true,
            curved: false,
            stroke_width: 1.2,
            opacity: 0.6,
            color: "steelblue".to_string(),
            group_colors: None,
            show_axis_ticks: true,
            axis_ticks: 5,
            show_mean: false,
            mean_stroke_width: 3.0,
            inverted_axes: vec![],
            legend_label: None,
            show_axis_bands: false,
        }
    }

    /// Set the axis (column) names.
    pub fn with_axis_names(mut self, names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.axis_names = names.into_iter().map(|n| n.into()).collect();
        self
    }

    /// Add a row with no group (uses the fallback color).
    pub fn with_row(mut self, values: impl IntoIterator<Item = impl Into<f64>>) -> Self {
        self.rows.push(ParallelRow { values: values.into_iter().map(|v| v.into()).collect(), group: None });
        self
    }

    /// Add a row assigned to a named group.
    pub fn with_row_group(mut self, group: impl Into<String>, values: impl IntoIterator<Item = impl Into<f64>>) -> Self {
        self.rows.push(ParallelRow { values: values.into_iter().map(|v| v.into()).collect(), group: Some(group.into()) });
        self
    }

    /// Add multiple ungrouped rows at once.
    pub fn with_rows(mut self, rows: impl IntoIterator<Item = Vec<f64>>) -> Self {
        for v in rows {
            self.rows.push(ParallelRow { values: v, group: None });
        }
        self
    }

    /// Add multiple rows belonging to the same group.
    pub fn with_group_rows(
        mut self,
        group: impl Into<String>,
        rows: impl IntoIterator<Item = Vec<f64>>,
    ) -> Self {
        let g = group.into();
        for v in rows {
            self.rows.push(ParallelRow { values: v, group: Some(g.clone()) });
        }
        self
    }

    /// Enable or disable per-axis normalisation (default: `true`).
    pub fn with_normalize(mut self, v: bool) -> Self {
        self.normalize = v;
        self
    }

    /// Draw smooth S-shaped bezier curves instead of straight polylines (default: `false`).
    ///
    /// Curves are cubic bezier segments whose control points are at the horizontal midpoint
    /// between each pair of adjacent axes, matching the y-coordinate of the start/end point.
    /// This produces the classic "flow" look common in D3 parallel coordinates.
    pub fn with_curved(mut self, v: bool) -> Self {
        self.curved = v;
        self
    }

    /// Set the polyline stroke width (default: `1.2`).
    pub fn with_stroke_width(mut self, v: f64) -> Self {
        self.stroke_width = v;
        self
    }

    /// Set the global polyline opacity (default: `0.6`).
    pub fn with_opacity(mut self, v: f64) -> Self {
        self.opacity = v;
        self
    }

    /// Set the fallback color when no groups are used.
    pub fn with_color(mut self, c: impl Into<String>) -> Self {
        self.color = c.into();
        self
    }

    /// Set explicit per-group colors.
    pub fn with_group_colors(mut self, colors: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.group_colors = Some(colors.into_iter().map(|c| c.into()).collect());
        self
    }

    /// Show or hide per-axis tick marks and value labels (default: `true`).
    pub fn with_axis_ticks(mut self, v: bool) -> Self {
        self.show_axis_ticks = v;
        self
    }

    /// Number of value ticks drawn on each axis (default: `5`).
    pub fn with_tick_count(mut self, n: usize) -> Self {
        self.axis_ticks = n.max(2);
        self
    }

    /// Draw a bold group-mean line over the individual polylines (default: `false`).
    ///
    /// One thick line per group passes through the mean value on each axis.
    /// Useful when individual lines are dense or heavily overlapping.
    pub fn with_mean(mut self, v: bool) -> Self {
        self.show_mean = v;
        self
    }

    /// Set the stroke width for mean lines (default: `3.0`).
    pub fn with_mean_stroke_width(mut self, v: f64) -> Self {
        self.mean_stroke_width = v;
        self
    }

    /// Invert a single axis so that larger values appear at the bottom (default: none inverted).
    ///
    /// An inverted axis is indicated by a small downward triangle beneath its label.
    /// Calling this method multiple times inverts each named index independently.
    pub fn with_invert_axis(mut self, axis_index: usize) -> Self {
        if self.inverted_axes.len() <= axis_index {
            self.inverted_axes.resize(axis_index + 1, false);
        }
        self.inverted_axes[axis_index] = true;
        self
    }

    /// Invert a set of axes by index (see [`with_invert_axis`](Self::with_invert_axis)).
    pub fn with_inverted_axes(mut self, indices: impl IntoIterator<Item = usize>) -> Self {
        for i in indices {
            if self.inverted_axes.len() <= i {
                self.inverted_axes.resize(i + 1, false);
            }
            self.inverted_axes[i] = true;
        }
        self
    }

    /// Attach a legend; the label is used as the legend group title.
    pub fn with_legend(mut self, label: impl Into<String>) -> Self {
        self.legend_label = Some(label.into());
        self
    }

    /// Draw a light grey band behind each axis line (default: `false`).
    pub fn with_axis_bands(mut self, v: bool) -> Self {
        self.show_axis_bands = v;
        self
    }

    // ── Internal helpers ─────────────────────────────────────────────────────

    /// Returns the color for group index `i`.
    pub(crate) fn color_for_group_idx(&self, i: usize) -> String {
        use crate::render::palette::Palette;
        if let Some(ref cv) = self.group_colors {
            if let Some(c) = cv.get(i) {
                if !c.is_empty() {
                    return c.clone();
                }
            }
        }
        let pal = Palette::category10();
        pal[i % pal.len()].to_string()
    }

    /// Returns `true` if axis `i` is inverted.
    pub(crate) fn is_inverted(&self, i: usize) -> bool {
        self.inverted_axes.get(i).copied().unwrap_or(false)
    }

    /// Unique, ordered group names (preserves first-seen order).
    pub(crate) fn groups(&self) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        let mut result = vec![];
        for row in &self.rows {
            if let Some(ref g) = row.group {
                if seen.insert(g.clone()) {
                    result.push(g.clone());
                }
            }
        }
        result
    }
}
