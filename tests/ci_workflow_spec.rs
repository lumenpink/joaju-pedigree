//! Test ensuring CI workflow is configured for sovereign registry and stable uploads.

#[test]
fn test_ci_workflow_contains_gambi_registry_and_http1() {
    let ci_yaml =
        std::fs::read_to_string(".forgejo/workflows/ci.yaml").expect("ci.yaml must exist");
    assert!(ci_yaml.contains("[registries.gambi]"));
    assert!(ci_yaml.contains("--http1.1"));
}
