# kuva streamgraph

Flowing stacked area chart with a displaced baseline (streamgraph).

Three baseline algorithms are available:

- **wiggle** (default) — Byron & Wattenberg optimal: minimises visual motion in the silhouette.
- **symmetric** — ThemeRiver: mirrors the stack around y = 0 at every x.
- **zero** — standard stacked area from y = 0 with smooth Catmull-Rom curves.

**Input:** a tabular file with x, group, and value columns (long format — one row per group per x value).

| Flag | Default | Description |
|------|---------|-------------|
| `--x-col <COL>` | `0` | X-axis column |
| `--group-col <COL>` | `1` | Group/category column |
| `--y-col <COL>` | `2` | Value column |
| `--baseline <S>` | `wiggle` | `wiggle`, `symmetric`, `zero` |
| `--order <S>` | `inside-out` | `inside-out`, `by-total`, `original` |
| `--linear` | off | Straight line segments instead of Catmull-Rom splines |
| `--normalize` | off | Normalise each column to 100 % |
| `--stroke` | off | White separator strokes between streams |
| `--no-labels` | — | Hide inline stream labels |
| `--min-label-height <F>` | `14.0` | Minimum band height (px) before label appears |
| `--fill-opacity <F>` | `0.85` | Fill opacity (0–1) |

```bash
# Wiggle (default) — gut microbiome over 52 weeks
kuva streamgraph data.tsv

# Symmetric baseline — ThemeRiver style
kuva streamgraph data.tsv --baseline symmetric

# 100% normalised — show proportional composition
kuva streamgraph data.tsv --normalize \
    --y-label "Proportion (%)"

# Linear segments with strokes and legend instead of labels
kuva streamgraph data.tsv --linear --stroke --no-labels --legend ""

# Custom columns
kuva streamgraph counts.tsv \
    --x-col week --group-col phylum --y-col abundance \
    --title "Weekly phylum abundance"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes, log scale.*
