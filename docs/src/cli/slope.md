# kuva slope

Slope chart — compare paired before/after values for multiple items on two axes.

**Input:** one row per item with columns for label, before value, and after value.

| Flag | Default | Description |
|---|---|---|
| `--label-col <COL>` | `0` | Item label column |
| `--before-col <COL>` | `1` | Before (left axis) value column |
| `--after-col <COL>` | `2` | After (right axis) value column |
| `--before-label <TEXT>` | `Before` | Left axis label |
| `--after-label <TEXT>` | `After` | Right axis label |
| `--color-up <CSS>` | `steelblue` | Color for items that increased |
| `--color-down <CSS>` | `firebrick` | Color for items that decreased |
| `--no-direction-colors` | off | Use a single uniform color for all lines |
| `--show-values` | off | Show value labels at each endpoint |
| `--line-width <PX>` | `1.5` | Stroke width of slope lines |
| `--dot-radius <PX>` | `4.0` | Radius of endpoint dots |

```bash
kuva slope data.tsv --label-col label --before-col before --after-col after

kuva slope data.tsv --label-col label --before-col q1 --after-col q2 \
    --before-label "Q1 2024" --after-label "Q2 2024" \
    --show-values --title "Quarterly Change"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
