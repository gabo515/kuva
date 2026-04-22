# `kuva radar`

Render a **radar / spider chart** from a tabular file. Each row becomes one series polygon; multiple rows can be grouped by a column to compute per-group means.

## Input format

Tab- or comma-separated file with one numeric column per axis. Use `--axes` to select which columns to use.

```tsv
label	speed	power	agility	stamina	technique
Warrior	8	9	5	8	6
Mage	4	6	6	5	10
Rogue	7	5	10	6	7
```

## Basic examples

```bash
# Each row is one polygon; label column for legend entries
kuva radar data.tsv --axes speed power agility stamina technique \
    --label-col label --legend -o radar.svg

# Group rows by a column; polygon = mean per group
kuva radar data.tsv --axes x1 x2 x3 x4 \
    --color-by species --legend -o groups.svg

# Filled polygons with shared scale
kuva radar data.tsv --axes a b c d e \
    --filled --min 0 --max 10 \
    --title "Performance Profile" -o radar_filled.svg

# Normalized axes (each axis scaled to [0,1] independently)
kuva radar data.tsv --axes var1 var2 var3 var4 \
    --normalize --dot-size 4 -o radar_norm.svg
```

## All flags

### Data selection

| Flag | Description |
|------|-------------|
| `--axes <COLS...>` | Axis columns (names or 0-based indices); at least 3 required |
| `--label-col <COL>` | Column of series labels (one label per row) |
| `--color-by <COL>` | Group rows by this column; one polygon per group (mean values) |

### Chart style

| Flag | Default | Description |
|------|---------|-------------|
| `--filled` | — | Fill each polygon with a semi-transparent color |
| `--opacity <F>` | `0.25` | Fill opacity (used with `--filled`) |
| `--min <F>` | `0` | Shared axis minimum value |
| `--max <F>` | data max | Shared axis maximum value |
| `--grid-lines <N>` | `5` | Number of concentric grid rings |
| `--normalize` | — | Normalise each axis independently to `[0, 1]` |
| `--dot-size <PX>` | — | Draw dots at polygon vertices |
| `--legend` | — | Show a legend |

### Output / appearance

| Flag | Description |
|------|-------------|
| `-o <FILE>` | Output file (`.svg`, `.png`, `.pdf`; default: stdout) |
| `--title <TEXT>` | Chart title |
| `--width <PX>` | Canvas width in pixels |
| `--height <PX>` | Canvas height in pixels |
| `--theme <NAME>` | Visual theme (`default`, `dark`, `minimal`, …) |
