/// Determines the direction cells are filled across the grid.
///
/// The default is [`RowMajorTopLeft`](FillOrder::RowMajorTopLeft), which fills
/// left-to-right, top-to-bottom (reading order).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillOrder {
    /// Left-to-right, top-to-bottom (reading order). Default.
    RowMajorTopLeft,
    /// Left-to-right, bottom-to-top (filling-up, like a progress bar).
    RowMajorBottomLeft,
    /// Top-to-bottom, left-to-right (column-first).
    ColMajorTopLeft,
    /// Bottom-to-top, left-to-right.
    ColMajorBottomLeft,
}

/// Shape used to render each cell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellShape {
    /// Filled square (default).
    Square,
    /// Filled circle inscribed in the cell.
    Circle,
}

/// A single category in a [`WafflePlot`].
#[derive(Debug, Clone)]
pub struct WaffleCategory {
    pub label: String,
    pub value: f64,
    /// Fill color as a CSS color string (e.g. `"steelblue"`, `"#4682b4"`).
    pub color: String,
}

/// Builder for a waffle chart.
///
/// Proportions are encoded as colored cells in a rectangular grid.
/// The Largest Remainder (Hamilton) method guarantees the filled cell count
/// always equals exactly `rows × cols`, regardless of rounding.
///
/// # Example
///
/// ```rust,no_run
/// use kuva::plot::{WafflePlot, FillOrder, CellShape};
/// use kuva::render::plots::Plot;
/// use kuva::render::layout::Layout;
/// use kuva::render::render::render_multiple;
/// use kuva::backend::svg::SvgBackend;
///
/// let waffle = WafflePlot::new()
///     .with_category("Treated",   45.0, "steelblue")
///     .with_category("Partial",   30.0, "gold")
///     .with_category("Untreated", 25.0, "#e74c3c")
///     .with_legend("Status")
///     .with_show_percents();
///
/// let plots = vec![Plot::Waffle(waffle)];
/// let layout = Layout::auto_from_plots(&plots).with_title("Treatment coverage");
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// std::fs::write("waffle.svg", svg).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct WafflePlot {
    pub categories: Vec<WaffleCategory>,
    /// Number of rows in the grid. Default: 10.
    pub rows: usize,
    /// Number of columns in the grid. Default: 10.
    pub cols: usize,
    /// Gap between cells as a fraction of cell size. `0.0` = no gap; `0.2`
    /// gives visible gutters. Default: `0.1`.
    pub gap: f64,
    /// Fill direction. Default: [`FillOrder::RowMajorTopLeft`].
    pub fill_order: FillOrder,
    /// Cell shape. Default: [`CellShape::Square`].
    pub shape: CellShape,
    /// Fill color for unused / background cells. Default: `"#e8e8e8"`.
    pub empty_color: String,
    /// When `Some`, a legend is rendered. The string is not currently used as a
    /// legend title but triggers legend entry generation in
    /// [`collect_legend_entries`](crate::render::render::collect_legend_entries).
    pub legend_label: Option<String>,
    /// When `true`, percentage of total is appended to each legend label.
    pub show_percents: bool,
    /// When `true`, cell count is appended to each legend label.
    pub show_counts: bool,
    /// Optional annotation displayed below the grid, e.g. `"■ = 1,000 people"`.
    pub unit_label: Option<String>,
}

impl Default for WafflePlot {
    fn default() -> Self { Self::new() }
}

impl WafflePlot {
    /// Create a waffle chart with default settings (10×10 grid, square cells,
    /// row-major top-left fill, 10% gap, light-grey background cells).
    pub fn new() -> Self {
        Self {
            categories: vec![],
            rows: 10,
            cols: 10,
            gap: 0.1,
            fill_order: FillOrder::RowMajorTopLeft,
            shape: CellShape::Square,
            empty_color: "#e8e8e8".into(),
            legend_label: None,
            show_percents: false,
            show_counts: false,
            unit_label: None,
        }
    }

    /// Add a category with a label, proportional value, and fill color.
    ///
    /// Values are proportional — only their relative magnitudes matter.
    /// Cells are assigned using Largest Remainder rounding so the total
    /// always equals exactly `rows × cols`.
    ///
    /// ```rust,no_run
    /// # use kuva::plot::WafflePlot;
    /// let w = WafflePlot::new()
    ///     .with_category("Yes", 60.0, "steelblue")
    ///     .with_category("No",  40.0, "tomato");
    /// ```
    pub fn with_category<L, V, C>(mut self, label: L, value: V, color: C) -> Self
    where
        L: Into<String>,
        V: Into<f64>,
        C: Into<String>,
    {
        self.categories.push(WaffleCategory {
            label: label.into(),
            value: value.into(),
            color: color.into(),
        });
        self
    }

    /// Add multiple categories at once.
    ///
    /// Each item is `(label, value, color)`.
    ///
    /// ```rust,no_run
    /// # use kuva::plot::WafflePlot;
    /// let w = WafflePlot::new()
    ///     .with_categories([
    ///         ("A", 50.0, "steelblue"),
    ///         ("B", 30.0, "gold"),
    ///         ("C", 20.0, "tomato"),
    ///     ]);
    /// ```
    pub fn with_categories<L, V, C>(
        mut self,
        items: impl IntoIterator<Item = (L, V, C)>,
    ) -> Self
    where
        L: Into<String>,
        V: Into<f64>,
        C: Into<String>,
    {
        for (l, v, c) in items {
            self.categories.push(WaffleCategory {
                label: l.into(),
                value: v.into(),
                color: c.into(),
            });
        }
        self
    }

    /// Set the grid dimensions.
    ///
    /// A 10×10 grid cleanly represents percentages (1 cell per percent).
    /// A 5×20 or 4×25 grid gives more horizontal space for wide canvases.
    ///
    /// ```rust,no_run
    /// # use kuva::plot::WafflePlot;
    /// // 5 rows × 20 cols = 100 cells, wide aspect ratio
    /// let w = WafflePlot::new().with_grid(5, 20);
    /// ```
    pub fn with_grid(mut self, rows: usize, cols: usize) -> Self {
        self.rows = rows.max(1);
        self.cols = cols.max(1);
        self
    }

    /// Set the number of rows. See also [`with_grid`](Self::with_grid).
    pub fn with_rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }

    /// Set the number of columns. See also [`with_grid`](Self::with_grid).
    pub fn with_cols(mut self, cols: usize) -> Self {
        self.cols = cols.max(1);
        self
    }

    /// Set the gap between cells as a fraction of cell size.
    ///
    /// `0.0` produces a solid grid with no gutters; `0.2` gives clearly
    /// separated cells. Default: `0.1`.
    pub fn with_gap(mut self, gap: f64) -> Self {
        self.gap = gap.clamp(0.0, 0.5);
        self
    }

    /// Set the fill direction.
    ///
    /// Controls where cell 0 is placed and which direction subsequent cells
    /// are assigned. Default: [`FillOrder::RowMajorTopLeft`] (reading order).
    pub fn with_fill_order(mut self, order: FillOrder) -> Self {
        self.fill_order = order;
        self
    }

    /// Set the cell shape.
    ///
    /// Default: [`CellShape::Square`]. Use [`CellShape::Circle`] for a more
    /// bubbly, infographic aesthetic.
    pub fn with_shape(mut self, shape: CellShape) -> Self {
        self.shape = shape;
        self
    }

    /// Set the fill color for background (unfilled) cells.
    ///
    /// Default: `"#e8e8e8"`. Set to the canvas background color (`"white"` or
    /// `"none"`) to hide empty cells entirely.
    pub fn with_empty_color<C: Into<String>>(mut self, color: C) -> Self {
        self.empty_color = color.into();
        self
    }

    /// Attach a legend.
    ///
    /// When set, [`render_multiple`](crate::render::render::render_multiple)
    /// adds a colored-square legend entry for each category. Use
    /// [`with_show_percents`](Self::with_show_percents) or
    /// [`with_show_counts`](Self::with_show_counts) to annotate entries.
    pub fn with_legend<S: Into<String>>(mut self, label: S) -> Self {
        self.legend_label = Some(label.into());
        self
    }

    /// Append the percentage of total to each legend label.
    ///
    /// Example: `"Treated (45.0%)"`. Requires [`with_legend`](Self::with_legend).
    pub fn with_show_percents(mut self) -> Self {
        self.show_percents = true;
        self
    }

    /// Append the cell count to each legend label.
    ///
    /// Example: `"Treated (45 cells)"`. Requires [`with_legend`](Self::with_legend).
    pub fn with_show_counts(mut self) -> Self {
        self.show_counts = true;
        self
    }

    /// Add a unit annotation displayed below the grid.
    ///
    /// Useful for absolute-count data where each cell represents a fixed
    /// quantity, e.g. `"■ = 100 people"`.
    ///
    /// ```rust,no_run
    /// # use kuva::plot::WafflePlot;
    /// let w = WafflePlot::new()
    ///     .with_category("Positive", 2700.0, "steelblue")
    ///     .with_category("Negative", 7300.0, "#e8e8e8")
    ///     .with_unit_label("■ = 100 people");
    /// ```
    pub fn with_unit_label<S: Into<String>>(mut self, label: S) -> Self {
        self.unit_label = Some(label.into());
        self
    }

    /// Total number of cells in the grid (`rows × cols`).
    pub fn total_cells(&self) -> usize {
        self.rows * self.cols
    }
}
