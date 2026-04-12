# kuva sunburst

Render a sunburst chart from a TSV/CSV file.

```
kuva sunburst [OPTIONS] [INPUT]
```

## Input format

By default the file should have two columns: `label` and `value`.
Add a `--parent` column for a two-level hierarchy.

```
label   value
Rust    40
Python  35
Go      25
```

Two-level with parent column:

```
category  item    value
Mammals   Dog     40
Mammals   Cat     35
Mammals   Bear    25
Birds     Eagle   60
Birds     Parrot  40
```

## Options

### Data mapping

| Flag | Default | Description |
|---|---|---|
| `--label <COL>` | `0` | Label column (name or 0-based index) |
| `--value <COL>` | `1` | Value column |
| `--parent <COL>` | — | Group rows by parent column (two-level hierarchy) |
| `--color-col <COL>` | — | Color values (`--color-by value`) or CSS colors (`--color-by explicit`) |
| `--color-by <MODE>` | `parent` | Color mode: `parent`, `value`, `explicit` |
| `--colormap <NAME>` | `viridis` | `viridis`, `inferno`, `turbo`, `grayscale` |

### Appearance

| Flag | Default | Description |
|---|---|---|
| `--inner-radius <F>` | — | Fractional inner hole (e.g. `0.3` for donut style) |
| `--start-angle <DEG>` | — | Starting angle in degrees (0 = north) |
| `--ring-gap <F>` | — | Gap in pixels between rings |
| `--min-label-angle <DEG>` | — | Minimum arc sweep for label to render |
| `--max-depth <N>` | — | Limit rendered depth |
| `--colorbar` | off | Show colorbar (value mode) |
| `--colorbar-label <STR>` | — | Colorbar label |
| `--no-tooltips` | off | Suppress hover tooltips |

### Output

| Flag | Description |
|---|---|
| `-o <FILE>` | Output file (`.svg`, `.png`, `.pdf`, or omit for stdout) |
| `--title <STR>` | Chart title |
| `--width <F>` | Canvas width in pixels |
| `--height <F>` | Canvas height in pixels |
| `--theme <NAME>` | Theme: `light` (default), `dark`, `minimal`, `publication` |

## Examples

```bash
# Flat sunburst from two-column TSV
kuva sunburst data.tsv -o sunburst.svg

# Two-level hierarchy grouped by parent column
kuva sunburst data.tsv --parent category --label item --value value -o sunburst.svg

# Donut style
kuva sunburst data.tsv --inner-radius 0.35 -o donut.svg

# Color by value with viridis colormap and colorbar
kuva sunburst data.tsv --color-by value --colorbar --colorbar-label "Score" -o colored.svg

# Terminal output
kuva sunburst data.tsv --terminal
```
