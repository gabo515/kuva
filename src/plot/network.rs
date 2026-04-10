/// A network / graph diagram: nodes connected by edges, laid out with
/// force-directed (Fruchterman–Reingold) or circular placement.
///
/// Supports both edge-list and adjacency-matrix input. Edges can be directed
/// (arrowheads) or undirected (plain lines). Self-loops are rendered as small
/// arcs. Edge weight controls stroke width and opacity.
///
/// # Pixel-space rendering
///
/// Like chord and sankey diagrams, the network plot is rendered entirely in
/// pixel space — it does not use the standard x/y axis system. A title set on
/// the `Layout` is still rendered.
///
/// # Example
///
/// ```rust,no_run
/// use kuva::plot::NetworkPlot;
/// use kuva::backend::svg::SvgBackend;
/// use kuva::render::render::render_multiple;
/// use kuva::render::layout::Layout;
/// use kuva::render::plots::Plot;
///
/// let net = NetworkPlot::new()
///     .with_edge("A", "B", 1.0)
///     .with_edge("A", "C", 2.0)
///     .with_edge("B", "C", 1.5)
///     .with_labels();
///
/// let plots = vec![Plot::Network(net)];
/// let layout = Layout::auto_from_plots(&plots)
///     .with_title("My Network");
///
/// let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
/// std::fs::write("network.svg", svg).unwrap();
/// ```

/// Layout algorithm for node placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkLayout {
    /// Fruchterman–Reingold force-directed layout (default).
    ForceDirected,
    /// Nodes evenly spaced on a circle.
    Circle,
}

/// A node in the network graph.
#[derive(Debug, Clone)]
pub struct NetworkNode {
    pub label: String,
    pub color: Option<String>,
    pub size: Option<f64>,
    pub group: Option<String>,
    /// Fixed position in normalised \[0, 1\] space. When `Some`, the layout
    /// algorithm will not move this node.
    pub position: Option<(f64, f64)>,
}

/// An edge connecting two nodes.
#[derive(Debug, Clone)]
pub struct NetworkEdge {
    /// Index into `NetworkPlot::nodes`.
    pub source: usize,
    /// Index into `NetworkPlot::nodes`.
    pub target: usize,
    pub weight: f64,
    pub color: Option<String>,
}

/// A network / graph diagram.
#[derive(Debug, Clone)]
pub struct NetworkPlot {
    pub nodes: Vec<NetworkNode>,
    pub edges: Vec<NetworkEdge>,
    /// Draw arrowheads on edges (default `false`).
    pub directed: bool,
    /// Node placement algorithm (default [`NetworkLayout::ForceDirected`]).
    pub layout: NetworkLayout,
    /// Base node radius in pixels (default 8.0).
    pub node_radius: f64,
    /// Edge stroke opacity 0.0–1.0 (default 0.6).
    pub edge_opacity: f64,
    /// Render node labels (default `false`).
    pub show_labels: bool,
    /// If set, generate a legend with one entry per unique group.
    pub legend_label: Option<String>,
    /// Override label font size (pixels).
    pub label_size: Option<u32>,
    /// Deferred adjacency matrix (expanded into edges by `resolve_matrix`).
    pending_matrix: Option<(Vec<Vec<f64>>, Vec<usize>)>,
    /// O(1) label→index lookup, kept in sync with `nodes`.
    node_map: std::collections::HashMap<String, usize>,
}

impl Default for NetworkPlot {
    fn default() -> Self { Self::new() }
}

impl NetworkPlot {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            directed: false,
            layout: NetworkLayout::ForceDirected,
            node_radius: 8.0,
            edge_opacity: 0.6,
            show_labels: false,
            legend_label: None,
            label_size: None,
            pending_matrix: None,
            node_map: std::collections::HashMap::new(),
        }
    }

    /// Find an existing node by label, or insert a new one and return its index.
    fn node_index(&mut self, label: &str) -> usize {
        if let Some(&idx) = self.node_map.get(label) {
            return idx;
        }
        let idx = self.nodes.len();
        self.nodes.push(NetworkNode {
            label: label.to_string(),
            color: None,
            size: None,
            group: None,
            position: None,
        });
        self.node_map.insert(label.to_string(), idx);
        idx
    }

    /// Add an edge, auto-creating source and target nodes by label if needed.
    pub fn with_edge<S: Into<String>>(mut self, source: S, target: S, weight: f64) -> Self {
        let src = source.into();
        let tgt = target.into();
        let si = self.node_index(&src);
        let ti = self.node_index(&tgt);
        self.edges.push(NetworkEdge { source: si, target: ti, weight, color: None });
        self
    }

    /// Bulk-add edges from an iterator of `(source, target, weight)`.
    pub fn with_edges<S, I>(mut self, edges: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = (S, S, f64)>,
    {
        for (src, tgt, w) in edges {
            self = self.with_edge(src, tgt, w);
        }
        self
    }

    /// Build a network from an N×N adjacency matrix.
    ///
    /// Non-zero entries become edges; the value is used as the weight.
    /// The matrix is stored and edges are expanded when needed (by
    /// [`compute_positions`] or [`resolve_matrix`]), so `.with_directed()`
    /// can be called before or after this method.
    pub fn with_matrix<S, L>(mut self, matrix: Vec<Vec<f64>>, labels: L) -> Self
    where
        S: Into<String>,
        L: IntoIterator<Item = S>,
    {
        let labels: Vec<String> = labels.into_iter().map(Into::into).collect();
        let indices: Vec<usize> = labels.iter().map(|l| self.node_index(l)).collect();
        self.pending_matrix = Some((matrix, indices));
        self
    }

    /// Expand a pending adjacency matrix into edges.  Called automatically
    /// by [`compute_positions`]; safe to call multiple times (no-op after
    /// the first).
    pub fn resolve_matrix(&mut self) {
        if let Some((matrix, indices)) = self.pending_matrix.take() {
            let n = indices.len();
            for i in 0..n {
                let j_start = if self.directed { 0 } else { i + 1 };
                for j in j_start..n {
                    if j >= matrix[i].len() { continue; }
                    let w = matrix[i][j];
                    if w.abs() < f64::EPSILON { continue; }
                    self.edges.push(NetworkEdge {
                        source: indices[i], target: indices[j], weight: w, color: None,
                    });
                }
                // Self-loops from diagonal (only when directed).
                if self.directed && i < matrix[i].len() {
                    let w = matrix[i][i];
                    if w.abs() >= f64::EPSILON {
                        self.edges.push(NetworkEdge {
                            source: indices[i], target: indices[i], weight: w, color: None,
                        });
                    }
                }
            }
        }
    }

    /// Declare a node explicitly (no-op if it already exists).
    pub fn with_node<S: Into<String>>(mut self, label: S) -> Self {
        let label = label.into();
        self.node_index(&label);
        self
    }

    /// Set the colour for a node, creating it if absent.
    pub fn with_node_color<S: Into<String>, C: Into<String>>(mut self, label: S, color: C) -> Self {
        let label = label.into();
        let idx = self.node_index(&label);
        self.nodes[idx].color = Some(color.into());
        self
    }

    /// Set the size for a node, creating it if absent.
    pub fn with_node_size<S: Into<String>>(mut self, label: S, size: f64) -> Self {
        let label = label.into();
        let idx = self.node_index(&label);
        self.nodes[idx].size = Some(size);
        self
    }

    /// Set the group for a node, creating it if absent.
    pub fn with_node_group<S: Into<String>, G: Into<String>>(mut self, label: S, group: G) -> Self {
        let label = label.into();
        let idx = self.node_index(&label);
        self.nodes[idx].group = Some(group.into());
        self
    }

    /// Fix a node's position in normalised \[0, 1\] space.
    pub fn with_node_position<S: Into<String>>(mut self, label: S, x: f64, y: f64) -> Self {
        let label = label.into();
        let idx = self.node_index(&label);
        self.nodes[idx].position = Some((x, y));
        self
    }

    /// Draw directed edges with arrowheads.
    pub fn with_directed(mut self) -> Self {
        self.directed = true;
        self
    }

    /// Set the layout algorithm.
    pub fn with_layout(mut self, layout: NetworkLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set the base node radius in pixels.
    pub fn with_node_radius(mut self, r: f64) -> Self {
        self.node_radius = r;
        self
    }

    /// Set edge stroke opacity (0.0–1.0).
    pub fn with_edge_opacity(mut self, opacity: f64) -> Self {
        self.edge_opacity = opacity;
        self
    }

    /// Show node labels beside each node.
    pub fn with_labels(mut self) -> Self {
        self.show_labels = true;
        self
    }

    /// Show a legend; one entry per unique group.
    pub fn with_legend<S: Into<String>>(mut self, label: S) -> Self {
        self.legend_label = Some(label.into());
        self
    }

    /// Override the label font size.
    pub fn with_label_size(mut self, size: u32) -> Self {
        self.label_size = Some(size);
        self
    }

    // ── Layout algorithms ─────────────────────────────────────────────

    /// Compute node positions in \[0, 1\] × \[0, 1\] space.
    ///
    /// Call [`resolve_matrix`] first if a matrix was provided via
    /// [`with_matrix`], or this will only see edges added via
    /// [`with_edge`]/[`with_edges`].
    pub fn compute_positions(&self) -> Vec<(f64, f64)> {
        match self.layout {
            NetworkLayout::ForceDirected => self.fruchterman_reingold(),
            NetworkLayout::Circle => self.circle_layout(),
        }
    }

    fn circle_layout(&self) -> Vec<(f64, f64)> {
        let n = self.nodes.len();
        if n == 0 { return vec![]; }
        if n == 1 { return vec![(0.5, 0.5)]; }
        (0..n).map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64)
                - std::f64::consts::FRAC_PI_2; // start at top
            let x = 0.5 + 0.5 * angle.cos();
            let y = 0.5 + 0.5 * angle.sin();
            (x, y)
        }).collect()
    }

    fn fruchterman_reingold(&self) -> Vec<(f64, f64)> {
        let n = self.nodes.len();
        if n == 0 { return vec![]; }
        if n == 1 { return vec![(0.5, 0.5)]; }

        let area = 1.0_f64;
        let k = (area / n as f64).sqrt();
        let iterations = 100;
        let mut temp = 0.1 * (n as f64).sqrt();
        let cooling = temp / iterations as f64;

        // Deterministic initial placement: grid with slight offset
        let cols = (n as f64).sqrt().ceil() as usize;
        let mut pos: Vec<(f64, f64)> = (0..n).map(|i| {
            let row = i / cols;
            let col = i % cols;
            let x = (col as f64 + 0.5) / cols as f64;
            let y = (row as f64 + 0.5) / cols as f64;
            // small perturbation to break symmetry
            let hash = ((i as u64).wrapping_mul(2654435761) & 0xFFFF) as f64 / 65536.0;
            (x + 0.01 * hash, y + 0.01 * (1.0 - hash))
        }).collect();

        // Honour user-supplied positions
        for (i, node) in self.nodes.iter().enumerate() {
            if let Some((px, py)) = node.position {
                pos[i] = (px, py);
            }
        }

        let fa = |d: f64| -> f64 { d * d / k };            // attractive
        let fr = |d: f64| -> f64 { k * k / (d + 1e-6) };   // repulsive

        for _ in 0..iterations {
            // Repulsive forces
            let mut disp = vec![(0.0_f64, 0.0_f64); n];
            for i in 0..n {
                for j in (i + 1)..n {
                    let dx = pos[i].0 - pos[j].0;
                    let dy = pos[i].1 - pos[j].1;
                    let dist = (dx * dx + dy * dy).sqrt().max(1e-6);
                    let force = fr(dist);
                    let fx = dx / dist * force;
                    let fy = dy / dist * force;
                    disp[i].0 += fx;
                    disp[i].1 += fy;
                    disp[j].0 -= fx;
                    disp[j].1 -= fy;
                }
            }

            // Gentle gravity toward centre — prevents disconnected components
            // from flying apart and compressing each other's internal layout.
            let gravity = 0.5;
            let cx = pos.iter().map(|p| p.0).sum::<f64>() / n as f64;
            let cy = pos.iter().map(|p| p.1).sum::<f64>() / n as f64;
            for i in 0..n {
                disp[i].0 -= gravity * (pos[i].0 - cx);
                disp[i].1 -= gravity * (pos[i].1 - cy);
            }

            // Attractive forces along edges
            for edge in &self.edges {
                let (si, ti) = (edge.source, edge.target);
                if si == ti { continue; } // skip self-loops
                let dx = pos[si].0 - pos[ti].0;
                let dy = pos[si].1 - pos[ti].1;
                let dist = (dx * dx + dy * dy).sqrt().max(1e-6);
                let force = fa(dist);
                let fx = dx / dist * force;
                let fy = dy / dist * force;
                disp[si].0 -= fx;
                disp[si].1 -= fy;
                disp[ti].0 += fx;
                disp[ti].1 += fy;
            }

            // Apply displacement capped by temperature
            for i in 0..n {
                if self.nodes[i].position.is_some() { continue; } // pinned
                let dx = disp[i].0;
                let dy = disp[i].1;
                let mag = (dx * dx + dy * dy).sqrt().max(1e-6);
                let cap = mag.min(temp);
                pos[i].0 += dx / mag * cap;
                pos[i].1 += dy / mag * cap;
            }

            temp -= cooling;
            if temp < 0.0 { temp = 0.0; }
        }

        // Normalise unpinned nodes to [0, 1].  Use a uniform scale
        // (max of xrange, yrange) so aspect ratio is preserved and gravity
        // controls the tightness of disconnected components.
        let free: Vec<usize> = (0..n)
            .filter(|&i| self.nodes[i].position.is_none())
            .collect();
        if !free.is_empty() {
            let (mut xmin, mut xmax) = (f64::INFINITY, f64::NEG_INFINITY);
            let (mut ymin, mut ymax) = (f64::INFINITY, f64::NEG_INFINITY);
            for &i in &free {
                xmin = xmin.min(pos[i].0); xmax = xmax.max(pos[i].0);
                ymin = ymin.min(pos[i].1); ymax = ymax.max(pos[i].1);
            }
            let xrange = xmax - xmin;
            let yrange = ymax - ymin;
            let scale = xrange.max(yrange).max(1e-6);
            // Centre in [0, 1] after uniform scaling.
            let x_offset = (1.0 - xrange / scale) / 2.0;
            let y_offset = (1.0 - yrange / scale) / 2.0;
            for &i in &free {
                pos[i].0 = (pos[i].0 - xmin) / scale + x_offset;
                pos[i].1 = (pos[i].1 - ymin) / scale + y_offset;
            }
        }

        pos
    }
}
