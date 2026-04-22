# `kuva scatter3d`

Render a **3D scatter plot** from a tabular file. Points are projected orthographically; depth ordering uses the painter's algorithm.

## Input format

Tab- or comma-separated file with at least three numeric columns for X, Y, and Z.

```tsv
x	y	z	group
1.2	3.4	2.1	A
2.5	1.8	4.3	B
3.1	4.2	1.5	A
```

## Basic examples

```bash
# Basic 3D scatter
kuva scatter3d data.tsv --x x --y y --z z -o scatter3d.svg

# Color points by group
kuva scatter3d data.tsv --x x --y y --z z --color-by group -o groups.svg

# Color by Z value using a colormap
kuva scatter3d data.tsv --x x --y y --z z \
    --z-color viridis --title "Z-colored scatter" -o zcolor.svg

# Custom view angle
kuva scatter3d data.tsv --x x --y y --z z \
    --azimuth -45 --elevation 40 -o angled.svg

# Depth shading + no bounding box
kuva scatter3d data.tsv --x x --y y --z z \
    --depth-shade --no-box -o depth.svg
```

## All flags

### Data selection

| Flag | Default | Description |
|------|---------|-------------|
| `--x <COL>` | `0` | X column (name or 0-based index) |
| `--y <COL>` | `1` | Y column |
| `--z <COL>` | `2` | Z column |
| `--color-by <COL>` | — | Group by column; one color per unique value |

### View and rendering

| Flag | Default | Description |
|------|---------|-------------|
| `--azimuth <DEG>` | `-60` | Azimuth viewing angle |
| `--elevation <DEG>` | `30` | Elevation viewing angle |
| `--z-color <MAP>` | — | Z-colormap (`viridis`, `inferno`, `grayscale`) |
| `--depth-shade` | — | Fade distant points for depth cue |
| `--z-axis-left` | — | Place Z-axis on the left side |
| `--no-grid` | — | Hide grid lines on back walls |
| `--no-box` | — | Hide wireframe bounding box |
| `--color <CSS>` | palette | Point color (overridden by `--color-by` or `--z-color`) |
| `--size <PX>` | — | Point radius in pixels |

### Axis labels

| Flag | Description |
|------|-------------|
| `--x-label <TEXT>` | X-axis label |
| `--y-label <TEXT>` | Y-axis label |
| `--z-label <TEXT>` | Z-axis label |

### Output / appearance

| Flag | Description |
|------|-------------|
| `-o <FILE>` | Output file (`.svg`, `.png`, `.pdf`; default: stdout) |
| `--title <TEXT>` | Chart title |
| `--width <PX>` | Canvas width |
| `--height <PX>` | Canvas height |
| `--theme <NAME>` | Visual theme |
