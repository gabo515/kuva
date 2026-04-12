/// Re-export the shared `ColorMap` type.
pub use crate::plot::histogram2d::ColorMap;

/// Re-export `TreemapNode` — sunburst uses the same hierarchical data model.
pub use crate::plot::treemap::TreemapNode;

/// How to derive the fill color of each sunburst arc.
#[derive(Clone, Default)]
pub enum SunburstColorMode {
    /// Each top-level root gets a distinct category10 color; descendants inherit it. **(default)**
    #[default]
    ByParent,
    /// Color leaf arcs by value (or a parallel `color_values` vector) using a colormap.
    /// Parent arcs are drawn as neutral `#e0e0e0`.
    ByValue(ColorMap),
    /// Use `TreemapNode::color` on each node.  Nodes without an explicit color fall back to `"#888888"`.
    Explicit,
}

/// Builder for a sunburst (radial hierarchy) plot.
///
/// A sunburst tiles a circle into concentric rings.  Each ring represents one
/// depth level; arc widths within a ring are proportional to node values.
/// Uses the same [`TreemapNode`] data model as [`crate::plot::treemap::TreemapPlot`].
///
/// # Basic usage
///
/// ```rust,no_run
/// use kuva::plot::sunburst::{SunburstPlot, TreemapNode};
/// use kuva::render::plots::Plot;
/// use kuva::render::layout::Layout;
/// use kuva::render::render::render_multiple;
/// use kuva::backend::svg::SvgBackend;
///
/// let plot = SunburstPlot::new()
///     .with_node(TreemapNode::new("Root", vec![
///         TreemapNode::leaf("A", 30.0),
///         TreemapNode::leaf("B", 45.0),
///         TreemapNode::leaf("C", 25.0),
///     ]));
///
/// let plots = vec![Plot::Sunburst(plot)];
/// let layout = Layout::auto_from_plots(&plots).with_title("Sunburst");
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// std::fs::write("sunburst.svg", svg).unwrap();
/// ```
#[derive(Clone)]
pub struct SunburstPlot {
    /// Top-level root nodes (forest supported — multiple roots share the innermost ring).
    pub roots: Vec<TreemapNode>,
    /// Optional parallel `color_values` for each leaf (depth-first order) in `ByValue` mode.
    pub color_values: Option<Vec<f64>>,
    /// Color mode.  Default: [`SunburstColorMode::ByParent`].
    pub color_mode: SunburstColorMode,
    /// Show arc labels.  Default: `true`.
    pub show_labels: bool,
    /// Minimum arc angle in degrees below which labels are suppressed.  Default: `15.0`.
    pub min_label_angle: f64,
    /// Fractional inner radius of the innermost ring (`0.0` = full disc, `0.3` = donut-style).
    /// Range `[0.0, 1.0)`.  Default: `0.0`.
    pub inner_radius_frac: f64,
    /// Limit how many depth levels are rendered.  `None` = unlimited.  Default: `None`.
    pub max_depth: Option<usize>,
    /// Emit SVG `<title>` hover tooltips.  Default: `true`.
    pub show_tooltips: bool,
    /// Show a colorbar in `ByValue` mode.  Default: `false`; auto-enabled by `.with_color_mode(ByValue(_))`.
    pub show_colorbar: bool,
    /// Override the colorbar label.  Auto-derived when `None`.
    pub colorbar_label: Option<String>,
    /// Clamp the colorbar scale to `(lo, hi)`.
    pub color_range: Option<(f64, f64)>,
    /// Gap in pixels between adjacent rings.  Default: `1.0`.
    pub ring_gap: f64,
    /// Starting angle in degrees (0 = top / 12-o'clock, clockwise).  Default: `0.0`.
    pub start_angle_deg: f64,
    /// Rotate labels to follow the arc tangent.  Set to `false` for upright horizontal labels.  Default: `true`.
    pub rotate_labels: bool,
}

impl Default for SunburstPlot {
    fn default() -> Self { Self::new() }
}

impl SunburstPlot {
    /// Create a `SunburstPlot` with default settings.
    pub fn new() -> Self {
        SunburstPlot {
            roots: vec![],
            color_values: None,
            color_mode: SunburstColorMode::ByParent,
            show_labels: true,
            min_label_angle: 15.0,
            inner_radius_frac: 0.0,
            max_depth: None,
            show_tooltips: true,
            show_colorbar: false,
            colorbar_label: None,
            color_range: None,
            ring_gap: 1.0,
            start_angle_deg: 0.0,
            rotate_labels: true,
        }
    }

    /// Add a root node.
    pub fn with_node(mut self, node: TreemapNode) -> Self {
        self.roots.push(node);
        self
    }

    /// Convenience: add a named parent with given children.
    pub fn with_children(mut self, label: impl Into<String>, children: Vec<TreemapNode>) -> Self {
        self.roots.push(TreemapNode::new(label, children));
        self
    }

    /// Set the color mode.  Automatically enables `show_colorbar` for `ByValue`.
    pub fn with_color_mode(mut self, mode: SunburstColorMode) -> Self {
        if matches!(mode, SunburstColorMode::ByValue(_)) {
            self.show_colorbar = true;
        }
        self.color_mode = mode;
        self
    }

    /// Supply a parallel `color_values` vector (leaf depth-first order) for `ByValue` coloring.
    pub fn with_color_values(mut self, vals: impl IntoIterator<Item = impl Into<f64>>) -> Self {
        self.color_values = Some(vals.into_iter().map(|v| v.into()).collect());
        self
    }

    /// Show / hide arc labels.
    pub fn with_show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Minimum arc angle (degrees) for a label to be rendered.
    pub fn with_min_label_angle(mut self, degrees: f64) -> Self {
        self.min_label_angle = degrees;
        self
    }

    /// Fractional inner radius (0.0 = solid disc, 0.3 = donut with 30% hole).
    pub fn with_inner_radius(mut self, frac: f64) -> Self {
        self.inner_radius_frac = frac.clamp(0.0, 0.95);
        self
    }

    /// Gap in pixels between adjacent rings.
    pub fn with_ring_gap(mut self, px: f64) -> Self {
        self.ring_gap = px;
        self
    }

    /// Limit render depth (root ring = 0).
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Enable / disable SVG hover tooltips.
    pub fn with_tooltips(mut self, show: bool) -> Self {
        self.show_tooltips = show;
        self
    }

    /// Show / hide the colorbar (only visible in `ByValue` mode).
    pub fn with_colorbar(mut self, show: bool) -> Self {
        self.show_colorbar = show;
        self
    }

    /// Override the colorbar label.
    pub fn with_colorbar_label(mut self, label: impl Into<String>) -> Self {
        self.colorbar_label = Some(label.into());
        self
    }

    /// Clamp the colorbar scale to `[lo, hi]`.
    pub fn with_color_range(mut self, lo: f64, hi: f64) -> Self {
        self.color_range = Some((lo, hi));
        self
    }

    /// Starting angle in degrees (0 = top, clockwise).
    pub fn with_start_angle(mut self, degrees: f64) -> Self {
        self.start_angle_deg = degrees;
        self
    }

    /// Rotate labels to follow the arc tangent (`true`, default) or keep them upright (`false`).
    pub fn with_rotate_labels(mut self, rotate: bool) -> Self {
        self.rotate_labels = rotate;
        self
    }

    /// Count all nodes for `estimated_primitives`.
    pub(crate) fn node_count(&self) -> usize {
        fn count(nodes: &[TreemapNode]) -> usize {
            nodes.iter().map(|n| 1 + count(&n.children)).sum()
        }
        count(&self.roots)
    }

    /// Maximum tree depth (root = 0).
    pub(crate) fn max_tree_depth(&self) -> usize {
        fn depth(nodes: &[TreemapNode], d: usize) -> usize {
            nodes.iter().map(|n| {
                if n.children.is_empty() { d } else { depth(&n.children, d + 1) }
            }).max().unwrap_or(d)
        }
        if self.roots.is_empty() { return 0; }
        depth(&self.roots, 0)
    }
}
