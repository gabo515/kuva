---
name: HorizonPlot notes
description: Architecture, API, color tinting, band rendering, annotation features, and tests for HorizonPlot
type: project
---

## Architecture

- `src/plot/horizon.rs` — `HorizonSeries`, `HorizonPlot` structs
- Standard axes (not pixel-space) — x-axis is drawn normally; NOT in `skip_axes`
- `bounds()` returns `((x_min, x_max), (0.5, n+0.5))` — real x bounds, y range for n categorical rows
- `y_categories` in `auto_from_plots`: `hp.series.iter().rev().map(|s| s.label.clone())` — reversed so series[0] renders at top

## Band rendering

- Divide value range into N bands; alpha = `b / n_bands` (lightest=1/N, darkest=1.0)
- Draw lightest band first, darkest last — they overlay via CSS opacity to produce darker appearance for higher values
- Color tinting via `PathData.opacity` on path element (solid hex fill color)
- `parse_hex_color(hex: &str) -> (u8, u8, u8)` — private utility in render.rs for #RRGGBB/#RGB
- Band early-exit: `any_pos`/`any_neg` check skips entire band if no point has nonzero contribution (`raw_v - band_lo > 1e-12`)
- Positive band path: `raw_v = (y - baseline).max(0.0)`; negative: `(baseline - y).max(0.0)`

## Auto canvas sizing

- When `row_height.is_some()`, height = `row_height * n_series + margin_top + margin_bottom`
- `auto_from_plots` detects `row_height` and sets `layout.height` accordingly

## Color auto-assignment (`with_series`)

- Cycles through category10 palette for `pos_color` by `self.series.len()` index
- `neg_color` fixed to `#d62728` (palette red) — universal signal for below-baseline
- `with_series_colored(label, x, y, pos_color, neg_color)` for explicit colors

## Value label annotations (`show_value_labels` / `show_sign_colors`)

- `show_value_labels=true`: draws `+{n_bands*pos_bw}` and `-{n_bands*neg_bw}` at right of each row
- `show_sign_colors=true`: colorizes `+` in `pos_color` and `-` in `neg_color` as separate Text primitives
- `show_sign_colors` has no visible effect unless `show_value_labels` is also true
- Right-margin expansion: `Layout.horizon_right_annot_px` field (default 0.0)
  - Set to 68.0 in `auto_from_plots` when `show_value_labels || show_sign_colors`
  - Added to `margin_right` in `ComputedLayout::from_layout`
  - Copied in `clone_layout` (figure.rs)
- Value formatting uses `TickFormat::Auto.format(scale)` (not `tick_format_auto`)
- Annotation x position: `computed.width - computed.margin_right + 6.0`
- Sign+value layout: sign char at `annot_x`, value at `annot_x + font_size * 0.65`

## Key struct fields

```rust
pub series: Vec<HorizonSeries>
pub n_bands: usize            // default 3
pub row_height: Option<f64>   // None = derive from canvas
pub baseline: f64             // default 0.0
pub value_max: Option<f64>    // None = derive from data
pub show_legend: bool
pub show_value_labels: bool
pub show_sign_colors: bool
```

## Tests (`tests/horizon_basic.rs`)

30 tests total including:
- Structural: empty, multi-series, custom colors, baseline, n_bands variants
- Unit: pos/neg band widths, x_range, n_series, value_max override
- Legend: distinct colors per series (auto-palette)
- Positive-only / negative-only path emission early-exit
- `show_value_labels`: text emitted, pos-only, sign_colors colorize, no-op without labels, margin expansion
- Showcase: server metrics, temperature anomaly, financial, dense 32-series (365 pts each), multi-series with annotations
