use std::collections::BTreeMap;

use clap::Args;

use kuva::plot::bar::BarPlot;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::render::render::render_multiple;

use crate::data::{ColSpec, DataTable, InputArgs};
use crate::layout_args::{BaseArgs, AxisArgs, apply_base_args, apply_axis_args};
use crate::output::write_output;

/// Bar chart from label and value columns.
#[derive(Args, Debug)]
pub struct BarArgs {
    /// Label column (0-based index or header name; default: 0).
    #[arg(long)]
    pub label_col: Option<ColSpec>,

    /// Value column (0-based index or header name; default: 1).
    #[arg(long)]
    pub value_col: Option<ColSpec>,

    /// Count occurrences of each unique value in this column (ignores --value-col).
    #[arg(long)]
    pub count_by: Option<ColSpec>,

    /// Aggregate --value-col by --label-col using this function: mean, median, sum, min, max.
    #[arg(long, value_name = "FUNC")]
    pub agg: Option<String>,

    /// Bar fill color (CSS string; default: "steelblue").
    #[arg(long)]
    pub color: Option<String>,

    /// Bar width as a fraction of the slot (default: 0.8).
    #[arg(long)]
    pub bar_width: Option<f64>,

    #[command(flatten)]
    pub input: InputArgs,

    #[command(flatten)]
    pub base: BaseArgs,
    #[command(flatten)]
    pub axis: AxisArgs,
}

pub fn run(args: BarArgs) -> Result<(), String> {
    let table = DataTable::parse(
        args.input.input.as_deref(),
        args.input.no_header,
        args.input.delimiter,
    )?;

    let color = args.color.unwrap_or_else(|| "steelblue".to_string());

    let pairs: Vec<(String, f64)> = if let Some(ref count_col) = args.count_by {
        let values = table.col_str(count_col)?;
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for v in values {
            *counts.entry(v).or_insert(0) += 1;
        }
        counts.into_iter().map(|(k, c)| (k, c as f64)).collect()
    } else if let Some(ref func) = args.agg {
        let label_col = args.label_col.unwrap_or(ColSpec::Index(0));
        let value_col = args.value_col.unwrap_or(ColSpec::Index(1));
        let labels = table.col_str(&label_col)?;
        let values = table.col_f64(&value_col)?;
        // Accumulate values per group (preserve insertion order via Vec).
        let mut order: Vec<String> = Vec::new();
        let mut groups: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for (label, val) in labels.into_iter().zip(values) {
            if !groups.contains_key(&label) {
                order.push(label.clone());
            }
            groups.entry(label).or_default().push(val);
        }
        order.into_iter().map(|label| {
            let vals = &groups[&label];
            let agg_val = match func.as_str() {
                "sum"    => vals.iter().sum(),
                "min"    => vals.iter().cloned().fold(f64::INFINITY, f64::min),
                "max"    => vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                "median" => {
                    let mut s = vals.clone();
                    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let n = s.len();
                    if n % 2 == 0 { (s[n/2 - 1] + s[n/2]) / 2.0 } else { s[n/2] }
                }
                _ => vals.iter().sum::<f64>() / vals.len() as f64, // "mean" + fallback
            };
            (label, agg_val)
        }).collect()
    } else {
        let label_col = args.label_col.unwrap_or(ColSpec::Index(0));
        let value_col = args.value_col.unwrap_or(ColSpec::Index(1));
        let labels = table.col_str(&label_col)?;
        let values = table.col_f64(&value_col)?;
        labels.into_iter().zip(values).collect()
    };

    let mut plot = BarPlot::new()
        .with_bars(pairs)
        .with_color(&color);

    if let Some(w) = args.bar_width {
        plot = plot.with_width(w);
    }

    let plots = vec![Plot::Bar(plot)];
    let layout = Layout::auto_from_plots(&plots);
    let layout = apply_base_args(layout, &args.base);
    let layout = apply_axis_args(layout, &args.axis);
    let layout = layout.with_x_tick_rotate(-45.0);
    let scene = render_multiple(plots, layout);
    write_output(scene, &args.base)
}
