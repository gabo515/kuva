use clap::Args;

use kuva::plot::network::{NetworkPlot, NetworkLayout};
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::render::render::render_multiple;

use crate::data::{ColSpec, DataTable, InputArgs};
use crate::layout_args::{BaseArgs, apply_base_args};
use crate::output::write_output;

/// Network / graph diagram from an edge list or adjacency matrix.
#[derive(Args, Debug)]
pub struct NetworkArgs {
    /// Read input as an N×N adjacency matrix instead of an edge list.
    #[arg(long)]
    pub matrix: bool,

    /// Source node column (edge-list mode; 0-based index or header name; default: 0).
    #[arg(long)]
    pub source_col: Option<ColSpec>,

    /// Target node column (edge-list mode; 0-based index or header name; default: 1).
    #[arg(long)]
    pub target_col: Option<ColSpec>,

    /// Edge weight column (edge-list mode; 0-based index or header name; optional).
    #[arg(long)]
    pub weight_col: Option<ColSpec>,

    /// Node group column for colouring (edge-list mode; 0-based index or header name).
    #[arg(long)]
    pub group_col: Option<ColSpec>,

    /// Draw directed edges with arrowheads.
    #[arg(long)]
    pub directed: bool,

    /// Layout algorithm: "force" (default) or "circle".
    #[arg(long, default_value = "force")]
    pub layout: String,

    /// Node radius in pixels (default: 8.0).
    #[arg(long)]
    pub node_radius: Option<f64>,

    /// Edge opacity 0.0–1.0 (default: 0.6).
    #[arg(long)]
    pub opacity: Option<f64>,

    /// Show node labels.
    #[arg(long)]
    pub labels: bool,

    /// Show a legend with this label.
    #[arg(long)]
    pub legend: Option<String>,

    #[command(flatten)]
    pub input: InputArgs,

    #[command(flatten)]
    pub base: BaseArgs,
}

pub fn run(args: NetworkArgs) -> Result<(), String> {
    let table = DataTable::parse(
        args.input.input.as_deref(),
        args.input.no_header,
        args.input.delimiter,
    )?;

    let mut plot = NetworkPlot::new();

    if args.directed { plot = plot.with_directed(); }
    if args.labels { plot = plot.with_labels(); }

    match args.layout.as_str() {
        "force" => {}
        "circle" => { plot = plot.with_layout(NetworkLayout::Circle); }
        other => return Err(format!("unknown layout '{other}'; expected 'force' or 'circle'")),
    }

    if let Some(r) = args.node_radius { plot = plot.with_node_radius(r); }
    if let Some(op) = args.opacity { plot = plot.with_edge_opacity(op); }
    if let Some(ref l) = args.legend { plot = plot.with_legend(l.clone()); }

    if args.matrix {
        // ── Adjacency matrix mode ─────────────────────────────────────
        if table.rows.is_empty() {
            return Err("network matrix input has no data rows".into());
        }
        let labels: Vec<String> = if let Some(ref hdr) = table.header {
            hdr[1..].to_vec()
        } else {
            table.rows.iter().map(|r| r[0].clone()).collect()
        };

        let matrix: Vec<Vec<f64>> = table.rows.iter().enumerate().map(|(r, row)| {
            row[1..].iter().enumerate().map(|(c, cell)| {
                cell.trim().parse::<f64>().map_err(|_| {
                    format!("row {r}, col {}: '{}' is not a number", c + 1, cell)
                })
            }).collect::<Result<Vec<f64>, String>>()
        }).collect::<Result<Vec<_>, _>>()?;

        plot = plot.with_matrix(matrix, labels);
    } else {
        // ── Edge-list mode ────────────────────────────────────────────
        let source_col = args.source_col.unwrap_or(ColSpec::Index(0));
        let target_col = args.target_col.unwrap_or(ColSpec::Index(1));

        let sources = table.col_str(&source_col)?;
        let targets = table.col_str(&target_col)?;

        let weights: Vec<f64> = if let Some(ref wc) = args.weight_col {
            table.col_f64(wc)?
        } else {
            vec![1.0; sources.len()]
        };

        for ((src, tgt), w) in sources.iter().zip(targets.iter()).zip(weights.iter()) {
            plot = plot.with_edge(src.clone(), tgt.clone(), *w);
        }

        // Apply group column if provided.
        if let Some(ref gc) = args.group_col {
            let groups = table.col_str(gc)?;
            // Build a map of node-label → group from the edge-list rows.
            // Each row has a source and target; assign the group from that row.
            for ((src, tgt), grp) in sources.iter().zip(targets.iter()).zip(groups.iter()) {
                plot = plot.with_node_group(src.clone(), grp.clone());
                plot = plot.with_node_group(tgt.clone(), grp.clone());
            }
        }
    }

    let plots = vec![Plot::Network(plot)];
    let layout = Layout::auto_from_plots(&plots);
    let layout = apply_base_args(layout, &args.base);
    let scene = render_multiple(plots, layout);
    write_output(scene, &args.base)
}
