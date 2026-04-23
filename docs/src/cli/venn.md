# kuva venn

Venn diagram — 2–4 overlapping sets with intersection counts labeled in each region.

**Input:** one row per element–set membership pair (element column + set column). Intersections are computed automatically.

| Flag | Default | Description |
|---|---|---|
| `--element-col <COL>` | `0` | Element/item column |
| `--set-col <COL>` | `1` | Set name column |
| `--proportional` | off | Scale circle areas proportional to set sizes |
| `--no-set-labels` | off | Hide set name labels |
| `--fill-opacity <F>` | `0.25` | Circle fill opacity |
| `--legend <LABEL>` | — | Add a legend |

```bash
kuva venn data.tsv --element-col gene --set-col set

kuva venn data.tsv --element-col gene --set-col pathway \
    --proportional --legend "Gene Sets" \
    --title "Pathway Overlap"
```

**Note:** supports 2, 3, or 4 sets. More than 4 sets are not supported.

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance.*
