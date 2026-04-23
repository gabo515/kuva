# kuva mosaic

Mosaic / Marimekko chart — two-way contingency table where column widths encode one marginal and stacked segments encode the other.

**Input:** one row per cell with column-category, row-category, and count/value columns.

| Flag | Default | Description |
|---|---|---|
| `--col-col <COL>` | `0` | Column-category column (determines bar widths) |
| `--row-col <COL>` | `1` | Row-category column (determines stacked segments) |
| `--value-col <COL>` | `2` | Count or value column |
| `--gap <PX>` | `2.0` | Pixel gap between tiles |
| `--no-percents` | off | Hide percentage labels inside tiles |
| `--show-values` | off | Show raw values inside tiles |
| `--legend <LABEL>` | — | Add a legend |

```bash
kuva mosaic data.tsv --col-col region --row-col outcome --value-col count

kuva mosaic data.tsv --col-col region --row-col outcome --value-col count \
    --show-values --title "Outcomes by Region"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
