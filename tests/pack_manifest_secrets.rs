#![cfg(feature = "serde")]

use greentic_types::PackManifest;

#[test]
fn deserializes_without_secret_requirements() {
    let manifest: PackManifest =
        serde_json::from_str(include_str!("fixtures/pack_manifest_without_secrets.json"))
            .expect("fixture deserializes");

    assert!(
        manifest.secret_requirements.is_empty(),
        "missing field should default to empty vec"
    );

    let value = serde_json::to_value(&manifest).expect("serialize");
    assert!(
        value.get("secret_requirements").is_none(),
        "empty requirements should be skipped on serialization"
    );
}

#[test]
fn roundtrips_manifest_with_secret_requirements() {
    let manifest: PackManifest = serde_json::from_str(include_str!(
        "fixtures/pack_manifest_with_secret_requirements.json"
    ))
    .expect("fixture deserializes");

    let requirement = manifest
        .secret_requirements
        .first()
        .expect("one requirement");
    assert_eq!(requirement.key.as_str(), "TEST_API_KEY");
    assert_eq!(
        requirement.scope.as_ref().map(|scope| scope.env.as_str()),
        Some("staging")
    );
    assert!(requirement.required, "required defaults to true");

    let value = serde_json::to_value(&manifest).expect("serialize");
    let secrets = value
        .get("secret_requirements")
        .and_then(|value| value.as_array())
        .expect("secret_requirements should serialize when non-empty");
    assert_eq!(secrets.len(), 1);
    assert_eq!(
        secrets[0].get("key"),
        Some(&serde_json::Value::String("TEST_API_KEY".into()))
    );
}
