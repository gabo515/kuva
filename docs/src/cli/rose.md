# `kuva rose`

Render a **Nightingale rose** (coxcomb) chart from a tabular file.

## Input format

Tab- or comma-separated file with at least two columns: a label column and a value column.

```tsv
direction	count
N	25
NE	18
E	12
SE	8
S	10
SW	14
W	20
NW	22
```

For multi-series mode, add a group column and use `--group-by`:

```tsv
direction	speed_class	count
N	low	15
N	high	8
NE	low	22
NE	high	12
```

## Basic examples

```bash
# Single-series rose chart
kuva rose data.tsv --label direction --value count -o rose.svg

# Wind rose from provided example data (stacked low/high speed)
kuva rose examples/data/rose.tsv --label direction \
    --group-by direction --mode stacked -o wind_rose.svg

# With compass direction labels
kuva rose bearings.tsv --value bearing --compass -o compass_rose.svg

# Donut (inner hole)
kuva rose data.tsv --inner-radius 0.3 -o donut_rose.svg
```

## All flags

### Data selection

| Flag | Description |
|------|-------------|
| `--label <COL>` | Label column (name or 0-based index; default: 0) |
| `--value <COL>` | Value column (name or 0-based index; default: 1) |
| `--group-by <COL>` | Group/series column for multi-series mode |

### Chart style

| Flag | Default | Description |
|------|---------|-------------|
| `--mode <MODE>` | `stacked` | Multi-series layout: `stacked` or `grouped` |
| `--encoding <ENC>` | `area` | Radius encoding: `area` (accurate) or `radius` |
| `--inner-radius <F>` | `0` | Fraction 0–1; creates a donut hole |
| `--gap <DEG>` | `1` | Angular gap between sectors (degrees) |
| `--start-angle <DEG>` | `0` | Start angle clockwise from north |
| `--no-clockwise` | — | Lay out sectors counterclockwise |
| `--no-grid` | — | Hide concentric grid rings |
| `--grid-lines <N>` | `4` | Number of concentric grid rings |
| `--no-labels` | — | Hide sector labels around the perimeter |
| `--show-values` | — | Show value labels at the tip of each sector |
| `--compass` | — | Replace labels with compass directions (N, NE, E, …) |
| `--legend <LABEL>` | — | Show legend (for multi-series plots) |

### Output / appearance

| Flag | Description |
|------|-------------|
| `-o <FILE>` | Output file (`.svg`, `.png`, `.pdf`; default: stdout) |
| `--title <TEXT>` | Chart title |
| `--width <PX>` | Canvas width in pixels |
| `--height <PX>` | Canvas height in pixels |
| `--theme <NAME>` | Visual theme (`default`, `dark`, `minimal`, …) |
| `--palette <NAME>` | Color palette name |

## Multi-series example

```bash
kuva rose examples/data/rose.tsv \
    --label direction \
    --value low_speed \
    --group-by direction \
    --legend "Wind speed" \
    --mode stacked \
    --title "Wind Rose" \
    -o wind_rose.svg
```
