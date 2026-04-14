# kuva ecdf

Empirical cumulative distribution function. Plots `F(x) = P(X ≤ x)` as a right-continuous step function. Multi-group plots overlay one curve per group; use `--confidence-band` to show DKW 95% bands.

**Input:** a tabular file with at least one numeric column. When `--color-by` is used, an additional categorical column drives the grouping.

| Flag | Default | Description |
|---|---|---|
| `--value <COL>` | `0` | Column of numeric values |
| `--color-by <COL>` | — | Group by this column; one curve per unique value |
| `--complementary` | off | Plot `1 - F(x)` (survival / exceedance probability) |
| `--confidence-band` | off | DKW 95% confidence band around each curve |
| `--rug` | off | Tick marks at each data point below the x-axis |
| `--percentile-lines <LIST>` | — | Comma-separated F values, e.g. `0.25,0.5,0.75` |
| `--markers` | off | Circle at each step endpoint (useful for small n) |
| `--smooth` | off | KDE-integrated smooth CDF instead of step function |
| `--stroke-width <F>` | `1.5` | Line stroke width |

```bash
# Basic ECDF
kuva ecdf data.tsv --value score --x-label "Score" --y-label "F(x)" --title "ECDF"

# Multi-group with confidence bands
kuva ecdf data.tsv --value expression --color-by group --confidence-band

# Complementary CDF with log x-axis (read lengths)
kuva ecdf reads.tsv --value length --complementary --rug --log-x \
    --x-label "Read length (bp)" --y-label "Fraction ≥ length"

# Percentile markers + rug
kuva ecdf data.tsv --value score --percentile-lines 0.25,0.5,0.75 --markers --rug

# Smooth CDF
kuva ecdf data.tsv --value score --color-by group --smooth
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes, log scale.*
