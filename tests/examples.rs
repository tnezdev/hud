use std::fs;

use hud::config::HudConfig;

/// Discover every committed `examples/*.toml` and verify each parses
/// successfully with the real config loader. A broken example config
/// will fail CI.
#[test]
fn committed_examples_parse_successfully() {
    let entries = fs::read_dir("examples")
        .expect("examples/ directory should exist")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "toml") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert!(
        !entries.is_empty(),
        "expected at least one examples/*.toml file"
    );

    for path in &entries {
        let result = HudConfig::load_from_path(path);
        assert!(
            result.is_ok(),
            "example config {} should parse: {:?}",
            path.display(),
            result.err()
        );
    }
}
