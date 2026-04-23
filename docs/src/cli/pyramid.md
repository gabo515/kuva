# kuva pyramid

Population pyramid — back-to-back horizontal bar chart for comparing two distributions across age groups or categories.

**Input:** one row per age group with label, left value, and right value columns.

| Flag | Default | Description |
|---|---|---|
| `--label-col <COL>` | `0` | Age/category label column |
| `--left-col <COL>` | `1` | Left-side value column |
| `--right-col <COL>` | `2` | Right-side value column |
| `--left-label <TEXT>` | `Left` | Label for the left side (e.g. `Male`) |
| `--right-label <TEXT>` | `Right` | Label for the right side (e.g. `Female`) |
| `--left-color <CSS>` | `#4C72B0` | Bar color for the left side |
| `--right-color <CSS>` | `#DD8452` | Bar color for the right side |
| `--normalize` | off | Display values as percentage of total |
| `--show-values` | off | Show value labels at bar tips |
| `--legend` | off | Add a legend |

```bash
kuva pyramid data.tsv --label-col age --left-col male --right-col female \
    --left-label Male --right-label Female

kuva pyramid data.tsv --label-col age --left-col male --right-col female \
    --left-label Male --right-label Female \
    --normalize --legend --title "Population by Age"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
