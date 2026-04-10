use kuva::plot::network::{NetworkPlot, NetworkLayout};
use kuva::render::{plots::Plot, layout::Layout, render::render_multiple};
use kuva::backend::svg::SvgBackend;

#[test]
fn network_basic() {
    let net = NetworkPlot::new()
        .with_edge("A", "B", 1.0)
        .with_edge("A", "C", 1.0)
        .with_edge("B", "C", 1.0)
        .with_edge("C", "D", 1.0)
        .with_labels();
    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Basic Network");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/network_basic.svg", svg).unwrap();
}

#[test]
fn network_directed() {
    let net = NetworkPlot::new()
        .with_edge("A", "B", 1.0)
        .with_edge("A", "C", 2.0)
        .with_edge("B", "C", 1.5)
        .with_edge("C", "D", 3.0)
        .with_edge("D", "A", 0.5)
        .with_directed()
        .with_labels();
    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Directed Network");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/network_directed.svg", svg).unwrap();
}

#[test]
fn network_circle_layout() {
    let net = NetworkPlot::new()
        .with_edges([
            ("A", "B", 1.0), ("B", "C", 1.0), ("C", "D", 1.0),
            ("D", "E", 1.0), ("E", "F", 1.0), ("F", "A", 1.0),
            ("A", "D", 0.5), ("B", "E", 0.5), ("C", "F", 0.5),
        ])
        .with_layout(NetworkLayout::Circle)
        .with_labels();
    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Circle Layout");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/network_circle.svg", svg).unwrap();
}

#[test]
fn network_self_loop() {
    let net = NetworkPlot::new()
        .with_edge("A", "B", 1.0)
        .with_edge("B", "C", 1.0)
        .with_edge("C", "C", 1.0) // self-loop
        .with_edge("C", "A", 1.0)
        .with_directed()
        .with_labels();
    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Self-Loop");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/network_self_loop.svg", svg).unwrap();
}

#[test]
fn network_matrix() {
    let matrix = vec![
        vec![0.0, 1.0, 1.0, 0.0],
        vec![1.0, 0.0, 1.0, 1.0],
        vec![1.0, 1.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0, 0.0],
    ];
    let net = NetworkPlot::new()
        .with_matrix(matrix, ["Alpha", "Beta", "Gamma", "Delta"])
        .with_labels();
    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("From Adjacency Matrix");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/network_matrix.svg", svg).unwrap();
}

#[test]
fn network_groups_legend() {
    let net = NetworkPlot::new()
        .with_edge("A", "B", 1.0)
        .with_edge("A", "C", 1.0)
        .with_edge("B", "D", 1.0)
        .with_edge("C", "D", 1.0)
        .with_node_group("A", "Input")
        .with_node_group("B", "Hidden")
        .with_node_group("C", "Hidden")
        .with_node_group("D", "Output")
        .with_labels()
        .with_legend("Layer");
    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Grouped Network");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/network_groups_legend.svg", svg).unwrap();
}

#[test]
fn network_weighted() {
    let net = NetworkPlot::new()
        .with_edge("A", "B", 1.0)
        .with_edge("A", "C", 5.0)
        .with_edge("B", "C", 2.0)
        .with_edge("C", "D", 10.0)
        .with_edge("D", "E", 0.5)
        .with_labels();
    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Weighted Edges");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/network_weighted.svg", svg).unwrap();
}

#[test]
fn network_node_sizes() {
    let net = NetworkPlot::new()
        .with_edge("Hub", "A", 1.0)
        .with_edge("Hub", "B", 1.0)
        .with_edge("Hub", "C", 1.0)
        .with_edge("Hub", "D", 1.0)
        .with_edge("A", "B", 1.0)
        .with_node_size("Hub", 20.0)
        .with_node_size("A", 12.0)
        .with_node_size("B", 8.0)
        .with_node_size("C", 5.0)
        .with_node_size("D", 3.0)
        .with_labels();
    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Variable Node Sizes");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/network_node_sizes.svg", svg).unwrap();
}

#[test]
fn network_disconnected() {
    // Three separate connected components with no edges between them.
    let net = NetworkPlot::new()
        // Component 1: triangle
        .with_edge("A1", "A2", 1.0)
        .with_edge("A2", "A3", 1.0)
        .with_edge("A3", "A1", 1.0)
        // Component 2: pair
        .with_edge("B1", "B2", 1.0)
        // Component 3: star
        .with_edge("C1", "C2", 1.0)
        .with_edge("C1", "C3", 1.0)
        .with_edge("C1", "C4", 1.0)
        .with_edge("C1", "C5", 1.0)
        .with_node_group("A1", "Alpha")
        .with_node_group("A2", "Alpha")
        .with_node_group("A3", "Alpha")
        .with_node_group("B1", "Beta")
        .with_node_group("B2", "Beta")
        .with_node_group("C1", "Gamma")
        .with_node_group("C2", "Gamma")
        .with_node_group("C3", "Gamma")
        .with_node_group("C4", "Gamma")
        .with_node_group("C5", "Gamma")
        .with_labels()
        .with_legend("Component");
    let plots = vec![Plot::Network(net)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Disconnected Components");
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("test_outputs/network_disconnected.svg", svg).unwrap();
}
