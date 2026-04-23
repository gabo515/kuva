# kuva pr

Precision-recall curve — evaluates binary classifiers when class imbalance makes ROC curves optimistic.

**Input:** one row per sample with a numeric score column and a binary label column (1 = positive, 0 = negative).

| Flag | Default | Description |
|---|---|---|
| `--score-col <COL>` | `0` | Classifier score column (higher = more positive) |
| `--label-col <COL>` | `1` | True label column (1/0 or true/false) |
| `--color-by <COL>` | — | Group column; one curve per unique value |
| `--no-baseline` | off | Hide the no-skill (prevalence) baseline |
| `--auc-label` | off | Append AUC-PR value to each curve's legend entry |
| `--legend <LABEL>` | — | Add a legend |

```bash
kuva pr data.tsv --score-col score --label-col label --auc-label

kuva pr data.tsv --score-col score --label-col label \
    --color-by model --auc-label --legend "Model" \
    --title "Precision-Recall Curves"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
