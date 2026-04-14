/// How bar colors are assigned in a funnel chart.
#[derive(Debug, Clone, Default)]
pub enum FunnelColorMode {
    /// All bars share a single palette color.  **(default)**
    #[default]
    Uniform,
    /// Each stage gets a distinct category10 palette color.
    ByStage,
    /// Bars darken progressively from top to bottom (hue from the first palette color).
    Gradient,
}

/// Orientation of the funnel (direction of flow).
#[derive(Debug, Clone, Default)]
pub enum FunnelOrientation {
    /// Top-to-bottom: widest bar at the top.  **(default)**
    #[default]
    Vertical,
    /// Left-to-right: widest bar on the left.
    Horizontal,
}

/// One stage (bar) in a [`FunnelPlot`].
#[derive(Debug, Clone)]
pub struct FunnelStage {
    pub label: String,
    pub value: f64,
    /// Optional explicit CSS color.  Falls back to the resolved color mode.
    pub color: Option<String>,
}

impl FunnelStage {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        FunnelStage { label: label.into(), value, color: None }
    }

    pub fn colored(label: impl Into<String>, value: f64, color: impl Into<String>) -> Self {
        FunnelStage { label: label.into(), value, color: Some(color.into()) }
    }
}

/// Funnel chart — shows attrition / conversion through ordered stages.
///
/// Each bar represents one stage; widths are proportional to stage values.
/// Trapezoidal connectors between bars make the attrition visually explicit.
///
/// # Basic usage
///
/// ```rust,no_run
/// use kuva::plot::funnel::FunnelPlot;
/// use kuva::render::{plots::Plot, layout::Layout, render::render_multiple};
/// use kuva::backend::svg::SvgBackend;
///
/// let plot = FunnelPlot::new()
///     .with_stage("Screened",   1200)
///     .with_stage("Eligible",    800)
///     .with_stage("Enrolled",    600)
///     .with_stage("Completed",   540);
///
/// let plots = vec![Plot::Funnel(plot)];
/// let layout = Layout::auto_from_plots(&plots).with_title("CONSORT Flow");
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// std::fs::write("funnel.svg", svg).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct FunnelPlot {
    /// Ordered stages, first = largest / entry point.
    pub stages: Vec<FunnelStage>,
    /// Orientation of the funnel (default: [`FunnelOrientation::Vertical`]).
    pub orientation: FunnelOrientation,
    /// Draw trapezoidal connectors between adjacent bars.  Default: `true`.
    pub show_connectors: bool,
    /// Connector fill opacity (0–1).  Default: `0.4`.
    /// Named to mirror [`SankeyPlot::link_opacity`](crate::plot::sankey::SankeyPlot::link_opacity).
    pub connector_opacity: f64,
    /// Show absolute value labels on each bar.  Default: `true`.
    pub show_values: bool,
    /// Show percentage-of-first-stage labels alongside value labels.  Default: `false`.
    pub show_percents: bool,
    /// Show step-to-step conversion rate in each connector.  Default: `true`.
    pub show_conversion: bool,
    /// Bar color mode.  Default: [`FunnelColorMode::Uniform`].
    pub color_mode: FunnelColorMode,
    /// Gap in pixels between adjacent bars.  Default: `4.0`.
    /// Named to mirror [`SankeyPlot::node_gap`](crate::plot::sankey::SankeyPlot::node_gap).
    pub stage_gap: f64,
    /// If set, a legend entry is rendered for each stage.
    /// Named to mirror [`SankeyPlot::legend_label`](crate::plot::sankey::SankeyPlot::legend_label).
    pub legend_label: Option<String>,
    // ── Mirror (diverging) mode ───────────────────────────────────────────────
    /// Right-side stages for a back-to-back diverging funnel.  `None` = standard funnel.
    pub mirror: Option<Vec<FunnelStage>>,
    /// Label placed above the left (standard) side in diverging mode.
    pub left_label: Option<String>,
    /// Label placed above the right (mirror) side in diverging mode.
    pub right_label: Option<String>,
}

impl Default for FunnelPlot {
    fn default() -> Self { Self::new() }
}

impl FunnelPlot {
    /// Create a [`FunnelPlot`] with default settings.
    pub fn new() -> Self {
        FunnelPlot {
            stages: vec![],
            orientation: FunnelOrientation::Vertical,
            show_connectors: true,
            connector_opacity: 0.4,
            show_values: true,
            show_percents: false,
            show_conversion: true,
            color_mode: FunnelColorMode::Uniform,
            stage_gap: 4.0,
            legend_label: None,
            mirror: None,
            left_label: None,
            right_label: None,
        }
    }

    // ── Data ──────────────────────────────────────────────────────────────────

    /// Add a stage using any numeric value type.
    pub fn with_stage(mut self, label: impl Into<String>, value: impl Into<f64>) -> Self {
        self.stages.push(FunnelStage::new(label, value.into()));
        self
    }

    /// Add a stage with an explicit CSS color.
    pub fn with_stage_color(
        mut self,
        label: impl Into<String>,
        value: impl Into<f64>,
        color: impl Into<String>,
    ) -> Self {
        self.stages.push(FunnelStage::colored(label, value.into(), color));
        self
    }

    /// Add multiple stages at once from an iterator of `(label, value)`.
    /// Mirrors [`SankeyPlot::with_links`](crate::plot::sankey::SankeyPlot::with_links).
    pub fn with_stages<S, I>(mut self, stages: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = (S, f64)>,
    {
        for (label, value) in stages {
            self.stages.push(FunnelStage::new(label, value));
        }
        self
    }

    // ── Mirror / diverging mode ───────────────────────────────────────────────

    /// Enable diverging (back-to-back) mode with a second set of stages.
    /// The right side mirrors the left side around a shared center axis.
    pub fn with_mirror(mut self, stages: impl IntoIterator<Item = FunnelStage>) -> Self {
        self.mirror = Some(stages.into_iter().collect());
        self
    }

    /// Add a mirror stage using `(label, value)` — convenience wrapper for `with_mirror`.
    pub fn with_mirror_stages<S, I>(mut self, stages: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = (S, f64)>,
    {
        let v: Vec<FunnelStage> = stages
            .into_iter()
            .map(|(l, v)| FunnelStage::new(l, v))
            .collect();
        self.mirror = Some(v);
        self
    }

    /// Set labels for the left and right sides of a diverging funnel.
    pub fn with_mirror_labels(
        mut self,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        self.left_label = Some(left.into());
        self.right_label = Some(right.into());
        self
    }

    // ── Appearance ────────────────────────────────────────────────────────────

    /// Set funnel orientation (vertical / horizontal).
    pub fn with_orientation(mut self, orientation: FunnelOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Show / hide trapezoidal connectors between bars.
    pub fn with_connectors(mut self, show: bool) -> Self {
        self.show_connectors = show;
        self
    }

    /// Set connector fill opacity.
    /// Mirrors [`SankeyPlot::with_link_opacity`](crate::plot::sankey::SankeyPlot::with_link_opacity).
    pub fn with_connector_opacity(mut self, opacity: f64) -> Self {
        self.connector_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Show / hide absolute value labels on each bar.
    pub fn with_show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Show / hide percentage-of-first-stage labels.
    pub fn with_show_percents(mut self, show: bool) -> Self {
        self.show_percents = show;
        self
    }

    /// Show / hide step-to-step conversion rate in connector areas.
    pub fn with_show_conversion(mut self, show: bool) -> Self {
        self.show_conversion = show;
        self
    }

    /// Set the bar color mode.
    pub fn with_color_mode(mut self, mode: FunnelColorMode) -> Self {
        self.color_mode = mode;
        self
    }

    /// Set the gap in pixels between adjacent bars.
    /// Mirrors [`SankeyPlot::with_node_gap`](crate::plot::sankey::SankeyPlot::with_node_gap).
    pub fn with_stage_gap(mut self, gap: f64) -> Self {
        self.stage_gap = gap.max(0.0);
        self
    }

    /// Enable the legend; each stage gets one entry.
    /// Mirrors [`SankeyPlot::with_legend`](crate::plot::sankey::SankeyPlot::with_legend).
    pub fn with_legend(mut self, label: impl Into<String>) -> Self {
        self.legend_label = Some(label.into());
        self
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Number of stages (used for estimated_primitives).
    pub(crate) fn stage_count(&self) -> usize {
        self.stages.len().max(self.mirror.as_ref().map_or(0, |m| m.len()))
    }

    /// Maximum value across all stages (both sides in mirror mode).
    pub(crate) fn max_value(&self) -> f64 {
        let left = self.stages.iter().map(|s| s.value).fold(0.0_f64, f64::max);
        let right = self.mirror.as_ref()
            .map(|m| m.iter().map(|s| s.value).fold(0.0_f64, f64::max))
            .unwrap_or(0.0);
        left.max(right)
    }
}
