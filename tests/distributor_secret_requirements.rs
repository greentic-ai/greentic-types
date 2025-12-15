#![cfg(feature = "serde")]

use greentic_types::{ResolveComponentResponse, SecretRequirement};

#[test]
fn resolve_response_defaults_missing_secret_requirements_to_none() {
    let resp: ResolveComponentResponse = serde_json::from_str(include_str!(
        "fixtures/resolve_component_response_without_secret_requirements.json"
    ))
    .expect("fixture deserializes");

    assert!(resp.secret_requirements.is_none());

    let value = serde_json::to_value(&resp).expect("serialize");
    assert!(
        value.get("secret_requirements").is_none(),
        "empty option should be skipped"
    );
}

#[test]
fn resolve_response_includes_secret_requirements_when_present() {
    let resp: ResolveComponentResponse = serde_json::from_str(include_str!(
        "fixtures/resolve_component_response_with_secret_requirements.json"
    ))
    .expect("fixture deserializes");

    let reqs = resp
        .secret_requirements
        .as_ref()
        .expect("secret requirements present");
    assert_eq!(reqs.len(), 1);

    let requirement: &SecretRequirement = &reqs[0];
    assert_eq!(requirement.key.as_str(), "TEST_API_KEY");
    assert_eq!(
        requirement.scope.as_ref().map(|scope| scope.env.as_str()),
        Some("prod")
    );

    let serialized = serde_json::to_value(&resp).expect("serialize");
    assert!(
        serialized.get("secret_requirements").is_some(),
        "non-empty option should serialize"
    );
}
