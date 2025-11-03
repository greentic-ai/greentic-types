#![cfg(feature = "serde")]

use greentic_types::pack_spec::PackSpec;

#[test]
fn pack_spec_roundtrip() {
    let doc = r#"
id: greentic.weather.demo
version: 0.1.0
flow_files:
  - flows/weather_bot.ygtc
template_dirs:
  - templates/
imports_required: [secrets.get, telemetry.emit]
tools:
  - name: weather_api
    source: embedded
    path: tools/weatherapi.wasm
    actions: [forecast_weather]
"#;
    let spec: PackSpec = serde_yaml_bw::from_str(doc).expect("valid pack spec");
    assert_eq!(spec.id, "greentic.weather.demo");
    assert_eq!(spec.version, "0.1.0");
    assert!(
        spec.flow_files
            .iter()
            .any(|f| f.ends_with("weather_bot.ygtc"))
    );
    assert!(spec.template_dirs.iter().any(|d| d == "templates/"));

    let serialized = serde_yaml_bw::to_string(&spec).expect("serialize");
    let roundtrip: PackSpec = serde_yaml_bw::from_str(&serialized).expect("roundtrip");
    assert_eq!(spec, roundtrip);
}

#[test]
fn pack_spec_defaults_on_missing_fields() {
    let doc = r#"
id: demo.x
version: 0.0.1
"#;
    let spec: PackSpec = serde_yaml_bw::from_str(doc).expect("valid pack spec");
    assert!(spec.flow_files.is_empty());
    assert!(spec.template_dirs.is_empty());
    assert!(spec.imports_required.is_empty());
    assert!(spec.tools.is_empty());
}
