use greentic_types::{
    ComponentCapabilities, ComponentManifest, ComponentProfiles, Flow, FlowKind, FlowNodes,
    FlowValidationError, Node,
};
use semver::Version;
use serde_json::Value;

fn sample_manifest(id: &str, supports: &[FlowKind]) -> ComponentManifest {
    ComponentManifest {
        id: id.parse().unwrap(),
        version: Version::parse("1.0.0").unwrap(),
        supports: supports.to_vec(),
        world: "test:component@1.0.0".into(),
        profiles: ComponentProfiles {
            default: Some("default".into()),
            supported: vec!["default".into(), "advanced".into()],
        },
        capabilities: ComponentCapabilities::default(),
        configurators: None,
    }
}

fn flow_with_node(component: &str, kind: FlowKind) -> Flow {
    let mut nodes: FlowNodes = FlowNodes::default();
    nodes.insert(
        "ingress".parse().unwrap(),
        Node {
            kind: "component-kind".into(),
            profile: None,
            component: Some(component.parse().unwrap()),
            config: Value::Null,
            routing: Value::Null,
        },
    );
    Flow {
        kind,
        id: format!("{}.flow", component).parse().unwrap(),
        description: None,
        nodes,
    }
}

#[test]
fn ingress_respects_insertion_order() {
    let mut nodes = FlowNodes::default();
    nodes.insert(
        "first".parse().unwrap(),
        Node {
            kind: "kind-1".into(),
            profile: None,
            component: None,
            config: Value::Null,
            routing: Value::Null,
        },
    );
    nodes.insert(
        "second".parse().unwrap(),
        Node {
            kind: "kind-2".into(),
            profile: None,
            component: None,
            config: Value::Null,
            routing: Value::Null,
        },
    );

    let flow = Flow {
        kind: FlowKind::Messaging,
        id: "flow.demo".parse().unwrap(),
        description: None,
        nodes,
    };

    let ingress = flow.ingress().expect("ingress");
    assert_eq!(ingress.0.as_str(), "first");
}

#[test]
fn validate_components_detects_missing_manifest() {
    let flow = flow_with_node("component.missing", FlowKind::Messaging);
    let err = flow.validate_components(|_| None).expect_err("should fail");
    assert!(matches!(err, FlowValidationError::MissingComponent { .. }));
}

#[test]
fn validate_components_checks_supports() {
    let flow = flow_with_node("component.a", FlowKind::Messaging);
    let manifest = sample_manifest("component.a", &[FlowKind::Events]);

    let err = flow
        .validate_components(|cid| {
            if cid.as_str() == "component.a" {
                Some(&manifest)
            } else {
                None
            }
        })
        .expect_err("unsupported flow kind should fail");

    assert!(matches!(
        err,
        FlowValidationError::UnsupportedComponent { .. }
    ));
}

#[test]
fn component_profile_selection() {
    let manifest = sample_manifest("component.profile", &[FlowKind::Messaging]);

    let default = manifest.select_profile(None).expect("default");
    assert_eq!(default, Some("default"));

    let explicit = manifest.select_profile(Some("advanced")).expect("advanced");
    assert_eq!(explicit, Some("advanced"));

    let err = manifest
        .select_profile(Some("unknown"))
        .expect_err("should fail");
    assert!(matches!(
        err,
        greentic_types::ComponentProfileError::UnsupportedProfile { .. }
    ));
}
