use greentic_types::adapters::component_v0_5_0_to_v0_6_0::adapt_component_qa_spec;
use greentic_types::schemas::component::v0_5_0::LegacyComponentQaSpec;
use greentic_types::schemas::component::v0_6_0::{ComponentQaSpec, QaMode};

#[test]
fn adapts_legacy_component_qa() {
    let legacy: LegacyComponentQaSpec =
        serde_json::from_str(include_str!("../fixtures/legacy/component_v0_5_0_qa.json"))
            .expect("legacy fixture");

    let bytes = adapt_component_qa_spec(QaMode::Setup, &legacy).expect("adapt");
    let spec = bytes.decode::<ComponentQaSpec>().expect("decode");

    assert_eq!(spec.mode, QaMode::Setup);
    assert_eq!(spec.title.key, "legacy.component.v0_5_0.title");
    assert_eq!(spec.title.fallback.as_deref(), Some("Setup"));
    assert_eq!(
        spec.description.as_ref().unwrap().key,
        "legacy.component.v0_5_0.description"
    );
    assert_eq!(
        spec.description.as_ref().unwrap().fallback.as_deref(),
        Some("legacy adapter test fixture (approximation)")
    );

    let api = &spec.questions[0];
    assert_eq!(api.id, "api_key");
    assert_eq!(api.label.key, "legacy.component.v0_5_0.api_key.label");
    assert_eq!(api.label.fallback.as_deref(), Some("API key"));

    let region = &spec.questions[1];
    assert_eq!(region.id, "region");
    assert_eq!(region.label.key, "legacy.component.v0_5_0.region.label");
    assert_eq!(region.label.fallback.as_deref(), Some("Region"));

    let keys = spec.i18n_keys();
    assert!(keys.contains("legacy.component.v0_5_0.title"));
    assert!(keys.contains("legacy.component.v0_5_0.description"));
    assert!(keys.contains("legacy.component.v0_5_0.api_key.label"));
    assert!(keys.contains("legacy.component.v0_5_0.region.label"));
    assert!(keys.contains("legacy.component.v0_5_0.region.option.eu"));
    assert!(keys.contains("legacy.component.v0_5_0.region.option.us"));

    let bytes_roundtrip = bytes.clone().canonicalize().expect("canonical");
    assert_eq!(bytes.as_slice(), bytes_roundtrip.as_slice());
}
