//! Network plot documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example network
//! ```
//!
//! SVGs are written to `docs/src/assets/network/`.

use kuva::plot::network::{NetworkPlot, NetworkLayout};
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

const OUT: &str = "docs/src/assets/network";

fn main() {
    std::fs::create_dir_all(OUT).expect("could not create docs/src/assets/network");

    basic();
    directed();
    grouped();

    println!("Network plot SVGs written to {OUT}/");
}

// ── Basic undirected network ──────────────────────────────────────────────

fn basic() {
    let net = NetworkPlot::new()
        .with_edge("TP53", "MDM2", 0.95)
        .with_edge("TP53", "BAX", 0.82)
        .with_edge("TP53", "CDKN1A", 0.78)
        .with_edge("MDM2", "TP53", 0.88)
        .with_edge("BRCA1", "TP53", 0.65)
        .with_edge("BRCA1", "RAD51", 0.72)
        .with_edge("RAD51", "BRCA2", 0.68)
        .with_edge("BRCA2", "BRCA1", 0.55)
        .with_edge("MYC", "CCND1", 0.91)
        .with_edge("MYC", "CDK4", 0.74)
        .with_edge("CCND1", "CDK4", 0.83)
        .with_labels();

    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gene Interaction Network");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/basic.svg"), svg).unwrap();
}

// ── Directed network with arrowheads ──────────────────────────────────────

fn directed() {
    let net = NetworkPlot::new()
        .with_edge("TP53", "MDM2", 0.95)
        .with_edge("TP53", "BAX", 0.82)
        .with_edge("TP53", "CDKN1A", 0.78)
        .with_edge("MDM2", "TP53", 0.88)
        .with_edge("BRCA1", "TP53", 0.65)
        .with_edge("MYC", "CCND1", 0.91)
        .with_edge("MYC", "CDK4", 0.74)
        .with_edge("CCND1", "CDK4", 0.83)
        .with_edge("CDK4", "RB1", 0.79)
        .with_edge("RB1", "E2F1", 0.86)
        .with_edge("E2F1", "MYC", 0.62)
        .with_directed()
        .with_labels();

    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gene Regulatory Network");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/directed.svg"), svg).unwrap();
}

// ── Grouped nodes with legend and circle layout ───────────────────────────

fn grouped() {
    let net = NetworkPlot::new()
        .with_edge("TP53", "MDM2", 0.95)
        .with_edge("TP53", "BAX", 0.82)
        .with_edge("MDM2", "TP53", 0.88)
        .with_edge("BRCA1", "TP53", 0.65)
        .with_edge("BRCA1", "RAD51", 0.72)
        .with_edge("RAD51", "BRCA2", 0.68)
        .with_edge("MYC", "CCND1", 0.91)
        .with_edge("CCND1", "CDK4", 0.83)
        .with_edge("CDK4", "RB1", 0.79)
        .with_edge("RB1", "E2F1", 0.86)
        .with_edge("E2F1", "MYC", 0.62)
        .with_edge("TP53", "RB1", 0.45)
        .with_node_group("TP53", "DNA damage")
        .with_node_group("MDM2", "DNA damage")
        .with_node_group("BAX", "DNA damage")
        .with_node_group("BRCA1", "DNA repair")
        .with_node_group("RAD51", "DNA repair")
        .with_node_group("BRCA2", "DNA repair")
        .with_node_group("MYC", "Cell cycle")
        .with_node_group("CCND1", "Cell cycle")
        .with_node_group("CDK4", "Cell cycle")
        .with_node_group("RB1", "Cell cycle")
        .with_node_group("E2F1", "Cell cycle")
        .with_layout(NetworkLayout::Circle)
        .with_labels()
        .with_legend("Pathway");

    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gene Network by Pathway");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/grouped.svg"), svg).unwrap();
}
