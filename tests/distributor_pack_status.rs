#![cfg(feature = "serde")]

use greentic_types::PackStatusResponseV2;

#[test]
fn pack_status_v2_defaults_secret_requirements_to_none() {
    let status: PackStatusResponseV2 = serde_json::from_str(include_str!(
        "fixtures/pack_status_response_v2_without_secret_requirements.json"
    ))
    .expect("fixture deserializes");

    assert!(status.secret_requirements.is_none());

    let value = serde_json::to_value(&status).expect("serialize");
    assert!(
        value.get("secret_requirements").is_none(),
        "empty option should be skipped"
    );
}

#[test]
fn pack_status_v2_includes_secret_requirements_when_present() {
    let status: PackStatusResponseV2 = serde_json::from_str(include_str!(
        "fixtures/pack_status_response_v2_with_secret_requirements.json"
    ))
    .expect("fixture deserializes");

    let reqs = status
        .secret_requirements
        .as_ref()
        .expect("secret requirements present");
    assert_eq!(reqs.len(), 1);
    assert_eq!(reqs[0].key.as_str(), "TEST_API_KEY");

    let value = serde_json::to_value(&status).expect("serialize");
    assert!(
        value.get("secret_requirements").is_some(),
        "non-empty option should serialize"
    );
}
