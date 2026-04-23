# kuva survival

Kaplan-Meier survival curve — estimates the survival function from time-to-event data with right-censoring.

**Input:** one row per subject with time, event indicator (1 = event occurred, 0 = censored), and optional group columns.

| Flag | Default | Description |
|---|---|---|
| `--time-col <COL>` | `0` | Follow-up time column |
| `--event-col <COL>` | `1` | Event indicator column (1 = event, 0 = censored) |
| `--group-col <COL>` | — | Group column; one curve per unique value |
| `--no-ci` | off | Hide Greenwood 95% confidence interval bands |
| `--no-censoring` | off | Hide censoring tick marks |
| `--line-width <PX>` | `2.0` | Stroke width of survival curves |
| `--legend <LABEL>` | — | Add a legend |

```bash
kuva survival data.tsv --time-col time --event-col event

kuva survival data.tsv --time-col time --event-col event \
    --group-col treatment --legend "Group" \
    --title "Kaplan-Meier Survival by Treatment"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
