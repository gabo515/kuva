# kuva horizon

Horizon chart — compact stacked time-series that folds values into colored bands, ideal for many series in limited vertical space.

**Input:** one row per observation with x (time), value, and optional group/series columns.

| Flag | Default | Description |
|---|---|---|
| `--x-col <COL>` | `0` | X-axis (time or sequence) column |
| `--value-col <COL>` | `1` | Numeric value column |
| `--group-col <COL>` | — | Series/group column; one row per unique value |
| `--n-bands <N>` | `3` | Number of color bands to fold into |
| `--row-height <PX>` | auto | Height in pixels of each series row |
| `--baseline <F>` | `0.0` | Zero-line value; values below are negative (cool colors) |
| `--value-labels` | off | Show scale annotations at the right end of each row |
| `--legend` | off | Add a legend |

```bash
kuva horizon data.tsv --x-col week --value-col value --group-col series

kuva horizon data.tsv --x-col time --value-col count --group-col region \
    --n-bands 4 --value-labels --title "Weekly Activity by Region"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
