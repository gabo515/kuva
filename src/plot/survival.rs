/// One group of survival observations for a Kaplan-Meier curve.
///
/// Each subject contributes one `time` (time to event or last follow-up) and
/// one `event` flag (`true` = event occurred, `false` = censored/right-truncated).
pub struct KMGroup {
    pub label: String,
    pub times: Vec<f64>,
    pub events: Vec<bool>,
    /// Per-group color override. `None` falls back to the plot palette.
    pub color: Option<String>,
}

/// Builder for a Kaplan-Meier survival plot.
///
/// Each group produces a step-function survival curve, optional confidence
/// bands (Greenwood's formula, linear scale), and optional censoring tick
/// marks. When multiple groups are present a log-rank p-value can be
/// annotated via [`with_pvalue_text`](Self::with_pvalue_text).
///
/// # Example
///
/// ```rust,no_run
/// use kuva::prelude::*;
///
/// let plot = SurvivalPlot::new()
///     .with_group("Treatment A", vec![5.0,8.0,12.0,15.0,20.0], vec![true,true,false,true,false])
///     .with_group("Treatment B", vec![3.0,6.0,9.0,14.0,18.0], vec![true,false,true,true,false])
///     .with_ci(true)
///     .with_legend("Group");
///
/// let plots = vec![Plot::from(plot)];
/// let layout = Layout::auto_from_plots(&plots)
///     .with_title("Overall Survival")
///     .with_x_label("Time (months)")
///     .with_y_label("Survival probability");
/// ```
pub struct SurvivalPlot {
    pub groups: Vec<KMGroup>,
    /// Fallback line color when no per-group colors are set and no palette is active.
    pub color: String,
    /// Per-group color overrides (indexed by group order).
    pub group_colors: Option<Vec<String>>,
    /// Line stroke width in pixels. Default `2.0`.
    pub line_width: f64,
    /// Draw Greenwood 95% CI bands. Default `false`.
    pub show_ci: bool,
    /// Opacity of CI bands. Default `0.2`.
    pub ci_alpha: f64,
    /// Draw censoring tick marks on the curves. Default `true`.
    pub show_censoring: bool,
    /// Half-height of censoring ticks in pixels. Default `4.0`.
    pub censoring_size: f64,
    /// Optional p-value / annotation text rendered in the upper-right of the plot area.
    pub pvalue_text: Option<String>,
    pub legend_label: Option<String>,
}

impl Default for SurvivalPlot {
    fn default() -> Self { Self::new() }
}

impl SurvivalPlot {
    /// Create a survival plot with default settings.
    pub fn new() -> Self {
        Self {
            groups: vec![],
            color: "steelblue".into(),
            group_colors: None,
            line_width: 2.0,
            show_ci: false,
            ci_alpha: 0.2,
            show_censoring: true,
            censoring_size: 4.0,
            pvalue_text: None,
            legend_label: None,
        }
    }

    /// Add a group with separate time and event vectors.
    ///
    /// `times`: time to event or censoring. `events`: `true` = event occurred.
    pub fn with_group(
        mut self,
        label: impl Into<String>,
        times: Vec<f64>,
        events: Vec<bool>,
    ) -> Self {
        self.groups.push(KMGroup { label: label.into(), times, events, color: None });
        self
    }

    /// Add a group with a per-group color override.
    pub fn with_colored_group(
        mut self,
        label: impl Into<String>,
        times: Vec<f64>,
        events: Vec<bool>,
        color: impl Into<String>,
    ) -> Self {
        self.groups.push(KMGroup { label: label.into(), times, events, color: Some(color.into()) });
        self
    }

    /// Set the fallback line color. Default `"steelblue"`.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Set per-group colors (indexed by group order). Falls back to category10.
    pub fn with_group_colors(mut self, colors: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.group_colors = Some(colors.into_iter().map(|c| c.into()).collect());
        self
    }

    /// Set line stroke width in pixels. Default `2.0`.
    pub fn with_line_width(mut self, w: f64) -> Self {
        self.line_width = w;
        self
    }

    /// Show 95% confidence bands (Greenwood's formula). Default `false`.
    pub fn with_ci(mut self, show: bool) -> Self {
        self.show_ci = show;
        self
    }

    /// Set confidence band opacity. Default `0.2`.
    pub fn with_ci_alpha(mut self, alpha: f64) -> Self {
        self.ci_alpha = alpha;
        self
    }

    /// Show censoring tick marks on curves. Default `true`.
    pub fn with_censoring(mut self, show: bool) -> Self {
        self.show_censoring = show;
        self
    }

    /// Set the half-height of censoring tick marks in pixels. Default `4.0`.
    pub fn with_censoring_size(mut self, size: f64) -> Self {
        self.censoring_size = size;
        self
    }

    /// Add a p-value or annotation string rendered in the upper-right corner.
    ///
    /// Typical use: `with_pvalue_text("p = 0.023")` or `"log-rank p < 0.001"`.
    pub fn with_pvalue_text(mut self, text: impl Into<String>) -> Self {
        self.pvalue_text = Some(text.into());
        self
    }

    /// Attach a legend to this plot (shows one entry per group).
    pub fn with_legend(mut self, label: impl Into<String>) -> Self {
        self.legend_label = Some(label.into());
        self
    }
}

// ── Internal KM computation (pub(crate) for use by the renderer) ──────────────

/// One KM step: (time, survival, ci_lo, ci_hi).
pub(crate) struct KMPoint {
    pub t: f64,
    pub s: f64,
    pub lo: f64,
    pub hi: f64,
}

/// Compute the Kaplan-Meier curve with Greenwood 95% CI for one group.
///
/// Returns a sorted list of `KMPoint` starting at `(0, 1, 1, 1)`.
pub(crate) fn km_curve(times: &[f64], events: &[bool]) -> Vec<KMPoint> {
    let mut result = vec![KMPoint { t: 0.0, s: 1.0, lo: 1.0, hi: 1.0 }];
    if times.is_empty() { return result; }

    let mut pairs: Vec<(f64, bool)> = times.iter().zip(events).map(|(&t, &e)| (t, e)).collect();
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let n_total = pairs.len();
    let mut survival = 1.0_f64;
    let mut greenwood = 0.0_f64;
    let mut at_risk = n_total;
    let mut i = 0;

    while i < n_total {
        let t = pairs[i].0;
        let mut j = i;
        while j < n_total && pairs[j].0 == t { j += 1; }

        let n_events = pairs[i..j].iter().filter(|&&(_, e)| e).count();

        if n_events > 0 {
            let ni = at_risk;
            survival *= 1.0 - n_events as f64 / ni as f64;
            let denom = ni * (ni - n_events);
            if denom > 0 {
                greenwood += n_events as f64 / denom as f64;
            }
            let se = (survival * survival * greenwood).sqrt();
            result.push(KMPoint {
                t,
                s: survival,
                lo: (survival - 1.96 * se).max(0.0),
                hi: (survival + 1.96 * se).min(1.0),
            });
        }

        at_risk -= j - i;
        i = j;
    }

    result
}

/// Return (time, survival_at_t) for each censored observation, using the
/// last KM step at or before `t_censor`.
pub(crate) fn censoring_levels(
    times: &[f64],
    events: &[bool],
    km: &[KMPoint],
) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for (&t, &ev) in times.iter().zip(events) {
        if ev { continue; }
        // Find survival at t: last KM point with km.t <= t
        let s = km.iter().rev().find(|p| p.t <= t).map_or(1.0, |p| p.s);
        out.push((t, s));
    }
    out
}
