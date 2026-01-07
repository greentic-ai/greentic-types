#![cfg(feature = "serde")]

use greentic_types::pack::extensions::component_manifests::{
    ComponentManifestIndexEntryV1, ComponentManifestIndexV1, EXT_COMPONENT_MANIFEST_INDEX_V1,
    ManifestEncoding, decode_component_manifest_index_v1_from_cbor_bytes,
    encode_component_manifest_index_v1_to_cbor_bytes,
};
use serde_json::json;

#[test]
fn component_manifest_extension_id_is_stable() {
    assert_eq!(
        EXT_COMPONENT_MANIFEST_INDEX_V1,
        "greentic.pack.component_manifests@v1"
    );
}

#[test]
fn component_manifest_index_roundtrips_and_matches_expected_shape() {
    let entries = vec![
        ComponentManifestIndexEntryV1 {
            component_id: "vendor.search".into(),
            manifest_file: "vendor.search.manifest.cbor".into(),
            encoding: ManifestEncoding::Cbor,
            content_hash: Some(
                "sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789".into(),
            ),
        },
        ComponentManifestIndexEntryV1 {
            component_id: "vendor.cache".into(),
            manifest_file: "vendor.cache.manifest.cbor".into(),
            encoding: ManifestEncoding::Cbor,
            content_hash: None,
        },
    ];
    let index = ComponentManifestIndexV1::new(entries);

    let value = index.to_extension_value().expect("to extension value");
    let expected = json!({
        "schema_version": 1,
        "entries": [
            {
                "component_id": "vendor.search",
                "manifest_file": "vendor.search.manifest.cbor",
                "encoding": "cbor",
                "content_hash": "sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
            },
            {
                "component_id": "vendor.cache",
                "manifest_file": "vendor.cache.manifest.cbor",
                "encoding": "cbor"
            }
        ]
    });
    assert_eq!(value, expected);

    let from_value =
        ComponentManifestIndexV1::from_extension_value(&value).expect("from extension value");
    assert_eq!(from_value, index);

    let cbor = encode_component_manifest_index_v1_to_cbor_bytes(&index).expect("encode cbor");
    let decoded = decode_component_manifest_index_v1_from_cbor_bytes(&cbor).expect("decode cbor");
    assert_eq!(decoded, index);
}
