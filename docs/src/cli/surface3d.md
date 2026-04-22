# `kuva surface3d`

Render a **3D surface plot** from a tabular file. Quads are depth-sorted and filled with a Z-colormap. Both long-format (x, y, z triples) and matrix (Z-value grid) inputs are supported.

## Input format

**Long format** (default) — one row per grid point:

```tsv
x	y	z
0	0	0.5
0	1	1.2
1	0	0.8
1	1	2.1
```

**Matrix format** (`--matrix`) — one row of Z values per line, no header. Row index = Y, column index = X.

```tsv
0.5	0.8	1.2	1.5
0.8	1.1	1.6	2.0
1.2	1.6	2.1	2.5
```

## Basic examples

```bash
# Long-format surface
kuva surface3d data.tsv --x x --y y --z z -o surface.svg

# With colormap and wireframe disabled
kuva surface3d data.tsv --x x --y y --z z \
    --z-color viridis --no-wireframe -o viridis.svg

# Matrix input, upsampled to 50×50 for smooth appearance
kuva surface3d matrix.tsv --matrix --resolution 50 -o smooth.svg

# Semi-transparent surface with custom view
kuva surface3d data.tsv --x x --y y --z z \
    --alpha 0.6 --azimuth -45 --elevation 35 -o alpha.svg

# Disable all decorations
kuva surface3d data.tsv --x x --y y --z z \
    --no-grid --no-box -o minimal.svg
```

## All flags

### Data selection

| Flag | Default | Description |
|------|---------|-------------|
| `--x <COL>` | `0` | X column (long format) |
| `--y <COL>` | `1` | Y column (long format) |
| `--z <COL>` | `2` | Z column (long format) |
| `--matrix` | — | Read input as a matrix of Z values |

### View and rendering

| Flag | Default | Description |
|------|---------|-------------|
| `--azimuth <DEG>` | `-60` | Azimuth viewing angle |
| `--elevation <DEG>` | `30` | Elevation viewing angle |
| `--z-color <MAP>` | — | Z-colormap (`viridis`, `inferno`, `grayscale`) |
| `--color <CSS>` | — | Uniform surface color (when no colormap) |
| `--alpha <F>` | `1.0` | Surface opacity (0–1) |
| `--no-wireframe` | — | Disable wireframe grid edges on the mesh |
| `--resolution <N>` | — | Upsample to N×N grid via bilinear interpolation |
| `--z-axis-left` | — | Place Z-axis on the left side |
| `--no-grid` | — | Hide grid lines on back walls |
| `--no-box` | — | Hide wireframe bounding box |

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
