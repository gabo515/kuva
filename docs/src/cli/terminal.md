# Terminal Output

The `--terminal` flag renders any plot directly in the terminal using Unicode braille characters, block fills, and ANSI 24-bit colour. No display, no file, no system dependencies — just a UTF-8 terminal.

This is especially useful on HPC clusters, remote servers, or any environment where opening an SVG or PNG is inconvenient.

---

## Usage

```bash
# Auto-detect terminal size
kuva scatter data.tsv --x x --y y --terminal

# Explicit dimensions (useful in scripts or multiplexers)
kuva bar counts.tsv --label-col gene --value-col count --terminal --term-width 120 --term-height 40

# Pipe from stdin
cat gwas.tsv | kuva manhattan --chr-col chr --pvalue-col pvalue --terminal
```

`--terminal` is mutually exclusive with `-o`. When both are absent, output defaults to SVG on stdout.

---

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--terminal` | off | Render to the terminal instead of a file |
| `--term-width N` | auto | Terminal width in character columns |
| `--term-height N` | auto | Terminal height in character rows |

Terminal dimensions are auto-detected via `ioctl(TIOCGWINSZ)` and fall back to 100×30 if detection fails. Override with `--term-width` / `--term-height` — useful inside tmux panes, CI logs, or when piping output.

---

## How it works

Each character cell maps to a 2×4 braille dot grid, giving an effective pixel resolution of `(cols×2) × (rows×4)`. Three rendering layers are composited on output, with text taking priority over braille:

| Layer | Characters | Used for |
|-------|-----------|----------|
| Braille | U+2800–U+28FF | Scatter points, line paths, curves, contour lines |
| Full block | `█` | Bar and histogram fills, legend colour swatches |
| Text | ASCII / UTF-8 | Tick labels, axis titles, legend entries |

Colour is output as ANSI 24-bit escape codes. All SVG path types are supported including cubic Bézier curves (tessellated to 20 segments) and filled polygons (scanline even-odd fill in braille space) — so Sankey ribbons, Chord arcs, Pie slices, and Contour fills all render correctly.

---

## Examples

**Scatter**

![scatter terminal](../assets/terminal/scatter.gif)

**Manhattan**

![manhattan terminal](../assets/terminal/manhattan.gif)

**Sankey**

![sankey terminal](../assets/terminal/sankey.gif)

**Contour**

![contour terminal](../assets/terminal/contour.gif)

**Candlestick**

![candlestick terminal](../assets/terminal/candlestick.gif)

---

## Supported plot types

All subcommands support `--terminal` except `upset`.

| Status | Subcommands |
|--------|------------|
| Supported | `scatter`, `line`, `bar`, `histogram`, `box`, `violin`, `strip`, `pie`, `waterfall`, `stacked-area`, `volcano`, `manhattan`, `candlestick`, `heatmap`, `hist2d`, `contour`, `dot`, `chord`, `sankey`, `phylo`, `synteny` |
| Not supported | `upset` — prints a message and exits cleanly; use `-o file.svg` instead |
