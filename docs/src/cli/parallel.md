# kuva parallel

Parallel coordinates — multivariate visualisation where each axis represents one variable and each observation is a polyline.

**Input:** one row per observation; `--value-cols` selects the numeric axes; an optional group column colors the lines.

| Flag | Default | Description |
|---|---|---|
| `--value-cols <COL>…` | required | Two or more numeric columns (names or indices) |
| `--group-col <COL>` | — | Group/color column |
| `--axis-names <NAME>…` | header or "Axis N" | Override axis labels |
| `--no-normalize` | off | Disable per-axis normalization to [0, 1] |
| `--curved` | off | Render smooth Bézier curves instead of polylines |
| `--opacity <F>` | `0.5` | Line opacity |
| `--show-mean` | off | Overlay a bold group-mean line |
| `--legend <LABEL>` | — | Add a legend |

```bash
kuva parallel data.tsv \
    --value-cols sepal_length sepal_width petal_length petal_width \
    --group-col species

kuva parallel data.tsv \
    --value-cols 0 1 2 3 --group-col 4 \
    --curved --show-mean --legend "Species" \
    --title "Iris Parallel Coordinates"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
