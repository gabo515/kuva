# kuva network

Network / graph diagram from an edge list or adjacency matrix.

**Edge-list input** (default): two columns for source and target nodes, with an optional weight column.

```
source	target	weight
TP53	MDM2	0.95
TP53	BAX	0.82
MDM2	TP53	0.88
```

**Matrix input** (`--matrix`): square N×N matrix — first column is the row label, header row supplies node names.

```
node	TP53	MDM2	BAX
TP53	0	0.95	0.82
MDM2	0.88	0	0
BAX	0	0	0
```

| Flag | Default | Description |
|---|---|---|
| `--matrix` | off | Read input as N×N adjacency matrix |
| `--source-col <COL>` | `0` | Source node column (index or name) |
| `--target-col <COL>` | `1` | Target node column (index or name) |
| `--weight-col <COL>` | — | Edge weight column |
| `--group-col <COL>` | — | Node group column for colouring |
| `--directed` | off | Draw arrowheads on edges |
| `--layout <ALG>` | `force` | Layout algorithm: `force` or `circle` |
| `--node-radius <PX>` | `8.0` | Node circle radius in pixels |
| `--opacity <F>` | `0.6` | Edge opacity |
| `--labels` | off | Show node labels |
| `--legend <LABEL>` | — | Show legend |

```bash
kuva network edges.tsv --source-col source --target-col target

kuva network edges.tsv --source-col source --target-col target \
    --weight-col weight --directed --labels --legend "interaction"

kuva network --matrix matrix.tsv --layout circle --labels
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes, log scale.*
