use crate::plot::colormap::ColorMap;

/// Where `(x, y)` sits relative to the rendered arrow.
///
/// Matches the matplotlib convention of the same name.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum QuiverPivot {
    /// `(x, y)` is the back of the arrow; it points *away* from that point. **(default)**
    #[default]
    Tail,
    /// `(x, y)` is the center of the shaft; arrow extends half in each direction.
    /// Reads naturally as "what the field is doing *at* this point."
    Middle,
    /// `(x, y)` is the arrow's tip; the shaft comes *into* the point.
    Tip,
}

/// A single arrow in a quiver plot.
///
/// `(x, y)` is the arrow's tail in data coordinates; `(u, v)` is the vector
/// displacement (also in data coordinates). The rendered arrow length is
/// `(u, v)` multiplied by the plot-level [`QuiverPlot::scale`] and mapped
/// through the axis transform.
#[derive(Debug, Clone)]
pub struct QuiverArrow {
    pub x: f64,
    pub y: f64,
    pub u: f64,
    pub v: f64,
    /// Per-arrow color override. `None` uses the plot-level color or colormap.
    pub color: Option<String>,
}

impl QuiverArrow {
    pub fn magnitude(&self) -> f64 {
        (self.u * self.u + self.v * self.v).sqrt()
    }
}

/// Builder for a quiver (2-D vector field) plot.
///
/// Each arrow has a tail at `(x, y)` and points in direction `(u, v)`. The
/// arrow is drawn as a line segment (the shaft) with a filled triangle
/// (the head) at the tip.
///
/// # Scaling
///
/// - [`QuiverPlot::scale`] is a multiplier applied to the `(u, v)` displacement
///   before mapping to pixel space. Default `1.0` draws each arrow at its
///   natural data-coordinate length.
/// - [`QuiverPlot::auto_scale`] computes `scale` so that the longest arrow
///   spans roughly `fraction` of the shorter axis of the data bounding box.
///   Useful when `(u, v)` have very different magnitudes from `(x, y)`.
///
/// # Coloring
///
/// Three modes, in order of priority:
/// 1. Per-arrow color via [`QuiverArrow::color`].
/// 2. Magnitude-driven colormap via [`QuiverPlot::with_color_map`]. The plot
///    reports its magnitude range so a colorbar is rendered automatically.
/// 3. Single color via [`QuiverPlot::with_color`]. Default `"steelblue"`.
///
/// # Example
///
/// ```rust,no_run
/// use kuva::plot::QuiverPlot;
/// use kuva::backend::svg::SvgBackend;
/// use kuva::render::render::render_multiple;
/// use kuva::render::layout::Layout;
/// use kuva::render::plots::Plot;
///
/// let mut arrows = Vec::new();
/// for i in 0..10 {
///     for j in 0..10 {
///         let x = i as f64;
///         let y = j as f64;
///         arrows.push((x, y, (y - 5.0) * 0.2, -(x - 5.0) * 0.2));
///     }
/// }
///
/// let plot = QuiverPlot::new()
///     .with_arrows(arrows)
///     .with_color("steelblue");
///
/// let plots = vec![Plot::Quiver(plot)];
/// let layout = Layout::auto_from_plots(&plots);
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// ```
#[derive(Debug, Clone)]
pub struct QuiverPlot {
    pub arrows: Vec<QuiverArrow>,
    /// Default arrow color (used when no colormap and no per-arrow color).
    /// Default `"steelblue"`.
    pub color: String,
    /// Multiplier applied to `(u, v)` before axis mapping.
    ///
    /// - `Some(s)` — use `s` directly (set via [`QuiverPlot::with_scale`]).
    /// - `None` — auto-compute so the longest arrow spans
    ///   [`QuiverPlot::auto_scale_fraction`] of the shorter tail span.
    ///   This is the default; it's what makes a zero-config quiver plot
    ///   look sensible regardless of the units of `(u, v)`.
    pub scale: Option<f64>,
    /// Fraction of the shorter tail-span used for the longest arrow when
    /// `scale` is `None`. Default `0.85`.
    pub auto_scale_fraction: f64,
    /// Shaft stroke width in pixels. Default `1.2`.
    pub shaft_width: f64,
    /// Explicit head length in pixels. `None` → proportional to shaft length
    /// (see [`QuiverPlot::head_ratio`]). Set via [`QuiverPlot::with_head`].
    pub head_length: Option<f64>,
    /// Explicit head half-width in pixels. `None` → `head_aspect * head_length`.
    pub head_width: Option<f64>,
    /// Fraction of shaft length used for the arrow head when `head_length`
    /// is `None`. Default `0.28`.
    pub head_ratio: f64,
    /// Head half-width as a fraction of head length when `head_width` is
    /// `None`. Default `0.45`.
    pub head_aspect: f64,
    /// Minimum head length in pixels (prevents tiny arrows from losing their head).
    /// Only applies when `head_length` is `None`. Default `4.0`.
    pub head_min_px: f64,
    /// Maximum head length in pixels (prevents long arrows from growing gigantic heads).
    /// Only applies when `head_length` is `None`. Default `14.0`.
    pub head_max_px: f64,
    /// Optional colormap applied to arrow magnitude. When set, overrides
    /// [`QuiverPlot::color`] for arrows without a per-arrow override.
    pub color_map: Option<ColorMap>,
    /// Optional explicit magnitude range `(min, max)` for colormap normalization.
    /// When `None`, derived from the data.
    pub color_range: Option<(f64, f64)>,
    /// Label shown next to the colorbar (when a colormap is active).
    pub color_legend_label: Option<String>,
    pub legend_label: Option<String>,
    /// When `true`, axis bounds come from arrow *tails* only — arrows may
    /// extend past the plot box. Default `false` (bounds include arrow tips,
    /// so nothing clips). Tight bounds produce a denser-looking field that
    /// better fills the plot area.
    pub tight_bounds: bool,
    /// Where `(x, y)` sits relative to the rendered arrow. Default `Tail`.
    pub pivot: QuiverPivot,
}

impl Default for QuiverPlot {
    fn default() -> Self { Self::new() }
}

impl QuiverPlot {
    /// Sample a vector-field closure on a regular `nx × ny` grid.
    ///
    /// `x_range` and `y_range` are `(min, max, n)` tuples where `n` is the
    /// number of samples along that axis (endpoints inclusive, matching
    /// `numpy.linspace`). The closure receives `(x, y)` and returns `(u, v)`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kuva::plot::QuiverPlot;
    /// let plot = QuiverPlot::from_function(
    ///     (-2.0, 2.0, 10),
    ///     (-2.0, 2.0, 10),
    ///     |x, y| (-y, x),   // rotational field
    /// );
    /// assert_eq!(plot.arrows.len(), 100);
    /// ```
    pub fn from_function<F>(
        x_range: (f64, f64, usize),
        y_range: (f64, f64, usize),
        mut f: F,
    ) -> Self
    where
        F: FnMut(f64, f64) -> (f64, f64),
    {
        let (x_min, x_max, nx) = x_range;
        let (y_min, y_max, ny) = y_range;
        let mut plot = Self::new();
        plot.arrows.reserve(nx.saturating_mul(ny));
        let step = |lo: f64, hi: f64, n: usize, i: usize| -> f64 {
            if n <= 1 { (lo + hi) * 0.5 }
            else { lo + (hi - lo) * (i as f64) / (n - 1) as f64 }
        };
        for i in 0..nx {
            let x = step(x_min, x_max, nx, i);
            for j in 0..ny {
                let y = step(y_min, y_max, ny, j);
                let (u, v) = f(x, y);
                plot.arrows.push(QuiverArrow { x, y, u, v, color: None });
            }
        }
        plot
    }

    /// Create an empty quiver plot with default styling.
    pub fn new() -> Self {
        Self {
            arrows: vec![],
            color: "steelblue".into(),
            scale: None,
            auto_scale_fraction: 0.85,
            shaft_width: 1.2,
            head_length: None,
            head_width: None,
            head_ratio: 0.28,
            head_aspect: 0.45,
            head_min_px: 4.0,
            head_max_px: 14.0,
            color_map: None,
            color_range: None,
            color_legend_label: None,
            legend_label: None,
            tight_bounds: false,
            pivot: QuiverPivot::Tail,
        }
    }

    /// Add one arrow at `(x, y)` with vector `(u, v)`.
    pub fn with_arrow(mut self, x: f64, y: f64, u: f64, v: f64) -> Self {
        self.arrows.push(QuiverArrow { x, y, u, v, color: None });
        self
    }

    /// Add many arrows from an iterator of `(x, y, u, v)` tuples.
    pub fn with_arrows(mut self, arrows: impl IntoIterator<Item = (f64, f64, f64, f64)>) -> Self {
        for (x, y, u, v) in arrows {
            self.arrows.push(QuiverArrow { x, y, u, v, color: None });
        }
        self
    }

    /// Add an arrow with a per-arrow color override.
    pub fn with_colored_arrow(
        mut self,
        x: f64, y: f64, u: f64, v: f64,
        color: impl Into<String>,
    ) -> Self {
        self.arrows.push(QuiverArrow { x, y, u, v, color: Some(color.into()) });
        self
    }

    /// Set the default arrow color.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Pin the scale multiplier to an explicit value. Overrides the default
    /// auto-scaling behavior.
    pub fn with_scale(mut self, s: f64) -> Self {
        self.scale = Some(s);
        self
    }

    /// Override the fraction of the shorter tail-span used when auto-scaling
    /// (i.e. when [`QuiverPlot::with_scale`] hasn't been called). Default `0.85`.
    ///
    /// Values closer to `1.0` produce longer arrows that fill the plot; smaller
    /// values leave more breathing room between arrows.
    pub fn with_auto_scale(mut self, fraction: f64) -> Self {
        self.scale = None;
        self.auto_scale_fraction = fraction;
        self
    }

    /// Resolve the scale multiplier, auto-computing when `scale` is `None`.
    pub fn effective_scale(&self) -> f64 {
        if let Some(s) = self.scale { return s; }
        if self.arrows.len() < 2 { return 1.0; }
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        let mut max_mag = 0.0_f64;
        for a in &self.arrows {
            x_min = x_min.min(a.x);
            x_max = x_max.max(a.x);
            y_min = y_min.min(a.y);
            y_max = y_max.max(a.y);
            max_mag = max_mag.max(a.magnitude());
        }
        // Use the smaller non-zero span; fall back to the other axis when
        // arrows are all collinear on one axis (span = 0 there).
        let x_span = x_max - x_min;
        let y_span = y_max - y_min;
        let span = match (x_span > 0.0, y_span > 0.0) {
            (true, true)  => x_span.min(y_span),
            (true, false) => x_span,
            (false, true) => y_span,
            (false, false) => 0.0,
        };
        if max_mag > 0.0 && span.is_finite() && span > 0.0 {
            self.auto_scale_fraction * span / max_mag
        } else {
            1.0
        }
    }

    /// Set the shaft stroke width in pixels. Default `1.2`.
    pub fn with_shaft_width(mut self, w: f64) -> Self {
        self.shaft_width = w;
        self
    }

    /// Pin arrow head dimensions to explicit pixel values. `length` is along
    /// the shaft, `half_width` is perpendicular to it. Overrides the default
    /// proportional sizing. Heads are still capped at the shaft length.
    pub fn with_head(mut self, length: f64, half_width: f64) -> Self {
        self.head_length = Some(length);
        self.head_width = Some(half_width);
        self
    }

    /// Pin just the head length in pixels. `with_head_width` remains
    /// independently overridable; unset dimensions fall back to the
    /// proportional default.
    pub fn with_head_length(mut self, length: f64) -> Self {
        self.head_length = Some(length);
        self
    }

    /// Pin just the head half-width in pixels.
    pub fn with_head_width(mut self, half_width: f64) -> Self {
        self.head_width = Some(half_width);
        self
    }

    /// Set the head length as a fraction of the shaft length (used when no
    /// explicit `with_head` is set). Default `0.28`.
    pub fn with_head_ratio(mut self, ratio: f64) -> Self {
        self.head_ratio = ratio;
        self
    }

    /// Resolve `(head_length, half_width)` in pixels for a shaft of length
    /// `shaft_px`. Honors explicit pixel overrides, else falls back to
    /// proportional sizing clamped by `head_min_px` / `head_max_px`.
    pub fn resolve_head(&self, shaft_px: f64) -> (f64, f64) {
        let length = match self.head_length {
            Some(px) => px.min(shaft_px),
            None => {
                let target = shaft_px * self.head_ratio;
                let lo = self.head_min_px.min(shaft_px);
                let hi = self.head_max_px.min(shaft_px);
                target.clamp(lo, hi)
            }
        };
        let half_w = match self.head_width {
            Some(px) => px,
            None => length * self.head_aspect,
        };
        (length, half_w)
    }

    /// Color arrows by magnitude using the given colormap. Overrides
    /// [`QuiverPlot::color`] for arrows without a per-arrow override.
    pub fn with_color_map(mut self, cmap: ColorMap) -> Self {
        self.color_map = Some(cmap);
        self
    }

    /// Override the magnitude range used for colormap normalization.
    /// Default: derived from the data.
    pub fn with_color_range(mut self, lo: f64, hi: f64) -> Self {
        self.color_range = Some((lo, hi));
        self
    }

    /// Label rendered next to the colorbar (when a colormap is active).
    pub fn with_color_legend_label(mut self, label: impl Into<String>) -> Self {
        self.color_legend_label = Some(label.into());
        self
    }

    /// Shorthand for the common "color by magnitude with a labeled colorbar"
    /// pattern. Equivalent to
    /// `self.with_color_map(cmap).with_color_legend_label(label)`.
    pub fn with_magnitude_colormap(
        mut self,
        cmap: ColorMap,
        label: impl Into<String>,
    ) -> Self {
        self.color_map = Some(cmap);
        self.color_legend_label = Some(label.into());
        self
    }

    /// Attach a legend entry for this series.
    pub fn with_legend(mut self, label: impl Into<String>) -> Self {
        self.legend_label = Some(label.into());
        self
    }

    /// Derive axis bounds from arrow tails only. Arrows may extend past the
    /// plot box. Useful for dense grids where tip-inclusive bounds produce
    /// too much whitespace around the field.
    pub fn with_tight_bounds(mut self) -> Self {
        self.tight_bounds = true;
        self
    }

    /// Where `(x, y)` sits relative to the rendered arrow. Default `Tail`.
    pub fn with_pivot(mut self, pivot: QuiverPivot) -> Self {
        self.pivot = pivot;
        self
    }

    /// Resolve an arrow's tail and tip endpoints in data coordinates,
    /// given a precomputed scale multiplier.
    pub fn endpoints_with_scale(
        &self,
        arrow: &QuiverArrow,
        scale: f64,
    ) -> ((f64, f64), (f64, f64)) {
        let du = arrow.u * scale;
        let dv = arrow.v * scale;
        let (sx, sy) = match self.pivot {
            QuiverPivot::Tail   => (0.0, 0.0),
            QuiverPivot::Middle => (-du * 0.5, -dv * 0.5),
            QuiverPivot::Tip    => (-du, -dv),
        };
        let tail = (arrow.x + sx, arrow.y + sy);
        let tip  = (tail.0 + du, tail.1 + dv);
        (tail, tip)
    }

    /// Min and max arrow magnitudes in the current data.
    pub fn magnitude_extent(&self) -> (f64, f64) {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for a in &self.arrows {
            let m = a.magnitude();
            lo = lo.min(m);
            hi = hi.max(m);
        }
        if !lo.is_finite() { return (0.0, 0.0); }
        (lo, hi)
    }
}
