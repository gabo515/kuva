use crate::plot::heatmap::ColorMap;
use crate::plot::scatter::MarkerShape;

/// A single 3D data point.
#[derive(Debug, Clone, Copy)]
pub struct Scatter3DPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Builder for a 3D scatter plot.
///
/// Projects 3D data to 2D using orthographic projection with configurable
/// view angles, then renders with the existing Scene/Primitive system.
/// Renders its own wireframe box, floor grid, ticks, and axis labels —
/// standard 2D axes are skipped.
///
/// # Example
///
/// ```rust,no_run
/// use kuva::plot::scatter3d::Scatter3DPlot;
/// use kuva::backend::svg::SvgBackend;
/// use kuva::render::render::render_multiple;
/// use kuva::render::layout::Layout;
/// use kuva::render::plots::Plot;
///
/// let scatter = Scatter3DPlot::new()
///     .with_data(vec![(1.0, 2.0, 3.0), (4.0, 5.0, 6.0), (7.0, 8.0, 9.0)])
///     .with_color("steelblue");
///
/// let plots = vec![Plot::Scatter3D(scatter)];
/// let layout = Layout::auto_from_plots(&plots)
///     .with_title("3D Scatter");
///
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// std::fs::write("scatter3d.svg", svg).unwrap();
/// ```
pub struct Scatter3DPlot {
    pub data: Vec<Scatter3DPoint>,
    /// Uniform point color (CSS color string). Default `"steelblue"`.
    pub color: String,
    /// Marker radius in pixels. Default `3.0`.
    pub size: f64,
    /// Legend label, if any.
    pub legend_label: Option<String>,
    /// Marker shape. Default `Circle`.
    pub marker: MarkerShape,
    /// Per-point sizes (overrides `size` when set).
    pub sizes: Option<Vec<f64>>,
    /// Per-point colors (overrides `color` when set).
    pub colors: Option<Vec<String>>,
    /// Marker fill opacity (0.0–1.0).
    pub marker_opacity: Option<f64>,
    /// Marker stroke width.
    pub marker_stroke_width: Option<f64>,
    /// Azimuth viewing angle in degrees. Default `-60.0`.
    pub azimuth: f64,
    /// Elevation viewing angle in degrees. Default `30.0`.
    pub elevation: f64,
    /// X-axis label.
    pub x_label: Option<String>,
    /// Y-axis label.
    pub y_label: Option<String>,
    /// Z-axis label.
    pub z_label: Option<String>,
    /// Show grid lines on floor plane. Default `true`.
    pub show_grid: bool,
    /// Show wireframe bounding box. Default `true`.
    pub show_box: bool,
    /// Number of grid/tick divisions per axis. Default `5`.
    pub grid_lines: usize,
    /// Fade distant points for depth cue. Default `false`.
    pub depth_shade: bool,
    /// Color points by Z value using a colormap.
    pub z_colormap: Option<ColorMap>,
    /// Place Z-axis on the right side. Default `true`.
    /// When `false`, the Z-axis is placed on the left visible edge.
    pub z_axis_right: bool,
}

impl Default for Scatter3DPlot {
    fn default() -> Self { Self::new() }
}

impl Scatter3DPlot {
    /// Create a 3D scatter plot with default settings.
    pub fn new() -> Self {
        Self {
            data: vec![],
            color: "steelblue".into(),
            size: 3.0,
            legend_label: None,
            marker: MarkerShape::Circle,
            sizes: None,
            colors: None,
            marker_opacity: None,
            marker_stroke_width: None,
            azimuth: -60.0,
            elevation: 30.0,
            x_label: None,
            y_label: None,
            z_label: None,
            show_grid: true,
            show_box: true,
            grid_lines: 5,
            depth_shade: false,
            z_colormap: None,
            z_axis_right: true,
        }
    }

    /// Set data from (x, y, z) tuples.
    pub fn with_data<I>(mut self, data: I) -> Self
    where
        I: IntoIterator<Item = (f64, f64, f64)>,
    {
        self.data = data
            .into_iter()
            .map(|(x, y, z)| Scatter3DPoint { x, y, z })
            .collect();
        self
    }

    /// Set data from pre-built points.
    pub fn with_points(mut self, points: Vec<Scatter3DPoint>) -> Self {
        self.data = points;
        self
    }

    /// Set the uniform point color (CSS color string).
    pub fn with_color<S: Into<String>>(mut self, color: S) -> Self {
        self.color = color.into();
        self
    }

    /// Set the marker radius in pixels.
    pub fn with_size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// Attach a legend label.
    pub fn with_legend<S: Into<String>>(mut self, label: S) -> Self {
        self.legend_label = Some(label.into());
        self
    }

    /// Set the marker shape.
    pub fn with_marker(mut self, marker: MarkerShape) -> Self {
        self.marker = marker;
        self
    }

    /// Set per-point sizes.
    pub fn with_sizes(mut self, sizes: Vec<f64>) -> Self {
        self.sizes = Some(sizes);
        self
    }

    /// Set per-point colors.
    pub fn with_colors<I, S>(mut self, colors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    /// Set marker fill opacity.
    pub fn with_marker_opacity(mut self, opacity: f64) -> Self {
        self.marker_opacity = Some(opacity);
        self
    }

    /// Set marker stroke width.
    pub fn with_marker_stroke_width(mut self, width: f64) -> Self {
        self.marker_stroke_width = Some(width);
        self
    }

    /// Set the azimuth viewing angle in degrees (default -60).
    pub fn with_azimuth(mut self, deg: f64) -> Self {
        self.azimuth = deg;
        self
    }

    /// Set the elevation viewing angle in degrees (default 30).
    pub fn with_elevation(mut self, deg: f64) -> Self {
        self.elevation = deg;
        self
    }

    /// Set the x-axis label.
    pub fn with_x_label<S: Into<String>>(mut self, label: S) -> Self {
        self.x_label = Some(label.into());
        self
    }

    /// Set the y-axis label.
    pub fn with_y_label<S: Into<String>>(mut self, label: S) -> Self {
        self.y_label = Some(label.into());
        self
    }

    /// Set the z-axis label.
    pub fn with_z_label<S: Into<String>>(mut self, label: S) -> Self {
        self.z_label = Some(label.into());
        self
    }

    /// Toggle floor grid lines.
    pub fn with_show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Toggle wireframe bounding box.
    pub fn with_show_box(mut self, show: bool) -> Self {
        self.show_box = show;
        self
    }

    /// Set the number of grid/tick divisions per axis.
    pub fn with_grid_lines(mut self, n: usize) -> Self {
        self.grid_lines = n;
        self
    }

    /// Enable depth shading (distant points become more transparent).
    pub fn with_depth_shade(mut self, enable: bool) -> Self {
        self.depth_shade = enable;
        self
    }

    /// Color points by Z value using a colormap.
    pub fn with_z_colormap(mut self, cmap: ColorMap) -> Self {
        self.z_colormap = Some(cmap);
        self
    }

    /// Place the Z-axis on the right (`true`, default) or left (`false`) side.
    pub fn with_z_axis_right(mut self, right: bool) -> Self {
        self.z_axis_right = right;
        self
    }
}
