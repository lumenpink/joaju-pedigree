//! Test ensuring CI workflow is configured for sovereign registry and stable uploads.

#[test]
fn test_ci_workflow_contains_gambi_registry_and_http1() {
    let ci_yaml =
        std::fs::read_to_string(".forgejo/workflows/ci.yaml").expect("ci.yaml must exist");
    assert!(ci_yaml.contains("[registries.gambi]"));
    assert!(ci_yaml.contains("--http1.1"));
    assert!(
        ci_yaml.contains("REGISTRY_TOKEN"),
        "CI workflow must support REGISTRY_TOKEN"
    );
    assert!(
        ci_yaml.contains("GAMBI_2LP_IN_TOKEN"),
        "CI workflow must support GAMBI_2LP_IN_TOKEN"
    );
    assert!(
        !ci_yaml.contains("[registries.lumen]"),
        "Do not declare duplicate registry with same URL to avoid cargo collision"
    );
    assert!(
        !ci_yaml.contains("libjoaju_pedigree-x86_64-linux.a\" || true"),
        "Upload must fail strictly without masking errors with '|| true'"
    );
}
