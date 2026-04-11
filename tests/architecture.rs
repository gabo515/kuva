//! Architectural boundary tests.
//!
//! Ensures the one-way dependency rule `plot → render` is not violated:
//! plot modules must never import from the render layer.

use std::fs;
use std::path::Path;

/// Known violations that predate this test.  Each entry is a filename
/// (relative to `src/plot/`) paired with the violating import fragment.
/// Shrink this list over time — never grow it.
const ALLOWED: &[(&str, &str)] = &[
    ("manhattan.rs", "crate::render::palette::Palette"),
    ("mosaic.rs", "crate::render::palette::Palette"),
    ("sankey.rs", "crate::render::layout::TickFormat"),
    ("venn.rs", "crate::render::palette::Palette"),
    ("parallel.rs", "crate::render::palette::Palette"),
    ("phylo.rs", "crate::render::render_utils::upgma"),
    ("phylo.rs", "crate::render::render_utils::linkage_to_nodes"),
];

fn is_allowed(file_name: &str, line: &str) -> bool {
    ALLOWED
        .iter()
        .any(|(f, import)| file_name == *f && line.contains(import))
}

#[test]
fn plot_modules_must_not_import_from_render() {
    let plot_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/plot");
    let mut violations = Vec::new();

    for entry in fs::read_dir(&plot_dir).expect("cannot read src/plot/") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }

        let file_name = path.file_name().unwrap().to_str().unwrap();
        let contents = fs::read_to_string(&path).unwrap();

        for (line_no, line) in contents.lines().enumerate() {
            let trimmed = line.trim();

            // Skip doc comments and regular comments
            if trimmed.starts_with("///") || trimmed.starts_with("//") {
                continue;
            }

            if trimmed.contains("crate::render") && !is_allowed(file_name, trimmed) {
                violations.push(format!(
                    "  src/plot/{}:{}: {}",
                    file_name,
                    line_no + 1,
                    trimmed,
                ));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "\nArchitecture violation: plot modules must not import from render.\n\
         The dependency flows Plot → Render, never backwards.\n\
         Config/data types needed by both layers belong in src/plot/.\n\n{}\n",
        violations.join("\n"),
    );
}
