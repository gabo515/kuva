# kuva funnel

Render a funnel chart from a tabular file.

## Input format

Two columns: stage label and value (in funnel order, widest stage first).

```
stage       count
Screened    1200
Eligible     840
Enrolled     720
Completed    648
```

For diverging mode, add a second value column for the right side:

```
stage       treatment   control
Screened    1200        1150
Eligible     840         810
Enrolled     720         690
Completed    648         620
```

## Usage

```
kuva funnel [OPTIONS] [INPUT]
```

## Data columns

| Flag | Default | Description |
|------|---------|-------------|
| `--label <COL>` | `0` | Stage label column (name or 0-based index). |
| `--value <COL>` | `1` | Stage value column. |
| `--mirror-col <COL>` | — | Right-side values — enables diverging back-to-back mode. |
| `--left-label <S>` | — | Label above the left (main) side in diverging mode. |
| `--right-label <S>` | — | Label above the right (mirror) side in diverging mode. |

## Appearance

| Flag | Default | Description |
|------|---------|-------------|
| `--orientation <MODE>` | `vertical` | `vertical` or `horizontal`. |
| `--color-by <MODE>` | `uniform` | `uniform`, `stage`, `gradient`. |
| `--no-connectors` | off | Hide trapezoidal connectors between bars. |
| `--connector-opacity <F>` | `0.4` | Connector fill opacity 0–1. |
| `--no-values` | off | Hide absolute value labels on bars. |
| `--show-percents` | off | Show percentage-of-first-stage alongside value labels. |
| `--no-conversion` | off | Hide step-to-step conversion rates in connectors. |
| `--stage-gap <F>` | `4.0` | Gap in pixels between adjacent bars. |
| `--legend <LABEL>` | — | Show a legend with this label. |

## Examples

```bash
# Basic vertical funnel
kuva funnel funnel.tsv --label stage --value count -o funnel.svg

# Horizontal orientation with percentage labels
kuva funnel funnel.tsv --orientation horizontal --show-percents -o funnel_h.svg

# Stage colors + gradient
kuva funnel funnel.tsv --color-by gradient -o funnel_grad.svg

# Diverging back-to-back (treatment vs control)
kuva funnel funnel.tsv --label stage --value n_screened --mirror-col n_placebo \
    --left-label Treatment --right-label Control -o funnel_mirror.svg

# No connectors, conversion only
kuva funnel funnel.tsv --no-connectors --no-values -o funnel_minimal.svg
```
