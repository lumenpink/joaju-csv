#[test]
fn test_rust_version_constant() {
    assert!(
        !joaju_csv::VERSION.is_empty(),
        "VERSION const cannot be empty"
    );
    assert!(
        joaju_csv::VERSION.starts_with("0.") || joaju_csv::VERSION.starts_with("1."),
        "VERSION should start with SemVer major digit"
    );
}

#[test]
fn test_ci_workflow_canonical_publish_and_no_legacy_daemon() {
    let ci_yaml = std::fs::read_to_string(".forgejo/workflows/ci.yml").expect("ci.yml must exist");
    assert!(
        !ci_yaml.contains("codeberg.org"),
        "Codeberg dispatch must be completely removed"
    );
    assert!(
        !ci_yaml.contains("4040"),
        "CI workflow must not contain legacy daemon dispatch on port 4040 (handled via Forgejo webhook -> Gossnab)"
    );
    assert!(
        ci_yaml.contains("cargo publish --registry lumen"),
        "CI workflow must publish to lumen registry"
    );
}

#[test]
fn test_ci_workflow_no_duplicate_registries() {
    let ci_yaml = std::fs::read_to_string(".forgejo/workflows/ci.yml").expect("ci.yml must exist");
    assert!(
        !ci_yaml.contains("[registries.gambi]"),
        "CI workflow must not configure duplicate gambi registry when lumen registry is configured"
    );
    assert!(
        ci_yaml.contains("[registries.lumen]"),
        "CI workflow must configure lumen registry"
    );
}
