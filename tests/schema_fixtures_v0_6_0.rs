use std::collections::BTreeSet;

use greentic_types::cbor::canonical;
use greentic_types::schemas::component::v0_6_0::{ComponentQaSpec, QaMode as ComponentQaMode};
use greentic_types::schemas::pack::v0_6_0::{PackDescribe, PackQaSpec, QaMode as PackQaMode};
use greentic_types::{CborBytes, Envelope, I18nText};

#[test]
fn pack_describe_fixture_roundtrip() {
    let bytes = include_bytes!("../fixtures/pack/describe_v0_6_0.cbor");
    let cbor = CborBytes::new(bytes.to_vec());
    let spec = cbor.decode::<PackDescribe>().expect("decode pack describe");
    let encoded = canonical::to_canonical_cbor(&spec).expect("encode pack describe");
    assert_eq!(bytes.as_slice(), encoded.as_slice());
}

#[test]
fn pack_qa_fixture_roundtrip() {
    let bytes = include_bytes!("../fixtures/pack/qa_setup_v0_6_0.cbor");
    let cbor = CborBytes::new(bytes.to_vec());
    let spec = cbor.decode::<PackQaSpec>().expect("decode pack qa");
    let encoded = canonical::to_canonical_cbor(&spec).expect("encode pack qa");
    assert_eq!(bytes.as_slice(), encoded.as_slice());
    assert_eq!(spec.mode, PackQaMode::Setup);
}

#[test]
fn component_qa_fixture_roundtrip() {
    let bytes = include_bytes!("../fixtures/component/qa_default_v0_6_0.cbor");
    let cbor = CborBytes::new(bytes.to_vec());
    let spec = cbor
        .decode::<ComponentQaSpec>()
        .expect("decode component qa");
    let encoded = canonical::to_canonical_cbor(&spec).expect("encode component qa");
    assert_eq!(bytes.as_slice(), encoded.as_slice());
    assert_eq!(spec.mode, ComponentQaMode::Default);
}

#[test]
fn envelope_roundtrip() {
    let spec = ComponentQaSpec {
        mode: ComponentQaMode::Default,
        title: I18nText::new("component.qa.default.title", Some("Default".to_string())),
        description: None,
        questions: Vec::new(),
        defaults: Default::default(),
    };

    let envelope =
        Envelope::new("component", "greentic.component.qa@0.6.0", 6, &spec).expect("envelope");
    let decoded: ComponentQaSpec = envelope.decode_body().expect("decode envelope");
    assert_eq!(decoded.mode, ComponentQaMode::Default);
    envelope.ensure_canonical().expect("canonical body");
}

#[test]
fn qa_i18n_keys_collects_expected_set() {
    let spec = ComponentQaSpec {
        mode: ComponentQaMode::Default,
        title: I18nText::new("component.qa.default.title", Some("Default".to_string())),
        description: Some(I18nText::new(
            "component.qa.default.description",
            Some("Defaults".to_string()),
        )),
        questions: vec![],
        defaults: Default::default(),
    };

    let keys = spec.i18n_keys();
    let expected: BTreeSet<String> = [
        "component.qa.default.title",
        "component.qa.default.description",
    ]
    .into_iter()
    .map(|value| value.to_string())
    .collect();

    assert_eq!(keys, expected);
}
