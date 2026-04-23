# kuva lollipop

Lollipop chart — dot-and-stem alternative to bar charts, useful for emphasising individual values.

**Input:** one row per data point with x (numeric or categorical) and y value columns.

| Flag | Default | Description |
|---|---|---|
| `--x-col <COL>` | `0` | X-value column (numeric or string; strings become categorical) |
| `--y-col <COL>` | `1` | Y-value column |
| `--label-col <COL>` | — | Optional label column (shown at each dot) |
| `--color <CSS>` | `steelblue` | Stem and dot color |
| `--baseline <F>` | `0.0` | Value at which stems originate |
| `--stem-width <PX>` | `1.5` | Stem stroke width |
| `--dot-radius <PX>` | `5.0` | Dot radius |
| `--no-baseline-line` | off | Hide the horizontal baseline rule |
| `--legend <LABEL>` | — | Add a legend entry |

```bash
kuva lollipop data.tsv --x-col gene --y-col expression

kuva lollipop data.tsv --x-col gene --y-col log2fc \
    --baseline 0 --color "#e15759" \
    --label-col gene --title "Differentially Expressed Genes"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
