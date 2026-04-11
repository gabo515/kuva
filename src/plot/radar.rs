/// Radar / spider chart — multivariate data plotted on radial axes.
///
/// Each axis represents one variable; values are mapped to radial distance
/// from the centre. Multiple series (observations or groups) can be overlaid
/// as filled or stroked polygons.
///
/// # Example
/// ```
/// use kuva::plot::radar::RadarPlot;
///
/// let plot = RadarPlot::new(vec!["Speed", "Power", "Agility", "Stamina", "Technique"])
///     .with_series_labeled(vec![0.8_f64, 0.6, 0.9, 0.7, 0.75], "Group A")
///     .with_series_labeled(vec![0.6_f64, 0.9, 0.5, 0.8, 0.70], "Group B")
///     .with_filled(true)
///     .with_legend(true);
/// ```
#[derive(Debug, Clone)]
pub struct RadarPlot {
    /// Names of the axes, in order (rendered clockwise from the start angle).
    pub axes: Vec<String>,
    /// Data series to plot.
    pub series: Vec<RadarSeries>,
    /// Reference polygons drawn as dashed overlays (e.g. a baseline or target).
    pub references: Vec<RadarReference>,
    /// Fill each polygon (default `false`).
    pub filled: bool,
    /// Fill opacity when `filled` is `true` (default `0.25`).
    pub opacity: f64,
    /// Shared value range `(min, max)`. `None` = auto from data.
    pub range: Option<(f64, f64)>,
    /// Per-axis value ranges. Index must match axis position; `None` entries use the shared range.
    pub axis_ranges: Vec<Option<(f64, f64)>>,
    /// Per-axis inversion flags. When `true` for axis `i`, high values plot near the centre.
    pub inverted_axes: Vec<bool>,
    /// Number of concentric grid rings (default `5`).
    pub grid_lines: usize,
    /// Draw grid rings and axis lines (default `true`).
    pub show_grid: bool,
    /// Draw grid rings as circles instead of polygons (default `false`).
    pub circular_grid: bool,
    /// Show a legend box (default `false`).
    pub show_legend: bool,
    /// Draw dots at each vertex (radius in px; `None` = no dots).
    pub dot_size: Option<f64>,
    /// Series polygon stroke width (default `1.5`).
    pub stroke_width: f64,
    /// Normalise each axis independently to \[0, 1\] before plotting.
    /// Useful when axes have different units. Grid labels become percentages.
    pub normalize: bool,
    /// Show the data value as a label at each polygon vertex (default `false`).
    pub vertex_labels: bool,
    /// Angle of the first axis in degrees, measured clockwise from north (default `−90` = top).
    pub start_angle_deg: f64,
    /// Draw short tick marks on each axis at every grid ring (default `false`).
    pub axis_ticks: bool,
}

/// One series (polygon) in a [`RadarPlot`].
#[derive(Debug, Clone)]
pub struct RadarSeries {
    /// One value per axis; length must equal `RadarPlot::axes.len()`.
    pub values: Vec<f64>,
    /// Optional display label (used in legend and tooltip).
    pub label: Option<String>,
    /// Override colour; `None` = palette-assigned.
    pub color: Option<String>,
    /// Per-axis ±error used to draw a shaded band around the polygon.
    pub errors: Option<Vec<f64>>,
    /// SVG `stroke-dasharray` for this series line (e.g. `"6,3"`).
    pub dasharray: Option<String>,
}

/// A reference polygon drawn as a dashed overlay in a [`RadarPlot`].
#[derive(Debug, Clone)]
pub struct RadarReference {
    /// One value per axis; length must equal `RadarPlot::axes.len()`.
    pub values: Vec<f64>,
    /// Label shown in the legend when `show_legend` is `true`.
    pub label: Option<String>,
    /// Override colour; `None` = `"#999999"`.
    pub color: Option<String>,
}

impl RadarPlot {
    /// Create a new radar plot with the given axis names.
    pub fn new(axes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            axes: axes.into_iter().map(Into::into).collect(),
            series: Vec::new(),
            references: Vec::new(),
            filled: false,
            opacity: 0.25,
            range: None,
            axis_ranges: Vec::new(),
            inverted_axes: Vec::new(),
            grid_lines: 5,
            show_grid: true,
            circular_grid: false,
            show_legend: false,
            dot_size: None,
            stroke_width: 1.5,
            normalize: false,
            vertex_labels: false,
            start_angle_deg: -90.0,
            axis_ticks: false,
        }
    }

    /// Add an unlabelled series.
    pub fn with_series(mut self, values: impl IntoIterator<Item = impl Into<f64>>) -> Self {
        self.series.push(RadarSeries {
            values: values.into_iter().map(Into::into).collect(),
            label: None,
            color: None,
            errors: None,
            dasharray: None,
        });
        self
    }

    /// Add a labelled series (label appears in legend).
    pub fn with_series_labeled(
        mut self,
        values: impl IntoIterator<Item = impl Into<f64>>,
        label: impl Into<String>,
    ) -> Self {
        self.series.push(RadarSeries {
            values: values.into_iter().map(Into::into).collect(),
            label: Some(label.into()),
            color: None,
            errors: None,
            dasharray: None,
        });
        self
    }

    /// Add a labelled series with an explicit colour.
    pub fn with_series_color(
        mut self,
        values: impl IntoIterator<Item = impl Into<f64>>,
        label: impl Into<String>,
        color: impl Into<String>,
    ) -> Self {
        self.series.push(RadarSeries {
            values: values.into_iter().map(Into::into).collect(),
            label: Some(label.into()),
            color: Some(color.into()),
            errors: None,
            dasharray: None,
        });
        self
    }

    /// Attach per-axis ±errors to the **most recently added** series.
    ///
    /// A shaded band is drawn between `value − error` and `value + error` on each axis.
    /// Call this immediately after `with_series*`.
    pub fn with_series_errors(
        mut self,
        errors: impl IntoIterator<Item = impl Into<f64>>,
    ) -> Self {
        if let Some(last) = self.series.last_mut() {
            last.errors = Some(errors.into_iter().map(Into::into).collect());
        }
        self
    }

    /// Set a custom SVG `stroke-dasharray` on the **most recently added** series.
    ///
    /// Example: `"6,3"` for dashes, `"2,2"` for dots. Call immediately after `with_series*`.
    pub fn with_series_dasharray(mut self, dasharray: impl Into<String>) -> Self {
        if let Some(last) = self.series.last_mut() {
            last.dasharray = Some(dasharray.into());
        }
        self
    }

    /// Add a reference polygon drawn as a dashed overlay.
    pub fn with_reference(
        mut self,
        values: impl IntoIterator<Item = impl Into<f64>>,
        label: impl Into<String>,
    ) -> Self {
        self.references.push(RadarReference {
            values: values.into_iter().map(Into::into).collect(),
            label: Some(label.into()),
            color: None,
        });
        self
    }

    /// Add a reference polygon with an explicit colour.
    pub fn with_reference_color(
        mut self,
        values: impl IntoIterator<Item = impl Into<f64>>,
        label: impl Into<String>,
        color: impl Into<String>,
    ) -> Self {
        self.references.push(RadarReference {
            values: values.into_iter().map(Into::into).collect(),
            label: Some(label.into()),
            color: Some(color.into()),
        });
        self
    }

    /// Fill each polygon with a semi-transparent colour.
    pub fn with_filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Set fill opacity (only used when `filled` is `true`).
    pub fn with_opacity(mut self, opacity: impl Into<f64>) -> Self {
        self.opacity = opacity.into();
        self
    }

    /// Set a shared value range for all axes.
    pub fn with_range(mut self, min: impl Into<f64>, max: impl Into<f64>) -> Self {
        self.range = Some((min.into(), max.into()));
        self
    }

    /// Override the value range for a single axis (by index).
    ///
    /// Per-axis ranges take priority over `with_range` and `with_normalize`.
    pub fn with_axis_range(
        mut self,
        axis: usize,
        min: impl Into<f64>,
        max: impl Into<f64>,
    ) -> Self {
        if self.axis_ranges.len() <= axis {
            self.axis_ranges.resize(axis + 1, None);
        }
        self.axis_ranges[axis] = Some((min.into(), max.into()));
        self
    }

    /// Mark a single axis as inverted: high values plot near the centre, low values at the rim.
    pub fn with_inverted_axis(mut self, axis: usize) -> Self {
        if self.inverted_axes.len() <= axis {
            self.inverted_axes.resize(axis + 1, false);
        }
        self.inverted_axes[axis] = true;
        self
    }

    /// Mark multiple axes as inverted (by index).
    pub fn with_inverted_axes(
        mut self,
        indices: impl IntoIterator<Item = usize>,
    ) -> Self {
        for axis in indices {
            if self.inverted_axes.len() <= axis {
                self.inverted_axes.resize(axis + 1, false);
            }
            self.inverted_axes[axis] = true;
        }
        self
    }

    /// Set the number of concentric grid rings.
    pub fn with_grid_lines(mut self, n: usize) -> Self {
        self.grid_lines = n;
        self
    }

    /// Show or hide grid rings and radial axis lines.
    pub fn with_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Draw grid rings as circles rather than polygons.
    pub fn with_circular_grid(mut self, circular: bool) -> Self {
        self.circular_grid = circular;
        self
    }

    /// Show a legend.
    pub fn with_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Draw dots at polygon vertices with the given radius in px.
    pub fn with_dot_size(mut self, size: impl Into<f64>) -> Self {
        self.dot_size = Some(size.into());
        self
    }

    /// Set the series polygon stroke width.
    pub fn with_stroke_width(mut self, w: impl Into<f64>) -> Self {
        self.stroke_width = w.into();
        self
    }

    /// Normalise each axis independently to \[0, 1\] (ignores `range`).
    /// Useful when axes have different units or scales. Grid labels become percentages.
    pub fn with_normalize(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }

    /// Show the data value as a small label at each polygon vertex.
    pub fn with_vertex_labels(mut self, show: bool) -> Self {
        self.vertex_labels = show;
        self
    }

    /// Set the starting angle (in degrees, clockwise from north) for the first axis.
    ///
    /// Default is `−90°` (axis 0 points straight up). Use `0°` for axis 0 pointing right.
    pub fn with_start_angle(mut self, deg: impl Into<f64>) -> Self {
        self.start_angle_deg = deg.into();
        self
    }

    /// Place axis `k` at the top (north) position.
    ///
    /// Equivalent to `with_start_angle(−90 − k × 360/n)`.
    pub fn with_start_axis(mut self, k: usize) -> Self {
        use std::f64::consts::PI;
        let n = self.axes.len().max(1);
        self.start_angle_deg = (-PI / 2.0 - k as f64 * 2.0 * PI / n as f64).to_degrees();
        self
    }

    /// Draw short tick marks on each axis at every grid ring intersection.
    pub fn with_axis_ticks(mut self, show: bool) -> Self {
        self.axis_ticks = show;
        self
    }
}
