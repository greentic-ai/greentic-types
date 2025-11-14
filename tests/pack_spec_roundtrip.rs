#![cfg(feature = "serde")]

use greentic_types::{
    ComponentManifest, Flow, FlowKind, PackComponentRef, PackFlowRef, PackManifest,
};

fn roundtrip_yaml_json<T>(doc: &str) -> T
where
    T: serde::de::DeserializeOwned + serde::Serialize + PartialEq + core::fmt::Debug,
{
    let value: T = serde_yaml_bw::from_str(doc).expect("valid yaml");
    let json = serde_json::to_string_pretty(&value).expect("serialize json");
    let roundtrip: T = serde_json::from_str(&json).expect("json roundtrip");
    assert_eq!(value, roundtrip);
    roundtrip
}

#[test]
fn flow_roundtrip_yaml_and_json() {
    let doc = r#"
kind: messaging
id: demo.messaging.flow
description: Sample messaging flow
nodes:
  ingress:
    kind: component-kind-1
    component: vendor.component.one
    profile: router
    config:
      greeting: "Hello"
    routing:
      default: handler
  handler:
    kind: component-kind-2
    component: vendor.component.two
    config:
      reply: "Ack"
    routing:
      default: finish
  finish:
    kind: messaging/reply
    config:
      text: "Done"
    routing: {}
"#;

    let flow: Flow = roundtrip_yaml_json(doc);
    assert_eq!(flow.kind, FlowKind::Messaging);
    assert_eq!(flow.id.as_str(), "demo.messaging.flow");
    assert_eq!(flow.nodes.len(), 3);
    let ingress: greentic_types::NodeId = "ingress".parse().unwrap();
    assert!(flow.nodes.contains_key(&ingress));
}

#[test]
fn component_manifest_roundtrip_yaml_and_json() {
    let doc = r#"
id: vendor.component.qa
version: 1.2.3
supports:
  - messaging
world: "vendor:qa@1.0.0"
profiles:
  default: stateless
  supported:
    - stateless
    - cached
capabilities:
  wasi:
    random: true
    clocks: true
    filesystem:
      mode: sandbox
      mounts:
        - name: scratch
          host_class: scratch
          guest_path: /tmp
    env:
      allow:
        - RUST_LOG
  host:
    secrets:
      required:
        - API_TOKEN
    messaging:
      inbound: true
      outbound: true
    telemetry:
      scope: tenant
configurators:
  basic: configure_component_basic
  full: configure_component_full
"#;

    let manifest: ComponentManifest = roundtrip_yaml_json(doc);
    assert_eq!(manifest.id.as_str(), "vendor.component.qa");
    assert_eq!(manifest.version.to_string(), "1.2.3");
    assert_eq!(manifest.profiles.default, Some("stateless".into()));
    assert!(manifest.capabilities.host.telemetry.as_ref().is_some());
}

#[test]
fn pack_manifest_roundtrip_yaml_and_json() {
    let doc = r#"
id: vendor.demo.pack
version: 0.1.0
name: "Demo Pack"
flows:
  - id: demo.messaging.flow
    file: flows/messaging.ygtc
components:
  - id: vendor.component.qa
    version_req: "^1.2"
    source: "oci://registry/components"
profiles:
  messaging:
    defaults:
      handler: stateless
component_sources:
  registry: "greentic-store"
connectors:
  messaging:
    teams:
      flow: demo.messaging.flow
"#;

    let manifest: PackManifest = roundtrip_yaml_json(doc);
    assert_eq!(manifest.id.as_str(), "vendor.demo.pack");
    assert_eq!(manifest.version.to_string(), "0.1.0");
    assert_eq!(manifest.flows.len(), 1);
    assert_eq!(manifest.components.len(), 1);

    let flow_ref = PackFlowRef {
        id: "demo.messaging.flow".parse().unwrap(),
        file: "flows/messaging.ygtc".into(),
    };
    assert_eq!(manifest.flows[0], flow_ref);

    let component_ref = PackComponentRef {
        id: "vendor.component.qa".parse().unwrap(),
        version_req: "^1.2".parse().unwrap(),
        source: Some("oci://registry/components".into()),
    };
    assert_eq!(manifest.components[0], component_ref);
}
