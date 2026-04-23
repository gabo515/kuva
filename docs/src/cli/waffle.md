# kuva waffle

Waffle chart — proportional grid of filled squares (or circles), one cell per unit.

**Input:** one row per category with label, value, and optionally color columns.

| Flag | Default | Description |
|---|---|---|
| `--label-col <COL>` | `0` | Category label column |
| `--value-col <COL>` | `1` | Proportional value column |
| `--color-col <COL>` | — | Per-category color column (CSS strings); defaults to category10 palette |
| `--rows <N>` | `10` | Number of grid rows |
| `--cols <N>` | `10` | Number of grid columns |
| `--gap <F>` | `0.1` | Gap between cells as a fraction of cell size |
| `--shape <SHAPE>` | `square` | Cell shape: `square` or `circle` |
| `--show-percents` | off | Append percentage to legend labels |
| `--show-counts` | off | Append cell count to legend labels |
| `--legend <LABEL>` | — | Add a legend |

```bash
kuva waffle data.tsv --label-col category --value-col value --color-col color

kuva waffle data.tsv --label-col category --value-col value \
    --shape circle --show-percents --legend "Energy Mix" \
    --title "Energy Sources"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance.*
