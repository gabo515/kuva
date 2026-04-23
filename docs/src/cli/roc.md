# kuva roc

ROC curve — receiver operating characteristic for binary classifiers, with optional AUC and confidence intervals.

**Input:** one row per sample with a numeric score column and a binary label column (1 = positive, 0 = negative).

| Flag | Default | Description |
|---|---|---|
| `--score-col <COL>` | `0` | Classifier score column (higher = more positive) |
| `--label-col <COL>` | `1` | True label column (1/0 or true/false) |
| `--color-by <COL>` | — | Group column; one curve per unique value |
| `--no-diagonal` | off | Hide the random-classifier diagonal reference line |
| `--ci` | off | Show DeLong 95% confidence interval band |
| `--auc-label` | off | Append AUC value to each curve's legend entry |
| `--legend <LABEL>` | — | Add a legend |

```bash
kuva roc data.tsv --score-col score --label-col label --auc-label

kuva roc data.tsv --score-col score --label-col label \
    --color-by model --ci --auc-label --legend "Model" \
    --title "ROC Curves"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
