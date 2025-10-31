#![cfg(feature = "schemars")]

use greentic_types::{Outcome, PackRef, SpanContext, TenantCtx};
use schemars::{
    schema::{RootSchema, SchemaObject},
    schema_for,
};

fn definition_names(schema: &RootSchema) -> Vec<String> {
    schema.definitions.keys().cloned().collect()
}

#[test]
fn tenant_context_schema_registered() {
    let schema = schema_for!(TenantCtx);
    assert!(
        schema.schema.object.is_some(),
        "TenantCtx root schema should be an object"
    );
    let defs = definition_names(&schema);
    assert!(
        defs.iter().any(|name| name.contains("Impersonation")),
        "Impersonation definition missing: {defs:?}"
    );
}

#[test]
fn span_context_schema_has_object() {
    let schema = schema_for!(SpanContext);
    assert!(
        schema.schema.object.is_some(),
        "SpanContext schema should be an object"
    );
}

#[test]
fn pack_schema_includes_signature() {
    let pack_schema = schema_for!(PackRef);
    let defs = definition_names(&pack_schema);
    assert!(
        defs.iter().any(|name| name.contains("Signature")),
        "Signature definition missing: {defs:?}"
    );
}

#[test]
fn outcome_schema_enumerates_variants() {
    let root = schema_for!(Outcome<String>);
    let SchemaObject { subschemas, .. } = root.schema;
    let variants = subschemas
        .as_ref()
        .and_then(|subs| subs.one_of.as_ref())
        .map(|list| list.len())
        .unwrap_or_default();
    assert!(variants >= 3, "Outcome schema should declare variants");
}
