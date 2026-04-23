# kuva calendar

Calendar heatmap — GitHub-style contribution grid showing daily values across weeks.

**Input:** one row per data point with a date column (`YYYY-MM-DD`) and a numeric value column.

| Flag | Default | Description |
|---|---|---|
| `--date-col <COL>` | `0` | Date column (`YYYY-MM-DD` format) |
| `--value-col <COL>` | `1` | Numeric value column |
| `--agg <AGG>` | `count` | Aggregation for multiple entries per day: `count`, `sum`, `mean`, `max` |
| `--year <YEAR>` | auto | Display a single full calendar year (Jan–Dec) |
| `--start <DATE>` | — | Start date of a custom range (use with `--end`) |
| `--end <DATE>` | — | End date of a custom range (use with `--start`) |
| `--no-legend` | off | Hide the color-scale legend |

```bash
kuva calendar data.tsv --date-col date --value-col count

kuva calendar data.tsv --date-col date --value-col commits \
    --agg sum --year 2024 --title "Commits in 2024"

kuva calendar data.tsv --date-col date --value-col value \
    --start 2024-01-01 --end 2024-06-30 \
    --title "H1 2024 Activity"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance.*
